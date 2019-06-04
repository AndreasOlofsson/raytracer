use glium::{
    glutin,
    texture::Texture2d,
    texture::RawImage2d,
    framebuffer::SimpleFrameBuffer,
    Surface,
};

pub fn ui_main<F>(mut render: F)
where F: FnMut((u32, u32), &mut [f32]) -> bool
{
    let mut width  = 640;
    let mut height = 480;

    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_dimensions((width, height).into())
        .with_title("Raytracer");
    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut pixels = vec![];
    let mut tex_frame = FrameBufferTexture::empty(&display, width, height);

    let mut close_requested = false;
    while !close_requested
    {
        let mut new_size = None;

        events_loop.poll_events(|event| {
            match event
            {
                glutin::Event::WindowEvent { event, .. } => {
                    match event
                    {
                        glutin::WindowEvent::CloseRequested => {
                            close_requested = true;
                        },
                        glutin::WindowEvent::Resized(size) => {
                            new_size = Some(size.into());
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        });

        if let Some((new_width, new_height)) = new_size
        {
            width = new_width;
            height = new_height;

            pixels = vec![0.0; width as usize * height as usize * 3];
            
            tex_frame = FrameBufferTexture::empty(&display, width, height);
        }

        if render((width, height), &mut pixels)
        {
            let target = display.draw();

            tex_frame.texture().write(
                glium::Rect {
                    left: 0,
                    bottom: 0,
                    width: width,
                    height: height,
                },
                RawImage2d::from_raw_rgb_reversed(
                    &pixels,
                    (width, height)
                )
            );
            tex_frame.framebuffer().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);

            target.finish().unwrap();
        }
    }
}

pub struct FrameBufferTexture<'a>
{
    texture: *mut Texture2d,
    framebuffer: SimpleFrameBuffer<'a>,
}

impl<'a> FrameBufferTexture<'a>
{
    pub fn empty<F>(facade: &F, width: u32, height: u32) -> FrameBufferTexture<'a>
    where F: glium::backend::Facade
    {
        unsafe
        {
            let texture = Box::into_raw(Box::from(
                Texture2d::empty(facade, width, height).unwrap()
            ));

            let framebuffer = SimpleFrameBuffer::new(facade, &*texture).unwrap();

            FrameBufferTexture {
                texture,
                framebuffer,
            }
        }
    }

    pub fn texture(&self) -> &Texture2d
    {
        unsafe
        {
            &*self.texture
        }
    }

    pub fn framebuffer(&self) -> &SimpleFrameBuffer<'a>
    {
        &self.framebuffer
    }
}

impl<'a> Drop for FrameBufferTexture<'a>
{
    fn drop(&mut self)
    {
        unsafe
        {
            Box::from_raw(self.texture);
        }
    }
}