use glutin::dpi::LogicalSize;

#[derive(Copy, Clone)]
pub struct WindowSize {
    inner: LogicalSize,
}

impl WindowSize {
    pub fn new(width: u32, height: u32) -> WindowSize {
        WindowSize {
            inner: LogicalSize::from((width, height)),
        }
    }
}

impl Into<LogicalSize> for WindowSize {
    fn into(self) -> LogicalSize {
        self.inner
    }
}

#[derive(Copy, Clone)]
pub struct WindowSettings {
    pub size: Option<WindowSize>,
}

impl WindowSettings {
    pub fn new() -> WindowSettings {
        WindowSettings { size: None }
    }
    pub fn with_size(mut self, size: WindowSize) -> WindowSettings {
        self.size = Some(size);
        self
    }
}
