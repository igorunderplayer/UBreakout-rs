use ggez::{
    graphics::{self, Canvas, Color},
    Context,
};

pub struct PlayerData {
    pub position_x: f32,
}

impl PlayerData {
    pub fn new() -> Self {
        Self { position_x: 500.0 }
    }
}

pub fn draw(ctx: &mut Context, canvas: &mut Canvas, player: &PlayerData) {
    let player_y: f32 = ctx.gfx.window().inner_size().height as f32 - 30.0;
    let player_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(player.position_x, player_y, 50.0, 20.0),
        Color::WHITE,
    )
    .expect("Error creating player mesh");

    canvas.draw(&player_mesh, graphics::DrawParam::default());
}
