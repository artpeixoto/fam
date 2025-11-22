use macroquad::math::U16Vec2;
use crate::application::direction::Axis::{Horizontal, Vertical};
use crate::application::grid::line::GridLine;
use crate::application::grid::pos::GridPos;

pub struct GridLimits(pub U16Vec2);
impl GridLimits {
    pub fn new(size: U16Vec2) -> Self{
        Self(size)
    }
}

// pub struct GridRect{
//     top_left    : GridPoint,
//     bottom_right: GridPoint,
// }
//
// impl GridRect{
//     pub fn new_from_2_points(p0: GridPoint, p1: GridPoint) -> Self{
//         let top = min(p0.0.y, p1.0.y);
//         let bottom = max(p0.0.y, p1.0.y);
//         let left = min(p0.0.x, p1.0.x);
//         let right = max(p0.0.x, p1.0.x);
//         Self{
//             top_left: grid_point(left, top),
//             bottom_right: grid_point( right, bottom)
//         }
//     }
// }



impl GridLimits {
    pub fn contains_line(&self, line: &GridLine) -> bool{
        let max_x = (self.0.x - 1) as i16;
        let max_y = (self.0.y - 1) as i16;
        
        let  in_max_limits = match line.axis{
            Horizontal => {
                line.index.x < max_x && line.index.y <= max_y
            }
            Vertical => {
                line.index.y < max_y &&  line.index.x <= max_x 
            }
        };
        let in_min_limits = line.index.x >= 0 && line.index.y >= 0;
        
        in_min_limits && in_max_limits
    }

    pub fn contains_point(&self, point: &GridPos) -> bool {
        #[allow(unused_parens)]
        (   point.x <= (self.0.x - 1) as i16
        &&  point.x >= 0
        &&  point.y <= (self.0.y - 1) as i16
        &&  point.y >= 0
        )
    }
}
