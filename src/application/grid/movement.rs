use std::ops::Add;
use macroquad::math::u16vec2;
use crate::application::direction::Direction;
use crate::application::direction::Direction::{Down, Left, Up};
use crate::application::direction::Axis::{Horizontal, Vertical};
use crate::application::grid::line::GridLine;
use crate::application::grid::pos::{grid_pos, GridPos};

impl Add<Direction> for GridPos {
    type Output = GridMovement;

    #[inline(always)]
    fn add(self, rhs: Direction) -> GridMovement {
        self.go(rhs)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct GridMovement {
    pub starting_point      : GridPos,
    pub move_dir            : Direction,
    pub line                : GridLine,
    pub destination_point   : GridPos
}

impl GridPos {
    pub fn all_moves(&self) -> Vec<GridMovement> {
        Direction
        ::all_directions()
        .filter_map(|dir|
            if self.y == 0 && dir == Up{
                return None;
            } else if self.x == 0 && dir == Left{
                return None;
            } else {
                Some(self.go(dir))
            }
        )
        .collect()
    }

    pub const fn go(self, dir: Direction) -> GridMovement {
        let (line, destination) = match dir {
            Up => {
                let target = grid_pos(self.x, self.y - 1);
                let line = GridLine{
                    index: target,
                    hor_or_ver: Vertical,
                };
                (line, target)
            }
            Direction::Right => {
                let target = grid_pos(self.x + 1, self.y );
                let line = GridLine{
                    index: self,
                    hor_or_ver: Horizontal,
                };
                (line, target)
            }
            Down => {
                let target = grid_pos(self.x, self.y + 1 );
                let line = GridLine{
                    index: self,
                    hor_or_ver: Vertical,
                };
                (line, target)
            }
            Left => {
                let target = grid_pos(self.x  - 1, self.y );
                let line = GridLine{
                    index: target,
                    hor_or_ver: Horizontal,
                };
                (line, target)
            }
        };
        
        GridMovement {
            starting_point   : self,
            move_dir         : dir,
            line,
            destination_point: destination
        }
    }
}
