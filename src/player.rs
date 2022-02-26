use crate::{
    TIME_STEP, Player
};

use bevy::{
    core::FixedTimestep,
    prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App){
        app
        .add_system(movement);
    }
}

fn movement(keyboard_input: Res<Input<KeyCode>>,mut query: Query<(&Player, &mut Transform, &Sprite)>) {
    let (player, mut transform, sprite_vec) = query.single_mut();
    let sprite: Vec2 = sprite_vec.custom_size.unwrap();
    let mut horizontal_direction = 0.0;
    let mut vertical_direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        horizontal_direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        horizontal_direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        vertical_direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        vertical_direction -= 1.0;
    }

    let translation = &mut transform.translation;
    // move the paddle horizontally
    translation.x += horizontal_direction * player.speed * TIME_STEP;
    translation.y += vertical_direction * player.speed * TIME_STEP;
    // bound the paddle within the walls
    translation.x = translation.x.min(640.0-sprite.x/2.).max(-640.0+sprite.x/2.);
    translation.y = translation.y.min(360.0-sprite.y/2.).max(-360.0+sprite.y/2.);

}