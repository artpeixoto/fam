use crate::application::{direction::Direction, grid::pos::{GridPos, GridSize, GridUnit, grid_pos, grid_size}};

#[derive(Clone, PartialEq, Eq, Debug,)]
pub struct GridRect{
    pub top_left    : GridPos,
    pub size        : GridSize,
}

pub const fn grid_rect(top_left: GridPos, size : GridSize) -> GridRect {
    GridRect::new(top_left, size)
}


impl GridRect{
    pub const fn new(top_left: GridPos, size: GridSize) -> Self{
        // TODO: adicionar logica para lidar com as possibilidades melhor
        Self{top_left, size}
    }
    
    pub fn new_from_points(GridPos{x: x0, y: y0} : GridPos, GridPos{x: x1, y: y1}: GridPos) -> Self{
        let (left, right) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (top, bottom) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        Self{
            top_left: grid_pos( left, top ),
            size    : grid_size( right - left, bottom - top ),
        }
    }
    pub fn pos(&self, dir: Direction) -> GridUnit{
        match dir{
            Direction::Up       => self.top_left.y,
            Direction::Left     => self.top_left.x,
            Direction::Down     => self.top_left.y + self.size.y,
            Direction::Right    => self.top_left.x + self.size.x,
        }
    }
}