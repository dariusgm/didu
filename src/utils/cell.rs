use super::direction::Direction;
use super::point::Point;
use super::powerup::Powerup;

#[derive(Clone, PartialEq, Copy, Eq, Hash)]
pub enum Cell {
    Empty,
    Player(Powerup),
    Exit,
    HorizontalWall,
    VerticalWall,
    CounterClockwiseEnemy(Direction),
    Void,
    Switch(u8),
    Door(u8),
    OneWayTeleporter(Point),
    BreakableGround,
    Invincibility,
}
