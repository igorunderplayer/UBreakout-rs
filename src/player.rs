use ggez::{
    graphics::{self, Canvas, Color},
    Context,
};

#[derive(Copy, Clone)]
pub struct PlayerData {
    pub position_x: f32,
    pub position_y: f32,
    pub width: f32,
    pub height: f32,
}

impl PlayerData {
    pub fn new() -> Self {
        Self {
            position_x: 500.0,
            position_y: 500.0,
            width: 100.0,
            height: 20.0,
        }
    }
}

pub fn draw(ctx: &mut Context, canvas: &mut Canvas, player: &PlayerData) {
    let player_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(
            player.position_x,
            player.position_y,
            player.width,
            player.height,
        ),
        Color::WHITE,
    )
    .expect("Error creating player mesh");

    canvas.draw(&player_mesh, graphics::DrawParam::default());
}
