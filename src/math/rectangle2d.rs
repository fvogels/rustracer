use super::point2d::Point2D;
use super::vector2d::Vector2D;

pub struct Rectangle2D {
    pub origin: Point2D,
    pub x_axis: Vector2D,
    pub y_axis: Vector2D,
}

impl Rectangle2D {
    pub fn new(origin: Point2D, x_axis: Vector2D, y_axis: Vector2D) -> Rectangle2D {
        Rectangle2D {
            origin,
            x_axis,
            y_axis,
        }
    }
}
