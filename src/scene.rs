use crate::{
    Camera,
    Object,
    Ray,
    HitRecord,
    RGB,
};

pub struct Scene
{
    pub camera: Camera,
    pub objects: Vec<Box<Object>>,
}

impl Scene
{
    pub fn new(camera: Camera, objects: Vec<Box<Object>>) -> Scene
    {
        Scene {
            camera,
            objects,
        }
    }

    pub fn camera(&mut self) -> &mut Camera
    {
        &mut self.camera
    }

    pub fn render(&self, pixels: &mut [u8])
    {
        let rays = self.camera.rays();

        for y in 0..self.camera.height()
        {
            for x in 0..self.camera.width()
            {
                let color = self.trace_ray(rays[x + y * self.camera.width()], 3).as_u8();
                let start = (x + y * self.camera.width()) * 3;

                pixels[start + 0] = color[0];
                pixels[start + 1] = color[1];
                pixels[start + 2] = color[2];
            }
        }
    }

    fn trace_ray(&self, ray: Ray, rem_bounces: u32) -> RGB
    {
        if rem_bounces == 0
        {
            RGB::black()
        }
        else if let Some(record) = self.hit(ray)
        {
            record.material.color
        }
        else
        {
            RGB::black()
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