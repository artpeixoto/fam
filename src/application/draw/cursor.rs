use HorOrVer::{Horizontal, Vertical};
use macroquad::color::Color;
use macroquad::math::IVec2;
use macroquad::shapes::draw_rectangle;
use crate::application::direction::{Direction, HorOrVer};
use crate::application::draw::grid_to_screen::GridToScreenMapper;
use crate::application::draw::port::PortDrawingDefns;
use crate::application::draw::shapes::{draw_line_pos, draw_rectangle_lines_pos};
use crate::application::grid::line;
use crate::application::grid::pos::{grid_dist, grid_size, GridPos, GridSize};
use crate::application::draw::pos::{dist, pos, Dist, Pos, ScreenUnit, Size};

#[derive(Clone, Debug)]
pub struct RectCursor{
    pos : Pos,
    size: Dist,
}

impl RectCursor{
    pub fn new(starting_pos: Pos, size: Dist) -> RectCursor{
        Self{ pos:starting_pos, size, }
    }

    pub fn remaining_size(&self) -> Dist{
        self.size
    }

    #[inline(always)]
    pub fn top_left(&self) -> Pos{
        self.pos
    }

    #[inline(always)]
    pub fn top_center(&self) -> Pos{
        pos(self.x_mid(), self.top())
    }

    #[inline(always)]
    pub fn top_right(&self) -> Pos{
        pos(self.right(), self.top())
    }


    #[inline(always)]
    pub fn mid_left(&self) -> Pos{
        pos(self.left(), self.y_mid())
    }

   #[inline(always)]
    pub fn mid_center(&self) -> Pos{
        pos(self.x_mid(), self.y_mid())
    }
  
    #[inline(always)]
    pub fn mid_right(&self) -> Pos{
        pos(self.right(), self.y_mid())
    }
 

    #[inline(always)]
    pub fn bottom_left(&self) -> Pos{
        pos(self.left(), self.bottom())
    }

   #[inline(always)]
    pub fn bottom_center(&self) -> Pos{
        pos(self.x_mid(), self.bottom())
    }
  
    #[inline(always)]
    pub fn bottom_right(&self) -> Pos{
        pos(self.right(), self.bottom())
    }
  
    pub fn moved_for_port(
        &self,
        port_dir            : Direction,
        port_drawing_info   : &PortDrawingDefns
    ) -> Self {
        self.clone_apply_return(|this|  this.move_for_port(port_dir, port_drawing_info))
    }


    #[inline(always)]
    pub const fn top(&self) -> ScreenUnit{
        self.pos.y   
    }

    #[inline(always)]
    pub const fn bottom(&self) -> ScreenUnit{
        self.pos.y  + self.size.y
    }

    #[inline(always)]
    pub const fn left(&self) -> ScreenUnit{
        self.pos.x 
    }
        
    #[inline(always)]
    pub const fn right(&self) -> ScreenUnit{
        self.pos.x  + self.size.x
    }

    #[inline(always)]
    pub const fn x_mid(&self) -> ScreenUnit{
        self.pos.x + self.size.x / 2
    }

    #[inline(always)]
    pub const fn y_mid(&self) -> ScreenUnit{
        self.pos.y + self.size.y / 2
    }

    pub fn rel_point(&self, x: f32, y: f32) -> Pos{
        self.pos + dist((x * self.size.x as f32) as i32, (y * self.size.y as f32) as i32) 
    }
    pub fn move_for_port(
        &mut self,
        port_dir            : Direction,
        port_drawing_info   : &PortDrawingDefns
    ) -> &mut Self {
        let port_len = port_drawing_info.full_len();
        match port_dir {
            Direction::Up       => {
                self.go(dist(0, port_len));
            }
            Direction::Left     => {
                self.go(dist(port_len, 0));
            }
            Direction::Down     => {
                self.change_size(dist(0, -port_len));
            }
            Direction::Right    => {
                self.change_size(dist(-port_len, 0));
            }
        }

        self
    }

    pub fn pad(&mut self, horizontal: i32, vertical: i32) -> &mut Self{
        self.go(dist(horizontal, vertical));
        self.change_size(-dist(horizontal, vertical));
        self
    }

    pub fn draw_rect(&self, color: Color){
        draw_rectangle(
            self.top_left().x as f32,
            self.top_left().y as f32,
            self.remaining_size().x as f32,
            self.remaining_size().y as f32,
            color
        );
    }
    pub fn get_line(&self, line_dir: Direction) -> (IVec2, IVec2){
        match line_dir{
            Direction::Up => (self.top_left(), self.top_right()),
            Direction::Right => (self.top_right(), self.bottom_right()),
            Direction::Down => (self.bottom_right(), self.bottom_left()),
            Direction::Left => (self.bottom_left(), self.top_left()),
        }
    }

    // pub fn draw_line_offset(&self, line: Direction, offset: ScreenUnit, color: Color, thickness: u8){
    // }
    pub fn draw_line(&self, line: Direction, color: Color, thickness: u8){
        let (start, end) = self.get_line(line);
        draw_line_pos(start, end,  thickness, color);
    }

    pub fn draw_rect_lines(&self, color: Color, thickness: f32){
        draw_rectangle_lines_pos(
            self.top_left(),
            self.remaining_size(),
            thickness,
            color 
        );
    }
    

    ///
    ///
    /// # Arguments
    ///
    /// * `split_dist`: distance of the cut from the cursor
    /// * `dir`: The direction of the cut
    ///
    /// returns: the source cursor moves the distance, to the new place, while the return is
    /// pointing at the starting place, with the reduced size
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    #[must_use]
    pub fn split(&mut self, split_dist: ScreenUnit, dir: HorOrVer) -> Self{
        let mut new = self.clone();
        self.go(match dir {
            Vertical   => pos( 0, split_dist),
            Horizontal => pos( split_dist, 0),
        });

        match dir{
            Vertical => {
                new.size.y = split_dist
            },
            Horizontal => {
                new.size.x = split_dist
            }
        }
        new
    }
    pub fn with_padding(&mut self, horizontal: i32, vertical: i32) -> Self{
        self.clone_apply_return(|this| this.pad(horizontal, vertical))
    }

    pub fn with_size(&mut self, size: Size) -> &mut Self{
        self.size = size;
        self
    }
    pub fn change_size(&mut self, size_change: Dist) -> &mut Self{
        self.size += size_change;
        self
    }

    #[inline(always)]
    pub fn with_size_changed(&self, size_change: Dist) -> Self{
        self.clone_apply_return(|this| {
            this.change_size(size_change)
        })
    }

    pub fn go(&mut self, movement: Dist) -> &mut Self{
        self.pos +=  movement;
        self.size -=  movement;
        self
    }

    #[inline(always)]
    pub fn after_going(&self, movement: Dist) -> Self{
        self.clone_apply_return(|this| this.go(movement))
    }
    #[inline(always)]
    fn clone_apply_return(&self, mut fun: impl FnMut(&mut Self) -> &mut Self) -> Self{
        let mut clone = self.clone();
        fun(&mut clone);
        clone
    }
}

impl GridToScreenMapper {
    pub fn  get_cursor_for_region(&self, top_left: GridPos, size:GridSize ) -> RectCursor{
        let screen_top_left = self.grid_to_screen_pos(top_left);
        let screen_bottom_right = self.grid_to_screen_pos((top_left + size));
        let screen_size = screen_bottom_right - screen_top_left;

        RectCursor{ pos: screen_top_left, size: screen_size }
    }

}