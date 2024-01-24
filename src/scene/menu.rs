use crate::UGameState;

use super::Scene;

#[derive(Copy, Clone)]
pub struct MenuScene {}

impl Scene for MenuScene {
    fn new(_ctx: &mut ggez::Context) -> Self {
        Self {}
    }

    fn update(
        &mut self,
        //scenes: &mut super::Manager,
        state: &mut UGameState,
        ctx: &mut ggez::Context,
    ) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, state: &mut UGameState, ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
}
