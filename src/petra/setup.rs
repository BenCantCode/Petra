use crate::petra::{self, material::TerrainMaterialPlugin};
use bevy::prelude::*;
use egui::{
    style::{WidgetVisuals, Widgets},
    Color32, Frame, Style,
};
//use goshawk::{rts_camera_system, RtsCamera, ZoomSettings, PanSettings};
use bevy::{
    reflect::TypeUuid,
    render::{mesh::shape, view::VisibleEntities},
};
use bevy_egui::{
    egui::{self, style::Visuals},
    EguiContext, EguiPlugin,
};
use bevy_mod_picking::*;
use petra::modify::{SelectedTool, Tool};

use super::{camera::CameraPlugin, terrain::Terrain};

fn setup_scene(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_title(String::from("Petra"));

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    /*commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });*/
}

pub fn setup() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(TerrainMaterialPlugin)
        .add_plugin(petra::modify::Modify)
        .add_plugin(EguiPlugin)
        .add_system(ui_example)
        .add_startup_system(setup_scene)
        .add_plugin(PickingPlugin)
        .add_plugin(petra::mesh::TerrainMeshPlugin)
        .add_plugin(CameraPlugin)
        .run();
}
fn ui_example(mut egui_context: ResMut<EguiContext>, mut selected_tool: ResMut<SelectedTool>, terrain: Res<Terrain>) {
    let ctx = egui_context.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Save").clicked() {
                    terrain.data.save_to_exr("test.exr").unwrap();
                }
            });
        });
    });
    egui::SidePanel::left("tool_panel").show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            if ui.button("Raise").clicked() {
                selected_tool.0 = Tool::Raise;
            }
            if ui.button("Erode").clicked() {
                selected_tool.0 = Tool::Erode;
            }
        });
    });
}
