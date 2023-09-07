use crate::utils::cell::Cell;
use crate::utils::level::Level;
use crate::utils::powerup::Powerup;
use crate::Point;
use crossterm::{
    cursor,
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, Result,
};
use std::io::Write;

#[derive(Debug)]
pub(crate) struct Drawing<W: Write> {
    stdout: W,
}

impl<W: Write> Drawing<W> {
    pub(crate) fn new(stdout: W) -> Self {
        Drawing { stdout }
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
        let mut keys: Vec<&Point> = level.data.keys().collect();
        keys.sort();
        for point in keys {
            let cell = level.data.get(point).unwrap();
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

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::utils::direction::Direction;
    use crate::utils::level::Level;

    use super::Cell;
    use super::Drawing;
    use super::Powerup;
    use crate::Point;
    use crossterm::Result;
    use regex::Regex;
    fn strip_ansi_codes(s: &str) -> String {
        let re = Regex::new(r"\x1b\[\d*(;\d*)*[a-zA-Z]").unwrap();
        re.replace_all(s, "").to_string()
    }

    #[test]
    fn show_timing() -> Result<()> {
        // Prepare a buffer to capture the output
        let mut buffer = Vec::new();

        // Initialize the Drawing object with the buffer
        let mut drawing = Drawing::new(&mut buffer);

        // Data to be passed to the show_timing method for testing
        let timing_data = vec![100, 200, 300];

        // Call the method to be tested
        drawing.show_timing(timing_data)?;

        // Convert the buffer to a String to verify its contents
        let output = String::from_utf8(buffer).unwrap();

        // Remove Ascii
        let escaped_output = strip_ansi_codes(&output);

        // Expected output based on the timing_data
        let expected_output = "Level | Time in ms0 | 100\n1 | 200\n2 | 300\n";

        // Assert that the method works as expected
        assert_eq!(escaped_output, expected_output);

        Ok(())
    }

    #[test]
    fn flush() {
        let buffer = Vec::new();
        let mut drawing = Drawing::new(buffer);
        let _ = drawing.flush();
        assert!(true);
    }

    #[test]
    fn init() {
        let buffer = Vec::new();
        let mut drawing = Drawing::new(buffer);
        let _ = drawing.init();
        assert!(true);
    }

    #[test]
    fn drawing_level_items() {
        let mut level = Level::empty(1, 1);
        level.update(Point { x: 0, y: 0 }, Cell::Empty);
        level.update(Point { x: 1, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 2, y: 0 }, Cell::Exit);
        level.update(Point { x: 3, y: 0 }, Cell::HorizontalWall);
        level.update(Point { x: 4, y: 0 }, Cell::VerticalWall);
        level.update(Point { x: 5, y: 0 }, Cell::Void);
        level.update(Point { x: 6, y: 0 }, Cell::Door(1));
        level.update(Point { x: 7, y: 0 }, Cell::Switch(1));
        level.update(Point { x: 8, y: 0 }, Cell::Invincibility);
        level.update(Point { x: 9, y: 0 }, Cell::BreakableGround);

        level.update(
            Point { x: 10, y: 0 },
            Cell::OneWayTeleporter(Point { x: 0, y: 0 }),
        );

        level.update(
            Point { x: 11, y: 0 },
            Cell::CounterClockwiseEnemy(Direction::Up),
        );

        level.update(Point { x: 12, y: 0 }, Cell::Player(Powerup::Invincible(5)));

        level.update(Point { x: 13, y: 0 }, Cell::Player(Powerup::Invincible(4)));
        level.update(Point { x: 14, y: 0 }, Cell::Player(Powerup::Invincible(3)));
        level.update(Point { x: 15, y: 0 }, Cell::Player(Powerup::Invincible(2)));
        level.update(Point { x: 16, y: 0 }, Cell::Player(Powerup::Invincible(1)));
        level.update(Point { x: 17, y: 0 }, Cell::Player(Powerup::Invincible(0)));
        level.update(Point { x: 18, y: 0 }, Cell::Player(Powerup::Invincible(6)));
        let mut buffer = Vec::new();
        let mut drawing = Drawing::new(&mut buffer);
        let _ = drawing.draw_level(&level);
        let _ = drawing.flush();
        let output = String::from_utf8(buffer).unwrap();

        // Remove Ascii
        let escaped_output = strip_ansi_codes(&output);

        // Expected output based on the Cells
        let expected_output = ".@X-| DSo?T§543210@";

        // Assert that the method works as expected
        assert_eq!(escaped_output, expected_output);
    }

    #[test]
    fn draw_help() {
        let mut buffer = Vec::new();
        let mut drawing = Drawing::new(&mut buffer);
        let _ = drawing.draw_help(3);
        let _ = drawing.flush();
        let output = String::from_utf8(buffer).unwrap();
        let escaped_output = strip_ansi_codes(&output);
        // expect all cells are explained in the help overview.
        let cell_types = vec!["@", "X", "|", "-", "D", "S", "o", "?"];
        for cell_type in cell_types {
            assert!(escaped_output.contains(cell_type))
        }
    }

    #[test]
    fn draw_ui() {
        let mut buffer = Vec::new();
        let mut drawing = Drawing::new(&mut buffer);
        let _ = drawing.draw_ui(1, 123456, 5);
        let _ = drawing.flush();
        let output = String::from_utf8(buffer).unwrap();
        let escaped_output = strip_ansi_codes(&output);
        let expected_output = "Level: 1, Time: 123456, h = toggle help";
        assert_eq!(escaped_output, expected_output);
    }
}
