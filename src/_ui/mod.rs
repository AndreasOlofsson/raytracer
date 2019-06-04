mod winit_conv;

mod debug_box;

use conrod_core::{
    widget,
    Widget,
    Positionable,
    Sizeable,
};
use glium::Surface;

use debug_box::SettingsBox;

widget_ids! {
    struct Ids {
        background,
        settings_box,
    }
}

pub fn ui_main<F>(mut render: F)
where F: FnMut((u32, u32), &mut [f32]) -> bool
{
    let mut width: u32 = 640;
    let mut height: u32 = 480;

    let window = glium::glutin::WindowBuilder::new()
        .with_title("Raytracer")
        .with_dimensions((width, height).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let mut events_loop = glium::glutin::EventsLoop::new();
    let display = WinitGliumDisplay(glium::Display::new(window, context, &events_loop).unwrap());

    let mut ui = conrod_core::UiBuilder::new([width as f64, height as f64]).build();

    let ids = Ids::new(ui.widget_id_generator());

    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut renderer = conrod_glium::Renderer::new(&*display).unwrap();

    let mut pixels: Vec<f32> = Vec::with_capacity((width * height * 3) as usize);

    pixels.extend(std::iter::repeat(0.5).take((width * height * 3) as usize));

    let mut raw_image = glium::texture::RawImage2d::from_raw_rgb_reversed(&pixels, (width, height));
    let mut texture = glium::texture::Texture2d::new(&display.0, raw_image).unwrap();
    let mut image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();
    let image = image_map.insert(texture);

    loop
    {
        let mut exit = false;
        let mut pending_resize = None;
        let mut handled_event = false;

        events_loop.poll_events(|event| {
            handled_event = true;

            use glium::glutin::{
                Event,
                WindowEvent,
                KeyboardInput,
                VirtualKeyCode,
            };

            match event.clone()
            {
                Event::WindowEvent { event, .. } => {
                    match event
                    {
                        WindowEvent::CloseRequested |
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => exit = true,
                        WindowEvent::Resized(size) => {
                            pending_resize = Some(size);
                        },
                        _ => (),
                    }
                },
                _ => (),
            }

            let input = if let Some(input) = winit_conv::convert_event(event, &display)
            {
                input
            }
            else
            {
                return;
            };

            ui.handle_event(input);
        });

        if exit
        {
            break;
        }

        if let Some(size) = pending_resize
        {
            width = size.width as u32;
            height = size.height as u32;

            let new_size = width as usize * height as usize * 3;

            pixels = vec![0.0; new_size];
        }

        if render((width, height), &mut pixels)
        {
            raw_image = glium::texture::RawImage2d::from_raw_rgb_reversed(&pixels, (width, height));
            texture = glium::texture::Texture2d::new(&display.0, raw_image).unwrap();
            image_map.replace(image, texture);

            println!("drawing");

            {
                let ui = &mut ui.set_widgets();
                let rect = ui.rect_of(ui.window).unwrap();

                widget::Image::new(image)
                    .w_h(rect.w(), rect.h())
                    .middle()
                    .set(ids.background, ui);

                // SettingsBox::new()
                //     .parent(ids.background)
                //     // .middle_of(ids.background)
                //     // .w_h(40.0, 40.0)
                //     .set(ids.settings_box, ui);
            }

            ui.needs_redraw();

            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&*display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&*display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }

    // events_loop.run_forever(|event| {
    //     match event.clone()
    //     {
    //         glium::glutin::Event::WindowEvent { event, .. } => {
    //             match event
    //             {
    //                 glium::glutin::WindowEvent::CloseRequested |
    //                 glium::glutin::WindowEvent::KeyboardInput {
    //                     input: glium::glutin::KeyboardInput {
    //                         virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
    //                         ..
    //                     },
    //                     ..
    //                 } => return glium::glutin::ControlFlow::Break,
    //                 glium::glutin::WindowEvent::Resized(_) => (),
    //                 _ => return glium::glutin::ControlFlow::Continue,
    //                 // _ => (),
    //             }
    //         },
    //         _ => return glium::glutin::ControlFlow::Continue,
    //     }

    //     let input = match winit_conv::convert_event(event, &display)
    //     {
    //         None => return glium::glutin::ControlFlow::Continue,
    //         Some(input) => input,
    //     };

    //     ui.handle_event(input);

    //     // Set the triangle widget.
    //     {
    //         let ui = &mut ui.set_widgets();
    //         let rect = ui.rect_of(ui.window).unwrap();
    //         let (l, r, b, t) = rect.l_r_b_t();
    //         let (c1, c2, c3) = (color::RED.to_rgb(), color::GREEN.to_rgb(), color::BLUE.to_rgb());

    //         let triangles = [
    //             Triangle([([l, b], c1), ([l, t], c2), ([r, t], c3)]),
    //             Triangle([([r, t], c1), ([r, b], c2), ([l, b], c3)]),
    //         ];

    //         // widget::Triangles::multi_color(triangles.iter().cloned())
    //         //     .with_bounding_rect(rect)
    //         //     .set(ids.triangles, ui);

    //         widget::Image::new(image)
    //             .w_h(rect.w(), rect.h())
    //             .middle()
    //             .set(ids.background, ui);

    //         // widget::Canvas::new()
    //         //     .color(conrod_core::color::DARK_BLUE)
    //         //     .set(ids.background, ui);

    //         SettingsBox::new()
    //             .parent(ids.background)
    //             // .middle_of(ids.background)
    //             // .w_h(40.0, 40.0)
    //             .set(ids.settings_box, ui);
    //     }

    //     // Draw the `Ui` if it has changed.
    //     if let Some(primitives) = ui.draw_if_changed() {
    //         renderer.fill(&*display, primitives, &image_map);
    //         let mut target = display.draw();
    //         target.clear_color(0.0, 0.0, 0.0, 1.0);
    //         renderer.draw(&*display, &mut target, &image_map).unwrap();
    //         target.finish().unwrap();
    //     }

    //     glium::glutin::ControlFlow::Continue
    // });
}

pub struct WinitGliumDisplay(glium::Display);

impl conrod_winit::WinitWindow for WinitGliumDisplay
{
    fn get_inner_size(&self) -> Option<(u32, u32)>
    {
        self.0.gl_window().get_inner_size().map(Into::into)
    }

    fn hidpi_factor(&self) -> f32
    {
        self.0.gl_window().get_hidpi_factor() as _
    }
}

impl std::ops::Deref for WinitGliumDisplay
{
    type Target = glium::Display;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}
