use crate::utils::point::Point;

use crossterm::event::KeyEvent;
use crossterm::event::KeyCode;

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

