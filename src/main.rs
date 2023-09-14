use crossterm::{event::poll, event::read, event::Event, terminal::enable_raw_mode, Result};

mod levels;
mod utils;

use levels::all;
use std::io::stdout;
use std::time::Duration;
use std::time::Instant;
use utils::level::Level;

use utils::cell::Cell;
use utils::drawing::Drawing;

use std::io::Write;
use utils::game_state::GameState;
use utils::point::Point;
use utils::powerup::Powerup;

fn game_loop(
    mut drawing: Drawing<impl Write>,
    mut game_state: GameState,
    levels: Vec<Level>,
    mut timing: Vec<u128>,
) -> Result<()> {
    drawing.init()?;

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
                        let new_position = game_state.point();

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
                        cloned_level.move_player(*point, new_position, max_x, max_y);
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

fn main() -> Result<()> {
    enable_raw_mode()?;
    let drawing = Drawing::new(stdout());
    let game_state = GameState::new();
    let levels = all::levels();
    // Store the duration for each level to later show it to the plaer
    let timing: Vec<u128> = vec![];
    game_loop(drawing, game_state, levels, timing)
}
