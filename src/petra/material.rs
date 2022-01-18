use bevy::{
    core_pipeline::Transparent3d,
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::*,
    render::{
        render_phase::{
            AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
            SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        view::{ComputedVisibility, ExtractedView, Msaa, Visibility},
        RenderApp, RenderStage,
    },
};

use super::modify::CursorPosition;

#[derive(Component)]
pub struct TerrainMaterial;

pub struct TerrainMaterialPlugin;

impl Plugin for TerrainMaterialPlugin {
    fn build(&self, app: &mut App) {
        let render_device = app.world.get_resource::<RenderDevice>().unwrap();
        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("cursor uniform buffer"),
            size: 16,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawCustom>()
            .insert_resource(CursorMeta {
                buffer,
                bind_group: None,
            })
            .init_resource::<CustomPipeline>()
            .init_resource::<SpecializedPipelines<CustomPipeline>>()
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

// add each entity with a mesh and a `TerrainMaterial` to every view's `Transparent3d` render phase using the `CustomPipeline`
fn queue_custom(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<CustomPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedPipelines<CustomPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    material_meshes: Query<(Entity, &MeshUniform), (With<Handle<Mesh>>, With<TerrainMaterial>)>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_custom = transparent_3d_draw_functions
        .read()
        .get_id::<DrawCustom>()
        .unwrap();

    let key = MeshPipelineKey::from_msaa_samples(msaa.samples)
        | MeshPipelineKey::from_primitive_topology(PrimitiveTopology::TriangleList);
    let pipeline = pipelines.specialize(&mut pipeline_cache, &custom_pipeline, key);

    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);
        for (entity, mesh_uniform) in material_meshes.iter() {
            transparent_phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function: draw_custom,
                distance: view_row_2.dot(mesh_uniform.transform.col(3)),
            });
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
    render_queue.write_buffer(&cursor_meta.buffer, 0, bevy::core::cast_slice(&[cursor.x]));
    render_queue.write_buffer(&cursor_meta.buffer, 4, bevy::core::cast_slice(&[cursor.y]));
    render_queue.write_buffer(
        &cursor_meta.buffer,
        8,
        bevy::core::cast_slice(&[cursor.radius]),
    );
    render_queue.write_buffer(
        &cursor_meta.buffer,
        12,
        bevy::core::cast_slice(&[cursor.hovering]),
    );
}

// create a bind group for the cursor uniform buffer
fn queue_cursor_bind_group(
    render_device: Res<RenderDevice>,
    mut cursor_meta: ResMut<CursorMeta>,
    pipeline: Res<CustomPipeline>,
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

pub struct CustomPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    cursor_bind_group_layout: BindGroupLayout,
}

impl FromWorld for CustomPipeline {
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
                        min_binding_size: BufferSize::new(16),
                    },
                    count: None,
                }],
            });

        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();

        CustomPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
            cursor_bind_group_layout,
        }
    }
}

impl SpecializedPipeline for CustomPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.shader.clone();
        let array_stride = 40;
        let vertex_attributes = vec![
            // Position
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 12,
                shader_location: 0,
            },
            // Normal
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 1,
            },
            // Real Position
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 24,
                shader_location: 3,
            },
            // Uv
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 32,
                shader_location: 2,
            },
        ];
        let array_stride = 40;
        descriptor.vertex.buffers = vec![VertexBufferLayout {
            array_stride: array_stride,
            step_mode: VertexStepMode::Vertex,
            attributes: vertex_attributes,
        }];
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
            self.cursor_bind_group_layout.clone(),
        ]);
        descriptor
    }
}

type DrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetTimeBindGroup<2>,
    DrawMesh,
);

struct SetTimeBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetTimeBindGroup<I> {
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
