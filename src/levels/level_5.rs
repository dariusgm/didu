use crate::utils::cell::Cell;
use crate::utils::direction::Direction;
use crate::utils::level::Level;
use crate::utils::point::Point;
use crate::utils::powerup::Powerup;
pub(crate) fn level_5() -> Level {
    let mut l = Level::empty(13, 18);

    l.update(
        Point { x: 0, y: 0 },
        Cell::CounterClockwiseEnemy(Direction::Right),
    );
    l.update(Point { x: 7, y: 0 }, Cell::Switch(3));
    l.update(Point { x: 8, y: 0 }, Cell::VerticalWall);
    l.update(Point { x: 9, y: 0 }, Cell::Void);
    l.update(Point { x: 11, y: 0 }, Cell::Door(1));

    l.update(Point { x: 0, y: 1 }, Cell::HorizontalWall);
    l.update(Point { x: 1, y: 1 }, Cell::HorizontalWall);
    l.update(Point { x: 2, y: 1 }, Cell::BreakableGround);
    l.update(Point { x: 3, y: 1 }, Cell::VerticalWall);
    l.update(Point { x: 5, y: 1 }, Cell::HorizontalWall);
    l.update(Point { x: 6, y: 1 }, Cell::HorizontalWall);
    l.update(Point { x: 7, y: 1 }, Cell::HorizontalWall);
    l.update(Point { x: 8, y: 1 }, Cell::VerticalWall);
    l.update(Point { x: 9, y: 1 }, Cell::Void);
    l.update(Point { x: 11, y: 1 }, Cell::VerticalWall);

    l.update(Point { x: 0, y: 2 }, Cell::Void);
    l.update(Point { x: 1, y: 2 }, Cell::BreakableGround);
    l.update(Point { x: 2, y: 2 }, Cell::BreakableGround);
    l.update(Point { x: 3, y: 2 }, Cell::VerticalWall);
    l.update(Point { x: 8, y: 2 }, Cell::VerticalWall);
    l.update(Point { x: 9, y: 2 }, Cell::Void);
    l.update(Point { x: 11, y: 2 }, Cell::VerticalWall);
    l.update(Point { x: 12, y: 2 }, Cell::Switch(1));

    l.update(Point { x: 0, y: 3 }, Cell::HorizontalWall);
    l.update(Point { x: 1, y: 3 }, Cell::Door(2));
    l.update(Point { x: 2, y: 3 }, Cell::HorizontalWall);
    l.update(Point { x: 3, y: 3 }, Cell::HorizontalWall);
    l.update(Point { x: 4, y: 3 }, Cell::HorizontalWall);
    l.update(Point { x: 5, y: 3 }, Cell::HorizontalWall);
    l.update(Point { x: 6, y: 3 }, Cell::VerticalWall);
    l.update(Point { x: 8, y: 3 }, Cell::Void);
    l.update(Point { x: 9, y: 3 }, Cell::Void);
    l.update(Point { x: 11, y: 3 }, Cell::VerticalWall);
    l.update(
        Point { x: 12, y: 3 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );

    l.update(
        Point { x: 0, y: 4 },
        Cell::CounterClockwiseEnemy(Direction::Right),
    );

    l.update(Point { x: 5, y: 4 }, Cell::Void);
    l.update(Point { x: 6, y: 4 }, Cell::VerticalWall);
    l.update(Point { x: 7, y: 4 }, Cell::Switch(2));
    l.update(Point { x: 8, y: 4 }, Cell::VerticalWall);
    l.update(Point { x: 9, y: 4 }, Cell::Void);
    l.update(Point { x: 11, y: 4 }, Cell::VerticalWall);
    l.update(
        Point { x: 12, y: 4 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );

    l.update(Point { x: 6, y: 5 }, Cell::VerticalWall);
    l.update(Point { x: 7, y: 5 }, Cell::HorizontalWall);
    l.update(Point { x: 8, y: 5 }, Cell::VerticalWall);
    l.update(Point { x: 9, y: 5 }, Cell::Void);
    l.update(Point { x: 11, y: 5 }, Cell::VerticalWall);
    l.update(
        Point { x: 12, y: 5 },
        Cell::CounterClockwiseEnemy(Direction::Down),
    );
    l.update(Point { x: 7, y: 6 }, Cell::Invincibility);
    l.update(Point { x: 8, y: 6 }, Cell::VerticalWall);
    l.update(Point { x: 9, y: 6 }, Cell::Void);
    l.update(Point { x: 10, y: 6 }, Cell::Invincibility);
    l.update(Point { x: 11, y: 6 }, Cell::VerticalWall);
    l.update(
        Point { x: 12, y: 6 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );

    l.update(Point { x: 2, y: 7 }, Cell::Switch(6));
    l.update(Point { x: 5, y: 7 }, Cell::VerticalWall);
    l.update(Point { x: 6, y: 7 }, Cell::Door(3));
    l.update(Point { x: 7, y: 7 }, Cell::HorizontalWall);
    l.update(Point { x: 8, y: 7 }, Cell::HorizontalWall);
    l.update(Point { x: 9, y: 7 }, Cell::HorizontalWall);
    l.update(Point { x: 10, y: 7 }, Cell::HorizontalWall);
    l.update(Point { x: 11, y: 7 }, Cell::VerticalWall);
    l.update(
        Point { x: 12, y: 7 },
        Cell::CounterClockwiseEnemy(Direction::Down),
    );

    l.update(Point { x: 5, y: 8 }, Cell::VerticalWall);
    l.update(Point { x: 6, y: 8 }, Cell::BreakableGround);
    l.update(Point { x: 12, y: 8 }, Cell::Invincibility);

    l.update(Point { x: 0, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 1, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 2, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 3, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 4, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 5, y: 9 }, Cell::VerticalWall);
    l.update(Point { x: 6, y: 9 }, Cell::Player(Powerup::None));
    l.update(Point { x: 7, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 8, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 9, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 10, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 11, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 12, y: 9 }, Cell::HorizontalWall);

    l.update(Point { x: 0, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 1, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 2, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 3, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 4, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 5, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 6, y: 11 }, Cell::Door(6));
    l.update(Point { x: 7, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 8, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 9, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 10, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 11, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 12, y: 11 }, Cell::HorizontalWall);

    l.update(Point { x: 4, y: 12 }, Cell::HorizontalWall);
    l.update(
        Point { x: 5, y: 12 },
        Cell::CounterClockwiseEnemy(Direction::Down),
    );
    l.update(Point { x: 6, y: 12 }, Cell::Invincibility);
    l.update(
        Point { x: 7, y: 12 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(Point { x: 8, y: 12 }, Cell::HorizontalWall);

    l.update(Point { x: 4, y: 13 }, Cell::HorizontalWall);
    l.update(
        Point { x: 5, y: 13 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(
        Point { x: 6, y: 13 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(
        Point { x: 7, y: 13 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(Point { x: 8, y: 13 }, Cell::HorizontalWall);

    l.update(Point { x: 4, y: 14 }, Cell::HorizontalWall);
    l.update(
        Point { x: 5, y: 14 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(
        Point { x: 6, y: 14 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(
        Point { x: 7, y: 14 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(Point { x: 8, y: 14 }, Cell::HorizontalWall);

    l.update(Point { x: 4, y: 15 }, Cell::HorizontalWall);
    l.update(
        Point { x: 5, y: 15 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(
        Point { x: 6, y: 15 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(
        Point { x: 7, y: 15 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(Point { x: 8, y: 15 }, Cell::HorizontalWall);
    l.update(Point { x: 4, y: 16 }, Cell::HorizontalWall);
    l.update(
        Point { x: 5, y: 16 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(
        Point { x: 6, y: 16 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(
        Point { x: 7, y: 16 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(Point { x: 8, y: 16 }, Cell::HorizontalWall);
    l.update(Point { x: 4, y: 17 }, Cell::HorizontalWall);
    l.update(
        Point { x: 5, y: 17 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(Point { x: 6, y: 17 }, Cell::Exit);
    l.update(
        Point { x: 7, y: 17 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );
    l.update(Point { x: 8, y: 17 }, Cell::HorizontalWall);
    l
}
