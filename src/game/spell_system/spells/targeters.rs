use std::f32::consts::PI;
use std::slice::Iter;
use std::sync::Arc;

use avian2d::collision::Collider;
use avian2d::prelude::{LinearVelocity, SpatialQuery, SpatialQueryFilter};
use bevy::app::{App, Update};
use bevy::log::info;
use bevy::math::{Quat, Vec3Swizzles};
use bevy::prelude::{
    Component, Entity, GlobalTransform, IntoSystemConfigs, Query, Transform, With, World,
};
use log::warn;

use crate::game::physics::GameLayer;
use crate::game::projectiles::ProjectileDamage;
use crate::game::spawn::player::Player;
use crate::game::spell_system::casting::SpellCastContext;
use crate::game::spell_system::{SpellComponent, SpellData, SpellEffect, SpellModifier};
use crate::game::Damageable;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, do_homing.in_set(AppSet::Update));
}

pub(super) fn get_spells() -> Vec<(SpellComponent, i32)> {
    vec![
        (
            SpellComponent {
                data: Box::new(HomingData {
                    homing_range: 100.0,
                    homing_rate: 0.1,
                    target: HomingTarget::ClosestEnemy,
                }),
                icon_id: 36,
            },
            15,
        ),
        // (
        //     SpellComponent {
        //         data: Box::new(HomingData {
        //             homing_range: 50.0,
        //             homing_rate: 0.99,
        //             target: HomingTarget::PlayerOrbit,
        //         }),
        //         icon_id: 38,
        //     },
        //     15,
        // ),
    ]
}

// spell components that change a spells direction either at cast or during flight

// 1. AutoAim
// 2. Homing
// 4. Bounce
// 5. Orbit

#[derive(Clone, Debug, Copy)]
pub enum HomingTarget {
    ClosestEnemy,
    #[allow(dead_code)]
    PlayerOrbit,
}
impl HomingTarget {
    pub fn get_name(&self) -> String {
        match self {
            HomingTarget::ClosestEnemy => "Enemies".to_string(),
            HomingTarget::PlayerOrbit => "Player".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct HomingData {
    pub homing_range: f32,
    pub homing_rate: f32,
    pub target: HomingTarget,
}
impl SpellData for HomingData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        let spell = iter.next()?.data.build(iter)?;
        Some(Arc::new(Homing {
            homing_range: self.homing_range,
            homing_rate: self.homing_rate,
            target: self.target,
            spell,
        }))
    }

    fn get_name(&self) -> String {
        "Homing: ".to_string() + &*self.target.get_name()
    }

    fn get_desc(&self) -> String {
        "Causes the spell to home in on: ".to_string() + &*self.target.get_name()
    }
}

#[derive(Debug, Clone)]
pub struct Homing {
    pub homing_range: f32,
    pub homing_rate: f32,
    pub target: HomingTarget,
    pub spell: Arc<dyn SpellEffect>,
}
impl SpellEffect for Homing {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let range = self.homing_range;
        let rate = self.homing_rate;
        let target = self.target;
        let modifier: SpellModifier = Box::new(move |e: Entity, mod_world: &mut World| {
            //spawn a homing child component on the projectile
            mod_world.entity_mut(e).insert(HomingComponent {
                rate,
                range,
                target,
            });
        });

        context.add_modifier("Homing Mod", modifier);
        info!("Cast Homing Mod");
        self.spell.cast(context, world);
    }
}

// homing component to be added to projectiles by the homing spell effect
#[derive(Component)]
pub struct HomingComponent {
    pub rate: f32,  // used for the lerp rate
    pub range: f32, // used for the collider range
    pub target: HomingTarget,
}

//system to take an objects linear velocity, that has a child homing component with a collider
// use that collider to find the nearest enemy within the homing range and lerp the velocity towards that enemy
pub fn do_homing(
    spatial_query: SpatialQuery,
    mut q_projectiles: Query<(
        &HomingComponent,
        &GlobalTransform,
        &mut Transform,
        &ProjectileDamage,
        &mut LinearVelocity,
    )>,
    q_targets: Query<(&GlobalTransform, &Damageable)>,
    q_player: Query<Entity, With<Player>>,
) {
    for (homing, p_gtr, mut p_tr, p_dm, mut p_lv) in q_projectiles.iter_mut() {
        let p_translation = p_gtr.translation();
        let p_team = &p_dm.team;

        let (target, distance): (Option<Entity>, f32) = match homing.target {
            HomingTarget::ClosestEnemy => {
                let near_enemies = spatial_query.shape_intersections(
                    &Collider::circle(homing.range),
                    p_translation.xy(),
                    0.,
                    SpatialQueryFilter::from_mask(GameLayer::Enemy),
                );

                let target = near_enemies
                    .iter()
                    .fold((f32::INFINITY, None), |(dist, prev), e| {
                        // get target entity
                        let Ok((tr, dm)) = q_targets.get(*e) else {
                            warn!("Failed to get target from homing child");
                            return (f32::MAX, prev);
                        };

                        //check target entity can be damaged
                        if dm.team == *p_team {
                            return (dist, prev);
                        }

                        let new_dist = p_translation.distance(tr.translation());

                        if new_dist < dist {
                            (new_dist, Some(*e))
                        } else {
                            (dist, prev)
                        }
                    });

                (target.1, 0.)
            }
            HomingTarget::PlayerOrbit => match q_player.get_single() {
                Ok(e) => (Some(e), homing.range),
                Err(_) => (None, 0.),
            },
        };

        if let Some(target) = target {
            let Ok((tr, _)) = q_targets.get(target) else {
                warn!("Failed to get target from homing child");
                return;
            };

            let target_translation = tr.translation();
            let mut direction = (target_translation - p_translation).xy();

            //if current distance > target, go towards target
            // otherwise go away from target
            let dist_sign = (target_translation.distance(p_translation) - distance).signum();
            direction *= dist_sign;

            let new_vec = p_lv
                .0
                .normalize()
                .lerp(direction.normalize(), homing.rate)
                .normalize();

            p_lv.0 = new_vec * p_lv.0.length();
            p_tr.rotation = Quat::from_rotation_z(new_vec.y.atan2(new_vec.x) - PI / 2.);
        }
    }
}
