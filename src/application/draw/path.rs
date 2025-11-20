use crate::application::draw::grid_to_screen::GridToScreenMapper;
use crate::application::draw::shapes::{draw_circle_pos, draw_line_pos};
use crate::application::grid::movement::GridMovement;
use crate::application::grid::path::Path;
use itertools::Itertools;
use macroquad::color::{BLUE, BROWN, Color, GOLD, GREEN, MAGENTA, PURPLE, RED};
use macroquad::prelude::draw_line;
use macroquad::shapes::draw_circle;
use std::borrow::Cow;

// pub type PathDrawingInfo = Color;
// pub struct PathDrawingInfo<'a>{
//     pub color: Cow<'a, Color>
// }

pub struct PathDrawingData {
    color: Color,
    width: u8,
}

impl Path {
    pub fn draw(&self, color: &Color, grid_transform: &GridToScreenMapper) {
        draw_path(&self, color, grid_transform);
    }
}

pub fn draw_path(path: &Path, drawing_info: &Color, grid_screen_transform: &GridToScreenMapper) {
    let draw_line = |gp0, gp1| {
        let (p0, p1) = (
            grid_screen_transform.grid_to_screen_pos(gp0),
            grid_screen_transform.grid_to_screen_pos(gp1),
        );

        draw_line_pos(p0, p1, 1, *drawing_info);
    };

    let draw_conn = |gp| {
        draw_circle_pos(
            &grid_screen_transform.grid_to_screen_pos(gp),
            2.0,
            *drawing_info,
        );
    };

    let mut last_movement = None;
    let mut last_position = path.get_starting_point();
    let mut movements = path.walk().tuple_windows().map(
        |(this_movement, next_movement): (GridMovement, GridMovement)| {
            last_movement = Some(next_movement.clone());
            last_position = next_movement.destination_point;
            (this_movement, next_movement)
        },
    );

    draw_conn(path.get_starting_point());
    while let Some((this_movement, next_movement)) = movements.next() {
        let this_move_dir = &this_movement.move_dir;
        let next_move_dir = &next_movement.move_dir;
        let (gp0, gp1) = {
            if this_move_dir.axis() != next_move_dir.axis() {
                // it is diagonal
                movements.next(); // advance once
                (
                    this_movement.starting_point,
                    next_movement.destination_point,
                )
            } else {
                // it is not diagonal
                (
                    this_movement.starting_point,
                    this_movement.destination_point,
                )
            }
        };
        draw_line(gp0, gp1);
    }
    if let Some(last_movement) = last_movement{
        draw_line(last_movement.starting_point, last_movement.destination_point);
    }
    draw_conn(last_position);
}
