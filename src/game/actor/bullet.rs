use bevy::prelude::*;

use crate::game::constants::ASSET_SCALE;

use super::super::scene::SceneAssets;

use super::super::ai::AiMode;
use super::super::components::*;
use super::super::events::WeaponFiredEvent;
use super::*;

#[derive(Clone, Default)]
pub enum BulletType {
    #[default]
    Standard,
    StandardEnemy,
}

#[derive(Bundle, Clone, Default)]
pub struct BulletActorBundle {
    pub actor: Actor,
    pub scene_bundle: StarRustSceneBundle,
    pub collider: Collider,
    pub ai: Ai,
    pub bullet: Bullet,
    pub timed_despawn: TimedOobDespawn,
}

pub trait AiBulletBundle {
    fn get_bullet_bundle(
        models: &Res<SceneAssets>,
        weapon_data: &WeaponFiredEvent,
    ) -> BulletActorBundle;
}

pub struct StandardBullet;

impl AiBulletBundle for StandardBullet {
    fn get_bullet_bundle(
        models: &Res<SceneAssets>,
        weapon_data: &WeaponFiredEvent,
    ) -> BulletActorBundle {
        return BulletActorBundle {
            actor: Actor {
                speed: Vec2::new(10.0, 10.0),
            },
            scene_bundle: StarRustSceneBundle {
                scene: models.default_bullet.clone(),
                transform: Transform::from_xyz(
                    weapon_data.translation.x,
                    weapon_data.translation.y,
                    2.0,
                )
                .with_scale(Vec3::splat(ASSET_SCALE))
                .with_rotation(weapon_data.rotation),
                ..default()
            },
            collider: Collider {
                hitmask: weapon_data.hitmask,
                damage: 1,
                rect: Vec2::new(0.2, 0.8),
            },
            ai: Ai {
                mode: AiMode::ChargeForward1,
                timer: Timer::default(),
            },
            bullet: Bullet {},
            timed_despawn: TimedOobDespawn {
                timer: Timer::from_seconds(0.05, TimerMode::Repeating),
            },
        };
    }
}

pub struct StandardEnemyBullet;

impl AiBulletBundle for StandardEnemyBullet {
    fn get_bullet_bundle(
        models: &Res<SceneAssets>,
        weapon_data: &WeaponFiredEvent,
    ) -> BulletActorBundle {
        let mut bullet = StandardBullet::get_bullet_bundle(models, weapon_data).clone();
        bullet.scene_bundle.scene = models.default_enemy_bullet.clone();
        return bullet;
    }
}
