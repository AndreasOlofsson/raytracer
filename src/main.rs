#[macro_use]
extern crate conrod_core;
#[macro_use]
extern crate conrod_winit;

mod ray;
mod camera;
mod object;
mod light;
mod scene;
mod material;
mod color;
mod math;

use ray::Ray;
use camera::Camera;
use object::{ Object, HitRecord, Sphere, };
use light::Light;
use scene::Scene;
use color::RGB;
use material::Material;

mod ui;

fn main() {
    let cam = Camera::look_at(
        math::Vec3::new(0.0, 0.0, -5.0),
        math::Vec3::new(0.0, 0.0, 0.0),
        math::Vec3::new(0.0, 1.0, 0.0),
        60.0,
        1,
        1,
    );

    let mut scene = Scene::new(
        RGB::new(0.5, 0.5, 0.5),
        cam,
        vec![
            // Ground
            Box::from(Sphere::new(
                math::Vec3::new(0.0, -1001.0, 0.0),
                1000.0,
                Material {
                    color: RGB::new(0.2, 0.2, 0.8),
                    reflectivity: 1.0
                },
            )),
            Box::from(Sphere::new(
                math::Vec3::new(-2.0, 0.0, 0.0),
                1.0,
                Material::diffuse(RGB::new(0.2, 1.0, 0.2))
            )),
            Box::from(Sphere::new(
                math::Vec3::new(0.0, 0.0, 0.0),
                1.0,
                Material {
                    color: RGB::new(1.0, 0.2, 0.2),
                    reflectivity: 0.9,
                }
            )),
        ],
        vec![
            Light::Hemi(light::Hemi::new(
                math::Vec3::new(-1.0, -1.0, 1.0), 
                RGB::new(0.5, 0.5, 0.5),
            )),
        ],
    );

    ui::ui_main(|(w, h), pixels| {
        if scene.camera().width() != w as usize || scene.camera().height() != h as usize
        {
            scene.camera().set_w_h((w as usize, h as usize));
            scene.iteration = 0;
        }

        scene.render(pixels);

        true
    });
}