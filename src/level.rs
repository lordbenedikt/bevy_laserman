use std::f32::consts::E;

use bevy::{render::render_resource::Texture, sprite::Anchor};
use bevy_rapier2d::{parry::shape::HeightField, rapier::control::EffectiveCharacterMovement};

use crate::{mesh_gen::ImageColliderMap, *};

#[derive(Default, Serialize, Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-ad21-9db0-4b8b380a2c12"]
pub struct Level {
    player_pos: Vec2,
}

#[derive(Resource, Default)]
pub struct AddPhysicsRequests {
    vec: Vec<AddPhysicsRequest>,
}

#[derive(Clone)]
pub struct AddPhysicsRequest {
    entity: Entity,
    img_path: String,
    rigid_body: RigidBody,
}

#[derive(Default, Resource)]
pub struct PolygonInMaking {
    vertices: Vec<Vec2>,
}

pub fn system_set() -> SystemSet {
    SystemSet::new()
        .with_system(create_polygon)
        .with_system(add_physics)
}

pub fn add_physics(
    mut commands: Commands,
    image_assets: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut add_physics_requests: ResMut<AddPhysicsRequests>,
    mut image_collider_map: ResMut<ImageColliderMap>,
    mut q: Query<&mut Transform>,
) {
    for i in (0..add_physics_requests.vec.len()).rev() {
        let request = &add_physics_requests.vec[i];
        if let Some(mut entity_commands) = commands.get_entity(request.entity) {
            if let Some(collider) = image_collider_map.0.get(&request.img_path) {
                entity_commands
                    .insert(collider.clone())
                    .insert(request.rigid_body)
                    .insert(Restitution::coefficient(0.7));
                continue;
            }

            let mut points: Vec<Vec2> = vec![];
            let mut indices: Vec<[u32; 2]> = vec![];

            let opt_contour =
                mesh_gen::Contour::from_image(&request.img_path, &asset_server, &image_assets, 2);
            if opt_contour.is_none() {
                continue;
            }

            let contour = opt_contour.unwrap();
            // dbg!(&contour.vertices);
            let line_strings = contour.to_simplified_line_strings(1);
            KinematicCharacterController::
            for line_string in line_strings.iter() {
                let first = points.len();
                for point in line_string.0.iter() {
                    points.push(Vec2::new(point.x, point.y));
                }
                for i in first..points.len() {
                    if i == points.len() - 1 {
                        indices.push([i as u32, first as u32]);
                    } else {
                        indices.push([i as u32, i as u32 + 1]);
                    }
                }
            }

            let size = contour.img.size();
            let points_centered = points
                .iter()
                .map(|point| Vec2::new(point.x - size.x / 2., point.y - size.y / 2.))
                .collect::<Vec<Vec2>>();
            let collider = Collider::convex_decomposition(&points_centered, &indices);

            image_collider_map
                .0
                .insert(request.img_path.clone(), collider.clone());

            entity_commands
                .insert(collider)
                .insert(request.rigid_body)
                .insert(Restitution::coefficient(0.7));

            add_physics_requests.vec.swap_remove(i);
        }
    }
}

pub fn spawn_object_at_mouse(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut add_physics_requests: ResMut<AddPhysicsRequests>,
    mouse: Res<Input<MouseButton>>,
    cursor_pos: Res<CursorPos>,
    egui_state: Res<my_egui::State>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let img_path = egui_state.img_path.clone();
        add_physics_requests.vec.push(AddPhysicsRequest {
            entity: commands
                .spawn(SpriteBundle {
                    texture: asset_server.load(&img_path),
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TransformBundle::from(Transform {
                    translation: cursor_pos.0.extend(0.),
                    scale: Vec3::splat(1.),
                    ..Default::default()
                }))
                .id(),
            img_path: img_path,
            rigid_body: if egui_state.rigid_body_fixed {
                RigidBody::Fixed
            } else {
                RigidBody::Dynamic
            },
        });
    }
}

fn create_ground_rect(commands: &mut Commands, min: Vec2, max: Vec2) {
    let w = (max.x - min.x).abs();
    let h = (max.y - min.y).abs();
    commands
        .spawn(Collider::cuboid(w / 2., h / 2.))
        .insert(RigidBody::Fixed)
        .insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(w, h)),
                color: Color::Rgba {
                    red: 0.2,
                    green: 7.,
                    blue: 0.,
                    alpha: 1.,
                },
                anchor: Anchor::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            (min.x + max.x) / 2.,
            (min.y + max.y) / 2.,
            0.0,
        )));
}

pub fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    /* Configure Physics. */
    rapier_config.gravity.y = -3000.0;

    /* Create the ground. */
    create_ground_rect(
        &mut commands,
        Vec2::new(-5000., -200.),
        Vec2::new(5000., -400.),
    );

    // for i in 0..10 {
    //     /* Create some boxes. */
    //     commands
    //         .spawn_empty()
    //         .insert(RigidBody::Dynamic)
    //         .insert(Collider::cuboid(50., 50.))
    //         .insert(Restitution::coefficient(0.7))
    //         .insert(TransformBundle::from(Transform::from_xyz(
    //             0.0,
    //             200.0 + 100.0 * i as f32,
    //             0.0,
    //         )));
    // }
}

pub fn create_polygon(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut polygon: ResMut<PolygonInMaking>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        polygon.vertices.push(cursor_pos.0);

        if keys.pressed(KeyCode::LControl) {
            let mut indices: Vec<[u32; 2]> = vec![];
            for i in 0..polygon.vertices.len() {
                let a = i as u32;
                let b = ((i + 1) % polygon.vertices.len()) as u32;
                indices.push([a, b]);
            }
            let collider = Collider::convex_decomposition(&polygon.vertices, &indices);
            commands
                .spawn_empty()
                .insert(RigidBody::Fixed)
                .insert(collider)
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
            polygon.vertices.clear();
        }
    }
}
