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
    terminal::{self, ClearType},
    ExecutableCommand, Result,
};
use std::{collections::HashMap, io::Stdout, time::Duration};
use std::{io::Write, time::Instant};
#[derive(Clone, PartialEq, Copy, Eq, Hash)]
enum Cell {
    Empty,
    Player,
    Exit,
    HorizontalWall,
    VerticalWall,
    Enemy,
    Void,
    Switch(u8),
    Door(u8),
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
                    // everything else can not be passed
                    _ => {}
                },

                None => {}
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
    fn draw_ui(&mut self, level_number: u32, elapsed_time: u128, score: u32) -> Result<()> {
        let (_, terminal_height) = crossterm::terminal::size()?;

        // Calculate status bar position
        let status_bar_position = terminal_height as usize - 1;

        // Clear the status bar line
        queue!(
            self.stdout,
            MoveTo(0, status_bar_position as u16),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
        )?;

        // Print status bar
        queue!(
            self.stdout,
            MoveTo(0, status_bar_position as u16),
            Print(format!(
                "Level: {}, Time: {}, Score: {}",
                level_number, elapsed_time, score
            )),
        )?;

        Ok(())
    }
    fn draw_level(&mut self, level: &Level) -> Result<()> {
        self.stdout.execute(terminal::Clear(ClearType::All))?;
        for (point, cell) in level.data.iter() {
            self.stdout
                .execute(MoveTo(point.x as u16, point.y as u16))?;
            match cell {
                Cell::Empty => {
                    self.stdout.execute(SetForegroundColor(Color::Blue))?;
                    print!(".");
                }
                Cell::Player => {
                    self.stdout.execute(SetForegroundColor(Color::Red))?;
                    print!("@");
                }
                Cell::Exit => {
                    self.stdout.execute(SetForegroundColor(Color::White))?;
                    self.stdout.execute(SetBackgroundColor(Color::Black))?;
                    print!("X");
                }
                Cell::Void => {
                    self.stdout.execute(SetForegroundColor(Color::Black))?;
                    self.stdout.execute(SetBackgroundColor(Color::Black))?;
                    print!(" ");
                }
                Cell::VerticalWall => {
                    self.stdout.execute(SetForegroundColor(Color::Grey))?;
                    self.stdout.execute(SetBackgroundColor(Color::Red))?;
                    print!("|");
                }
                Cell::HorizontalWall => {
                    self.stdout.execute(SetForegroundColor(Color::Grey))?;
                    self.stdout.execute(SetBackgroundColor(Color::Red))?;
                    print!("-");
                }

                Cell::Door(_) => {
                    self.stdout.execute(SetForegroundColor(Color::Red))?;
                    print!("D");
                }
                Cell::Switch(_) => {
                    self.stdout.execute(SetForegroundColor(Color::Green))?;
                    print!("S");
                }
                _ => {}
            }
            self.stdout.execute(ResetColor)?;
        }
        Ok(())
    }
}
fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut drawing = Drawing::new();
    // let mut level = level_1();
    let mut level = level_2();
    let level_start = Instant::now();
    let (max_x, max_y) = level.size();
    drawing.init()?;
    let mut run = true;
    while run {
        drawing.draw_level(&level)?;
        drawing.draw_ui(1, level_start.elapsed().as_secs() as u128, 0)?;
        drawing.flush()?;

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
                            if cell == Cell::Enemy || cell == Cell::Void {
                                run = false
                            }
                        }
                        level.move_player(*point, new_position, max_x, max_y);
                    }
                }
            }

            None => print!("Game Over"),
        }
        // drawing.flush()?;
    }
    drawing.reset()?;
    drawing.flush()?;
    Ok(())
}
