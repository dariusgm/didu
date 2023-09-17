use crate::utils::cell::Cell;
use crate::utils::level::Level;
use crate::utils::point::Point;
use crate::utils::powerup::Powerup;

pub(crate) fn level_2() -> Level {
    let mut level_data = Level::empty(5, 5);
    level_data.update(Point { x: 0, y: 4 }, Cell::Player(Powerup::None));

    level_data.update(Point { x: 1, y: 1 }, Cell::VerticalWall);
    level_data.update(Point { x: 1, y: 2 }, Cell::VerticalWall);
    level_data.update(Point { x: 1, y: 3 }, Cell::VerticalWall);
    level_data.update(Point { x: 1, y: 4 }, Cell::VerticalWall);

    level_data.update(Point { x: 2, y: 1 }, Cell::Switch(1));
    level_data.update(Point { x: 2, y: 2 }, Cell::Void);
    level_data.update(Point { x: 2, y: 3 }, Cell::Void);
    level_data.update(Point { x: 2, y: 4 }, Cell::Void);

    level_data.update(Point { x: 3, y: 1 }, Cell::VerticalWall);
    level_data.update(Point { x: 3, y: 2 }, Cell::VerticalWall);
    level_data.update(Point { x: 3, y: 3 }, Cell::VerticalWall);
    level_data.update(Point { x: 3, y: 4 }, Cell::VerticalWall);

    level_data.update(Point { x: 4, y: 3 }, Cell::Door(1));
    level_data.update(Point { x: 4, y: 4 }, Cell::Exit);
    level_data
}
#[cfg(test)]
mod tests {
    use super::Cell;
    use super::Powerup;
    use crate::levels::level_2::level_2;
    use crate::Point;
    #[test]
    fn cells() {
        let level = level_2();
        if let Some(Cell::Player(Powerup::None)) = level.data.get(&Point { x: 0, y: 4 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::VerticalWall) = level.data.get(&Point { x: 1, y: 1 }) {
        } else {
            panic!("Test failed")
        };
        if let Some(Cell::VerticalWall) = level.data.get(&Point { x: 1, y: 2 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::VerticalWall) = level.data.get(&Point { x: 1, y: 3 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::VerticalWall) = level.data.get(&Point { x: 1, y: 4 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::Switch(1)) = level.data.get(&Point { x: 2, y: 1 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::Void) = level.data.get(&Point { x: 2, y: 2 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::Void) = level.data.get(&Point { x: 2, y: 3 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::Void) = level.data.get(&Point { x: 2, y: 4 }) {
        } else {
            panic!("Test failed")
        };

        if let Some(Cell::VerticalWall) = level.data.get(&Point { x: 3, y: 1 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::VerticalWall) = level.data.get(&Point { x: 3, y: 2 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::VerticalWall) = level.data.get(&Point { x: 3, y: 3 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::VerticalWall) = level.data.get(&Point { x: 3, y: 4 }) {
        } else {
            panic!("Test failed");
        }

        if let Some(Cell::Door(1)) = level.data.get(&Point { x: 4, y: 3 }) {
        } else {
            panic!("Test failed");
        }
        if let Some(Cell::Exit) = level.data.get(&Point { x: 4, y: 4 }) {
        } else {
            panic!("Test failed");
        }
    }
}
