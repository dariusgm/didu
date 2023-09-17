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

#[cfg(test)]
mod tests {

    use crate::levels::level_1::level_1;

    use super::Cell;
    use super::Powerup;
    use crate::Point;

    #[test]
    fn cell_0_0() {
        let level = level_1();
        if let Some(Cell::Player(Powerup::None)) = level.data.get(&Point { x: 0, y: 0 }) {
        } else {
            panic!("Test failed");
        }
    }
    #[test]
    fn cell_3_3() {
        let level = level_1();
        if let Some(Cell::Exit) = level.data.get(&Point { x: 3, y: 3 }) {
        } else {
            panic!("Test failed");
        }
    }
}
