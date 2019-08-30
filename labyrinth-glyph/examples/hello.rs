use glium::glutin::{Api, GlProfile, GlRequest};
use glium::{glutin, Surface};

use labyrinth_glyph::glyph_brush::{rusttype::Font, Section};
use labyrinth_glyph::GlyphBrush;

pub fn main() {
    let events_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)))
        .with_srgb(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let dejavu: &[u8] = include_bytes!("../fonts/DejaVuSans-2.37.ttf");
    let fonts = vec![Font::from_bytes(dejavu).unwrap()];

    let mut glyph_brush = GlyphBrush::new(&display, fonts);

    loop {
        let screen_dims = display.get_framebuffer_dimensions();

        glyph_brush.queue(Section {
            text: "Hello, World!",
            bounds: (screen_dims.0 as f32, screen_dims.1 as f32 / 2.0),
            ..Section::default()
        });
        glyph_brush.queue(Section {
            text: "This is in the middle of the screen",
            bounds: (screen_dims.0 as f32, screen_dims.1 as f32 / 2.0),
            screen_position: (0.0, screen_dims.1 as f32 / 2.0),
            scale: glyph_brush::rusttype::Scale::uniform(16.0),
            ..Section::default()
        });

        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 0.0), 1.0);
        glyph_brush.draw_queued(&display, &mut target);
        target.finish().unwrap();

        let mut exit = false;
        events_loop.run(move |event, _, _| match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => exit = true,
                _ => (),
            },
            _ => (),
        });
    }
}
