#[macro_use]
extern crate conrod_core;
#[macro_use]
extern crate conrod_winit;

mod ray;
mod camera;
mod object;
mod scene;
mod material;
mod util;
mod color;
mod math;

use ray::Ray;
use camera::Camera;
use object::{ Object, HitRecord, Sphere, };
use scene::Scene;
use color::RGB;
use material::Material;

mod ui;

fn main() {
    let cam = Camera::look_at(
        math::Vec3::new(0.0, 0.0, -5.0),
        math::Vec3::new(0.0, 0.0, 0.0),
        math::Vec3::new(0.0, 1.0, 0.0),
        90.0,
        50,
        50,
    );

    let mut scene = Scene::new(
        cam,
        vec![
            Box::from(Sphere::new(
                math::Vec3::new(0.0, 0.0, 0.0),
                1.0,
                Material::diffuse(RGB::new(1.0, 0.0, 0.0))
            )),
        ]
    );

    ui::ui_main(|(w, h), pixels| {
        scene.camera().set_w_h((w as usize, h as usize));
        scene.render(pixels);

        true
    });
}