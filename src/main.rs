#[warn(dead_code)]

mod drop;
mod player;

use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use player::PlayerPlugin;
use drop::DropPlugin;

const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct ActiveDrops(u32);

struct WinSize {
    h: f32,
    w: f32,
}

#[derive(Component)]
struct Player{
    speed: f32,
    health: f32,
}

#[derive(Component)]
struct Drop{
    drop_type: DropType,
    additional_health: f32,
}

enum DropType{
    APPLE,
    PEAR,
}

fn main() {
    App::new()
		.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(DropPlugin)
        .add_plugin(PlayerPlugin)
        .add_system(player_pickup_drop)
		.run();
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.,-200.,10.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player {speed: 500., health: 100.});

    commands.insert_resource(WinSize {
        h: window.height(),
        w: window.width(),
    });

    commands.insert_resource(ActiveDrops(0u32));
}


fn player_pickup_drop(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform, &Sprite)>,
    drop_query: Query<(Entity,&Drop, &Transform, &Sprite)>,
    mut active_drops: ResMut<ActiveDrops>
) {

    let (mut player, player_transform, player_sprite) = player_query.single_mut();

    for(drop_entity,drop, drop_transform, drop_sprite) in drop_query.iter() {

        let collision = collide(
            player_transform.translation,
            player_sprite.custom_size.unwrap(),
            drop_transform.translation,
            drop_sprite.custom_size.unwrap());

        if let Some(_) = collision {
            commands.entity(drop_entity).despawn();
            active_drops.0 -= 1;
            player.health += drop.additional_health;
        }

    }
}