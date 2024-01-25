use ggez::{
    graphics::{self, Canvas, Color},
    Context,
};

use crate::{
    enemy::{self, EnemyData},
    player::PlayerData,
};

pub const SPEED: f32 = 200.0;

#[derive(Copy, Clone)]
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
}

pub fn update(
    ball: &mut BallData,
    player: &mut PlayerData,
    enemies: &mut Vec<EnemyData>,
    ctx: &mut Context,
) {
    ball.position_x += ball.direction_x * (SPEED * ctx.time.delta().as_secs_f32());
    ball.position_y += ball.direction_y * (SPEED * ctx.time.delta().as_secs_f32());

    check_collision(ball, enemies, player, ctx);
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

fn check_collision(
    ball: &mut BallData,
    enemies: &mut Vec<EnemyData>,
    player: &mut PlayerData,
    ctx: &mut Context,
) {
    ////////////////////////////////////////////////////////////////////////
    // Check ball collision with screen edges
    // Left edge
    if ball.position_x - ball.radius <= 0.0 {
        ball.direction_x = 1.0;
    }
    // Right edge
    if ball.position_x + ball.radius >= ctx.gfx.window().inner_size().width as f32 {
        ball.direction_x = -1.0;
    }
    // Top edge
    if ball.position_y - ball.radius <= 0.0 {
        ball.direction_y = 1.0;
    }
    // Bottom edge
    if ball.position_y + ball.radius >= ctx.gfx.window().inner_size().height as f32 {
        ball.direction_y = -1.0;
    }
    //
    ////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////
    // Check ball collision with player
    let is_in_x =
        ball.position_x >= player.position_x && ball.position_x <= player.position_x + player.width;

    if ball.position_y + ball.radius >= player.position_y && is_in_x {
        ball.direction_y = -ball.direction_y;
        return;
    }
    //
    ////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////
    // Check ball collision with enemy
    let mut i = 0;
    while i < enemies.len() {
        let entitie = &mut enemies[i];
        let is_in_x = ball.position_x >= entitie.position_x
            && ball.position_x <= entitie.position_x + enemy::WIDTH;

        let is_in_y = ball.position_y >= entitie.position_y
            && ball.position_y <= entitie.position_y + enemy::HEIGHT;

        if is_in_x {
            let distance_bot =
                f32::abs((ball.position_y - ball.radius) - (entitie.position_y + enemy::HEIGHT));

            let distance_top = f32::abs((ball.position_y + ball.radius) - (entitie.position_y));

            if ball.position_y + ball.radius >= entitie.position_y
                && ball.position_y - ball.radius <= entitie.position_y + enemy::HEIGHT
            {
                if distance_bot < distance_top {
                    ball.direction_y = -ball.direction_y;
                    println!("Hit bot");
                    enemies.remove(i);
                    return;
                } else {
                    ball.direction_y = -ball.direction_y;
                    println!("hit top");
                    enemies.remove(i);
                    return;
                }
            }
        }

        if is_in_y {
            let distance_left = f32::abs((ball.position_x + ball.radius) - (entitie.position_x));

            let distance_right =
                f32::abs((ball.position_x - ball.radius) - (entitie.position_x + enemy::WIDTH));

            if ball.position_x + ball.radius >= entitie.position_x
                && ball.position_x - ball.radius <= entitie.position_x
            {
                if distance_left < distance_right {
                    ball.direction_x = -ball.direction_x;
                    println!("Hit right");
                    enemies.remove(i);
                    return;
                } else {
                    ball.direction_x = -ball.direction_x;
                    println!("hit left");
                    enemies.remove(i);
                    return;
                }
            }
        }

        i += 1;
    }
    //
    ////////////////////////////////////////////////////////////////////////
}
