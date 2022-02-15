
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        info!("W or Up");
    }

    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        info!("A or Left");
    }

    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        info!("S or Down");
    }

    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        info!("D or Right");
    }
}