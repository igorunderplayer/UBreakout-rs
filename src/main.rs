use std::thread::sleep;
use std::time::Duration;
use std::vec;

use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{event, graphics, Context, ContextBuilder, GameResult};

mod ball;
mod enemy;
mod player;

use ball::BallData;
use enemy::{EnemyData, EnemyTier};
use player::PlayerData;

struct UGame {
    player: PlayerData,
    points: i32,
    enemies: Vec<EnemyData>,
    ball: BallData,
}

impl UGame {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            player: PlayerData::new(),
            points: 0,
            enemies: vec![
                EnemyData::new(60.0, 60.0, EnemyTier::Normal),
                EnemyData::new(140.0, 60.0, EnemyTier::Normal),
                EnemyData::new(260.0, 60.0, EnemyTier::Normal),
                EnemyData::new(380.0, 60.0, EnemyTier::Normal),
                EnemyData::new(500.0, 60.0, EnemyTier::Normal),
            ],
            ball: BallData::new(),
        }
    }
}

impl EventHandler for UGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player.position_y = ctx.gfx.window().inner_size().height as f32 - 30.0;

        ball::update(self, ctx);

        if ctx
            .keyboard
            .is_key_pressed(ggez::input::keyboard::KeyCode::D)
        {
            self.player.position_x += 250.0 * ctx.time.delta().as_secs_f32();
        }

        if ctx
            .keyboard
            .is_key_pressed(ggez::input::keyboard::KeyCode::A)
        {
            self.player.position_x -= 250.0 * ctx.time.delta().as_secs_f32();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Draw FPS text

        #[cfg(debug_assertions)]
        {
            let fps = ctx.time.fps();
            let fps_text = graphics::Text::new(format!("FPS: {}", fps));
            canvas.draw(&fps_text, graphics::DrawParam::default());
        }

        // Player draw logic
        player::draw(ctx, &mut canvas, &self.player);

        // Ball draw logic
        ball::draw(ctx, &mut canvas, &self.ball);

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
        .window_mode(ggez::conf::WindowMode::default().resizable(false))
        .build()
        .expect("Couldn't create ggez context");

    let ugame = UGame::new(&mut ctx);

    event::run(ctx, event_loop, ugame);
}
