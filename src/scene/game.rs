use ggez::graphics::{self, Color};

use crate::{
    ball::{self, BallData},
    data::{self, LevelData},
    enemy, input,
    player::{self, PlayerData},
    UGameState,
};

use super::Scene;

#[derive(Clone)]
pub struct GameScene {
    player: PlayerData,
    ball: BallData,
    actual_wave: usize,
    level: LevelData,
}

impl Scene for GameScene {
    fn new(_ctx: &mut ggez::Context, state: &mut UGameState) -> Self {
        let json_string = include_str!("../level1.json");
        let coisa = data::load_from_json(json_string);
        Self {
            player: PlayerData::new(),
            ball: BallData::new(),
            level: coisa,
            actual_wave: 0,
        }
    }

    fn on_create(&mut self, state: &mut UGameState) -> ggez::GameResult {
        if let Some(level) = &state.level {
            println!("Starting dynamic level");

            self.level = level.clone()
        }
        Ok(())
    }
    fn update(
        &mut self,
        scene_manager: &mut super::Manager,
        state: &mut UGameState,
        ctx: &mut ggez::Context,
    ) -> ggez::GameResult {
        self.player.position_y = ctx.gfx.window().inner_size().height as f32 - 30.0;

        ball::update(
            &mut self.ball,
            &mut self.player,
            &mut self.level.waves[self.actual_wave].enemies,
            ctx,
        );
        input::update(&mut self.player, ctx);

        Ok(())
    }

    fn draw(&mut self, state: &mut UGameState, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        player::draw(ctx, &mut canvas, &self.player);
        ball::draw(ctx, &mut canvas, &self.ball);

        for enemy in &self.level.waves[self.actual_wave].enemies {
            enemy::draw(ctx, &mut canvas, &enemy)
        }

        // Draw FPS text in screen if running in debug mode
        #[cfg(debug_assertions)]
        {
            let fps = ctx.time.fps();
            let fps_text = graphics::Text::new(format!("FPS: {}", fps));
            canvas.draw(&fps_text, graphics::DrawParam::default());
        }

        canvas.finish(ctx)
    }
}
