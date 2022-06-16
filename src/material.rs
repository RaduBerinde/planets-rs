use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Isometry3, Matrix3, Matrix4, Point2, Point3, Vector3};
use kiss3d::resource::Material;
use kiss3d::resource::{Effect, Mesh, ShaderAttribute, ShaderUniform};
use kiss3d::scene::ObjectData;

/// The default material used to draw objects.
pub struct MyMaterial {
    effect: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    normal: ShaderAttribute<Vector3<f32>>,
    tex_coord: ShaderAttribute<Point2<f32>>,
    light: ShaderUniform<Point3<f32>>,
    color: ShaderUniform<Point3<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    scale: ShaderUniform<Matrix3<f32>>,
    ntransform: ShaderUniform<Matrix3<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
}

impl MyMaterial {
    /// Creates a new `MyMaterial`.
    pub fn new() -> MyMaterial {
        // load the effect
        let mut effect = Effect::new_from_str(OBJECT_VERTEX_SRC, OBJECT_FRAGMENT_SRC);

        effect.use_program();

        // get the variables locations
        MyMaterial {
            pos: effect.get_attrib("position").unwrap(),
            normal: effect.get_attrib("normal").unwrap(),
            tex_coord: effect.get_attrib("tex_coord").unwrap(),
            light: effect.get_uniform("light_position").unwrap(),
            color: effect.get_uniform("color").unwrap(),
            transform: effect.get_uniform("transform").unwrap(),
            scale: effect.get_uniform("scale").unwrap(),
            ntransform: effect.get_uniform("ntransform").unwrap(),
            view: effect.get_uniform("view").unwrap(),
            proj: effect.get_uniform("proj").unwrap(),
            effect,
        }
    }

    fn activate(&mut self) {
        self.effect.use_program();
        self.pos.enable();
        self.normal.enable();
        self.tex_coord.enable();
    }

    fn deactivate(&mut self) {
        self.pos.disable();
        self.normal.disable();
        self.tex_coord.disable();
    }
}

impl Material for MyMaterial {
    fn render(
        &mut self,
        pass: usize,
        transform: &Isometry3<f32>,
        scale: &Vector3<f32>,
        camera: &mut dyn Camera,
        light: &Light,
        data: &ObjectData,
        mesh: &mut Mesh,
    ) {
        let ctxt = Context::get();
        self.activate();

        /*
         *
         * Setup camera and light.
         *
         */
        camera.upload(pass, &mut self.proj, &mut self.view);

        let pos = match *light {
            Light::Absolute(ref p) => *p,
            Light::StickToCamera => camera.eye(),
        };

        self.light.upload(&pos);

        /*
         *
         * Setup object-related stuffs.
         *
         */
        let formated_transform = transform.to_homogeneous();
        let formated_ntransform = transform.rotation.to_rotation_matrix().into_inner();
        let formated_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));

        unsafe {
            self.transform.upload(&formated_transform);
            self.ntransform.upload(&formated_ntransform);
            self.scale.upload(&formated_scale);

            mesh.bind(&mut self.pos, &mut self.normal, &mut self.tex_coord);

            ctxt.active_texture(Context::TEXTURE0);
            ctxt.bind_texture(Context::TEXTURE_2D, Some(&*data.texture()));

            if data.surface_rendering_active() {
                self.color.upload(data.color());

                if data.backface_culling_enabled() {
                    ctxt.enable(Context::CULL_FACE);
                } else {
                    ctxt.disable(Context::CULL_FACE);
                }

                let _ = ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL);
                ctxt.draw_elements(
                    Context::TRIANGLES,
                    mesh.num_pts() as i32,
                    Context::UNSIGNED_SHORT,
                    0,
                );
            }

            if data.lines_width() != 0.0 {
                self.color
                    .upload(data.lines_color().unwrap_or(data.color()));

                ctxt.disable(Context::CULL_FACE);
                ctxt.line_width(data.lines_width());

                if ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::LINE) {
                    ctxt.draw_elements(
                        Context::TRIANGLES,
                        mesh.num_pts() as i32,
                        Context::UNSIGNED_SHORT,
                        0,
                    );
                } else {
                    mesh.bind_edges();
                    ctxt.draw_elements(
                        Context::LINES,
                        mesh.num_pts() as i32 * 2,
                        Context::UNSIGNED_SHORT,
                        0,
                    );
                }
                ctxt.line_width(1.0);
            }

            if data.points_size() != 0.0 {
                self.color.upload(data.color());

                ctxt.disable(Context::CULL_FACE);
                ctxt.point_size(data.points_size());
                if ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::POINT) {
                    ctxt.draw_elements(
                        Context::TRIANGLES,
                        mesh.num_pts() as i32,
                        Context::UNSIGNED_SHORT,
                        0,
                    );
                } else {
                    ctxt.draw_elements(
                        Context::POINTS,
                        mesh.num_pts() as i32,
                        Context::UNSIGNED_SHORT,
                        0,
                    );
                }
                ctxt.point_size(1.0);
            }
        }

        mesh.unbind();
        self.deactivate();
    }
}

/// Vertex shader of the default object material.
pub static OBJECT_VERTEX_SRC: &str = A_VERY_LONG_STRING;
/// Fragment shader of the default object material.
pub static OBJECT_FRAGMENT_SRC: &str = ANOTHER_VERY_LONG_STRING;

const A_VERY_LONG_STRING: &str = include_str!("default.vert");

// phong-like lighting (heavily) inspired
// http://www.mathematik.uni-marburg.de/~thormae/lectures/graphics1/code/WebGLShaderLightMat/ShaderLightMat.html
const ANOTHER_VERY_LONG_STRING: &str = include_str!("default.frag");

///// A material that draws normals of an object.
//pub struct MyMaterial {
//    shader: Effect,
//    position: ShaderAttribute<Point3<f32>>,
//    normal: ShaderAttribute<Vector3<f32>>,
//    proj: ShaderUniform<Matrix4<f32>>,
//    view: ShaderUniform<Matrix4<f32>>,
//    transform: ShaderUniform<Matrix4<f32>>,
//    scale: ShaderUniform<Matrix3<f32>>,
//}
//
//impl MyMaterial {
//    /// Creates a new MyMaterial.
//    pub fn new() -> MyMaterial {
//        let mut shader = Effect::new_from_str(NORMAL_VERTEX_SRC, NORMAL_FRAGMENT_SRC);
//
//        shader.use_program();
//
//        MyMaterial {
//            position: shader.get_attrib("position").unwrap(),
//            normal: shader.get_attrib("normal").unwrap(),
//            transform: shader.get_uniform("transform").unwrap(),
//            scale: shader.get_uniform("scale").unwrap(),
//            view: shader.get_uniform("view").unwrap(),
//            proj: shader.get_uniform("proj").unwrap(),
//            shader,
//        }
//    }
//}
//
//impl Material for MyMaterial {
//    fn render(
//        &mut self,
//        pass: usize,
//        transform: &Isometry3<f32>,
//        scale: &Vector3<f32>,
//        camera: &mut dyn Camera,
//        _: &Light,
//        data: &ObjectData,
//        mesh: &mut Mesh,
//    ) {
//        let ctxt = Context::get();
//        //// enable/disable culling.
//        //if data.backface_culling_enabled() {
//        //    ctxt.enable(Context::CULL_FACE)
//        //} else {
//        //    ctxt.disable(Context::CULL_FACE)
//        //}
//
//        self.shader.use_program();
//        self.position.enable();
//        self.normal.enable();
//
//        /*
//         *
//         * Setup camera and light.
//         *
//         */
//        camera.upload(pass, &mut self.view, &mut self.proj);
//
//        /*
//         *
//         * Setup object-related stuffs.
//         *
//         */
//        let formated_transform = transform.to_homogeneous();
//        let formated_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));
//
//        self.transform.upload(&formated_transform);
//        self.scale.upload(&formated_scale);
//
//        mesh.bind_coords(&mut self.position);
//        mesh.bind_normals(&mut self.normal);
//        mesh.bind_faces();
//
//        ctxt.disable(Context::CULL_FACE);
//        ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL);
//        ctxt.draw_elements(
//            Context::TRIANGLES,
//            mesh.num_pts() as i32,
//            Context::UNSIGNED_SHORT,
//            0,
//        );
//
//        mesh.unbind();
//
//        self.position.disable();
//        self.normal.disable();
//    }
//}
//
///// A vertex shader for coloring each point of an object depending on its normal.
//pub static NORMAL_VERTEX_SRC: &str = A_VERY_LONG_STRING;
//
///// A fragment shader for coloring each point of an object depending on its normal.
//pub static NORMAL_FRAGMENT_SRC: &str = ANOTHER_VERY_LONG_STRING;
//
//const A_VERY_LONG_STRING: &str = "#version 100
//attribute vec3 position;
//attribute vec3 normal;
//uniform mat4 proj;
//uniform mat4 view;
//uniform mat4 transform;
//uniform mat3 scale;
//varying vec3 ls_normal;
//void main() {
//    ls_normal   = normal;
//    gl_Position = proj * view * transform * vec4(scale * position, 1.0);
//}
//";
////gl_Position = proj * view * transform * vec4(scale * position, 1.0);
//
//const ANOTHER_VERY_LONG_STRING: &str = "#version 100
//#ifdef GL_FRAGMENT_PRECISION_HIGH
//   precision highp float;
//#else
//   precision mediump float;
//#endif
//varying vec3 ls_normal;
//void main() {
//    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
//}
//";
//// gl_FragColor = vec4((ls_normal + 1.0) / 2.0, 1.0);
//
