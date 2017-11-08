#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub id: i64,
    pub me: bool,
    pub strategy_crashed: bool,
    pub score: i32,
    pub remaining_action_cooldown_ticks: i32,
}
