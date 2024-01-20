use ggez::{
    graphics::{self, Canvas, Color},
    mint::Point2,
    Context,
};

pub struct BallData {
    pub radius: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub direction_x: f32,
    pub direction_y: f32,
}

impl BallData {
    pub fn new() -> Self {
        Self {
            radius: 12.0,
            position_x: 500.0,
            position_y: 500.0,
            direction_x: -1.0,
            direction_y: -1.0,
        }
    }

    pub fn on_collide(self: &Self) {}
}

pub fn draw(ctx: &mut Context, canvas: &mut Canvas, ball: &BallData) {
    let ball_mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        ggez::mint::Point2 {
            x: ball.position_x,
            y: ball.position_y,
        },
        ball.radius,
        0.1,
        Color::YELLOW,
    )
    .expect("Couldn't create ball mesh");

    canvas.draw(&ball_mesh, graphics::DrawParam::default())
}
