use crate::utils::cell::Cell;
use crate::utils::level::Level;
use crate::utils::point::Point;
use crate::utils::powerup::Powerup;
pub(crate) fn level_4() -> Level {
    let mut level_data = Level::empty(31, 23);
    for i in 0..23 {
        level_data.update(Point { x: 26, y: i }, Cell::Void);
        level_data.update(Point { x: 28, y: i }, Cell::Void);
    }

    for i in 0..30 {
        level_data.update(Point { x: i, y: 0 }, Cell::HorizontalWall);
        level_data.update(Point { x: i, y: 7 }, Cell::HorizontalWall);
        level_data.update(Point { x: i, y: 8 }, Cell::Void);
        level_data.update(Point { x: i, y: 9 }, Cell::HorizontalWall);
        level_data.update(Point { x: i, y: 13 }, Cell::HorizontalWall);
        level_data.update(Point { x: i, y: 14 }, Cell::Void);
        level_data.update(Point { x: i, y: 15 }, Cell::HorizontalWall);
        level_data.update(Point { x: i, y: 23 }, Cell::HorizontalWall);
    }

    // Place Vertical Wall
    for i in 0..23 {
        level_data.update(Point { x: 25, y: i }, Cell::VerticalWall);
        level_data.update(Point { x: 29, y: i }, Cell::VerticalWall);
    }

    // Place all wrong teleporters for "day"
    for i in (1..21).step_by(2) {
        level_data.update(
            Point { x: i, y: 2 },
            Cell::OneWayTeleporter(Point { x: 27, y: 2 }),
        );
        level_data.update(
            Point { x: i, y: 4 },
            Cell::OneWayTeleporter(Point { x: 27, y: 4 }),
        );
        level_data.update(
            Point { x: i, y: 6 },
            Cell::OneWayTeleporter(Point { x: 27, y: 6 }),
        );
    }
    level_data.update(
        Point { x: 21, y: 6 },
        Cell::OneWayTeleporter(Point { x: 27, y: 6 }),
    );

    // Place all wrong teleporters for "month"
    for i in (1..25).step_by(2) {
        level_data.update(
            Point { x: i, y: 11 },
            Cell::OneWayTeleporter(Point { x: 27, y: 11 }),
        )
    }

    // Place all wrong teleporters for "year"
    for i in (1..21).step_by(2) {
        level_data.update(
            Point { x: i, y: 17 },
            Cell::OneWayTeleporter(Point { x: 27, y: 16 }),
        );
        level_data.update(
            Point { x: i, y: 19 },
            Cell::OneWayTeleporter(Point { x: 27, y: 18 }),
        );
        level_data.update(
            Point { x: i, y: 21 },
            Cell::OneWayTeleporter(Point { x: 27, y: 20 }),
        );
    }
    // add teleporter for day -> month
    level_data.update(
        Point { x: 9, y: 2 },
        Cell::OneWayTeleporter(Point { x: 24, y: 11 }),
    );
    // add teleporter for month -> year
    level_data.update(
        Point { x: 15, y: 11 },
        Cell::OneWayTeleporter(Point { x: 24, y: 18 }),
    );
    // add teleporter year -> exit
    level_data.update(
        Point { x: 5, y: 21 },
        Cell::OneWayTeleporter(Point { x: 30, y: 0 }),
    );
    level_data.update(Point { x: 24, y: 4 }, Cell::Player(Powerup::None));
    level_data.update(Point { x: 30, y: 23 }, Cell::Exit);
    level_data
}
