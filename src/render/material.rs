use std::any::Any;
use std::rc::Rc;

use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Isometry3, Matrix3, Matrix4, Point2, Point3, Vector3};
use kiss3d::resource::Material;
use kiss3d::resource::{Effect, Mesh, ShaderAttribute, ShaderUniform};
use kiss3d::scene::ObjectData;

use crate::body::Body;

/// The default material used to draw objects.
pub struct MyMaterial {
    effect: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    normal: ShaderAttribute<Vector3<f32>>,
    tex_coord: ShaderAttribute<Point2<f32>>,
    color: ShaderUniform<Point3<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    scale: ShaderUniform<Matrix3<f32>>,
    ntransform: ShaderUniform<Matrix3<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    /*light_pos: ShaderUniform<Point3<f32>>,
    light_radius: ShaderUniform<f32>,
    occluder_pos: ShaderUniform<Point3<f32>>,
    occluder_radius: ShaderUniform<f32>,
    */
}

#[derive(Default)]
pub struct BodyLightingData {
    pub light_pos: Vector3<f32>,
    pub light_radius: f32,

    pub occluder_pos: Vector3<f32>,
    pub occluder_radius: f32,
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
            color: effect.get_uniform("color").unwrap(),
            transform: effect.get_uniform("transform").unwrap(),
            scale: effect.get_uniform("scale").unwrap(),
            ntransform: effect.get_uniform("ntransform").unwrap(),
            view: effect.get_uniform("view").unwrap(),
            proj: effect.get_uniform("proj").unwrap(),
            //light_pos: effect.get_uniform("light_pos").unwrap(),
            //light_radius: effect.get_uniform("light_radius").unwrap(),
            //occluder_pos: effect.get_uniform("occluder_pos").unwrap(),
            //occluder_radius: effect.get_uniform("occluder_radius").unwrap(),
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
        _: &Light,
        data: &ObjectData,
        mesh: &mut Mesh,
    ) {
        let ctxt = Context::get();
        self.activate();

        /*
         *
         * Setup camera.
         *
         */
        camera.upload(pass, &mut self.proj, &mut self.view);

        /*
         *
         * Setup object-related stuffs.
         *
         */
        let formated_transform = transform.to_homogeneous();
        let formated_ntransform = transform.rotation.to_rotation_matrix().into_inner();
        let formated_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));

        self.transform.upload(&formated_transform);
        self.ntransform.upload(&formated_ntransform);
        self.scale.upload(&formated_scale);

        mesh.bind(&mut self.pos, &mut self.normal, &mut self.tex_coord);

        let lighting = data
            .user_data()
            .downcast_ref::<Rc<BodyLightingData>>()
            .unwrap();

        ctxt.active_texture(Context::TEXTURE0);
        ctxt.bind_texture(Context::TEXTURE_2D, Some(&*data.texture()));

        self.color.upload(data.color());

        ctxt.enable(Context::CULL_FACE);
        let _ = ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL);
        ctxt.draw_elements(
            Context::TRIANGLES,
            mesh.num_pts() as i32,
            Context::UNSIGNED_SHORT,
            0,
        );

        mesh.unbind();
        self.deactivate();
    }
}

const OBJECT_VERTEX_SRC: &str = include_str!("material.vert");

const OBJECT_FRAGMENT_SRC: &str = include_str!("material.frag");
