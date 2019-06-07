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
            rng: rand::rngs::StdRng::from_entropy(),
        }
    }

    pub fn camera(&mut self) -> &mut Camera
    {
        &mut self.camera
    }

    pub fn trace_ray(&mut self, ray: Ray, inside: bool, rem_bounces: u32) -> RGB
    {
        if rem_bounces <= 0
        {
            return RGB::black();
        }

        if let Some(record) = self.hit(ray)
        {
            let hit_point = ray.point_at_dist(record.offset);

            if inside
            {
                let dir = ray.dir.refract(-record.normal, 1.0 / record.material.r_index as f64).unwrap_or_else(|| ray.dir.reflect(-record.normal));

                // TODO roughness on ray exit
                // TODO total internal reflection
                return self.trace_ray(Ray::new(hit_point, dir), false, rem_bounces);
            }

            let mut color = RGB::black();

            for light in self.lights.iter()
            {
                match light
                {
                    Light::Hemi(hemi) => {
                        if record.material.reflectivity != 1.0
                        {
                            if let None = self.hit(Ray::new(hit_point, -hemi.direction))
                            {
                                let mut intensity = (-record.normal.dot(hemi.direction)).max(0.0) as f32;

                                if record.material.reflectivity != 0.0
                                {
                                    intensity = intensity.powf(1.0 / (1.0 - record.material.reflectivity));
                                    intensity *= (record.material.reflectivity - 2.0) / (record.material.reflectivity - 1.0);
                                }

                                color += hemi.color * intensity; // TODO diffuse using reflectivity
                            }
                        }
                    },
                }
            }

            if record.material.reflectivity == 1.0
            {
                color += self.trace_ray(ray.reflect_at(record.offset, record.normal), false, rem_bounces - 1);
            }
            else if record.material.reflectivity == 0.0
            {
                let dir = Vec3::random_half_sphere(&mut self.rng, record.normal);
                color += self.trace_ray(Ray::new(hit_point, dir), false, rem_bounces - 1);
            }
            else
            {
                let diffuse = Vec3::random_half_sphere(&mut self.rng, record.normal);
                let reflective = ray.dir.reflect(record.normal);

                let dir = (diffuse * (1.0 - record.material.reflectivity as f64) + reflective * record.material.reflectivity as f64).normalized();

                color += self.trace_ray(Ray::new(hit_point, dir), false, rem_bounces - 1);
            }


            if record.material.opacity < 1.0
            {
                let refract_dir = ray.dir.refract(record.normal, record.material.r_index as f64);

                if record.material.reflectivity == 1.0
                {
                    let inner_color = self.trace_ray(Ray::new(hit_point, ray.dir.refract(record.normal, record.material.r_index as f64)), true, rem_bounces - 1);

                    color = color * record.material.opacity + inner_color * (1.0 - record.material.opacity);
                }
                else
                {
                    let random_dir = Vec3::random_half_sphere(&mut self.rng, -record.normal);
                    let dir = (random_dir * (1.0 - record.material.reflectivity as f64) + refract_dir * record.material.reflectivity as f64).normalized();

                    let inner_color = self.trace_ray(Ray::new(hit_point, dir), true, rem_bounces - 1);

                    color = color * record.material.opacity + inner_color * (1.0 - record.material.opacity);
                }
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