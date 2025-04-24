use crate::*;

// use bevy_rapier2d::prelude::*;
// use ron::*;
// use std::fs;
// use std::io::Write;

#[derive(Component)]
struct Polygon {
    vertices: Vec<Vec2>,
}

// pub fn system_set() -> SystemSet {
//     // SystemSet::new().with_system(save).with_system(load)
// }

// fn save(keys: Res<Input<KeyCode>>, world: &World, q: Query<&Collider>) {
//     if keys.pressed(KeyCode::LControl) && keys.just_pressed(KeyCode::Key1) {
//         let mut scene_world = World::new();

//         // for collider in q.iter() {
//         //     scene_world.spawn().insert_bundle((

//         //     ));
//         // }

//         let type_registry = world.resource::<TypeRegistry>();
//         dbg!(type_registry);

//         let scene = DynamicScene::from_world(&scene_world, type_registry);

//         info!("{}", scene.serialize_ron(type_registry).unwrap());
        
//         let mut file = fs::File::create("assets/scenes/load_scene_example.scn.ron").expect("failed to create file");
//         file.write_all(scene.serialize_ron(type_registry).unwrap().as_bytes()).expect("failed to write to file");
//         // let serialized = ron::to_string(level.as_ref()).expect("Failed to serialize level!");
//         // let filename = "level_1";
//         // let mut file = fs::File::create(format!("assets/levels/{}.lvl", filename))
//         //     .expect("Failed to create file!");
//         // file.write_all(serialized.as_bytes()).unwrap();
//     }
// }

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lvl_assets: Res<Assets<level::Level>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.pressed(KeyCode::LAlt) && keys.just_pressed(KeyCode::Key1) {
        let lvl_name = "level_1";

        let lvl_handle = asset_server.load(&format!("assets/levels/{}.lvl", lvl_name));
        let opt_level = lvl_assets.get(&lvl_handle);

        if let Some(level) = opt_level {}
    }
}
