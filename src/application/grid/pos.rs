use std::ops::{Add, Neg, Sub};
use macroquad::math::{u16vec2, I16Vec2, U16Vec2};



// #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
// pub struct GridPos {
//     pub x: i16,
//     pub y: i16
// }
pub type GridPos = I16Vec2;
pub type GridDist = I16Vec2;
pub type GridSize = I16Vec2;
pub type GridUnit = i16;

pub use macroquad::math::i16vec2 as grid_size;
pub use macroquad::math::i16vec2 as grid_dist;
pub use macroquad::math::i16vec2 as grid_pos;

// pub const fn grid_pos(x: i16, y: i16 ) -> GridPos {
//     GridPos {x, y}
// }
// impl Into<GridPos> for I16Vec2 {
//     fn into(self) -> GridPos {
//         GridPos {x: self.x, y: self.y}
//     }
// }

// impl Add<I16Vec2> for GridPos {
//     type Output = GridPos;

//     fn add(self, rhs: I16Vec2) -> Self::Output {
//         GridPos {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y
//         }
//     }
// }

// impl Sub<I16Vec2> for GridPos {
//     type Output = GridPos;
//     fn sub(self, rhs: I16Vec2) -> Self::Output {
//         self + rhs.neg()
//     }
// }

// impl Sub<GridPos> for GridPos {
//     type Output = I16Vec2;

//     fn sub(self, rhs: GridPos) -> Self::Output {
//         I16Vec2::new(
//             self.x - rhs.x ,
//             self.y - rhs.y,
//         )
//     }
// }
// impl GridPos {
//     pub fn to_fgrid_point(&self) -> GridFPos {
//         GridFPos {x: self.x as f32, y: self.y as f32}
//     }
// }

#[derive(Clone, Copy, PartialEq,  Debug)]
pub struct GridFPos {
    pub x: f32,
    pub y: f32
}

impl GridFPos {
    
}