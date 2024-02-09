use ggez::event::EventHandler;
use ggez::{event, Context, ContextBuilder, GameResult};

mod ball;
mod data;
mod enemy;
mod input;
mod player;
mod scene;
mod ui;
struct UGameState {}

struct UGame {
    scenes: scene::Manager,
    state: UGameState,
}

impl UGame {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            scenes: scene::Manager::new(ctx),
            state: UGameState {},
        }
    }
}

impl EventHandler for UGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.scenes.update(&mut self.state, ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.scenes.draw(&mut self.state, ctx)
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("UGame", "igor_underplayer")
        .window_setup(ggez::conf::WindowSetup::default().title("UGame"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .resizable(false)
                .dimensions(800.0, 600.0),
        )
        .build()
        .expect("Couldn't create ggez context");

    let ugame = UGame::new(&mut ctx);

    event::run(ctx, event_loop, ugame);
}
