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

mod levels;
mod utils;

use levels::all;
use std::{io::Stdout, time::Duration};
use std::{io::Write, time::Instant};
use utils::cell::Cell;
use utils::level::Level;
use utils::point::Point;
use utils::powerup::Powerup;
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
            Print("  = Void, you should not walk on it.")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 4),
            Print("| = Vertical wall that you can't pass.")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 5),
            Print("- = Horizontal wall that you can't pass.")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 6),
            Print("S = A switch that opens a door. ")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 7),
            Print("D = A door. You need the correct Switch to open it.")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 8),
            Print("§ = An enemy! Watch out!")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 9),
            Print("T = A one way teleporter. ")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 10),
            Print("? = Breakable ground. Will transfer to void after passed once.")
        )?;
        queue!(
            self.stdout,
            MoveTo(max_x + 4, 11),
            Print("o = Surprise Candy. Makes you feel really strong!")
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
                Cell::Player(Powerup::None) => {
                    queue!(self.stdout, SetForegroundColor(Color::Red), Print("@"))?;
                }
                Cell::Player(Powerup::Invincible(5)) => {
                    queue!(self.stdout, SetForegroundColor(Color::Red), Print("5"))?;
                }
                Cell::Player(Powerup::Invincible(4)) => {
                    queue!(self.stdout, SetForegroundColor(Color::Red), Print("4"))?;
                }
                Cell::Player(Powerup::Invincible(3)) => {
                    queue!(self.stdout, SetForegroundColor(Color::Red), Print("3"))?;
                }
                Cell::Player(Powerup::Invincible(2)) => {
                    queue!(self.stdout, SetForegroundColor(Color::Red), Print("2"))?;
                }
                Cell::Player(Powerup::Invincible(1)) => {
                    queue!(self.stdout, SetForegroundColor(Color::Red), Print("1"))?;
                }
                Cell::Player(Powerup::Invincible(0)) => {
                    queue!(self.stdout, SetForegroundColor(Color::Red), Print("0"))?;
                }
                Cell::Player(Powerup::Invincible(_)) => {
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
    let levels = all::levels();

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

                            if let Some(&player_cell) = cloned_level.data.get(&point) {
                                match player_cell {
                                    Cell::Player(Powerup::None) => {
                                        if let Some(&cell) = cloned_level.data.get(&new_position) {
                                            match cell {
                                                Cell::CounterClockwiseEnemy(_) => restart = true,
                                                Cell::Void => restart = true,
                                                _ => {}
                                            }
                                        }
                                    }
                                    // Using powerup, no checks here
                                    _ => {}
                                }
                            }
                            // check if player lost
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
    Ok(())
}
