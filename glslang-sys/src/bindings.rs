/* automatically generated by rust-bindgen 0.70.1 */

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum glslang_stage_t {
    Vertex = 0,
    TesselationControl = 1,
    TesselationEvaluation = 2,
    Geometry = 3,
    Fragment = 4,
    Compute = 5,
    RayGeneration = 6,
    Intersect = 7,
    AnyHit = 8,
    ClosestHit = 9,
    Miss = 10,
    Callable = 11,
    Task = 12,
    Mesh = 13,
}
impl glslang_stage_mask_t {
    pub const VERTEX: glslang_stage_mask_t = glslang_stage_mask_t(1);
}
impl glslang_stage_mask_t {
    pub const TESSCONTROL: glslang_stage_mask_t = glslang_stage_mask_t(2);
}
impl glslang_stage_mask_t {
    pub const TESSEVALUATION: glslang_stage_mask_t = glslang_stage_mask_t(4);
}
impl glslang_stage_mask_t {
    pub const GEOMETRY: glslang_stage_mask_t = glslang_stage_mask_t(8);
}
impl glslang_stage_mask_t {
    pub const FRAGMENT: glslang_stage_mask_t = glslang_stage_mask_t(16);
}
impl glslang_stage_mask_t {
    pub const COMPUTE: glslang_stage_mask_t = glslang_stage_mask_t(32);
}
impl glslang_stage_mask_t {
    pub const RAYGEN: glslang_stage_mask_t = glslang_stage_mask_t(64);
}
impl glslang_stage_mask_t {
    pub const INTERSECT: glslang_stage_mask_t = glslang_stage_mask_t(128);
}
impl glslang_stage_mask_t {
    pub const ANYHIT: glslang_stage_mask_t = glslang_stage_mask_t(256);
}
impl glslang_stage_mask_t {
    pub const CLOSESTHIT: glslang_stage_mask_t = glslang_stage_mask_t(512);
}
impl glslang_stage_mask_t {
    pub const MISS: glslang_stage_mask_t = glslang_stage_mask_t(1024);
}
impl glslang_stage_mask_t {
    pub const CALLABLE: glslang_stage_mask_t = glslang_stage_mask_t(2048);
}
impl glslang_stage_mask_t {
    pub const TASK: glslang_stage_mask_t = glslang_stage_mask_t(4096);
}
impl glslang_stage_mask_t {
    pub const MESH: glslang_stage_mask_t = glslang_stage_mask_t(8192);
}
impl ::std::ops::BitOr<glslang_stage_mask_t> for glslang_stage_mask_t {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        glslang_stage_mask_t(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for glslang_stage_mask_t {
    #[inline]
    fn bitor_assign(&mut self, rhs: glslang_stage_mask_t) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<glslang_stage_mask_t> for glslang_stage_mask_t {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        glslang_stage_mask_t(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for glslang_stage_mask_t {
    #[inline]
    fn bitand_assign(&mut self, rhs: glslang_stage_mask_t) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct glslang_stage_mask_t(pub ::std::os::raw::c_int);
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum glslang_source_t {
    None = 0,
    GLSL = 1,
    HLSL = 2,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum glslang_client_t {
    None = 0,
    Vulkan = 1,
    OpenGL = 2,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum glslang_target_language_t {
    None = 0,
    SPIRV = 1,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum glslang_target_client_version_t {
    Vulkan1_0 = 4194304,
    Vulkan1_1 = 4198400,
    Vulkan1_2 = 4202496,
    Vulkan1_3 = 4206592,
    OpenGL450 = 450,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum glslang_target_language_version_t {
    SPIRV1_0 = 65536,
    SPIRV1_1 = 65792,
    SPIRV1_2 = 66048,
    SPIRV1_3 = 66304,
    SPIRV1_4 = 66560,
    SPIRV1_5 = 66816,
    SPIRV1_6 = 67072,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum glslang_texture_sampler_transform_mode_t {
    Keep = 0,
    UpgradeTextureRemoveSampler = 1,
}
impl glslang_messages_t {
    pub const DEFAULT: glslang_messages_t = glslang_messages_t(0);
}
impl glslang_messages_t {
    pub const RELAXED_ERRORS: glslang_messages_t = glslang_messages_t(1);
}
impl glslang_messages_t {
    pub const SUPPRESS_WARNINGS: glslang_messages_t = glslang_messages_t(2);
}
impl glslang_messages_t {
    pub const AST: glslang_messages_t = glslang_messages_t(4);
}
impl glslang_messages_t {
    pub const SPV_RULES: glslang_messages_t = glslang_messages_t(8);
}
impl glslang_messages_t {
    pub const VULKAN_RULES: glslang_messages_t = glslang_messages_t(16);
}
impl glslang_messages_t {
    pub const ONLY_PREPROCESSOR: glslang_messages_t = glslang_messages_t(32);
}
impl glslang_messages_t {
    pub const READ_HLSL: glslang_messages_t = glslang_messages_t(64);
}
impl glslang_messages_t {
    pub const CASCADING_ERRORS: glslang_messages_t = glslang_messages_t(128);
}
impl glslang_messages_t {
    pub const KEEP_UNCALLED: glslang_messages_t = glslang_messages_t(256);
}
impl glslang_messages_t {
    pub const HLSL_OFFSETS: glslang_messages_t = glslang_messages_t(512);
}
impl glslang_messages_t {
    pub const DEBUG_INFO: glslang_messages_t = glslang_messages_t(1024);
}
impl glslang_messages_t {
    pub const HLSL_ENABLE_16BIT_TYPES: glslang_messages_t = glslang_messages_t(2048);
}
impl glslang_messages_t {
    pub const HLSL_LEGALIZATION: glslang_messages_t = glslang_messages_t(4096);
}
impl glslang_messages_t {
    pub const HLSL_DX9_COMPATIBLE: glslang_messages_t = glslang_messages_t(8192);
}
impl glslang_messages_t {
    pub const BUILTIN_SYMBOL_TABLE: glslang_messages_t = glslang_messages_t(16384);
}
impl glslang_messages_t {
    pub const ENHANCED: glslang_messages_t = glslang_messages_t(32768);
}
impl glslang_messages_t {
    pub const ABSOLUTE_PATH: glslang_messages_t = glslang_messages_t(65536);
}
impl glslang_messages_t {
    pub const DISPLAY_ERROR_COLUMN: glslang_messages_t = glslang_messages_t(131072);
}
impl ::std::ops::BitOr<glslang_messages_t> for glslang_messages_t {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        glslang_messages_t(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for glslang_messages_t {
    #[inline]
    fn bitor_assign(&mut self, rhs: glslang_messages_t) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<glslang_messages_t> for glslang_messages_t {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        glslang_messages_t(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for glslang_messages_t {
    #[inline]
    fn bitand_assign(&mut self, rhs: glslang_messages_t) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct glslang_messages_t(pub ::std::os::raw::c_int);
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum glslang_profile_t {
    None = 1,
    Core = 2,
    Compatibility = 4,
    ES = 8,
}
impl glslang_shader_options_t {
    pub const DEFAULT: glslang_shader_options_t = glslang_shader_options_t(0);
}
impl glslang_shader_options_t {
    pub const AUTO_MAP_BINDINGS: glslang_shader_options_t = glslang_shader_options_t(1);
}
impl glslang_shader_options_t {
    pub const AUTO_MAP_LOCATIONS: glslang_shader_options_t = glslang_shader_options_t(2);
}
impl glslang_shader_options_t {
    pub const VULKAN_RULES_RELAXED: glslang_shader_options_t = glslang_shader_options_t(4);
}
impl ::std::ops::BitOr<glslang_shader_options_t> for glslang_shader_options_t {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        glslang_shader_options_t(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for glslang_shader_options_t {
    #[inline]
    fn bitor_assign(&mut self, rhs: glslang_shader_options_t) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<glslang_shader_options_t> for glslang_shader_options_t {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        glslang_shader_options_t(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for glslang_shader_options_t {
    #[inline]
    fn bitand_assign(&mut self, rhs: glslang_shader_options_t) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct glslang_shader_options_t(pub ::std::os::raw::c_int);
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum glslang_resource_type_t {
    Sampler = 0,
    Texture = 1,
    Image = 2,
    UBO = 3,
    SSBO = 4,
    UAV = 5,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glslang_shader_s {
    _unused: [u8; 0],
}
pub type glslang_shader_t = glslang_shader_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glslang_program_s {
    _unused: [u8; 0],
}
pub type glslang_program_t = glslang_program_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glslang_mapper_s {
    _unused: [u8; 0],
}
pub type glslang_mapper_t = glslang_mapper_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glslang_resolver_s {
    _unused: [u8; 0],
}
pub type glslang_resolver_t = glslang_resolver_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glslang_version_s {
    pub major: ::std::os::raw::c_int,
    pub minor: ::std::os::raw::c_int,
    pub patch: ::std::os::raw::c_int,
    pub flavor: *const ::std::os::raw::c_char,
}
pub type glslang_version_t = glslang_version_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glslang_limits_s {
    pub non_inductive_for_loops: bool,
    pub while_loops: bool,
    pub do_while_loops: bool,
    pub general_uniform_indexing: bool,
    pub general_attribute_matrix_vector_indexing: bool,
    pub general_varying_indexing: bool,
    pub general_sampler_indexing: bool,
    pub general_variable_indexing: bool,
    pub general_constant_matrix_vector_indexing: bool,
}
pub type glslang_limits_t = glslang_limits_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct glslang_resource_s {
    pub max_lights: ::std::os::raw::c_int,
    pub max_clip_planes: ::std::os::raw::c_int,
    pub max_texture_units: ::std::os::raw::c_int,
    pub max_texture_coords: ::std::os::raw::c_int,
    pub max_vertex_attribs: ::std::os::raw::c_int,
    pub max_vertex_uniform_components: ::std::os::raw::c_int,
    pub max_varying_floats: ::std::os::raw::c_int,
    pub max_vertex_texture_image_units: ::std::os::raw::c_int,
    pub max_combined_texture_image_units: ::std::os::raw::c_int,
    pub max_texture_image_units: ::std::os::raw::c_int,
    pub max_fragment_uniform_components: ::std::os::raw::c_int,
    pub max_draw_buffers: ::std::os::raw::c_int,
    pub max_vertex_uniform_vectors: ::std::os::raw::c_int,
    pub max_varying_vectors: ::std::os::raw::c_int,
    pub max_fragment_uniform_vectors: ::std::os::raw::c_int,
    pub max_vertex_output_vectors: ::std::os::raw::c_int,
    pub max_fragment_input_vectors: ::std::os::raw::c_int,
    pub min_program_texel_offset: ::std::os::raw::c_int,
    pub max_program_texel_offset: ::std::os::raw::c_int,
    pub max_clip_distances: ::std::os::raw::c_int,
    pub max_compute_work_group_count_x: ::std::os::raw::c_int,
    pub max_compute_work_group_count_y: ::std::os::raw::c_int,
    pub max_compute_work_group_count_z: ::std::os::raw::c_int,
    pub max_compute_work_group_size_x: ::std::os::raw::c_int,
    pub max_compute_work_group_size_y: ::std::os::raw::c_int,
    pub max_compute_work_group_size_z: ::std::os::raw::c_int,
    pub max_compute_uniform_components: ::std::os::raw::c_int,
    pub max_compute_texture_image_units: ::std::os::raw::c_int,
    pub max_compute_image_uniforms: ::std::os::raw::c_int,
    pub max_compute_atomic_counters: ::std::os::raw::c_int,
    pub max_compute_atomic_counter_buffers: ::std::os::raw::c_int,
    pub max_varying_components: ::std::os::raw::c_int,
    pub max_vertex_output_components: ::std::os::raw::c_int,
    pub max_geometry_input_components: ::std::os::raw::c_int,
    pub max_geometry_output_components: ::std::os::raw::c_int,
    pub max_fragment_input_components: ::std::os::raw::c_int,
    pub max_image_units: ::std::os::raw::c_int,
    pub max_combined_image_units_and_fragment_outputs: ::std::os::raw::c_int,
    pub max_combined_shader_output_resources: ::std::os::raw::c_int,
    pub max_image_samples: ::std::os::raw::c_int,
    pub max_vertex_image_uniforms: ::std::os::raw::c_int,
    pub max_tess_control_image_uniforms: ::std::os::raw::c_int,
    pub max_tess_evaluation_image_uniforms: ::std::os::raw::c_int,
    pub max_geometry_image_uniforms: ::std::os::raw::c_int,
    pub max_fragment_image_uniforms: ::std::os::raw::c_int,
    pub max_combined_image_uniforms: ::std::os::raw::c_int,
    pub max_geometry_texture_image_units: ::std::os::raw::c_int,
    pub max_geometry_output_vertices: ::std::os::raw::c_int,
    pub max_geometry_total_output_components: ::std::os::raw::c_int,
    pub max_geometry_uniform_components: ::std::os::raw::c_int,
    pub max_geometry_varying_components: ::std::os::raw::c_int,
    pub max_tess_control_input_components: ::std::os::raw::c_int,
    pub max_tess_control_output_components: ::std::os::raw::c_int,
    pub max_tess_control_texture_image_units: ::std::os::raw::c_int,
    pub max_tess_control_uniform_components: ::std::os::raw::c_int,
    pub max_tess_control_total_output_components: ::std::os::raw::c_int,
    pub max_tess_evaluation_input_components: ::std::os::raw::c_int,
    pub max_tess_evaluation_output_components: ::std::os::raw::c_int,
    pub max_tess_evaluation_texture_image_units: ::std::os::raw::c_int,
    pub max_tess_evaluation_uniform_components: ::std::os::raw::c_int,
    pub max_tess_patch_components: ::std::os::raw::c_int,
    pub max_patch_vertices: ::std::os::raw::c_int,
    pub max_tess_gen_level: ::std::os::raw::c_int,
    pub max_viewports: ::std::os::raw::c_int,
    pub max_vertex_atomic_counters: ::std::os::raw::c_int,
    pub max_tess_control_atomic_counters: ::std::os::raw::c_int,
    pub max_tess_evaluation_atomic_counters: ::std::os::raw::c_int,
    pub max_geometry_atomic_counters: ::std::os::raw::c_int,
    pub max_fragment_atomic_counters: ::std::os::raw::c_int,
    pub max_combined_atomic_counters: ::std::os::raw::c_int,
    pub max_atomic_counter_bindings: ::std::os::raw::c_int,
    pub max_vertex_atomic_counter_buffers: ::std::os::raw::c_int,
    pub max_tess_control_atomic_counter_buffers: ::std::os::raw::c_int,
    pub max_tess_evaluation_atomic_counter_buffers: ::std::os::raw::c_int,
    pub max_geometry_atomic_counter_buffers: ::std::os::raw::c_int,
    pub max_fragment_atomic_counter_buffers: ::std::os::raw::c_int,
    pub max_combined_atomic_counter_buffers: ::std::os::raw::c_int,
    pub max_atomic_counter_buffer_size: ::std::os::raw::c_int,
    pub max_transform_feedback_buffers: ::std::os::raw::c_int,
    pub max_transform_feedback_interleaved_components: ::std::os::raw::c_int,
    pub max_cull_distances: ::std::os::raw::c_int,
    pub max_combined_clip_and_cull_distances: ::std::os::raw::c_int,
    pub max_samples: ::std::os::raw::c_int,
    pub max_mesh_output_vertices_nv: ::std::os::raw::c_int,
    pub max_mesh_output_primitives_nv: ::std::os::raw::c_int,
    pub max_mesh_work_group_size_x_nv: ::std::os::raw::c_int,
    pub max_mesh_work_group_size_y_nv: ::std::os::raw::c_int,
    pub max_mesh_work_group_size_z_nv: ::std::os::raw::c_int,
    pub max_task_work_group_size_x_nv: ::std::os::raw::c_int,
    pub max_task_work_group_size_y_nv: ::std::os::raw::c_int,
    pub max_task_work_group_size_z_nv: ::std::os::raw::c_int,
    pub max_mesh_view_count_nv: ::std::os::raw::c_int,
    pub max_mesh_output_vertices_ext: ::std::os::raw::c_int,
    pub max_mesh_output_primitives_ext: ::std::os::raw::c_int,
    pub max_mesh_work_group_size_x_ext: ::std::os::raw::c_int,
    pub max_mesh_work_group_size_y_ext: ::std::os::raw::c_int,
    pub max_mesh_work_group_size_z_ext: ::std::os::raw::c_int,
    pub max_task_work_group_size_x_ext: ::std::os::raw::c_int,
    pub max_task_work_group_size_y_ext: ::std::os::raw::c_int,
    pub max_task_work_group_size_z_ext: ::std::os::raw::c_int,
    pub max_mesh_view_count_ext: ::std::os::raw::c_int,
    pub __bindgen_anon_1: glslang_resource_s__bindgen_ty_1,
    pub limits: glslang_limits_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union glslang_resource_s__bindgen_ty_1 {
    pub max_dual_source_draw_buffers_ext: ::std::os::raw::c_int,
    pub maxDualSourceDrawBuffersEXT: ::std::os::raw::c_int,
}
pub type glslang_resource_t = glslang_resource_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glsl_include_result_s {
    pub header_name: *const ::std::os::raw::c_char,
    pub header_data: *const ::std::os::raw::c_char,
    pub header_length: usize,
}
pub type glsl_include_result_t = glsl_include_result_s;
pub type glsl_include_local_func = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        header_name: *const ::std::os::raw::c_char,
        includer_name: *const ::std::os::raw::c_char,
        include_depth: usize,
    ) -> *mut glsl_include_result_t,
>;
pub type glsl_include_system_func = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        header_name: *const ::std::os::raw::c_char,
        includer_name: *const ::std::os::raw::c_char,
        include_depth: usize,
    ) -> *mut glsl_include_result_t,
>;
pub type glsl_free_include_result_func = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        result: *mut glsl_include_result_t,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glsl_include_callbacks_s {
    pub include_system: glsl_include_system_func,
    pub include_local: glsl_include_local_func,
    pub free_include_result: glsl_free_include_result_func,
}
pub type glsl_include_callbacks_t = glsl_include_callbacks_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glslang_input_s {
    pub language: glslang_source_t,
    pub stage: glslang_stage_t,
    pub client: glslang_client_t,
    pub client_version: glslang_target_client_version_t,
    pub target_language: glslang_target_language_t,
    pub target_language_version: glslang_target_language_version_t,
    #[doc = " Shader source code"]
    pub code: *const ::std::os::raw::c_char,
    pub default_version: ::std::os::raw::c_int,
    pub default_profile: glslang_profile_t,
    pub force_default_version_and_profile: ::std::os::raw::c_int,
    pub forward_compatible: ::std::os::raw::c_int,
    pub messages: glslang_messages_t,
    pub resource: *const glslang_resource_t,
    pub callbacks: glsl_include_callbacks_t,
    pub callbacks_ctx: *mut ::std::os::raw::c_void,
}
pub type glslang_input_t = glslang_input_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct glslang_spv_options_s {
    pub generate_debug_info: bool,
    pub strip_debug_info: bool,
    pub disable_optimizer: bool,
    pub optimize_size: bool,
    pub disassemble: bool,
    pub validate: bool,
    pub emit_nonsemantic_shader_debug_info: bool,
    pub emit_nonsemantic_shader_debug_source: bool,
    pub compile_only: bool,
    pub optimize_allow_expanded_id_bound: bool,
}
pub type glslang_spv_options_t = glslang_spv_options_s;
extern "C" {
    pub fn glslang_get_version(version: *mut glslang_version_t);
}
extern "C" {
    pub fn glslang_initialize_process() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn glslang_finalize_process();
}
extern "C" {
    pub fn glslang_shader_create(input: *const glslang_input_t) -> *mut glslang_shader_t;
}
extern "C" {
    pub fn glslang_shader_delete(shader: *mut glslang_shader_t);
}
extern "C" {
    pub fn glslang_shader_set_preamble(
        shader: *mut glslang_shader_t,
        s: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn glslang_shader_shift_binding(
        shader: *mut glslang_shader_t,
        res: glslang_resource_type_t,
        base: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn glslang_shader_shift_binding_for_set(
        shader: *mut glslang_shader_t,
        res: glslang_resource_type_t,
        base: ::std::os::raw::c_uint,
        set: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn glslang_shader_set_options(
        shader: *mut glslang_shader_t,
        options: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn glslang_shader_set_glsl_version(
        shader: *mut glslang_shader_t,
        version: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn glslang_shader_set_default_uniform_block_set_and_binding(
        shader: *mut glslang_shader_t,
        set: ::std::os::raw::c_uint,
        binding: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn glslang_shader_set_default_uniform_block_name(
        shader: *mut glslang_shader_t,
        name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn glslang_shader_set_resource_set_binding(
        shader: *mut glslang_shader_t,
        bindings: *const *const ::std::os::raw::c_char,
        num_bindings: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn glslang_shader_preprocess(
        shader: *mut glslang_shader_t,
        input: *const glslang_input_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn glslang_shader_parse(
        shader: *mut glslang_shader_t,
        input: *const glslang_input_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn glslang_shader_get_preprocessed_code(
        shader: *mut glslang_shader_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn glslang_shader_set_preprocessed_code(
        shader: *mut glslang_shader_t,
        code: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn glslang_shader_get_info_log(
        shader: *mut glslang_shader_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn glslang_shader_get_info_debug_log(
        shader: *mut glslang_shader_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn glslang_program_create() -> *mut glslang_program_t;
}
extern "C" {
    pub fn glslang_program_delete(program: *mut glslang_program_t);
}
extern "C" {
    pub fn glslang_program_add_shader(
        program: *mut glslang_program_t,
        shader: *mut glslang_shader_t,
    );
}
extern "C" {
    pub fn glslang_program_link(
        program: *mut glslang_program_t,
        messages: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn glslang_program_add_source_text(
        program: *mut glslang_program_t,
        stage: glslang_stage_t,
        text: *const ::std::os::raw::c_char,
        len: usize,
    );
}
extern "C" {
    pub fn glslang_program_set_source_file(
        program: *mut glslang_program_t,
        stage: glslang_stage_t,
        file: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn glslang_program_map_io(program: *mut glslang_program_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn glslang_program_map_io_with_resolver_and_mapper(
        program: *mut glslang_program_t,
        resolver: *mut glslang_resolver_t,
        mapper: *mut glslang_mapper_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn glslang_program_SPIRV_generate(program: *mut glslang_program_t, stage: glslang_stage_t);
}
extern "C" {
    pub fn glslang_program_SPIRV_generate_with_options(
        program: *mut glslang_program_t,
        stage: glslang_stage_t,
        spv_options: *mut glslang_spv_options_t,
    );
}
extern "C" {
    pub fn glslang_program_SPIRV_get_size(program: *mut glslang_program_t) -> usize;
}
extern "C" {
    pub fn glslang_program_SPIRV_get(
        program: *mut glslang_program_t,
        arg1: *mut ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn glslang_program_SPIRV_get_ptr(
        program: *mut glslang_program_t,
    ) -> *mut ::std::os::raw::c_uint;
}
extern "C" {
    pub fn glslang_program_SPIRV_get_messages(
        program: *mut glslang_program_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn glslang_program_get_info_log(
        program: *mut glslang_program_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn glslang_program_get_info_debug_log(
        program: *mut glslang_program_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn glslang_glsl_mapper_create() -> *mut glslang_mapper_t;
}
extern "C" {
    pub fn glslang_glsl_mapper_delete(mapper: *mut glslang_mapper_t);
}
extern "C" {
    pub fn glslang_glsl_resolver_create(
        program: *mut glslang_program_t,
        stage: glslang_stage_t,
    ) -> *mut glslang_resolver_t;
}
extern "C" {
    pub fn glslang_glsl_resolver_delete(resolver: *mut glslang_resolver_t);
}
extern "C" {
    pub fn glslang_resource() -> *mut glslang_resource_t;
}
extern "C" {
    pub fn glslang_default_resource() -> *const glslang_resource_t;
}
extern "C" {
    pub fn glslang_default_resource_string() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn glslang_decode_resource_limits(
        resources: *mut glslang_resource_t,
        config: *mut ::std::os::raw::c_char,
    );
}
