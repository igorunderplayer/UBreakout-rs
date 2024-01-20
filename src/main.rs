use std::vec;

use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{event, graphics, Context, ContextBuilder, GameResult};

mod enemy;
mod player;

use enemy::{EnemyData, EnemyTier};
use player::PlayerData;

struct UGame {
    player: PlayerData,
    points: i32,
    enemies: Vec<EnemyData>,
}

impl UGame {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            player: PlayerData::new(),
            points: 0,
            enemies: vec![EnemyData::new(20.0, 20.0, EnemyTier::Normal)],
        }
    }
}

impl EventHandler for UGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if _ctx
            .keyboard
            .is_key_pressed(ggez::input::keyboard::KeyCode::D)
        {
            self.player.position_x += 200.0 * _ctx.time.delta().as_secs_f32();
        }

        if _ctx
            .keyboard
            .is_key_pressed(ggez::input::keyboard::KeyCode::A)
        {
            self.player.position_x -= 200.0 * _ctx.time.delta().as_secs_f32();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Draw FPS text
        let fps = ctx.time.fps();
        let fps_text = graphics::Text::new(format!("FPS: {}", fps));
        canvas.draw(&fps_text, graphics::DrawParam::default());

        // Player draw logic
        player::draw(ctx, &mut canvas, &self.player);

        // Enemies draw logic
        for enemy in &self.enemies {
            enemy::draw(ctx, &mut canvas, &enemy)
        }

        canvas.finish(ctx)
    }
}

fn main() {
    println!("Hello, world!");

    let (mut ctx, event_loop) = ContextBuilder::new("UGame", "igor_underplayer")
        .window_setup(ggez::conf::WindowSetup::default().title("UGame"))
        .window_mode(ggez::conf::WindowMode::default().resizable(true))
        .build()
        .expect("Couldn't create ggez context");

    let ugame = UGame::new(&mut ctx);

    event::run(ctx, event_loop, ugame);
}
