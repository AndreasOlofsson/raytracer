// #[macro_use]
// extern crate conrod_core;
// #[macro_use]
// extern crate conrod_winit;

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
                Material::reflective(
                    RGB::new(0.2, 0.2, 0.8),
                    0.6,
                ),
            )),
            // Left sphere
            Box::from(Sphere::new(
                math::Vec3::new(-2.0, 0.0, 0.0),
                1.0,
                Material::diffuse(RGB::new(0.2, 1.0, 0.2))
            )),
            // Center sphere
            Box::from(Sphere::new(
                math::Vec3::new(0.0, 0.0, 0.0),
                1.0,
                Material::reflective(
                    RGB::new(1.0, 0.2, 0.2),
                    1.0,
                ),
            )),
            // Right sphere
            Box::from(Sphere::new(
                math::Vec3::new(2.0, 0.0, 0.0),
                1.0,
                Material::reflective(
                    RGB::new(1.0, 0.2, 1.0),
                    0.8,
                ),
            )),
            // Front-left sphere
            Box::from(Sphere::new(
                math::Vec3::new(-0.75, -0.5, -1.5),
                0.5,
                Material::transparent(
                    RGB::new(1.0, 1.0, 1.0),
                    1.0,
                    0.1,
                    0.64,
                ),
            )),
            // Front-right sphere
            Box::from(Sphere::new(
                math::Vec3::new(0.75, -0.5, -1.5),
                0.5,
                Material::transparent(
                    RGB::new(1.0, 1.0, 1.0),
                    0.95,
                    0.05,
                    0.3,
                ),
            )),
        ],
        vec![
            Light::Hemi(light::Hemi::new(
                math::Vec3::new(-1.0, -1.0, 1.5),
                RGB::new(0.6, 0.6, 0.6),
            )),
        ],
    );

    let mut iteration = 0;
    let mut line = 0;

    ui::ui_main(|(w, h), pixels| {
        if scene.camera().width() != w as usize || scene.camera().height() != h as usize
        {
            scene.camera().set_w_h((w as usize, h as usize));
            iteration = 0;
            line = 0;
        }

        for _ in 0..50
        {
            for (x, ray) in scene.camera().line_rays(line).into_iter().enumerate()
            {
                let color = scene.trace_ray(ray, false, 10);
                let color = color.gamma(0.45);
                let start = (x + line * w as usize) * 3;

                pixels[start + 0] = (pixels[start + 0] * iteration as f32 + color.r) / (iteration as f32 + 1.0);
                pixels[start + 1] = (pixels[start + 1] * iteration as f32 + color.g) / (iteration as f32 + 1.0);
                pixels[start + 2] = (pixels[start + 2] * iteration as f32 + color.b) / (iteration as f32 + 1.0);
            }

            if line + 1 >= h as usize
            {
                line = 0;
                iteration += 1;
            }
            else
            {
                line += 1;
            }
        }

        true
    });
}