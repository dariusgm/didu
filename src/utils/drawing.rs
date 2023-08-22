use crate::utils::cell::Cell;
use crate::utils::level::Level;
use crate::utils::powerup::Powerup;
use crossterm::{
    cursor,
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, Result,
};
use std::{io::Stdout};
use std::{io::Write};

#[derive(Debug)]
pub(crate) struct Drawing {
    stdout: Stdout,
}

impl Drawing {
    pub(crate) fn new() -> Self {
        Drawing {
            stdout: std::io::stdout(),
        }
    }
    pub(crate) fn show_timing(&mut self, timing: Vec<u128>) -> Result<()> {
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

    pub(crate) fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }

    pub(crate) fn init(&mut self) -> Result<()> {
        self.stdout.execute(cursor::Hide)?;
        Ok(())
    }
    pub(crate) fn reset(&mut self) -> Result<()> {
        self.stdout.execute(cursor::Show)?;
        self.stdout.execute(ResetColor)?;
        self.stdout.execute(terminal::Clear(ClearType::All))?;
        Ok(())
    }
    pub(crate) fn draw_ui(
        &mut self,
        level_number: usize,
        elapsed_time: u128,
        max_y: u16,
    ) -> Result<()> {
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

    pub(crate) fn draw_help(&mut self, max_x: u16) -> Result<()> {
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

    pub(crate) fn draw_level(&mut self, level: &Level) -> Result<()> {
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
