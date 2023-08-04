use crossterm::{
    cursor,
    cursor::MoveTo,
    event::poll as poll_event,
    event::read as read_event,
    event::Event as Event_,
    event::KeyCode as KeyCode_,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::enable_raw_mode,
    terminal::{self, ClearType},
    ExecutableCommand, Result,
};
use std::{collections::HashMap, io::Stdout, time::Duration};

#[derive(Clone, PartialEq, Copy, Eq, Hash)]
enum Cell {
    Empty,
    Player,
    Exit,
    Wall,
    Enemy,
}

#[derive(Clone, PartialEq, Copy, Eq, Hash)]
enum GlobalState {
    Start,
    Running,
    Terminate,
    Finished,
}

#[derive(Clone, PartialEq, Copy, Eq, Hash, Debug)]
struct Point {
    x: i8,
    y: i8,
}

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
    fn update(&mut self, point: Point, cell: Cell) {
        self.data.insert(point.clone(), cell.clone());
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
            println!("{:?}", new_position);
            // Handle collosions here
            match self.data.get(&new_position) {
                Some(cell) => match cell {
                    // moved on empty space
                    Cell::Empty => {
                        self.update(player, Cell::Empty);
                        self.update(new_position, Cell::Player);
                    }
                    _ => {}
                },

                None => {}
            }
        } else {
            println!("Moved to invalid position")
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
        return (max_x, max_y);
    }
}

struct Levels {
    levels: Vec<Level>,
}

fn level_1() -> Level {
    let mut level_data = Level::empty(4, 4);
    level_data.update(Point { x: 0, y: 0 }, Cell::Player);
    level_data.update(Point { x: 3, y: 3 }, Cell::Exit);
    level_data
}

fn draw_level(level: &Level, mut stdout: &Stdout) -> Result<()> {
    stdout.execute(terminal::Clear(ClearType::All))?;
    for (point, cell) in level.data.iter() {
        stdout.execute(MoveTo(point.x as u16, point.y as u16))?;
        match cell {
            Cell::Empty => {
                stdout.execute(SetForegroundColor(Color::Blue))?;
                print!(".");
            }
            Cell::Player => {
                stdout.execute(SetForegroundColor(Color::Red))?;
                print!("@");
            }
            Cell::Exit => {
                stdout.execute(SetForegroundColor(Color::Green))?;
                print!("X");
            }
            _ => {}
        }
        stdout.execute(ResetColor)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    let mut level = level_1();
    let (max_x, max_y) = level.size();
    stdout.execute(cursor::Hide)?;
    let mut run = true;
    while run {
        draw_level(&level, &stdout)?;
        let player_point = &level.player_position();

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
                                run = false;
                                Point {
                                    x: point.x,
                                    y: point.y,
                                }
                            }
                            KeyCode_::Char('q') => {
                                run = false;
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

                        println!("{:?}", new_position);
                        // check if finished
                        if let Some(finish_point) = level.finish_position() {
                            if finish_point.x == new_position.x && finish_point.y == new_position.y
                            {
                                run = false
                            }
                        }
                        // check if player lost
                        if let Some(&cell) = level.data.get(&new_position) {
                            if cell == Cell::Enemy {
                                run = false
                            }
                        }
                        level.move_player(*point, new_position, max_x, max_y);
                    }
                }
            }

            None => print!("Game Over"),
        }
    }
    stdout.execute(cursor::Show)?;
    stdout.execute(ResetColor)?;
    stdout.execute(terminal::Clear(ClearType::All))?;
    Ok(())
}
