use serde::{Deserialize, Serialize};
use serde_json;

use crate::enemy::EnemyData;

#[derive(Deserialize, Serialize, Clone)]
pub struct LevelData {
    pub name: String,
    pub waves: Vec<WaveData>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WaveData {
    pub enemies: Vec<EnemyData>,
}

pub fn load_from_json(json_string: &str) -> LevelData {
    let data: LevelData =
        serde_json::from_str(json_string).expect("Failed to deserialize level data");

    return data;
}
