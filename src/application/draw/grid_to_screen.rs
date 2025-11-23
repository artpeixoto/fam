use std::ops::Not;
use macroquad::color::DARKGRAY;
use macroquad::math::Rect;
use macroquad::prelude::draw_rectangle;
use macroquad::shapes::draw_line;
use crate::application::direction::Direction::{Down, Right};
use crate::application::draw::shapes::draw_line_pos;
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::grid_limits::GridLimits;
use crate::application::grid::line::GridLine;
use crate::application::grid::movement::GridMovement;
use crate::application::grid::pos::{grid_pos, grid_size, GridFPos, GridPos, GridSize, GridUnit};
use crate::application::draw::pos::{fpos, pos, size, Pos, ScreenUnit, Size, ToFPosExt};


pub struct GridScreenTransformer {
    pub top_left    : Pos,
    pub spacing     : Pos,
}

impl GridScreenTransformer {
    pub fn new(grid_size: &GridLimits, rect: Rect) -> Self{
        let (top_left, spacing)  = Self::calculate_inner_data(grid_size, &rect);
        Self{
            top_left,
            spacing,
        }
    }

    // terrible return type lmao
    fn calculate_inner_data(grid_size: &GridLimits, rect: &Rect) -> (Pos, Pos){
        let spacing = (rect.size() / grid_size.0.as_vec2()).as_ivec2();
        let top_left = pos(rect.x as i32, rect.y as i32);
        (top_left, spacing)
    }

    pub fn update_transform(&mut self, grid_size: &GridLimits, rect: Rect) {
        let (top_left, spacing)  = Self::calculate_inner_data(grid_size, &rect);
        self.top_left = top_left;
        self.spacing = spacing;
    }
    pub const fn grid_to_screen_size(&self, grid_size: GridSize) -> Size{
        size(
            grid_size.x as ScreenUnit * self.spacing.x,
            grid_size.y as ScreenUnit * self.spacing.y,
        )
    }
    pub fn screen_to_grid_size(&self, screen_size: Size) -> GridSize{
        (screen_size / self.spacing).as_i16vec2()
    }

    // pub fn screen_to_grid_fpos(&self, pos: &Pos) -> GridPos {
    //     let in_grid = (*pos - self.top_left) / self.spacing;
    //     in_grid.as_i16vec2().into()
    // }

    pub fn screen_to_nearest_grid_pos(&self, pos: Pos) -> GridPos {
        let in_grid = (pos - self.top_left) / self.spacing;
        in_grid.as_i16vec2().into()
    }

    pub fn fgrid_to_screen_pos(&self, grid_point: GridFPos) -> Pos{
        let x =  (grid_point.x *  self.spacing.x as f32) + self.top_left.x as f32;
        let y = (grid_point.y *  self.spacing.y as f32) + self.top_left.y as f32;
        pos(x as i32, y as i32)
    }
    pub fn grid_to_screen_pos(&self, grid_point: GridPos) -> Pos{
        self.top_left +
        pos(
            grid_point.x as i32  * self.spacing.x,
            grid_point.y as i32 * self.spacing.y
        )
    }
    pub fn get_line(&self, grid_line: GridLine) -> (Pos, Pos){
        let [p0, p1] = grid_line.points();
        (self.grid_to_screen_pos(p0), self.grid_to_screen_pos(p1))
    }
}

pub fn draw_path_grid(
    grid_transform  : &GridScreenTransformer,
    grid_size       : &GridLimits,
    blocked_points  : &BlockedPoints,
) {
    let draw_grid_point = |pos: &Pos|{
        let pos = pos.as_fpos();

        draw_rectangle(
            pos.x - 0.5,
            pos.y - 0.5,
            1.0,
            1.0,
            DARKGRAY.with_alpha(0.5)
        );
    };

    // let draw_grid_line = |p0: &Pos, p1: &Pos|{
    //     draw_line_pos(
    //         *p0,
    //         *p1,
    //         1,
    //         DARKGRAY.with_alpha(0.1)
    //     )
    // };

    for y in 0..grid_size.0.y as i16{
        for x in 0..grid_size.0.x as i16{
            let grid_point = grid_pos(x, y);
            let grid_point_pos = grid_transform.grid_to_screen_pos(grid_point);
            if blocked_points.point_is_available(&grid_point).not() { continue }


            draw_grid_point(&grid_point_pos);

            let GridMovement { destination_point: bottom_point,.. } = grid_point + Down;

            if blocked_points.point_is_available(&bottom_point) && grid_size.contains_point(&bottom_point){
                let other_grid_point_pos = grid_transform.grid_to_screen_pos(bottom_point);
                // draw_grid_line(&grid_point_pos, &other_grid_point_pos);
            }

            let GridMovement { destination_point: right_point,.. } = grid_point + Right;

            if blocked_points.point_is_available(&right_point) && grid_size.contains_point(&right_point){
                let other_grid_point_pos = grid_transform.grid_to_screen_pos(right_point);
                // draw_grid_line(&grid_point_pos, &other_grid_point_pos);
            }
        }
    }
}
