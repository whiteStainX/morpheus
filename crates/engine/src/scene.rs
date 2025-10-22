use crate::render::TerminalRenderer;

pub struct Context<'a> {
    pub renderer: &'a mut TerminalRenderer,
    // pub input: &'a InputState, // Will be added later
}

pub trait Scene {
    fn on_start(&mut self, ctx: &mut Context);
    fn on_update(&mut self, ctx: &mut Context, dt: f32);
    fn on_draw(&mut self, ctx: &mut Context);
    // fn on_exit(&mut self, ctx: &mut Context); // Optional, for cleanup
}
