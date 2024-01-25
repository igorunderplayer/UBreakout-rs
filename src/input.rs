use ggez::Context;

use crate::player::PlayerData;

pub fn update(player: &mut PlayerData, ctx: &mut Context) {
    if ctx
        .keyboard
        .is_key_pressed(ggez::input::keyboard::KeyCode::D)
    {
        player.position_x += 250.0 * ctx.time.delta().as_secs_f32();
    }

    if ctx
        .keyboard
        .is_key_pressed(ggez::input::keyboard::KeyCode::A)
    {
        player.position_x -= 250.0 * ctx.time.delta().as_secs_f32();
    }
}
