use ggez::{Context, GameResult};

use crate::UGameState;

use self::{game::GameScene, menu::MenuScene};

pub mod game;
pub mod menu;

pub trait Scene {
    fn new(ctx: &mut Context, state: &mut UGameState) -> Self;
    fn update(
        &mut self,
        scene_manager: &mut Manager,
        state: &mut UGameState,
        ctx: &mut Context,
    ) -> GameResult;
    fn on_create(&mut self, state: &mut UGameState) -> GameResult;
    fn draw(&mut self, state: &mut UGameState, ctx: &mut Context) -> GameResult;
}

pub enum Scenes {
    None,
    Menu,
    Game,
}

pub struct Manager {
    actual_scene: Scenes,
    menu_scene: MenuScene,
    game_scene: GameScene,
}

impl Manager {
    pub fn new(ctx: &mut Context, state: &mut UGameState) -> Self {
        Self {
            actual_scene: Scenes::Menu,
            menu_scene: MenuScene::new(ctx, state),
            game_scene: GameScene::new(ctx, state),
        }
    }

    pub fn set_scene(&mut self, ctx: &mut Context, state: &mut UGameState, scene: Scenes) {
        match &scene {
            Scenes::Game => {
                let _ = self.game_scene.on_create(state);
            }

            Scenes::Menu => {
                let _ = self.menu_scene.on_create(state);
            }

            Scenes::None => {}
        }

        self.actual_scene = scene;
    }

    pub fn update(&mut self, state: &mut UGameState, ctx: &mut Context) -> GameResult {
        let mut menu_scene = self.menu_scene.clone();
        let mut game_scene = self.game_scene.clone();
        match self.actual_scene {
            Scenes::Menu => {
                let _ = menu_scene.update(self, state, ctx);
                self.menu_scene = menu_scene;
                Ok(())
            }
            Scenes::Game => {
                let _ = game_scene.update(self, state, ctx);
                self.game_scene = game_scene;
                Ok(())
            }
            Scenes::None => Ok(()),
        }
    }
    pub fn draw(&mut self, state: &mut UGameState, ctx: &mut Context) -> GameResult {
        match self.actual_scene {
            Scenes::Menu => self.menu_scene.draw(state, ctx),
            Scenes::Game => self.game_scene.draw(state, ctx),
            Scenes::None => Ok(()),
        }
    }
}
