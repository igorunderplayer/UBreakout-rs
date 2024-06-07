use std::{
    env,
    fs::{self, DirEntry, File},
    path::{Path, PathBuf},
};

use ggez::graphics::{self, Color, TextLayout};
use homedir::get_my_home;

use crate::{
    data::{self, LevelData},
    ui::{self, button},
    UGameState,
};

use super::{game, Scene};

#[derive(Clone)]
struct LevelDataWithPath {
    path: String,
    data: LevelData,
}

#[derive(Clone)]
pub struct MenuScene {
    levels: Vec<LevelDataWithPath>,
    ui: ui::Manager,
}

impl Scene for MenuScene {
    fn new(_ctx: &mut ggez::Context, _state: &mut UGameState) -> Self {
        let ui = ui::Manager::new();
        let levels = Vec::new();

        Self { levels, ui }
    }

    fn on_create(&mut self, state: &mut UGameState) -> ggez::GameResult {
        self.levels.clear();
        self.ui = ui::Manager::new();

        let game_dir = get_my_home().unwrap().unwrap().join(".ubreakoutrs");
        let levels_directory = game_dir.join("levels");

        fs::create_dir_all(&game_dir).unwrap();
        fs::create_dir_all(&levels_directory).unwrap();

        let files = match fs::read_dir(levels_directory) {
            Ok(v) => v,
            _ => {
                let _ = fs::create_dir("./levels");
                fs::read_dir("./levels").unwrap()
            }
        }
        .map(|f| f.unwrap().path());

        self.levels = files
            .map(|dir_entry| {
                let string = fs::read_to_string(&dir_entry).unwrap();
                let data = data::load_from_json(&string);

                LevelDataWithPath {
                    path: dir_entry.to_str().unwrap().to_owned(),
                    data,
                }
            })
            .collect();

        let mut i: usize = 0;
        for level in &self.levels {
            self.ui.buttons.push(button::Button::new(
                format!("level-{}-{}", level.path, level.data.name),
                level.data.name.clone(),
                300.0,
                30.0 + (i * 70) as f32,
                200.0,
                60.0,
            ));

            i += 1;
        }
        Ok(())
    }

    fn update(
        &mut self,
        scene_manager: &mut super::Manager,
        state: &mut UGameState,
        ctx: &mut ggez::Context,
    ) -> ggez::GameResult {
        self.ui.update(state, ctx);

        for button in &self.ui.buttons {
            let button_data: Vec<&str> = button.id.split("-").collect();

            if button.pressed {
                match button_data[0] {
                    "level" => {
                        println!("Button data: {}", button_data[1]);

                        let level_path = button_data[1];
                        let string = fs::read_to_string(&level_path).unwrap();
                        println!("Level strinng: {}", string);
                        let data = data::load_from_json(&string);

                        state.level = Some(data);
                        scene_manager.set_scene(ctx, state, super::Scenes::Game)
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, state: &mut UGameState, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.ui.draw(&mut canvas, ctx);

        if self.levels.is_empty() {
            let game_dir = get_my_home()
                .unwrap()
                .unwrap()
                .join(".ubreakoutrs")
                .join("levels");

            let alert_text = graphics::Text::new(format!(
                "No level data found, you can place them in the directory: {}",
                game_dir.to_str().unwrap()
            ));
            canvas.draw(&alert_text, graphics::DrawParam::default());
        }
        canvas.finish(ctx)
    }
}
