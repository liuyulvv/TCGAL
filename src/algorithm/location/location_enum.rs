#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Segment2Location {
    On,
    Left,
    Right,
    Collinear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Circle2Location {
    On,
    Inside,
    Outside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Polygon2Location {
    On,
    Inside,
    Outside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Triangle2Location {
    On,
    Inside,
    Outside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2Ray2Location {
    On,
    Left,
    Right,
    Collinear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point2ArcSegment2Location {
    On,
    Left,
    Right,
}
