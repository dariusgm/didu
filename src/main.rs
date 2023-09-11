use crossterm::event::KeyEvent;
use crossterm::{
    event::poll, event::read, event::Event, event::KeyCode, terminal::enable_raw_mode, Result,
};

mod levels;
mod utils;

use levels::all;
use std::io::stdout;
use std::time::Duration;
use std::time::Instant;

use utils::cell::Cell;
use utils::drawing::Drawing;

use utils::point::Point;
use utils::powerup::Powerup;

struct GameState {
    event: Option<KeyEvent>,
    // terminates the entire game
    terminate: bool,
    // responsible for game loop
    run: bool,
    // draw help overlay
    help: bool,
    // current player position or new point, depending on the context
    point: Point,
    // restarts the current level
    restart: bool,
}

impl GameState {
    fn new() -> Self {
        GameState {
            event: None,
            terminate: false,
            run: true,
            restart: false,
            help: false,
            point: Point { x: 0, y: 0 },
        }
    }
    fn is_terminate(&self) -> bool {
        self.terminate
    }

    fn is_run(&self) -> bool {
        self.run
    }

    fn is_help(&self) -> bool {
        self.help
    }

    fn running(&self) -> Self {
        GameState {
            event: self.event,
            terminate: false,
            run: true,
            help: false,
            point: Point { x: 0, y: 0 },
            restart: self.restart,
        }
    }

    fn new_point(&self, point: Point) -> Self {
        GameState {
            event: self.event,
            terminate: self.terminate,
            run: self.run,
            point: point,
            help: self.help,
            restart: self.restart,
        }
    }
    fn restart(&self) -> Self {
        GameState {
            event: self.event,
            terminate: self.terminate,
            run: true,
            help: self.help,
            point: self.point,
            restart: true,
        }
    }

    fn help(&self) -> Self {
        GameState {
            event: self.event,
            terminate: self.terminate,
            run: self.run,
            point: self.point,
            help: !self.help,
            restart: self.restart,
        }
    }

    fn terminate(&self) -> Self {
        GameState {
            event: self.event,
            terminate: true,
            run: false,
            point: self.point,
            help: self.help,
            restart: false,
        }
    }

    fn stop(&self) -> Self {
        GameState {
            event: self.event,
            terminate: self.terminate,
            run: false,
            point: self.point,
            help: self.help,
            restart: self.restart,
        }
    }
    fn is_restart(&self) -> bool {
        self.restart
    }

    // check if finished
    fn is_finish(&self, finish_position: Option<Point>) -> bool {
        if let Some(finish) = finish_position {
            finish.x == self.point.x && finish.y == self.point.y
        } else {
            false
        }
    }

    // Update movement and global game state
    fn update_player_position(&self, event: KeyEvent) -> GameState {
        let point = self.point;
        match event.code {
            KeyCode::Up => self.new_point(Point {
                x: point.x,
                y: point.y - 1,
            }),
            KeyCode::Down => self.new_point(Point {
                x: point.x,
                y: point.y + 1,
            }),
            KeyCode::Left => self.new_point(Point {
                x: point.x - 1,
                y: point.y,
            }),
            KeyCode::Right => self.new_point(Point {
                x: point.x + 1,
                y: point.y,
            }),
            KeyCode::Esc => self.terminate(),
            KeyCode::Char('q') => self.terminate(),
            KeyCode::Char('r') => self.restart(),

            KeyCode::Char('h') => self.help(),
            _ => self.new_point(Point {
                x: point.x,
                y: point.y,
            }),
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut drawing = Drawing::new(stdout());
    let mut game_state = GameState::new();
    let levels = all::levels();

    drawing.init()?;

    // Store the duration for each level to later show it to the plaer
    let mut timing: Vec<u128> = vec![];
    // for each level, clone the level into the buffer for modification.
    // Than run the game loop
    for (level_index, level) in levels.iter().enumerate() {
        if game_state.is_terminate() {
            break;
        }
        let mut cloned_level = level.clone();
        // As we are not terminating, we need to enable the global level loop.
        game_state = game_state.running();

        // Initialize the player position to the game state to further handle the movement of the player
        if let Some(player_position) = cloned_level.player_position() {
            game_state = game_state.new_point(player_position);
        }
        let (max_x, max_y) = level.size();
        // Count time needed
        let mut level_start = Instant::now();
        let enemy_move_interval = Duration::from_millis(500); // Enemies move every 500 ms
        let mut last_enemy_move = Instant::now();
        while game_state.is_run() {
            if game_state.is_restart() {
                cloned_level = level.clone();
                level_start = Instant::now();
                game_state = GameState::new();
                last_enemy_move = Instant::now();
            }

            drawing.draw_level(&cloned_level)?;
            drawing.draw_ui(
                level_index + 1,
                level_start.elapsed().as_secs() as u128,
                max_y as u16,
            )?;
            if game_state.is_help() {
                drawing.draw_help(max_x as u16)?;
            }
            drawing.flush()?;

            if let Some(point) = &cloned_level.player_position() {
                if poll(Duration::from_millis(100))? {
                    if let Event::Key(event) = read()? {
                        // This is the position the player wants to move
                        game_state = game_state.update_player_position(event);
                        let new_position = game_state.point;

                        if game_state.is_finish(level.finish_position()) {
                            game_state = game_state.stop();
                            timing.push(level_start.elapsed().as_millis());
                        }

                        // collision forcing a restart when no powerup is active
                        if let Some(Cell::Player(Powerup::None)) = cloned_level.data.get(point) {
                            if let Some(Cell::CounterClockwiseEnemy(_)) =
                                cloned_level.data.get(&new_position)
                            {
                                game_state = game_state.restart();
                            }

                            if let Some(Cell::Void) = cloned_level.data.get(&new_position) {
                                game_state = game_state.restart();
                            }
                        }
                        // update player position on the instance of the current level
                        // here we do the validation and handle all the allowed moves.
                        cloned_level.move_player(*point, new_position.clone(), max_x, max_y);
                        // now we need to sync back the player position to the game state.
                        // Maybe we move this somewhere else...
                        if let Some(moved_player_position) = cloned_level.player_position() {
                            game_state = game_state.new_point(moved_player_position)
                        }
                    }
                }

                // Move enemies after user input
                // Only call this when we still have a player on the map
                if last_enemy_move.elapsed() >= enemy_move_interval {
                    cloned_level.update_enemies();
                    last_enemy_move = Instant::now();
                }
            // player was removed via enemy or void. force restart.
            } else {
                game_state = game_state.restart();
            }
        }
    }
    drawing.reset()?;
    drawing.flush()?;
    if !timing.is_empty() {
        drawing.show_timing(timing)?;
    }
    Ok(())
}
