use std::{
    f32::consts::{FRAC_PI_2, PI},
    path::Path,
};

use kiss3d::{
    nalgebra::{Translation3, UnitQuaternion, Vector3},
    resource::{MaterialManager, TextureManager},
    scene::SceneNode,
    window::Window,
};

pub struct Skybox {
    node: SceneNode,
}

impl Skybox {
    pub fn new(window: &mut Window, size: f32) -> Self {
        //return Self { node: window.add_group(), };
        let half_size = size * 0.5;
        let mut node = window.add_group();

        TextureManager::get_global_manager(|tm| tm.set_generate_mipmaps(false));
        println!("Loading skybox textures");
        let path = |suffix: &str| format!("./media/skybox_{}.png", suffix);
        //let path = |suffix: &str| format!("./media/test_{}.png", suffix);

        let mut up = node.add_quad(size, size, 1, 1);
        up.set_local_translation(Translation3::new(0.0, 0.0, half_size));
        up.set_local_rotation(UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI));
        up.set_texture_from_file(Path::new(&path("up")), "skybox-up");

        let mut down = node.add_quad(size, size, 1, 1);
        down.set_local_translation(Translation3::new(0.0, 0.0, -half_size));
        let down_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), PI);
        down.set_local_rotation(down_rot);
        down.set_texture_from_file(Path::new(&path("down")), "skybox-down");

        let mut front = node.add_quad(size, size, 1, 1);
        let front_rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), FRAC_PI_2) * down_rot;
        front.set_local_rotation(front_rot);
        front.set_local_translation(Translation3::new(0.0, half_size, 0.0));
        front.set_texture_from_file(Path::new(&path("front")), "skybox-front");

        let mut left = node.add_quad(size, size, 1, 1);
        left.set_local_rotation(
            UnitQuaternion::from_axis_angle(&Vector3::z_axis(), FRAC_PI_2) * front_rot,
        );
        left.set_local_translation(Translation3::new(-half_size, 0.0, 0.0));
        left.set_texture_from_file(Path::new(&path("left")), "skybox-left");

        let mut right = node.add_quad(size, size, 1, 1);
        right.set_local_rotation(
            UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -FRAC_PI_2) * front_rot,
        );
        right.set_local_translation(Translation3::new(half_size, 0.0, 0.0));
        right.set_texture_from_file(Path::new(&path("right")), "skybox-right");

        let mut back = node.add_quad(size, size, 1, 1);
        back.set_local_rotation(
            UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI) * front_rot,
        );
        back.set_local_translation(Translation3::new(0.0, -half_size, 0.0));
        back.set_texture_from_file(Path::new(&path("back")), "skybox-back");

        let mat = MaterialManager::get_global_manager(|m| m.get("flat").unwrap());

        for mut n in [up, down, front, left, right, back] {
            n.set_material(mat.clone());
            n.set_color(0.5, 0.5, 0.5);
        }

        Self { node }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.node.set_visible(visible);
    }

    pub fn is_visible(&self) -> bool {
        self.node.is_visible()
    }
}
