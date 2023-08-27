use super::cell::Cell;
use super::direction::Direction;
use super::point::Point;
use super::powerup::Powerup;
use std::collections::HashMap;
#[derive(Clone)]
pub struct Level {
    pub(crate) data: HashMap<Point, Cell>,
}

impl Level {
    pub fn empty(width: u8, height: u8) -> Self {
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

    pub fn update_enemies(&mut self) {
        for (&point, &cell) in self.data.clone().iter() {
            // try move
            let optional_target_point = match cell {
                Cell::CounterClockwiseEnemy(Direction::Right) => Some(Point {
                    x: point.x + 1,
                    y: point.y,
                }),
                Cell::CounterClockwiseEnemy(Direction::Up) => Some(Point {
                    x: point.x,
                    y: point.y - 1,
                }),
                Cell::CounterClockwiseEnemy(Direction::Down) => Some(Point {
                    x: point.x,
                    y: point.y + 1,
                }),
                Cell::CounterClockwiseEnemy(Direction::Left) => Some(Point {
                    x: point.x - 1,
                    y: point.y,
                }),
                _ => None,
            };
            let target_rotation = match cell {
                Cell::CounterClockwiseEnemy(Direction::Up) => Direction::Left,
                Cell::CounterClockwiseEnemy(Direction::Down) => Direction::Right,
                Cell::CounterClockwiseEnemy(Direction::Left) => Direction::Down,
                _ => Direction::Up,
            };
            if let Some(target_point) = optional_target_point {
                let cloned_cell = &self.data.get(&target_point).cloned();
                match cloned_cell {
                    // we can move
                    Some(Cell::Empty) => {
                        self.update(point, Cell::Empty);
                        self.update(target_point, cell);
                    }
                    // remove player from grid
                    Some(Cell::Player(Powerup::None)) => {
                        self.update(point, Cell::Empty);
                        self.update(target_point, cell)
                    }
                    // player will eat us
                    Some(Cell::Player(Powerup::Invincible(_))) => self.update(point, Cell::Empty),
                    // rotate enemy for the following cases without moving it.
                    Some(Cell::VerticalWall) => {
                        self.update(point, Cell::CounterClockwiseEnemy(target_rotation));
                    }
                    Some(Cell::HorizontalWall) => {
                        self.update(point, Cell::CounterClockwiseEnemy(target_rotation));
                    }
                    Some(Cell::Void) => {
                        self.update(point, Cell::CounterClockwiseEnemy(target_rotation));
                    }
                    Some(Cell::Door(_)) => {
                        self.update(point, Cell::CounterClockwiseEnemy(target_rotation));
                    }
                    Some(Cell::Switch(_)) => {
                        self.update(point, Cell::CounterClockwiseEnemy(target_rotation));
                    }
                    // collision with something else not implemented.
                    // It would require data structure change to have two elements on the same
                    // cell.
                    Some(_) => {}
                    // Out of bounds in front of us, need to rotate as well.
                    None => self.update(point, Cell::CounterClockwiseEnemy(target_rotation)),
                }
            }
        }
    }

    pub fn update(&mut self, point: Point, cell: Cell) {
        self.data.insert(point, cell);
    }
    pub fn door_position(&self, switch_id: u8) -> Option<Point> {
        for (&point, &cell) in self.data.iter() {
            if let Cell::Door(door_id) = cell {
                if door_id == switch_id {
                    return Some(point);
                }
            }
        }
        None
    }
    pub fn finish_position(&self) -> Option<Point> {
        for (&point, &cell) in self.data.iter() {
            if cell == Cell::Exit {
                return Some(point);
            }
        }
        None
    }

    pub fn player_position(&self) -> Option<Point> {
        for (&point, &cell) in self.data.iter() {
            match cell {
                Cell::Player(Powerup::Invincible(_)) => return Some(point),
                Cell::Player(Powerup::None) => return Some(point),
                _ => {}
            }
        }
        None
    }

    fn move_player_regular(&mut self, player: Point, new_position: Point, player_struct: Cell) {
        // Handle collosions here that will not reset the level
        if let Some(cell) = self.data.get(&new_position).cloned() {
            match cell {
                // moved on empty space
                Cell::Empty => {
                    self.update(player, Cell::Empty);
                    self.update(new_position, player_struct);
                }

                // Triggering a switch removes the switch and the related door
                Cell::Switch(switch_id) => {
                    self.update(player, Cell::Empty);
                    self.update(new_position, player_struct);
                    if let Some(door_position) = self.door_position(switch_id) {
                        self.update(door_position, Cell::Empty);
                    }
                }
                // Triggering a teleporter, moves me to the destination
                Cell::OneWayTeleporter(destination_point) => {
                    self.update(player, Cell::Empty);
                    self.update(new_position, Cell::Empty);
                    self.update(destination_point, player_struct);
                }
                //Triggering Invincibility Candy
                Cell::Invincibility => {
                    self.update(player, Cell::Empty);
                    self.update(new_position, Cell::Player(Powerup::Invincible(5)));
                }
                // Move over breakable ground. Replace with Void.
                Cell::BreakableGround => {
                    self.update(player, Cell::Void);
                    self.update(new_position, player_struct);
                }
                // everything else can not be passed
                _ => {}
            }
        }
    }

    fn move_player_invincible(&mut self, player: Point, new_position: Point, player_struct: Cell) {
        // we are invincible for the amount of "moves".
        // This allows us to eat enemies.
        // And to run over void.

        let new_player_struct = match player_struct {
            Cell::Player(Powerup::Invincible(0)) => Cell::Player(Powerup::None),
            Cell::Player(Powerup::Invincible(moves)) => {
                Cell::Player(Powerup::Invincible(moves - 1))
            }
            _ => Cell::Player(Powerup::None),
        };
        let target_cell = self.data.get(&new_position).cloned().unwrap();
        match target_cell {
            // We can run over void in invincibility
            Cell::Void => {
                self.update(player, Cell::Empty);
                self.update(new_position, new_player_struct);
            }
            // we can remove enemies
            Cell::CounterClockwiseEnemy(_) => {
                self.update(player, Cell::Empty);
                self.update(new_position, new_player_struct)
            }
            // else, handle normal movement
            _ => self.move_player_regular(player, new_position, new_player_struct),
        }
    }

    pub fn move_player(&mut self, player: Point, new_position: Point, max_x: i8, max_y: i8) {
        // Handle out of bounds
        if new_position.x >= 0
            && new_position.x <= max_x
            && new_position.y >= 0
            && new_position.y <= max_y
        {
            let player_struct = self.data.get(&player).cloned().unwrap();
            match player_struct {
                Cell::Player(Powerup::None) => {
                    self.move_player_regular(player, new_position, player_struct)
                }
                Cell::Player(Powerup::Invincible(_)) => {
                    self.move_player_invincible(player, new_position, player_struct)
                }

                _ => {}
            }
        }
    }

    pub fn size(&self) -> (i8, i8) {
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
        (max_x, max_y)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let level = Level::empty(2, 2);
        for x in 0..2 {
            for y in 0..2 {
                if let Some(&c) = level.data.get(&Point { x, y }) {
                    assert!(c == Cell::Empty)
                } else {
                    assert!(false)
                }
            }
        }
    }
    #[test]
    fn test_size() {
        let level = Level::empty(3, 2);
        let tuple = level.size();
        let max_x = tuple.0;
        let max_y = tuple.1;
        assert_eq!(max_x, 2);
        assert_eq!(max_y, 1);
    }

    #[test]
    fn test_finish_position() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        if let Some(finish_point) = level.finish_position() {
            assert_eq!(finish_point, Point { x: 1, y: 1 });
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_finish_position_missing() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        assert_eq!(level.finish_position(), None)
    }

    #[test]
    fn test_player_position() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        if let Some(playerpoint) = level.player_position() {
            assert_eq!(playerpoint, Point { x: 0, y: 0 });
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_player_invincible_position() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::Invincible(1)));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        if let Some(playerpoint) = level.player_position() {
            assert_eq!(playerpoint, Point { x: 0, y: 0 });
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_player_position_missing() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        assert_eq!(level.player_position(), None)
    }

    #[test]
    fn test_move_player_out_of_left_bound() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: -1, y: 0 }, 1, 1);
        if let Some(player) = level.data.get(&Point { x: 0, y: 0 }) {
            match player {
                Cell::Player(Powerup::None) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_move_player_out_of_right_bound() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 2, y: 0 }, 1, 1);
        if let Some(player) = level.data.get(&Point { x: 0, y: 0 }) {
            match player {
                Cell::Player(Powerup::None) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_move_player_out_of_lower_bound() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 2 }, 1, 1);
        if let Some(player) = level.data.get(&Point { x: 0, y: 0 }) {
            match player {
                Cell::Player(Powerup::None) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_player_regular_right() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);
        if let Some(player) = level.data.get(&Point { x: 0, y: 1 }) {
            match player {
                Cell::Player(Powerup::None) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_move_player_regular_left() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 1, y: 1 }, Cell::Player(Powerup::None));
        level.update(Point { x: 0, y: 0 }, Cell::Exit);
        level.move_player(Point { x: 1, y: 1 }, Point { x: 0, y: 1 }, 1, 1);
        if let Some(player) = level.data.get(&Point { x: 0, y: 1 }) {
            match player {
                Cell::Player(Powerup::None) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_player_regular_down() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);
        if let Some(player) = level.data.get(&Point { x: 0, y: 1 }) {
            match player {
                Cell::Player(Powerup::None) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_move_player_regular_up() {
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 1, y: 1 }, Cell::Player(Powerup::None));
        level.update(Point { x: 0, y: 0 }, Cell::Exit);
        level.move_player(Point { x: 1, y: 1 }, Point { x: 1, y: 0 }, 1, 1);
        if let Some(player) = level.data.get(&Point { x: 1, y: 0 }) {
            match player {
                Cell::Player(Powerup::None) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_move_on_breakable_ground() {
        // @.  -->  V.
        // ?X       @X
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 0, y: 1 }, Cell::BreakableGround);
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);
        if let Some(player) = level.data.get(&Point { x: 0, y: 0 }) {
            match player {
                Cell::Void => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_on_invincibility_candy() {
        // @.  -->  ..
        // oX       5X
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 0, y: 1 }, Cell::Invincibility);
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);
        if let Some(player) = level.data.get(&Point { x: 0, y: 1 }) {
            match player {
                Cell::Player(Powerup::Invincible(5)) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_invincibility_candy_finish() {
        // @.  -->  ..
        // oX       5X
        let mut level = Level::empty(2, 2);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::Invincible(0)));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);
        if let Some(player) = level.data.get(&Point { x: 0, y: 1 }) {
            match player {
                Cell::Player(Powerup::None) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_on_oneway_teleporter() {
        // @..  -->  ...
        // TX.       .X.
        // ...       ..@
        let mut level = Level::empty(3, 3);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(
            Point { x: 0, y: 1 },
            Cell::OneWayTeleporter(Point { x: 2, y: 2 }),
        );
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);

        // Test teleporter removed
        if let Some(previous_teleporter) = level.data.get(&Point { x: 0, y: 1 }) {
            match previous_teleporter {
                Cell::Empty => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
        // Test Player position
        if let Some(player) = level.data.get(&Point { x: 2, y: 2 }) {
            match player {
                Cell::Player(Powerup::None) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_on_oneway_teleporter_when_invincible() {
        // @..  -->  ...
        // TX.       .X.
        // ...       ..@
        let mut level = Level::empty(3, 3);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::Invincible(5)));
        level.update(
            Point { x: 0, y: 1 },
            Cell::OneWayTeleporter(Point { x: 2, y: 2 }),
        );
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);

        // Test teleporter removed
        if let Some(previous_teleporter) = level.data.get(&Point { x: 0, y: 1 }) {
            match previous_teleporter {
                Cell::Empty => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
        // Test Player position
        if let Some(player) = level.data.get(&Point { x: 2, y: 2 }) {
            match player {
                Cell::Player(Powerup::Invincible(4)) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_on_switch() {
        // @..  -->  ...
        // SX.       @X.
        // .D.       ...
        let mut level = Level::empty(3, 3);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::None));
        level.update(Point { x: 0, y: 1 }, Cell::Switch(1));
        level.update(Point { x: 1, y: 2 }, Cell::Door(1));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);

        // Test Door removed
        if let Some(previous_door) = level.data.get(&Point { x: 1, y: 2 }) {
            match previous_door {
                Cell::Empty => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
        //
        // Test Player position
        if let Some(player) = level.data.get(&Point { x: 0, y: 1 }) {
            match player {
                Cell::Player(Powerup::None) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_as_invinciblle_on_switch() {
        // 5..  -->  ...
        // SX.       @X.
        // .D.       ...
        let mut level = Level::empty(3, 3);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::Invincible(5)));
        level.update(Point { x: 0, y: 1 }, Cell::Switch(1));
        level.update(Point { x: 1, y: 2 }, Cell::Door(1));
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);

        // Test Door removed
        if let Some(previous_door) = level.data.get(&Point { x: 1, y: 2 }) {
            match previous_door {
                Cell::Empty => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
        //
        // Test Player position
        if let Some(player) = level.data.get(&Point { x: 0, y: 1 }) {
            match player {
                Cell::Player(Powerup::Invincible(4)) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_move_as_invinciblle_on_void() {
        // 5..  -->  ...
        // VX.       4X.
        // ...       ...
        let mut level = Level::empty(3, 3);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::Invincible(5)));
        level.update(Point { x: 0, y: 1 }, Cell::Void);
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);

        // Test Player position
        if let Some(player) = level.data.get(&Point { x: 0, y: 1 }) {
            match player {
                Cell::Player(Powerup::Invincible(4)) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }

        // Test Void Position
        if let Some(void) = level.data.get(&Point { x: 0, y: 0 }) {
            match void {
                Cell::Empty => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_move_as_invinciblle_on_enemy() {
        // 5..  -->  ...
        // §X.       4X.
        // ...       ...
        let mut level = Level::empty(3, 3);
        level.update(Point { x: 0, y: 0 }, Cell::Player(Powerup::Invincible(5)));
        level.update(
            Point { x: 0, y: 1 },
            Cell::CounterClockwiseEnemy(Direction::Up),
        );
        level.update(Point { x: 1, y: 1 }, Cell::Exit);
        level.move_player(Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, 1, 1);

        // Test Player position
        if let Some(player) = level.data.get(&Point { x: 0, y: 1 }) {
            match player {
                Cell::Player(Powerup::Invincible(4)) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_update_enemies_move_up() {
        // ...  -->  §..
        // §..       ...
        // ...       ...
        let mut level = Level::empty(3, 3);
        level.update(
            Point { x: 0, y: 1 },
            Cell::CounterClockwiseEnemy(Direction::Up),
        );
        level.update_enemies();
        if let Some(enemy) = level.data.get(&Point { x: 0, y: 0 }) {
            match enemy {
                Cell::CounterClockwiseEnemy(Direction::Up) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
    #[test]
    fn test_update_enemies_move_down() {
        // ...  -->  ...
        // §..       ...
        // ...       §..
        let mut level = Level::empty(3, 3);
        level.update(
            Point { x: 0, y: 1 },
            Cell::CounterClockwiseEnemy(Direction::Down),
        );
        level.update_enemies();
        if let Some(enemy) = level.data.get(&Point { x: 0, y: 2 }) {
            match enemy {
                Cell::CounterClockwiseEnemy(Direction::Down) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
    #[test]
    fn test_update_enemies_move_right() {
        // §..  -->  .§.
        // ...       ...
        // ...       ...
        let mut level = Level::empty(3, 3);
        level.update(
            Point { x: 0, y: 0 },
            Cell::CounterClockwiseEnemy(Direction::Right),
        );
        level.update_enemies();
        if let Some(enemy) = level.data.get(&Point { x: 1, y: 0 }) {
            match enemy {
                Cell::CounterClockwiseEnemy(Direction::Right) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
    #[test]
    fn test_update_enemies_move_left() {
        // .§.  -->  §..
        // ...       ...
        // ...       ...
        let mut level = Level::empty(3, 3);
        level.update(
            Point { x: 1, y: 0 },
            Cell::CounterClockwiseEnemy(Direction::Left),
        );
        level.update_enemies();
        if let Some(enemy) = level.data.get(&Point { x: 0, y: 0 }) {
            match enemy {
                Cell::CounterClockwiseEnemy(Direction::Left) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
    #[test]
    fn test_update_counterclockwise_to_up() {
        // ..  -->  ..
        // .§       .§
        let mut level = Level::empty(2, 2);
        level.update(
            Point { x: 1, y: 1 },
            Cell::CounterClockwiseEnemy(Direction::Right),
        );
        level.update_enemies();
        if let Some(enemy) = level.data.get(&Point { x: 1, y: 1 }) {
            match enemy {
                Cell::CounterClockwiseEnemy(Direction::Up) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
    #[test]
    fn test_update_counterclockwise_to_left() {
        // .§  -->  .§
        // ..       ..
        let mut level = Level::empty(2, 2);
        level.update(
            Point { x: 1, y: 0 },
            Cell::CounterClockwiseEnemy(Direction::Up),
        );
        level.update_enemies();
        if let Some(enemy) = level.data.get(&Point { x: 1, y: 0 }) {
            match enemy {
                Cell::CounterClockwiseEnemy(Direction::Left) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
    #[test]
    fn test_update_counterclockwise_to_down() {
        // §.  -->  §.
        // ..       ..
        let mut level = Level::empty(2, 2);
        level.update(
            Point { x: 0, y: 0 },
            Cell::CounterClockwiseEnemy(Direction::Left),
        );
        level.update_enemies();
        if let Some(enemy) = level.data.get(&Point { x: 0, y: 0 }) {
            match enemy {
                Cell::CounterClockwiseEnemy(Direction::Down) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
    #[test]
    fn test_update_counterclockwise_to_right() {
        // ..  -->  ..
        // §.       §.
        let mut level = Level::empty(2, 2);
        level.update(
            Point { x: 0, y: 1 },
            Cell::CounterClockwiseEnemy(Direction::Down),
        );
        level.update_enemies();
        if let Some(enemy) = level.data.get(&Point { x: 0, y: 1 }) {
            match enemy {
                Cell::CounterClockwiseEnemy(Direction::Right) => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
}
