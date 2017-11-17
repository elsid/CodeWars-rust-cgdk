use model::{ActionType, Game, Move, Player, World};
use strategy::Strategy;

pub struct MyStrategy {
    iter: i32,
}

impl Default for MyStrategy {
    fn default() -> Self {
        MyStrategy {
            iter: 0,
        }
    }
}

impl Strategy for MyStrategy {
    fn move_(&mut self, me: &Player, world: &World, game: &Game, move_: &mut Move) {
        if world.tick_index == 0 {
            move_.action = ActionType::ClearAndSelect;
            move_.right = world.width;
            move_.bottom = world.height;
            return;
        }

        if world.tick_index == 1 {
            move_.action = ActionType::Move;
            move_.x = 500.0;
            move_.y = 0.0;
            move_.max_speed = 0.01;
            return;
        }

        if me.remaining_action_cooldown_ticks > 0 || world.tick_index % 5 != 0 {
            return;
        }

        self.iter += 1;

        if self.iter <= 99 {
            move_.action = ActionType::Assign;
            move_.group = self.iter;
            return;
        }

        let k = if self.iter % 2 == 0 {
            -1.0
        } else {
            1.0
        };

        move_.action = ActionType::Move;
        move_.x = k * world.width / 2.0;
        move_.y = k * world.height / 2.0;
    }
}
