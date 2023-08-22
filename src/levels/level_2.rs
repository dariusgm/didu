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
