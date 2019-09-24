mod settings;

pub use settings::{WindowSettings, WindowSize};

use crate::runner::Event;

pub struct Window {
    event_loop: glutin::event_loop::EventLoop<Event>,
    display: glium::Display,
}

impl Window {
    pub fn new(settings: &WindowSettings) -> Window {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();
        let context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_depth_buffer(24)
            .with_multisampling(4)
            .with_srgb(true);
        let mut builder = glutin::window::WindowBuilder::new().with_title("Labyrinth");
        if let Some(size) = settings.size {
            builder = builder.with_inner_size(size.into());
        }

        let display = glium::Display::new(builder, context, &event_loop).unwrap();

        Window {
            event_loop,
            display,
        }
    }

    pub fn get_event_proxy(&self) -> glutin::event_loop::EventLoopProxy<Event> {
        self.event_loop.create_proxy()
    }

    pub fn run_event_loop<F>(self, handler: F) -> !
    where
        F: 'static
            + FnMut(
                glutin::event::Event<Event>,
                &glutin::event_loop::EventLoopWindowTarget<Event>,
                &mut glutin::event_loop::ControlFlow,
            ),
    {
        self.event_loop.run(handler);
    }

    pub fn display(&self) -> &glium::Display {
        &self.display
    }

    pub fn event_loop(&self) -> &glutin::event_loop::EventLoop<Event> {
        &self.event_loop
    }

    pub fn take(self) -> (glium::Display, glutin::event_loop::EventLoop<Event>) {
        (self.display, self.event_loop)
    }

    pub fn center(&self) {
        let display = self.display.gl_window();
        let window = display.window();
        let monitor = window.primary_monitor();
        window.set_maximized(false);
        let outer_size = window.outer_size();
        let dpi = monitor.hidpi_factor();
        let monitor_pos = monitor.position().to_logical(dpi);
        let monitor_size = monitor.size().to_logical(dpi);
        window.set_outer_position(glutin::dpi::LogicalPosition::from((
            monitor_pos.x + monitor_size.width / 2.0 - outer_size.width / 2.0,
            monitor_pos.y + monitor_size.height / 2.0 - outer_size.height / 2.0,
        )))
    }
}
