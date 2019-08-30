use std::ops::Deref;

use glium::{
    backend::{Context, Facade},
    Surface,
};

use crate::game::context::SharedContext;
use crate::game::counter::Counter;
use crate::game::Game;
use crate::game::object::Object;

use labyrinth_glyph::glyph_brush::{rusttype::Font, Section};
use labyrinth_glyph::GlyphBrush;

use glyph_brush::{SectionText, VariedSection};

pub trait RenderTarget<T, F>
where
    T: Surface,
    F: Facade,
{
    fn render(&self, target: &mut T, facade: &F, context: SharedContext);
}

pub struct Renderer<'a> {
    pub counter: Counter,
    font: Option<GlyphBrush<'a, 'a>>,
}

impl<'a> Renderer<'a> {
    pub fn new() -> Renderer<'a> {
        Renderer {
            counter: Counter::new(),
            font: None,
        }
    }

    pub fn render<T, F>(&mut self, game: &Game, target: &mut T, facade: &F, context: SharedContext)
    where
        T: Surface,
        F: Facade + Deref<Target = Context>,
    {
        let model = context.borrow().get_object(&String::from("ObjPortal")).unwrap();
        model.borrow().render(target, facade, context);

        let mut brush = self.font.get_or_insert_with(|| {
            let font: &[u8] =
                include_bytes!("/home/pepijn/Projects/labyrinth/assets/ConnectionII.ttf");
            GlyphBrush::new(facade, vec![Font::from_bytes(font).unwrap()])
        });
        let screen_dims = target.get_dimensions();
        brush.queue(VariedSection {
            text: vec![SectionText {
                text: format!("fps: {:.1}", self.counter.get_rate()).as_str(),
                color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            }],
            bounds: (screen_dims.0 as f32, screen_dims.1 as f32),
            ..Default::default()
        });
        brush.draw_queued(facade, target);
        self.counter.count(|| {})
    }
}
