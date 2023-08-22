use crate::utils::cell::Cell;
use crate::utils::level::Level;
use crate::utils::point::Point;
use crate::utils::powerup::Powerup;
pub fn level_1() -> Level {
    let mut level_data = Level::empty(4, 4);
    level_data.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
    level_data.update(Point { x: 3, y: 3 }, Cell::Exit);
    level_data
}
