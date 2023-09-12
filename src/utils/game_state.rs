use crate::utils::point::Point;

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

pub struct GameState {
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
    pub fn new() -> Self {
        GameState {
            event: None,
            terminate: false,
            run: true,
            restart: false,
            help: false,
            point: Point { x: 0, y: 0 },
        }
    }
    pub fn is_terminate(&self) -> bool {
        self.terminate
    }

    pub fn is_run(&self) -> bool {
        self.run
    }

    pub fn is_help(&self) -> bool {
        self.help
    }

    pub fn running(&self) -> Self {
        GameState {
            event: self.event,
            terminate: false,
            run: true,
            help: false,
            point: Point { x: 0, y: 0 },
            restart: self.restart,
        }
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn new_point(&self, point: Point) -> Self {
        GameState {
            event: self.event,
            terminate: self.terminate,
            run: self.run,
            point,
            help: self.help,
            restart: self.restart,
        }
    }
    pub fn restart(&self) -> Self {
        GameState {
            event: self.event,
            terminate: self.terminate,
            run: true,
            help: self.help,
            point: self.point,
            restart: true,
        }
    }

    pub fn help(&self) -> Self {
        GameState {
            event: self.event,
            terminate: self.terminate,
            run: self.run,
            point: self.point,
            help: !self.help,
            restart: self.restart,
        }
    }

    pub fn terminate(&self) -> Self {
        GameState {
            event: self.event,
            terminate: true,
            run: false,
            point: self.point,
            help: self.help,
            restart: false,
        }
    }

    pub fn stop(&self) -> Self {
        GameState {
            event: self.event,
            terminate: self.terminate,
            run: false,
            point: self.point,
            help: self.help,
            restart: self.restart,
        }
    }
    pub fn is_restart(&self) -> bool {
        self.restart
    }

    // check if finished
    pub fn is_finish(&self, finish_position: Option<Point>) -> bool {
        if let Some(finish) = finish_position {
            finish.x == self.point.x && finish.y == self.point.y
        } else {
            false
        }
    }

    // Update movement and global game state
    pub fn update_player_position(&self, event: KeyEvent) -> GameState {
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
#[cfg(test)]
mod tests {

    use super::GameState;
    use super::Point;
    use crossterm::event::KeyCode;
    use crossterm::event::KeyEvent;
    use crossterm::event::KeyModifiers;
    #[test]
    fn new() {
        let game_state = GameState::new();
        assert_eq!(game_state.point.x, 0);
        assert_eq!(game_state.point.y, 0);
        assert_eq!(game_state.event, None);
        assert!(game_state.is_run());
        assert!(!game_state.is_help());
        assert!(!game_state.is_restart());
        assert!(!game_state.is_terminate());
        assert!(game_state.is_finish(Some(Point { x: 0, y: 0 })));
        assert!(!game_state.is_finish(None));
    }
    #[test]
    fn move_up() {
        let event = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        let game_state = GameState::new();

        let new_state = game_state.update_player_position(event);
        assert_eq!(new_state.point.x, 0);
        assert_eq!(new_state.point.y, -1);
    }
    #[test]
    fn move_right() {
        let event = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
        let game_state = GameState::new();

        let new_state = game_state.update_player_position(event);
        assert_eq!(new_state.point.x, 1);
        assert_eq!(new_state.point.y, 0);
    }
    #[test]
    fn move_left() {
        let event = KeyEvent::new(KeyCode::Left, KeyModifiers::NONE);
        let game_state = GameState::new();

        let new_state = game_state.update_player_position(event);
        assert_eq!(new_state.point.x, -1);
        assert_eq!(new_state.point.y, 0);
    }
    #[test]
    fn move_down() {
        let event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        let game_state = GameState::new();

        let new_state = game_state.update_player_position(event);
        assert_eq!(new_state.point.x, 0);
        assert_eq!(new_state.point.y, 1);
    }
    #[test]
    fn escape_game() {
        let event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        let game_state = GameState::new();

        let new_state = game_state.update_player_position(event);
        assert!(new_state.terminate);
    }

    #[test]
    fn q_game() {
        let event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        let game_state = GameState::new();

        let new_state = game_state.update_player_position(event);
        assert!(new_state.terminate);
    }
    #[test]
    fn restart_game() {
        let event = KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE);
        let game_state = GameState::new();

        let new_state = game_state.update_player_position(event);
        assert!(new_state.is_restart());
    }
    #[test]
    fn help() {
        let event = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        let game_state = GameState::new();

        let new_state = game_state.update_player_position(event);
        assert!(new_state.is_help());
    }
    #[test]
    fn to_running_state() {
        let game_state = GameState::new();
        let new_state = game_state.running();
        assert_eq!(new_state.event, game_state.event);
        assert_eq!(new_state.terminate, false);
        assert_eq!(new_state.run, true);
        assert_eq!(new_state.help, false);
        assert_eq!(new_state.point, Point { x: 0, y: 0 });
        assert_eq!(new_state.restart, game_state.restart);
    }

    #[test]
    fn to_restart_state() {
        let game_state = GameState::new();
        let new_state = game_state.restart();
        assert_eq!(new_state.event, game_state.event);
        assert_eq!(new_state.terminate, game_state.terminate);
        assert_eq!(new_state.run, true);
        assert_eq!(new_state.help, game_state.help);
        assert_eq!(new_state.point, game_state.point);
        assert_eq!(new_state.restart, true);
    }
    #[test]
    fn to_help_state() {
        let game_state = GameState::new();
        let new_state = game_state.help();
        assert_eq!(new_state.event, game_state.event);
        assert_eq!(new_state.terminate, game_state.terminate);
        assert_eq!(new_state.run, game_state.run);
        assert_eq!(!new_state.help, game_state.help);
        assert_eq!(new_state.point, game_state.point);
        assert_eq!(new_state.restart, game_state.restart);
    }

    #[test]
    fn to_terminate_state() {
        let game_state = GameState::new();
        let new_state = game_state.terminate();
        assert_eq!(new_state.event, game_state.event);
        assert_eq!(new_state.terminate, true);
        assert_eq!(new_state.run, false);
        assert_eq!(new_state.help, game_state.help);
        assert_eq!(new_state.point, game_state.point);
        assert_eq!(new_state.restart, false);
    }

    #[test]
    fn to_stop_state() {
        let game_state = GameState::new();
        let new_state = game_state.stop();
        assert_eq!(new_state.event, game_state.event);
        assert_eq!(new_state.terminate, game_state.terminate);
        assert_eq!(new_state.run, false);
        assert_eq!(new_state.help, game_state.help);
        assert_eq!(new_state.point, game_state.point);
        assert_eq!(new_state.restart, game_state.restart);
    }
}
