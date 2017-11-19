use model::{ActionType, Game, Action, Player, World};
use strategy::Strategy;

#[derive(Default)]
pub struct MyStrategy;

impl Strategy for MyStrategy {
    fn act(&mut self, me: &Player, world: &World, game: &Game, action: &mut Action) {
        if world.tick_index == 0 {
            action.action = Some(ActionType::ClearAndSelect);
            action.right = world.width;
            action.bottom = world.height;
            return;
        }

        if world.tick_index == 1 {
            action.action = Some(ActionType::Move);
            action.x = world.width / 2.0;
            action.y = world.height / 2.0;
        }
    }
}
