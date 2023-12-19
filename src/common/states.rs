use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    LoadAssets,
    Setup,
    PlayerTurn,
    EnemyTurn,
    PlanEnemyTurn,
    ActEnemyTurn,
}
