use std::ops::{Mul, Neg};
use macroquad::math::{vec2, Vec2};
use crate::application::draw::pos::{dist, Dist, ScreenUnit};

pub use Direction::*;
pub use HorOrVer::{*};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy )]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn all_directions() -> impl Iterator<Item=Direction> + Clone + Sized{
        use Direction::*;
        [Up, Right, Down, Left].into_iter()
    }
}



impl Direction{
    pub fn rotate_ccw(&self) -> Direction{
        match self{
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }
    pub fn rotate_cw(&self) -> Direction{
        match self{
            Up    => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}
impl Mul<u32> for &Direction {
    type Output = Dist;

    fn mul(self, rhs: u32) -> Self::Output {
        self.mul(rhs as i32)
    }
}
impl Mul<u32> for Direction {
    type Output = Dist;

    fn mul(self, rhs: u32) -> Self::Output {
        self.mul(rhs as i32)
    }
}

impl Mul<ScreenUnit> for &Direction {
    type Output = Dist;

    fn mul(self, rhs: i32) -> Self::Output {
        (*self) .mul(rhs)
    }
}

impl Mul<ScreenUnit> for Direction {
    type Output = Dist;

    fn mul(self, rhs: i32) -> Self::Output {
        match self{
            Up      => dist(0, -rhs),
            Down    => dist(0, rhs),
            Right   => dist(rhs, 0),
            Left    => dist(-rhs, 0),
        }
    }
}


impl Mul<f32> for &Direction {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        match self{
            Up      => vec2(0.0, -rhs),
            Down    => vec2(0.0, rhs),
            Right   => vec2(rhs, 0.0),
            Left    => vec2(-rhs, 0.0),
        }
    }
}
impl Mul<f32> for Direction {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        match self{
            Up      => vec2(0.0, -rhs),
            Down    => vec2(0.0, rhs),
            Right   => vec2(rhs, 0.0),
            Left    => vec2(-rhs, 0.0),
        }
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self{
            Up       => Down,
            Right    => Left,
            Down     => Up,
            Left     => Right,
        }
    }
}

impl Direction {
    pub const fn horizontal_or_vertical(&self) -> HorOrVer{
        match self{
            Up    |
            Down  => Vertical,
            Left  |
            Right => Horizontal,
        }
    }
}


#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy )]
pub enum HorOrVer{
    Horizontal,
    Vertical,
}