use ggez::{
    graphics::{self, Canvas, Color},
    Context,
};

use crate::{enemy, UGame};

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

pub fn update(game: &mut UGame, ctx: &mut Context) {
    // Check ball collision with screen edges
    game.ball.position_x += game.ball.direction_x * (500.0 * ctx.time.delta().as_secs_f32());
    game.ball.position_y += game.ball.direction_y * (500.0 * ctx.time.delta().as_secs_f32());

    if game.ball.position_x - game.ball.radius <= 0.0
        || game.ball.position_x + game.ball.radius >= ctx.gfx.window().inner_size().width as f32
    {
        game.ball.direction_x = -game.ball.direction_x;
        return;
    }

    if game.ball.position_y - game.ball.radius <= 0.0
        || game.ball.position_y + game.ball.radius >= ctx.gfx.window().inner_size().height as f32
    {
        game.ball.direction_y = -game.ball.direction_y;
        return;
    }
    ////////////////////////////////////////////////////////////////////////
    // Check ball collision with player
    let is_in_x = game.ball.position_x >= game.player.position_x
        && game.ball.position_x <= game.player.position_x + game.player.width;

    if game.ball.position_y + game.ball.radius >= game.player.position_y && is_in_x {
        game.ball.direction_y = -game.ball.direction_y;
        return;
    }

    ////////////////////////////////////////////////////////////////////////
    // Check ball collision with enemy

    let mut i = 0;
    while i < game.enemies.len() {
        let entitie = &mut game.enemies[i];
        let is_in_x = game.ball.position_x >= entitie.position_x
            && game.ball.position_x <= entitie.position_x + enemy::WIDTH;

        let is_in_y = game.ball.position_y >= entitie.position_y
            && game.ball.position_y <= entitie.position_y + enemy::HEIGHT;

        if is_in_x {
            let distance_bot = f32::abs(
                (game.ball.position_y - game.ball.radius) - (entitie.position_y + enemy::HEIGHT),
            );

            let distance_top =
                f32::abs((game.ball.position_y + game.ball.radius) - (entitie.position_y));

            if game.ball.position_y + game.ball.radius >= entitie.position_y
                && game.ball.position_y - game.ball.radius <= entitie.position_y + enemy::HEIGHT
            {
                if (distance_bot < distance_top) {
                    game.ball.direction_y = -game.ball.direction_y;
                    println!("Hit bot");
                    return;
                } else {
                    game.ball.direction_y = -game.ball.direction_y;
                    println!("hit top");
                    return;
                }
            }

            // Problema: mesmo se tiver acima vai estar como se hitou top.
            // Descobrir saber se bateu na parte de cima ou debaixo.

            //let hit_top = game.ball.position_y + game.ball.radius
        }

        if is_in_y {
            let distance_left =
                f32::abs((game.ball.position_x + game.ball.radius) - (entitie.position_x));

            let distance_right = f32::abs(
                (game.ball.position_x - game.ball.radius) - (entitie.position_x + enemy::WIDTH),
            );

            if game.ball.position_x + game.ball.radius >= entitie.position_x
                && game.ball.position_x - game.ball.radius <= entitie.position_x
            {
                if (distance_left < distance_right) {
                    game.ball.direction_x = -game.ball.direction_x;
                    println!("Hit righjt");
                    return;
                } else {
                    game.ball.direction_x = -game.ball.direction_x;
                    println!("hit left");
                    return;
                }
            }
        }

        i += 1;
    }
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
