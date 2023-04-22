use bevy_rapier2d;

use crate::*;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Resource)]
pub struct Player {
    pub entity: Option<Entity>,
    pub sprite_entity: Option<Entity>,
    pub on_ground: bool,
    pub move_direction: i32,
    pub velocity: Vec2,
    pub fall_speed: f32,
}

impl Default for Player {
    fn default() -> Player {
        return Player {   
            entity: None,
            sprite_entity: None,
            on_ground: false,
            move_direction: 0,
            velocity: Vec2::default(),
            fall_speed: 1.0,
        }
    }
}

const SPEED: f32 = 13.0;
const RUN_SPEED: f32 = 13.0;
const JUMP_SPEED: f32 = 20.0;
const PLY_COLLIDER_HALF_HEIGHT: f32 = 68.0;
const PLY_COLLIDER_RADIUS: f32 = 17.0;


pub fn system_set() -> SystemSet {
    SystemSet::new()
        .with_system(player_run_walk)
        .with_system(player_fall)
        .with_system(apply_vel_to_controller)
}


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut player: ResMut<Player>,
) {
    let player_entity = commands
        .spawn(PlayerMarker)
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::capsule_y(
            PLY_COLLIDER_HALF_HEIGHT,
            PLY_COLLIDER_RADIUS,
        ))
        .insert(KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(5.0)),
            ..Default::default()
        })
        .insert(SpriteBundle {
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..Default::default()
        })
        .with_children(|child_builder| {
            let texture_handle = asset_server.load("spritesheet_player.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(120.0, 133.0), 9, 1, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let player_sprite_entity = child_builder
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        translation: Vec3::new(0.0, 5.0, 0.0),
                        scale: Vec3::splat(1.4),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(anim::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
                .id();
            player.sprite_entity = Some(player_sprite_entity);
        })
        .id();
    player.entity = Some(player_entity);
}


fn player_run_walk(
    keys: Res<Input<KeyCode>>,
    mut q: Query<&mut KinematicCharacterController, With<PlayerMarker>>,
    mut q_sprite: Query<&mut Transform>,
    mut player: ResMut<Player>,
) {
    if let Some(mut char_controller) = q.iter_mut().next() {
        let sign = if keys.pressed(KeyCode::Left) {
            -1.0
        } else if keys.pressed(KeyCode::Right) {
            1.0
        } else {
            player.move_direction = 0;
            player.velocity.x = 0.0;
            0.0
        };
        let speed = if keys.pressed(KeyCode::LShift) {
            player.move_direction = 2 * sign as i32;
            RUN_SPEED
        } else {
            player.move_direction = sign as i32;
            SPEED
        };
        player.velocity.x =speed * sign;
        if sign != 0.0 {
            let mut sprite_transform = q_sprite.get_mut(player.sprite_entity.unwrap()).unwrap();
            if sign * sprite_transform.scale.x > 0.0 {
                sprite_transform.scale.x *= -1.0;
            }
        }
    }
}


fn player_fall(
    mut q: Query<(&mut KinematicCharacterController, &KinematicCharacterControllerOutput), With<PlayerMarker>>,
    mut player: ResMut<Player>,
) {
    if let Some((mut char_controller, char_controller_output)) = q.iter_mut().next() {
        if !char_controller_output.grounded {
            player.velocity.y -= player.fall_speed;
        } else {
            player.velocity.y = 0;
        }
    }
}


fn player_jump(
    keys: Res<Input<KeyCode>>,
    mut q: Query<&mut Velocity, With<PlayerMarker>>,
    mut player: ResMut<Player>,
    rapier_context: Res<RapierContext>,
    q_collider: Query<&Collider, With<PlayerMarker>>,
    q_transform: Query<&Transform, With<PlayerMarker>>,
) {
    let mut velocity = q.iter_mut().next().unwrap();

    // Check whether player is grounded
    player.on_ground = false;
    if velocity.linvel.y >= -30.0 {
        let shape_pos = q_transform.iter().next().unwrap().translation.truncate()
            + Vec2::new(0.0, -(PLY_COLLIDER_HALF_HEIGHT + 5.0));
        let shape = &Collider::ball(PLY_COLLIDER_RADIUS - 3.0);
        if rapier_context
            .intersection_with_shape(
                shape_pos,
                0.0,
                shape,
                QueryFilter::new().exclude_collider(player.entity.unwrap()),
            )
            .is_some()
        {
            player.on_ground = true;
        }
        // if rapier_context
        //     .cast_shape(
        //         shape_pos,
        //         0.0,
        //         Vec2::new(0.0, -1.0),
        //         shape,
        //         1.0,
        //         QueryFilter::new().exclude_collider(player.entity.unwrap()),
        //     )
        //     .is_some()
        // {
        //     player.on_ground = true;
        // }
        // rapier_context.intersections_with_point(
        //     shape_pos + Vec2::new(0.0, -87.0),
        //     QueryFilter::new().exclude_collider(player.entity.unwrap()),
        //     |_| {
        //         player.on_ground = true;
        //         false
        //     },
        // );
    }

    // If on_ground and up was pressed, Jump
    if keys.just_pressed(KeyCode::Up) && player.on_ground {
        velocity.linvel = Vec2::new(velocity.linvel.x, velocity.linvel.y + JUMP_SPEED);
        player.on_ground = false;
    }
}


fn apply_vel_to_controller(
    player: Res<Player>, 
    mut q: Query<&mut KinematicCharacterController, With<PlayerMarker>>
) {
    if let Some(mut char_controller) = q.iter_mut().next() {
        char_controller.translation = Some(player.velocity);
    }
}