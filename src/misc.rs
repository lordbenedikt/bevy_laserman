use crate::*;

#[derive(Component)]
pub struct MainCamera;

pub fn system_set() -> SystemSet {
    SystemSet::new().with_system(get_mouse_position)
}

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(new_camera_2d()).insert(MainCamera);
    
    // Euler Angles Test
    // let quat_1 = Quat::from_euler(EulerRot::XYZ, 1.0, 2.0, 3.0);
    // let quat_2 = Quat::from_euler(EulerRot::XYZ, 5.0, 7.0, 5.0);

    // let rot_1 = quat_1 * quat_2;
    // let rot_2 = quat_2 * quat_1;

    // dbg!(&rot_1);
    // dbg!(&rot_2);

    // commands.spawn_bundle(SpriteBundle {
    //     transform: Transform {
    //         rotation: rot_1,
    //         translation: Vec3::new(-100.,0.,0.),
    //         scale: Vec3::splat(100.),
    //     },
    //     ..Default::default()
    // });
    // commands.spawn_bundle(SpriteBundle {
    //     transform: Transform {
    //         rotation: rot_2,
    //         translation: Vec3::new(100.,0.,0.),
    //         scale: Vec3::splat(100.),
    //     },
    //     ..Default::default()
    // });
}

fn new_camera_2d() -> Camera2dBundle {
    let far = 1000.0;
    let mut camera = Camera2dBundle::default();
    camera.projection = OrthographicProjection {
        far,
        scaling_mode: ScalingMode::WindowSize,
        scale: 3f32,
        ..Default::default()
    };
    return camera;
}

pub fn get_mouse_position(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // resource that stores cursor position
    mut cursor_pos: ResMut<CursorPos>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        cursor_pos.0 = Vec2::new(world_pos.x, world_pos.y);
    }
}
