use ggez::{
    graphics::{self, Canvas, Color},
    Context,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum EnemyTier {
    Normal,
}

pub const HEIGHT: f32 = 20.0;
pub const WIDTH: f32 = 50.0;

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct EnemyData {
    pub position_x: f32,
    pub position_y: f32,
    pub tier: EnemyTier,
}

impl EnemyData {
    pub fn new(position_x: f32, position_y: f32, tier: EnemyTier) -> Self {
        Self {
            position_x,
            position_y,
            tier,
        }
    }
}

pub fn draw(ctx: &mut Context, canvas: &mut Canvas, enemy: &EnemyData) {
    let enemy_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(enemy.position_x, enemy.position_y, WIDTH, HEIGHT),
        Color::WHITE,
    )
    .unwrap();

    canvas.draw(&enemy_mesh, graphics::DrawParam::default());
}
