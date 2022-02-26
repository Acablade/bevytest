use crate::{
	ActiveDrops, Drop, DropType,WinSize,
};

use bevy::{
    core::FixedTimestep,
    prelude::*,
};

use rand::{thread_rng, Rng};

pub struct DropPlugin;

impl Plugin for DropPlugin {
    fn build(&self, app: &mut bevy::prelude::App){
        app.add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1f64))
            .with_system(drop_spawn),
        );
    }
}

fn drop_spawn(
    mut commands: Commands,
    mut active_drops: ResMut<ActiveDrops>,
    win_size: Res<WinSize>,
) {


    if active_drops.0 < 5 {
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. -100.;
        let h_span = win_size.h / 2. -100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::TOMATO,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x,y,0.),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Drop {drop_type: DropType::APPLE, additional_health: 20.});
        

        active_drops.0 += 1;
    }

}