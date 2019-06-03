use crate::{
    Camera,
    Object,
    Light,
    Ray,
    HitRecord,
    RGB,
    math::Vec3,
};

pub struct Scene
{
    pub sky: RGB,
    pub camera: Camera,
    pub objects: Vec<Box<Object>>,
    pub lights: Vec<Light>,
    pub iteration: u32,
    pub rng: rand::rngs::StdRng,
}

impl Scene
{
    pub fn new(sky: RGB, camera: Camera, objects: Vec<Box<Object>>, lights: Vec<Light>) -> Scene
    {
        use rand::FromEntropy;

        Scene {
            sky,
            camera,
            objects,
            lights,
            iteration: 0,
            rng: rand::rngs::StdRng::from_entropy(),
        }
    }

    pub fn camera(&mut self) -> &mut Camera
    {
        &mut self.camera
    }

    pub fn render(&mut self, pixels: &mut [f32])
    {
        let rays = self.camera.rays();

        self.rng = rand::SeedableRng::seed_from_u64(self.iteration as u64);

        for y in 0..self.camera.height()
        {
            for x in 0..self.camera.width()
            {
                // TODO optimize blending
                let color = self.trace_ray(rays[x + y * self.camera.width()], 3);
                let start = (x + y * self.camera.width()) * 3;

                pixels[start + 0] = (pixels[start + 0] * self.iteration as f32 + color.r) / (self.iteration as f32 + 1.0);
                pixels[start + 1] = (pixels[start + 1] * self.iteration as f32 + color.g) / (self.iteration as f32 + 1.0);
                pixels[start + 2] = (pixels[start + 2] * self.iteration as f32 + color.b) / (self.iteration as f32 + 1.0);
            }
        }

        self.iteration += 1;
    }

    fn trace_ray(&mut self, ray: Ray, rem_bounces: u32) -> RGB
    {
        if rem_bounces <= 0
        {
            return RGB::black();
        }

        if let Some(record) = self.hit(ray)
        {
            let hit_point = ray.point_at_dist(record.offset);

            let mut color = RGB::black();

            for light in self.lights.iter()
            {
                match light
                {
                    Light::Hemi(hemi) => {
                        if let None = self.hit(Ray::new(hit_point, -hemi.direction))
                        {
                            color += hemi.color * (-record.normal.dot(hemi.direction)).max(0.0) as f32; // TODO diffuse using reflectivity
                        }
                    },
                }
            }

            if record.material.reflectivity == 1.0
            {
                color += self.trace_ray(ray.reflect_at(record.offset, record.normal), rem_bounces - 1);
            }
            else if record.material.reflectivity == 0.0
            {
                let dir = Vec3::random_half_sphere(&mut self.rng, record.normal);
                // TODO reflectivity bias
                color += self.trace_ray(Ray::new(hit_point, dir), rem_bounces - 1);
            }
            else
            {
                let diffuse = Vec3::random_half_sphere(&mut self.rng, record.normal);
                let reflective = ray.dir.reflect(record.normal);

                let dir = (diffuse * (1.0 - record.material.reflectivity as f64) + reflective * record.material.reflectivity as f64).normalized();

                color += self.trace_ray(Ray::new(hit_point, dir), rem_bounces - 1);
            }

            color * record.material.color
        }
        else
        {
            self.sky
        }
    }

    fn hit(&self, ray: Ray) -> Option<HitRecord>
    {
        let mut nearest = std::f64::INFINITY;
        let mut current_record = None;

        for object in self.objects.iter()
        {
            if let Some(record) = object.hit(ray, (0.001, std::f64::INFINITY))
            {
                if record.offset < nearest
                {
                    nearest = record.offset;
                    current_record = Some(record);
                }
            }
        }

        current_record
    }
}