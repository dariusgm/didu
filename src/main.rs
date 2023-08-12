use crossterm::{
    cursor,
    cursor::MoveTo,
    event::poll as poll_event,
    event::read as read_event,
    event::Event as Event_,
    event::KeyCode as KeyCode_,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::enable_raw_mode,
    terminal::{self, Clear, ClearType},
    ExecutableCommand, Result,
};
use std::{collections::HashMap, io::Stdout, time::Duration};
use std::{io::Write, time::Instant};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, PartialEq, Copy, Eq, Hash)]
enum Cell {
    Empty,
    Player,
    Exit,
    HorizontalWall,
    VerticalWall,
    CounterClockwiseEnemy(Direction),
    Void,
    Switch(u8),
    Door(u8),
    OneWayTeleporter(Point),
    BreakableGround,
    Invincibility,
}

#[derive(Clone, PartialEq, Copy, Eq, Hash, Debug)]
struct Point {
    x: i8,
    y: i8,
}
#[derive(Clone)]
struct Level {
    data: HashMap<Point, Cell>,
}

impl Level {
    fn empty(width: u8, height: u8) -> Self {
        let mut data = HashMap::new();

        for x in 0..width {
            for y in 0..height {
                data.insert(
                    Point {
                        x: x as i8,
                        y: y as i8,
                    },
                    Cell::Empty,
                );
            }
        }

        Self { data }
    }

    fn update_enemies(&mut self) {
        for (&point, &cell) in self.data.clone().iter() {
            // try move
            let optional_target_point = match cell {
                Cell::CounterClockwiseEnemy(Direction::Right) => Some(Point {
                    x: point.x + 1,
                    y: point.y,
                }),
                Cell::CounterClockwiseEnemy(Direction::Up) => Some(Point {
                    x: point.x,
                    y: point.y - 1,
                }),
                Cell::CounterClockwiseEnemy(Direction::Down) => Some(Point {
                    x: point.x,
                    y: point.y + 1,
                }),
                Cell::CounterClockwiseEnemy(Direction::Left) => Some(Point {
                    x: point.x - 1,
                    y: point.y,
                }),
                _ => None,
            };
            let target_rotation = match cell {
                Cell::CounterClockwiseEnemy(Direction::Up) => Direction::Left,
                Cell::CounterClockwiseEnemy(Direction::Down) => Direction::Right,
                Cell::CounterClockwiseEnemy(Direction::Left) => Direction::Down,
                _ => Direction::Up,
            };
            if let Some(target_point) = optional_target_point {
                let cloned_cell = &self.data.get(&target_point).cloned();
                match cloned_cell {
                    // we can move
                    Some(Cell::Empty) => {
                        self.update(point, Cell::Empty);
                        self.update(target_point, cell);
                    }
                    // remove player from grid.
                    Some(Cell::Player) => {
                        self.update(point, Cell::Empty);
                        self.update(target_point, cell)
                    }
                    // rotate enemy for the following cases without moving it.
                    Some(Cell::VerticalWall) => {
                        self.update(point, Cell::CounterClockwiseEnemy(target_rotation));
                    }
                    Some(Cell::HorizontalWall) => {
                        self.update(point, Cell::CounterClockwiseEnemy(target_rotation));
                    }
                    Some(Cell::Void) => {
                        self.update(point, Cell::CounterClockwiseEnemy(target_rotation));
                    }
                    Some(Cell::Door(_)) => {
                        self.update(point, Cell::CounterClockwiseEnemy(target_rotation));
                    }
                    Some(Cell::Switch(_)) => {
                        self.update(point, Cell::CounterClockwiseEnemy(target_rotation));
                    }
                    // collision with something else not implemented.
                    // It would require data structure change to have two elements on the same
                    // cell.
                    Some(_) => {}
                    // Out of bounds in front of us, need to rotate as well.
                    None => self.update(point, Cell::CounterClockwiseEnemy(target_rotation)),
                }
            }
        }
    }

    fn update(&mut self, point: Point, cell: Cell) {
        self.data.insert(point, cell);
    }
    fn door_position(&self, switch_id: u8) -> Option<Point> {
        for (&point, &cell) in self.data.iter() {
            if let Cell::Door(door_id) = cell {
                if door_id == switch_id {
                    return Some(point);
                }
            }
        }
        None
    }
    fn finish_position(&self) -> Option<Point> {
        for (&point, &cell) in self.data.iter() {
            if cell == Cell::Exit {
                return Some(point);
            }
        }
        None
    }

    fn player_position(&self) -> Option<Point> {
        for (&point, &cell) in self.data.iter() {
            if cell == Cell::Player {
                return Some(point);
            }
        }
        None
    }

    fn move_player(&mut self, player: Point, new_position: Point, max_x: i8, max_y: i8) {
        // Handle out of bounds
        if new_position.x >= 0
            && new_position.x <= max_x
            && new_position.y >= 0
            && new_position.y <= max_y
        {
            // Handle collosions here that will not reset the level
            if let Some(cell) = self.data.get(&new_position).cloned() {
                match cell {
                    // moved on empty space
                    Cell::Empty => {
                        self.update(player, Cell::Empty);
                        self.update(new_position, Cell::Player);
                    }

                    // Triggering a switch removes the switch and the related door
                    Cell::Switch(switch_id) => {
                        self.update(player, Cell::Empty);
                        self.update(new_position, Cell::Player);
                        if let Some(door_position) = self.door_position(switch_id) {
                            self.update(door_position, Cell::Empty);
                        }
                    }
                    // Triggering a teleporter, moves me to the destination
                    Cell::OneWayTeleporter(destination_point) => {
                        self.update(player, Cell::Empty);
                        self.update(new_position, Cell::Empty);
                        self.update(destination_point, Cell::Player)
                    }
                    // everything else can not be passed
                    _ => {}
                }
            }
        } else {
            // moved to invalid position, ignoring
        }
    }

    fn size(&self) -> (i8, i8) {
        let mut max_x = 0;
        let mut max_y = 0;
        for &point in self.data.keys() {
            if point.x > max_x {
                max_x = point.x
            }

            if point.y > max_y {
                max_y = point.y
            }
        }
        (max_x, max_y)
    }
}

fn level_1() -> Level {
    let mut level_data = Level::empty(4, 4);
    level_data.update(Point { x: 0, y: 0 }, Cell::Player);
    level_data.update(Point { x: 3, y: 3 }, Cell::Exit);
    level_data
}

fn level_2() -> Level {
    let mut level_data = Level::empty(5, 5);
    level_data.update(Point { x: 0, y: 4 }, Cell::Player);

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

fn level_3() -> Level {
    let mut level_data = Level::empty(8, 3);
    level_data.update(Point { x: 0, y: 0 }, Cell::Void);
    level_data.update(Point { x: 0, y: 1 }, Cell::Player);
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

fn level_4() -> Level {
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
    level_data.update(Point { x: 24, y: 4 }, Cell::Player);
    level_data.update(Point { x: 30, y: 23 }, Cell::Exit);
    level_data
}

fn level_5() -> Level {
    let mut l = Level::empty(13, 18);

    l.update(
        Point { x: 0, y: 0 },
        Cell::CounterClockwiseEnemy(Direction::Right),
    );
    l.update(Point { x: 7, y: 0 }, Cell::Switch(0));
    l.update(Point { x: 8, y: 0 }, Cell::VerticalWall);
    l.update(Point { x: 9, y: 0 }, Cell::Void);
    l.update(Point { x: 11, y: 0 }, Cell::Door(0));

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
    l.update(Point { x: 12, y: 2 }, Cell::Switch(0));

    l.update(Point { x: 0, y: 3 }, Cell::HorizontalWall);
    l.update(Point { x: 1, y: 3 }, Cell::Door(0));
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
    l.update(Point { x: 7, y: 4 }, Cell::Switch(0));
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

    l.update(Point { x: 2, y: 6 }, Cell::Switch(0));
    l.update(Point { x: 3, y: 6 }, Cell::Switch(0));
    l.update(Point { x: 8, y: 6 }, Cell::VerticalWall);
    l.update(Point { x: 9, y: 6 }, Cell::Void);
    l.update(Point { x: 10, y: 6 }, Cell::Invincibility);
    l.update(Point { x: 11, y: 6 }, Cell::VerticalWall);
    l.update(
        Point { x: 12, y: 6 },
        Cell::CounterClockwiseEnemy(Direction::Up),
    );

    l.update(Point { x: 2, y: 7 }, Cell::Switch(0));
    l.update(Point { x: 3, y: 7 }, Cell::Switch(0));
    l.update(Point { x: 5, y: 7 }, Cell::VerticalWall);
    l.update(Point { x: 6, y: 7 }, Cell::Door(0));
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
    l.update(Point { x: 6, y: 9 }, Cell::Player);
    l.update(Point { x: 7, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 8, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 9, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 10, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 11, y: 9 }, Cell::HorizontalWall);
    l.update(Point { x: 12, y: 9 }, Cell::HorizontalWall);

    l.update(Point { x: 3, y: 10 }, Cell::Invincibility);
    l.update(Point { x: 5, y: 10 }, Cell::Door(0));
    l.update(Point { x: 7, y: 10 }, Cell::Door(0));
    l.update(
        Point { x: 12, y: 10 },
        Cell::OneWayTeleporter(Point { x: 0, y: 0 }),
    );

    l.update(Point { x: 0, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 1, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 2, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 3, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 4, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 5, y: 11 }, Cell::HorizontalWall);
    l.update(Point { x: 6, y: 11 }, Cell::Door(0));
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

    l.update(
        Point { x: 2, y: 13 },
        Cell::OneWayTeleporter(Point { x: 0, y: 0 }),
    );
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
    l.update(
        Point { x: 11, y: 13 },
        Cell::OneWayTeleporter(Point { x: 0, y: 0 }),
    );

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

#[derive(Debug)]
struct Drawing {
    stdout: Stdout,
}

impl Drawing {
    fn new() -> Self {
        Drawing {
            stdout: std::io::stdout(),
        }
    }
    fn show_timing(&mut self, timing: Vec<u128>) -> Result<()> {
        queue!(self.stdout, MoveTo(0, 0), Print("Level | Time in ms"))?;
        for (i, t) in timing.iter().enumerate() {
            queue!(
                self.stdout,
                MoveTo(0, i as u16 + 1),
                Print(format!("{:?} | {:?}\n", i, t))
            )?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }

    fn init(&mut self) -> Result<()> {
        self.stdout.execute(cursor::Hide)?;
        Ok(())
    }
    fn reset(&mut self) -> Result<()> {
        self.stdout.execute(cursor::Show)?;
        self.stdout.execute(ResetColor)?;
        self.stdout.execute(terminal::Clear(ClearType::All))?;
        Ok(())
    }
    fn draw_ui(&mut self, level_number: usize, elapsed_time: u128, max_y: u16) -> Result<()> {
        // Clear the status bar line
        queue!(
            self.stdout,
            MoveTo(0, max_y + 2),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
        )?;

        // Print status bar
        queue!(
            self.stdout,
            MoveTo(0, max_y + 2),
            Print(format!(
                "Level: {}, Time: {}, h = toggle help",
                level_number, elapsed_time
            )),
        )?;

        Ok(())
    }

    fn draw_help(&mut self, max_x: u16) -> Result<()> {
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 0),
            Print("@ = Player - Use Arrow keys to move around.")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 1),
            Print("X = Exit that you need to reach.")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 2),
            Print(". = Empty Space - you can walk here.")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 3),
            Print(
                "  = Void, you should not walk on it.
"
            )
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 4),
            Print(
                "| = Vertical wall that you can't pass.
"
            )
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 5),
            Print(
                "- = Horizontal wall that you can't pass.
"
            )
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 6),
            Print("S = A switch that opens a door. ")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 7),
            Print(
                "D = A door. You need the correct Switch to open it.
"
            )
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 8),
            Print(
                "§ = An enemy! Watch out!
"
            )
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 9),
            Print("T = A one way teleporter. ")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 10),
            Print(
                "? = Breakable ground. Will transfer to void after passed once.
"
            )
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 11),
            Print(
                "o = Surprise Candy. Makes you feel really strong!
"
            )
        )?;
        Ok(())
    }

    fn draw_level(&mut self, level: &Level) -> Result<()> {
        queue!(self.stdout, Clear(ClearType::All))?;
        for (point, cell) in level.data.iter() {
            queue!(self.stdout, MoveTo(point.x as u16, point.y as u16))?;
            match cell {
                Cell::Empty => {
                    queue!(self.stdout, SetForegroundColor(Color::Blue), Print("."))?;
                }
                Cell::Player => {
                    queue!(self.stdout, SetForegroundColor(Color::Red), Print("@"))?;
                }
                Cell::Exit => {
                    queue!(
                        self.stdout,
                        SetForegroundColor(Color::White),
                        SetBackgroundColor(Color::Black),
                        Print("X")
                    )?;
                }
                Cell::Void => {
                    queue!(
                        self.stdout,
                        SetForegroundColor(Color::Black),
                        SetBackgroundColor(Color::Black),
                        Print(" ")
                    )?;
                }
                Cell::VerticalWall => {
                    queue!(
                        self.stdout,
                        SetForegroundColor(Color::Grey),
                        SetBackgroundColor(Color::Red),
                        Print("|")
                    )?;
                }
                Cell::HorizontalWall => {
                    queue!(
                        self.stdout,
                        SetForegroundColor(Color::Grey),
                        SetBackgroundColor(Color::Red),
                        Print("-")
                    )?;
                }
                Cell::Door(_) => {
                    queue!(self.stdout, SetForegroundColor(Color::Red), Print("D"))?;
                }
                Cell::Switch(_) => {
                    queue!(self.stdout, SetForegroundColor(Color::Green), Print("S"))?;
                }
                Cell::CounterClockwiseEnemy(_) => {
                    queue!(self.stdout, SetForegroundColor(Color::DarkRed), Print("§"))?;
                }
                Cell::OneWayTeleporter(_) => {
                    queue!(self.stdout, SetForegroundColor(Color::DarkBlue), Print("T"))?;
                }
                Cell::BreakableGround => {
                    queue!(self.stdout, SetForegroundColor(Color::Grey), Print("?"))?;
                }
                Cell::Invincibility => {
                    queue!(
                        self.stdout,
                        SetBackgroundColor(Color::Yellow),
                        SetForegroundColor(Color::White),
                        Print("o")
                    )?;
                }
            }
            queue!(self.stdout, ResetColor)?;
        }
        self.stdout.flush()?; // Flush the queued commands
        Ok(())
    }
}
fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut drawing = Drawing::new();
    let levels = vec![level_5(), level_1(), level_2(), level_3(), level_4()];
    drawing.init()?;
    let mut timing: Vec<u128> = vec![];
    let mut terminate = false;
    // draw help overlay
    let mut draw_help = false;
    // for each level, clone the level into the buffer for modification.
    // Run than the game loop
    for (level_index, level) in levels.iter().enumerate() {
        let mut cloned_level = level.clone();

        let (max_x, max_y) = level.size();
        // Count time needed
        let mut level_start = Instant::now();
        // responsible for game loop
        let mut run = true;
        // restart current level when failed
        let mut restart = false;
        let enemy_move_interval = Duration::from_millis(500); // Enemies move every 500 ms
        let mut last_enemy_move = Instant::now();
        while run {
            if restart {
                cloned_level = level.clone();
                level_start = Instant::now();
                restart = false;
                last_enemy_move = Instant::now();
            }

            if terminate {
                continue;
            }
            drawing.draw_level(&cloned_level)?;
            drawing.draw_ui(
                level_index + 1,
                level_start.elapsed().as_secs() as u128,
                max_y as u16,
            )?;
            if draw_help {
                drawing.draw_help(max_x as u16)?;
            }
            drawing.flush()?;

            let player_point = &cloned_level.player_position();

            match player_point {
                Some(point) => {
                    if poll_event(Duration::from_millis(100))? {
                        if let Event_::Key(event) = read_event()? {
                            let new_position = match event.code {
                                KeyCode_::Up => Point {
                                    x: point.x,
                                    y: point.y - 1,
                                },
                                KeyCode_::Down => Point {
                                    x: point.x,
                                    y: point.y + 1,
                                },
                                KeyCode_::Left => Point {
                                    x: point.x - 1,
                                    y: point.y,
                                },
                                KeyCode_::Right => Point {
                                    x: point.x + 1,
                                    y: point.y,
                                },
                                KeyCode_::Esc => {
                                    terminate = true;
                                    Point {
                                        x: point.x,
                                        y: point.y,
                                    }
                                }
                                KeyCode_::Char('q') => {
                                    terminate = true;
                                    Point {
                                        x: point.x,
                                        y: point.y,
                                    }
                                }
                                KeyCode_::Char('r') => {
                                    restart = true;
                                    Point {
                                        x: point.x,
                                        y: point.y,
                                    }
                                }

                                KeyCode_::Char('h') => {
                                    draw_help = !draw_help;
                                    Point {
                                        x: point.x,
                                        y: point.y,
                                    }
                                }
                                _ => Point {
                                    x: point.x,
                                    y: point.y,
                                },
                            };

                            // check if finished
                            if let Some(finish_point) = cloned_level.finish_position() {
                                if finish_point.x == new_position.x
                                    && finish_point.y == new_position.y
                                {
                                    run = false;
                                    timing.push(level_start.elapsed().as_millis());
                                }
                            }
                            // check if player lost
                            if let Some(&cell) = cloned_level.data.get(&new_position) {
                                match cell {
                                    Cell::CounterClockwiseEnemy(_) => restart = true,
                                    Cell::Void => restart = true,
                                    _ => {}
                                }
                            }
                            cloned_level.move_player(*point, new_position, max_x, max_y);
                        }
                    }

                    // Move enemies after user input
                    if last_enemy_move.elapsed() >= enemy_move_interval {
                        cloned_level.update_enemies();
                        last_enemy_move = Instant::now();
                    }
                }

                // player was removed via enemy. restart.
                None => restart = true,
            }
        }
    }
    drawing.reset()?;
    drawing.flush()?;
    drawing.show_timing(timing)?;
    drawing.flush()?;
    drawing.reset()?;
    Ok(())
}
