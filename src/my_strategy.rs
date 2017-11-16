use code_wars::model::{ActionType, Game, Move, Player, World};

#[derive(Clone, Debug)]
pub struct MyStrategy {}

impl MyStrategy {
    pub fn new() -> Self {
        MyStrategy {}
    }

    pub fn move_(&mut self, me: &Player, world: &World, game: &Game, move_: &mut Move) {
        if world.tick_index == 0 {
            move_.action = ActionType::ClearAndSelect;
            move_.right = world.width;
            move_.bottom = world.height;
            return;
        }

        if world.tick_index == 1 {
            move_.action = ActionType::Move;
            move_.x = world.width / 2.0;
            move_.y = world.height / 2.0;
        }
    }
}
