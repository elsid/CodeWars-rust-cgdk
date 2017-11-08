use super::model::{ActionType, Game, Move, Player, World};

#[derive(Clone, Debug)]
pub struct MyStrategy {}

impl MyStrategy {
    pub fn new() -> Self {
        MyStrategy {}
    }

    pub fn move_(&mut self, me: &Player, world: &World, game: &Game, move_: &mut Move) {
        if world.tick_index == 0 {
            move_
                .set_action(ActionType::ClearAndSelect)
                .set_right(world.width)
                .set_bottom(world.height);
            return;
        }

        if world.tick_index == 1 {
            move_
                .set_action(ActionType::Move)
                .set_x(world.width / 2.0)
                .set_y(world.height / 2.0);
        }
    }
}
