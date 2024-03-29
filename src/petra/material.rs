use bevy::{
    core_pipeline::Transparent3d,
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
            SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        view::{ExtractedView, Msaa},
        RenderApp, RenderStage,
    },
};

use super::{mesh::ATTRIBUTE_REAL_POSITION, modify::CursorPosition};

#[derive(Component)]
pub struct TerrainMaterial;

pub struct TerrainMaterialPlugin;

impl Plugin for TerrainMaterialPlugin {
    fn build(&self, app: &mut App) {
        let render_device = app.world.resource::<RenderDevice>();
        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("cursor uniform buffer"),
            size: std::mem::size_of::<ExtractedCursor>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawCustom>()
            .insert_resource(CursorMeta {
                buffer,
                bind_group: None,
            })
            .init_resource::<TerrainPipeline>()
            .init_resource::<SpecializedMeshPipelines<TerrainPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_cursor)
            .add_system_to_stage(RenderStage::Extract, extract_terrain_material)
            .add_system_to_stage(RenderStage::Prepare, prepare_cursor)
            .add_system_to_stage(RenderStage::Queue, queue_custom)
            .add_system_to_stage(RenderStage::Queue, queue_cursor_bind_group);
    }
}

// extract the `CustomMaterial` component into the render world
fn extract_terrain_material(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    mut query: Query<Entity, With<TerrainMaterial>>,
) {
    let mut values = Vec::with_capacity(*previous_len);
    for entity in query.iter_mut() {
        values.push((entity, (TerrainMaterial,)));
    }
    *previous_len = values.len();
    commands.insert_or_spawn_batch(values);
}

// add each entity with a mesh and a `CustomMaterial` to every view's `Transparent3d` render phase using the `CustomPipeline`
#[allow(clippy::too_many_arguments)]
fn queue_custom(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<TerrainPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<TerrainPipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    render_meshes: Res<RenderAssets<Mesh>>,
    material_meshes: Query<(Entity, &MeshUniform, &Handle<Mesh>), With<TerrainMaterial>>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_custom = transparent_3d_draw_functions
        .read()
        .get_id::<DrawCustom>()
        .unwrap();

    let key = MeshPipelineKey::from_msaa_samples(msaa.samples)
        | MeshPipelineKey::from_primitive_topology(PrimitiveTopology::TriangleList);

    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);
        for (entity, mesh_uniform, mesh_handle) in material_meshes.iter() {
            if let Some(mesh) = render_meshes.get(mesh_handle) {
                let pipeline = pipelines
                    .specialize(&mut pipeline_cache, &custom_pipeline, key, &mesh.layout)
                    .unwrap();
                transparent_phase.add(Transparent3d {
                    entity,
                    pipeline,
                    draw_function: draw_custom,
                    distance: view_row_2.dot(mesh_uniform.transform.col(3)),
                });
            }
        }
    }
}

#[derive(Default)]
struct ExtractedCursor {
    x: f32,
    y: f32,
    radius: f32,
    hovering: u32,
}

// extract the passed cursor into a resource in the render world
fn extract_cursor(mut commands: Commands, cursor: Res<CursorPosition>) {
    commands.insert_resource(ExtractedCursor {
        x: cursor.pos.x,
        y: cursor.pos.y,
        radius: cursor.radius,
        hovering: cursor.hovering,
    });
}

struct CursorMeta {
    buffer: Buffer,
    bind_group: Option<BindGroup>,
}

// write the extracted cursor into the corresponding uniform buffer
fn prepare_cursor(
    cursor: Res<ExtractedCursor>,
    cursor_meta: ResMut<CursorMeta>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(
        &cursor_meta.buffer,
        0,
        bevy::core::cast_slice(&[cursor.x, cursor.y, cursor.radius]),
    );
    render_queue.write_buffer(
        &cursor_meta.buffer,
        12,
        bevy::core::cast_slice(&[cursor.hovering]),
    )
}

// create a bind group for the cursor uniform buffer
fn queue_cursor_bind_group(
    render_device: Res<RenderDevice>,
    mut cursor_meta: ResMut<CursorMeta>,
    pipeline: Res<TerrainPipeline>,
) {
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.cursor_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: cursor_meta.buffer.as_entire_binding(),
        }],
    });
    cursor_meta.bind_group = Some(bind_group);
}

pub struct TerrainPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    cursor_bind_group_layout: BindGroupLayout,
}

impl FromWorld for TerrainPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let shader = asset_server.load("shaders/terrain.wgsl");

        let render_device = world.get_resource_mut::<RenderDevice>().unwrap();
        let cursor_bind_group_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("cursor bind group"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            std::mem::size_of::<ExtractedCursor>() as u64
                        ),
                    },
                    count: None,
                }],
            });

        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();

        TerrainPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
            cursor_bind_group_layout,
        }
    }
}

impl SpecializedMeshPipeline for TerrainPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;
        descriptor.vertex.shader = self.shader.clone();
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_REAL_POSITION.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
            self.cursor_bind_group_layout.clone(),
        ]);
        Ok(descriptor)
    }
}

type DrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetCursorBindGroup<2>,
    DrawMesh,
);

struct SetCursorBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetCursorBindGroup<I> {
    type Param = SRes<CursorMeta>;

    fn render<'w>(
        _view: Entity,
        _item: Entity,
        cursor_meta: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let cursor_bind_group = cursor_meta.into_inner().bind_group.as_ref().unwrap();
        pass.set_bind_group(I, cursor_bind_group, &[]);

        RenderCommandResult::Success
    }
}
