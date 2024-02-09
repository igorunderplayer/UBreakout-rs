use std::fs::{self, File};

use ggez::graphics::{self, Color, TextLayout};

use crate::{
    data::{self, LevelData},
    ui::{self, button},
    UGameState,
};

use super::Scene;

#[derive(Clone)]
pub struct MenuScene {
    levels: Vec<LevelData>,
    ui: ui::Manager,
}

impl Scene for MenuScene {
    fn new(_ctx: &mut ggez::Context) -> Self {
        let files = match fs::read_dir("./levels") {
            Ok(v) => v,
            _ => {
                let _ = fs::create_dir("./levels");
                fs::read_dir("./levels").unwrap()
            }
        };

        let levels: Vec<LevelData> = files
            .map(|dir_entry| {
                let string = fs::read_to_string(dir_entry.unwrap().path()).unwrap();
                data::load_from_json(&string)
            })
            .collect();

        let mut ui = ui::Manager::new();

        let mut i: f32 = 0.0;
        for level in &levels {
            ui.buttons.push(button::Button::new(
                format!("level-{}-{}", i, level.name),
                level.name.clone(),
                300.0,
                30.0 + (i * 70.0),
                200.0,
                60.0,
            ));

            i += 1.0;
        }

        Self { levels, ui }
    }

    fn update(
        &mut self,
        scene_manager: &mut super::Manager,
        state: &mut UGameState,
        ctx: &mut ggez::Context,
    ) -> ggez::GameResult {
        self.ui.update(state, ctx);

        for button in &self.ui.buttons {
            if button.pressed {}
        }
        Ok(())
    }

    fn draw(&mut self, state: &mut UGameState, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.ui.draw(&mut canvas, ctx);
        canvas.finish(ctx)
    }
}
