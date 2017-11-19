use model::{Game, Action, Player, World};

pub trait Strategy: Default {
    fn act(&mut self, me: &Player, world: &World, game: &Game, action: &mut Action);
}
