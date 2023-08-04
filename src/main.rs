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
    Enemy(Direction),
    Void,
    Switch(u8),
    Door(u8),
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
        for (&point, &cell) in self.data.iter() {
            match cell {
                // try move right
                Cell::Enemy(Direction::Right) => {}
                _ => {}
            }
        }
    }

    fn update(&mut self, point: Point, cell: Cell) {
        self.data.insert(point.clone(), cell.clone());
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
            match self.data.get(&new_position).cloned() {
                Some(cell) => match cell {
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
    let mut level_data = Level::empty(3, 5);
    level_data.update(Point { x: 0, y: 0 }, Cell::Enemy(Direction::Right));
    level_data.update(Point { x: 0, y: 2 }, Cell::Enemy(Direction::Up));
    level_data.update(Point { x: 0, y: 4 }, Cell::Player);

    level_data.update(Point { x: 1, y: 1 }, Cell::Void);
    level_data.update(Point { x: 1, y: 3 }, Cell::Void);
    level_data.update(Point { x: 1, y: 4 }, Cell::Void);

    level_data.update(Point { x: 2, y: 0 }, Cell::Enemy(Direction::Down));
    level_data.update(Point { x: 2, y: 4 }, Cell::Exit);
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
    fn draw_ui(&mut self, level_number: usize, elapsed_time: u128) -> Result<()> {
        // Clear the status bar line
        queue!(
            self.stdout,
            MoveTo(0, 10),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
        )?;

        // Print status bar
        queue!(
            self.stdout,
            MoveTo(0, 10),
            Print(format!("Level: {}, Time: {}", level_number, elapsed_time)),
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
                Cell::Enemy(_) => {
                    self.stdout.execute(SetForegroundColor(Color::DarkRed))?;
                    print!("§")
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
    let levels = vec![level_3(), level_1(), level_2()];
    drawing.init()?;
    let mut timing: Vec<u128> = vec![];
    let mut terminate = false;
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
            drawing.draw_ui(level_index + 1, level_start.elapsed().as_secs() as u128)?;
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
                                    Cell::Enemy(_) => restart = true,
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

                None => print!("Game Over"),
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
