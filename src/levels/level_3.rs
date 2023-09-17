use crate::utils::cell::Cell;
use crate::utils::direction::Direction;
use crate::utils::level::Level;
use crate::utils::point::Point;
use crate::utils::powerup::Powerup;
pub(crate) fn level_3() -> Level {
    let mut level_data = Level::empty(8, 3);
    level_data.update(Point { x: 0, y: 0 }, Cell::Void);
    level_data.update(Point { x: 0, y: 1 }, Cell::Player(Powerup::None));
    level_data.update(Point { x: 0, y: 2 }, Cell::Void);

    level_data.update(Point { x: 1, y: 0 }, Cell::Void);
    level_data.update(Point { x: 1, y: 2 }, Cell::Void);

    level_data.update(
        Point { x: 3, y: 0 },
        Cell::CounterClockwiseEnemy(Direction::Left),
    );
    level_data.update(Point { x: 3, y: 1 }, Cell::Void);
    level_data.update(
        Point { x: 3, y: 2 },
        Cell::CounterClockwiseEnemy(Direction::Right),
    );

    level_data.update(Point { x: 4, y: 1 }, Cell::Void);

    level_data.update(
        Point { x: 5, y: 1 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );

    level_data.update(Point { x: 6, y: 0 }, Cell::Void);
    level_data.update(Point { x: 6, y: 2 }, Cell::Void);

    level_data.update(Point { x: 7, y: 0 }, Cell::Void);
    level_data.update(Point { x: 7, y: 1 }, Cell::Exit);
    level_data.update(Point { x: 7, y: 2 }, Cell::Void);

    level_data
}

#[cfg(test)]
mod tests {
    use crate::levels::level_3::level_3;
    use crate::utils::direction::Direction;
    use crate::Cell;
    use crate::Point;
    use crate::Powerup;
    #[test]
    fn cells() {
        let level = level_3();
        if let Some(Cell::Void) = level.data.get(&Point { x: 0, y: 0 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Player(Powerup::None)) = level.data.get(&Point { x: 0, y: 1 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Void) = level.data.get(&Point { x: 0, y: 2 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Void) = level.data.get(&Point { x: 1, y: 0 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Void) = level.data.get(&Point { x: 1, y: 2 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::CounterClockwiseEnemy(Direction::Left)) =
            level.data.get(&Point { x: 3, y: 0 })
        {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Void) = level.data.get(&Point { x: 3, y: 1 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::CounterClockwiseEnemy(Direction::Right)) =
            level.data.get(&Point { x: 3, y: 2 })
        {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Void) = level.data.get(&Point { x: 4, y: 1 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::CounterClockwiseEnemy(Direction::Up)) =
            level.data.get(&Point { x: 5, y: 1 })
        {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Void) = level.data.get(&Point { x: 6, y: 0 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Void) = level.data.get(&Point { x: 6, y: 2 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Void) = level.data.get(&Point { x: 7, y: 0 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Exit) = level.data.get(&Point { x: 7, y: 1 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Void) = level.data.get(&Point { x: 7, y: 2 }) {
        } else {
            panic!("Test failed");
        };
        if let Some(Cell::Void) = level.data.get(&Point { x: 4, y: 1 }) {
        } else {
            panic!("Test failed");
        };
    }
}
