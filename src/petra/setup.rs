use super::shader;
use crate::petra;
use bevy::{math::vec3, pbr::AmbientLight, prelude::*, render::camera::{Camera, VisibleEntities}};
use egui::{Color32, Frame, Style, style::{WidgetVisuals, Widgets}};
use goshawk::{rts_camera_system, RtsCamera, ZoomSettings, PanSettings};
use bevy::{
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{asset_shader_defs_system, ShaderDefs, ShaderStage, ShaderStages},
    },
};
use bevy_egui::{EguiContext, EguiPlugin, egui::{self, style::Visuals}};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_mod_picking::*;
use petra::modify::{SelectedTool, Tool};

fn setup_scene(
    commands: &mut Commands,
    mut windows: ResMut<Windows>
) {
    windows.get_primary_mut().unwrap().set_title(String::from("Petra"));


    let proj = petra::camera::OrthoProjection::default();
    let cam_name = bevy::render::render_graph::base::camera::CAMERA_3D;

    let mut camera = Camera::default();
    camera.name = Some(cam_name.to_string());

    commands
        .spawn((
            camera,
            proj,
            VisibleEntities::default(),
            Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),  //temporary, gets overwritten by goshawk
            GlobalTransform::default(),
        ))
        .with(RtsCamera {
            looking_at: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(ZoomSettings {
            scroll_accel: 10.0,
            max_velocity: 50.0,
            idle_deceleration: 200.0,
            angle_change_zone: 500.0..=550.0,
            distance_range: 500.0..=1000.0,
            ..Default::default()
        })
        .with(PanSettings {
            mouse_accel: 0.0, // No mouse movement.
            keyboard_accel: 100.0,
            idle_deceleration: 75.0,
            max_speed: 25.0,
            ..Default::default()
        })
        .with(PickSource::default());
}

pub fn setup() {
    use bevy::render::camera::camera_system;

    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui_example.system())
        
        .add_startup_system(setup_scene.system())
        .add_plugin(PickingPlugin)
        .add_plugin(petra::mesh::TerrainMeshPlugin)
        .add_plugin(petra::modify::Modify)
        .add_system(rts_camera_system.system())
        .add_system_to_stage(
            bevy::app::stage::POST_UPDATE,
            camera_system::<petra::camera::OrthoProjection>.system(),
        )
        .run();
}
fn ui_example(mut egui_context: ResMut<EguiContext>, mut selected_tool: ResMut<SelectedTool>) {
    let ctx = &mut egui_context.ctx;
    egui::TopPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Quit").clicked {
                    std::process::exit(0);
                }
            });
        });
    });
    egui::SidePanel::left("tool_panel", 64.0).show(ctx, |ui| {
        egui::ScrollArea::auto_sized().show(ui, |ui| {
            if ui.button("Raise").clicked {
                selected_tool.0 = Tool::Raise;
            }
            if ui.button("Erode").clicked {
                selected_tool.0 = Tool::Erode;
            }
        });
    });
}