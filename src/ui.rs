use ggez::graphics::{self, Color, TextLayout};

use crate::UGameState;

pub mod button;

#[derive(Clone)]
pub struct Manager {
    pub buttons: Vec<button::Button>,
}

impl Manager {
    pub fn new() -> Manager {
        Self {
            buttons: Vec::new(),
        }
    }

    pub fn create_button(button: button::Button) {}

    pub fn update(&mut self, state: &mut UGameState, ctx: &mut ggez::Context) {
        let mouse_position = ctx.mouse.position();

        for button in &mut self.buttons {
            if mouse_position.x >= button.pos_x
                && mouse_position.y >= button.pos_y
                && mouse_position.x <= button.pos_x + button.width
                && mouse_position.y <= button.pos_y + button.height
            {
                button.hover = true;

                if ctx.mouse.button_pressed(ggez::event::MouseButton::Left) {
                    button.pressed = true
                } else {
                    button.pressed = false
                }
            } else {
                button.hover = false;
            }
        }
    }

    pub fn draw(&mut self, canvas: &mut graphics::Canvas, ctx: &mut ggez::Context) {
        for button in &self.buttons {
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(button.pos_x, button.pos_y, button.width, button.height),
                if button.hover {
                    Color::from_rgba(255, 255, 255, 125)
                } else {
                    Color::WHITE
                },
            )
            .expect("Error creating player mesh");

            let mut text = graphics::Text::new(format!("{}", button.text));

            text.set_layout(TextLayout::center()).set_scale(22.0);

            canvas.draw(&mesh, graphics::DrawParam::default());
            canvas.draw(
                &text,
                graphics::DrawParam::from([
                    button.pos_x + (button.width / 2.0),
                    button.pos_y + (button.height / 2.0),
                ])
                .color(Color::BLACK),
            );
        }
    }
}
