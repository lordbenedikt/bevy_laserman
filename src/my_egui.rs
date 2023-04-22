use crate::*;
use bevy_egui::{
    egui::{self},
    EguiContext,
};
use std::fs;

#[derive(Resource)]
pub struct State {
    pub img_path: String,
    pub rigid_body_fixed: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            img_path: "flask.png".to_string(),
            rigid_body_fixed: false,
        }
    }
}

pub fn system_set() -> SystemSet {
    SystemSet::new().with_system(choose_image_menu)
}

fn get_paths_of_folder(folder: &str) -> Vec<String> {
    let filenames: Vec<String> = fs::read_dir(folder)
        .unwrap()
        .map(|read_dir| {
            let mut components = vec![];
            for component in read_dir.unwrap().path().iter() {
                let comp_string = component.to_str().unwrap();
                if !comp_string.eq(".") && !comp_string.eq("assets") {
                    components.push(component.to_str().unwrap().to_string());
                }
            }
            components.join("/")
        })
        .collect();
    filenames
}

pub fn choose_image_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State>,
    mouse: Res<Input<MouseButton>>,
) {
    let mut filenames: Vec<String> = get_paths_of_folder("./assets/fruit/");
    filenames.append(&mut get_paths_of_folder("./assets/underwater"));

    // Show Window
    let response = egui::Window::new("Choose Image")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                let choose_img_path = egui::ComboBox::from_id_source("image_path")
                    .selected_text(&state.img_path)
                    .show_ui(ui, |ui| {
                        for filename in filenames {
                            let option = ui.selectable_value(
                                &mut state.img_path,
                                filename.clone(),
                                &filename,
                            );
                        }
                    });
                ui.checkbox(&mut state.rigid_body_fixed, "fixed");
            });
        })
        .unwrap()
        .response;

    // check_mouse_interaction(&mut egui_context, response, &mut state, &mouse);
}

// fn check_mouse_interaction(
//     egui_context: &mut EguiContext,
//     response: egui::Response,
//     state: &mut State,
//     mouse: &Input<MouseButton>,
// ) {
//     // Check whether mouse is hovering window
//     if let Some(hover_pos) = egui_context.ctx_mut().pointer_hover_pos() {
//         if response.rect.contains(hover_pos) {
//             state.ui_hover = true;
//             if mouse.just_pressed(MouseButton::Left) {
//                 state.ui_drag = true;
//             }
//         } else {
//             state.ui_hover |= false;
//             if mouse.just_pressed(MouseButton::Left) {
//                 state.ui_drag |= false;
//             }
//         }
//     }
// }
