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
use utils::drawing::Drawing;
use utils::level::Level;
use utils::point::Point;
use utils::powerup::Powerup;

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
