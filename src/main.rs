use bevy::app::CreatePlugin;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(creature_movement_system),
        )
        .add_system(keyboard_input_system)
        .run();
}

#[derive(Component)]
struct Creature {
    velocity: Vec3,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Creature {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
}

fn creature_movement_system(mut creature_query: Query<(&Creature, &mut Transform)>) {
    let (creature, mut transform) = creature_query.single_mut();
    transform.translation += creature.velocity * TIME_STEP;
}

// TODO Implement camera panning
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
