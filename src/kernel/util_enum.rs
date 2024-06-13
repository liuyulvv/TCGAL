#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnDirection {
    Left,
    Right,
    Collinear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge2Type {
    Segment,
    Arc,
}
