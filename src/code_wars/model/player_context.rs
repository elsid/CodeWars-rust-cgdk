use super::player::Player;
use super::world::World;

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerContext {
    pub player: Player,
    pub world: World,
}
