#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub id: i64,
    pub me: bool,
    pub strategy_crashed: bool,
    pub score: i32,
    pub remaining_action_cooldown_ticks: i32,
    pub remaining_nuclear_strike_cooldown_ticks: i32,
    pub next_nuclear_strike_vehicle_id: i64,
    pub next_nuclear_strike_tick_index: i32,
    pub next_nuclear_strike_x: f64,
    pub next_nuclear_strike_y: f64,
}
