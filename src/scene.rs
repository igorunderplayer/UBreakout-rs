use std::borrow::BorrowMut;

use ggez::{Context, GameResult};

use crate::{UGame, UGameState};

use self::{game::GameScene, menu::MenuScene};

mod game;
mod menu;

pub trait Scene {
    fn new(ctx: &mut Context) -> Self;
    fn update(
        &mut self,
        //scenes: &mut self::Manager,
        state: &mut UGameState,
        ctx: &mut Context,
    ) -> GameResult;
    fn draw(&mut self, state: &mut UGameState, ctx: &mut Context) -> GameResult;
}

#[derive(Clone)]
pub enum Scenes {
    Menu(menu::MenuScene),
    Game(game::GameScene),
}

pub struct Manager {
    actual_scene: Scenes,
}

impl Manager {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            actual_scene: Scenes::Game(GameScene::new(ctx)),
        }
    }

    pub fn update(&mut self, state: &mut UGameState, ctx: &mut Context) -> GameResult {
        match &mut self.actual_scene {
            Scenes::Menu(scene) => {
                scene.update(state, ctx);

                Ok(())
            }
            Scenes::Game(scene) => scene.update(state, ctx),
        }
    }
    pub fn draw(&mut self, state: &mut UGameState, ctx: &mut Context) -> GameResult {
        match &mut self.actual_scene {
            Scenes::Menu(scene) => scene.draw(state, ctx),
            Scenes::Game(scene) => scene.draw(state, ctx),
        }
    }
}
