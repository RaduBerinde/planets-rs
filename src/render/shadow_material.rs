use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Isometry3, Matrix3, Matrix4, Point2, Point3, Vector3, Vector4};
use kiss3d::resource::{Effect, Mesh, ShaderAttribute, ShaderUniform};
use kiss3d::resource::{Material, Texture};
use kiss3d::scene::ObjectData;

/// Material used to render a body. It calculates the shadow (eclipse) caused
/// by another body.
pub struct ShadowMaterial {
    effect: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    normal: ShaderAttribute<Vector3<f32>>,
    tex_coord: ShaderAttribute<Point2<f32>>,
    day_color: ShaderUniform<Point3<f32>>,
    night_color: ShaderUniform<Point3<f32>>,
    day_tex: ShaderUniform<i32>,
    night_tex: ShaderUniform<i32>,
    transform: ShaderUniform<Matrix4<f32>>,
    ntransform: ShaderUniform<Matrix3<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    light_pos: ShaderUniform<Point3<f32>>,
    light_radius: ShaderUniform<f32>,
    occluder_pos: ShaderUniform<Point3<f32>>,
    occluder_radius: ShaderUniform<f32>,
}

// BodyLightingData is used for the object's generic data (inside a Rc RefCell).
#[derive(Default)]
pub struct BodyLightingData {
    pub day_texture: Option<Rc<Texture>>,
    pub day_color: Point3<f32>,

    pub night_texture: Option<Rc<Texture>>,
    pub night_color: Point3<f32>,

    pub light_pos: Point3<f32>,
    pub light_radius: f32,

    pub occluder_pos: Point3<f32>,
    pub occluder_radius: f32,
}

impl ShadowMaterial {
    pub fn new() -> ShadowMaterial {
        // load the effect
        let mut effect = Effect::new_from_str(OBJECT_VERTEX_SRC, OBJECT_FRAGMENT_SRC);

        effect.use_program();

        // get the variables locations
        ShadowMaterial {
            pos: effect.get_attrib("position").unwrap(),
            normal: effect.get_attrib("normal").unwrap(),
            tex_coord: effect.get_attrib("tex_coord").unwrap(),
            day_color: effect.get_uniform("day_color").unwrap(),
            night_color: effect.get_uniform("night_color").unwrap(),
            day_tex: effect.get_uniform("day_tex").unwrap(),
            night_tex: effect.get_uniform("night_tex").unwrap(),
            transform: effect.get_uniform("transform").unwrap(),
            ntransform: effect.get_uniform("ntransform").unwrap(),
            view: effect.get_uniform("view").unwrap(),
            proj: effect.get_uniform("proj").unwrap(),
            light_pos: effect.get_uniform("light_pos").unwrap(),
            light_radius: effect.get_uniform("light_radius").unwrap(),
            occluder_pos: effect.get_uniform("occluder_pos").unwrap(),
            occluder_radius: effect.get_uniform("occluder_radius").unwrap(),
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

impl Material for ShadowMaterial {
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
        let formated_transform = transform.to_homogeneous()
            * Matrix4::from_diagonal(&Vector4::new(scale.x, scale.y, scale.z, 1.0));
        let formated_ntransform = transform.rotation.to_rotation_matrix().into_inner();

        self.transform.upload(&formated_transform);
        self.ntransform.upload(&formated_ntransform);

        mesh.bind(&mut self.pos, &mut self.normal, &mut self.tex_coord);

        let lighting_rc = data
            .user_data()
            .downcast_ref::<Rc<RefCell<BodyLightingData>>>()
            .unwrap();
        let lighting = (**lighting_rc).borrow();

        ctxt.active_texture(Context::TEXTURE0);
        ctxt.bind_texture(
            Context::TEXTURE_2D,
            lighting.day_texture.as_ref().map(Rc::borrow),
        );
        ctxt.active_texture(Context::TEXTURE1);
        ctxt.bind_texture(
            Context::TEXTURE_2D,
            lighting.night_texture.as_ref().map(Rc::borrow),
        );

        self.day_color.upload(&lighting.day_color);
        self.night_color.upload(&lighting.night_color);
        // Associate day_tex with TEXTURE0 and night_tex with TEXTURE1.
        self.day_tex.upload(&0);
        self.night_tex.upload(&1);

        self.light_pos.upload(&lighting.light_pos);
        self.light_radius.upload(&lighting.light_radius);
        self.occluder_pos.upload(&lighting.occluder_pos);
        self.occluder_radius.upload(&lighting.occluder_radius);

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

const OBJECT_VERTEX_SRC: &str = include_str!("shadow_material.vert");

const OBJECT_FRAGMENT_SRC: &str = include_str!("shadow_material.frag");
