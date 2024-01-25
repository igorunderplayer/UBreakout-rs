use ggez::graphics::{self, Color};

use crate::UGameState;

use super::Scene;

#[derive(Clone, Copy)]
pub struct MenuScene {}

impl Scene for MenuScene {
    fn new(_ctx: &mut ggez::Context) -> Self {
        Self {}
    }

    fn update(
        &mut self,
        scene_manager: &mut super::Manager,
        state: &mut UGameState,
        ctx: &mut ggez::Context,
    ) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, state: &mut UGameState, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(30.0, 30.0, 200.0, 200.0),
            Color::WHITE,
        )
        .expect("Error creating player mesh");

        canvas.draw(&player_mesh, graphics::DrawParam::default());
        canvas.finish(ctx)
    }
}
