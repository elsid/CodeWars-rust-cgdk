use model::{Game, Move, Player, World};

pub trait Strategy: Default {
    fn move_(&mut self, me: &Player, world: &World, game: &Game, move_: &mut Move);
}
