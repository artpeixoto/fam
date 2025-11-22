use macroquad::math::{I16Vec2, U16Vec2};
use crate::application::direction::Axis;
use crate::application::direction::Axis::{Horizontal, Vertical};
use crate::application::grid::pos::{grid_pos, GridPos};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct GridLine {
    pub (in super) index    : GridPos,
    pub axis                : Axis,
}

impl GridLine {
    pub fn points(&self) -> [GridPos;2] {
        let first = grid_pos(self.index.x, self.index.y);
        let second = match  self.axis {
            Horizontal  => first + I16Vec2::new(1, 0),
            Vertical    => first + I16Vec2::new(0, 1),
        };
        [first, second]
    }
}
