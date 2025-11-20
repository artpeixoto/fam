use std::ops::RangeInclusive;
use macroquad::color::{Color, WHITE};
use macroquad::prelude::{draw_arc, draw_rectangle};
use macroquad::shapes::{draw_circle, draw_rectangle_lines};
use palette::num::Clamp;
use crate::application::draw::pos::{Dist, FDist, FPos, Pos, ToFDistExt, ToFPosExt};
use crate::tools::used_in::UsedIn;

pub fn draw_line_pos(p0: Pos, p1: Pos, width: u8, color: Color){
    let p0 = p0.as_fpos();
    let p1 = p1.as_fpos();
    macroquad::prelude::draw_line(
        p0.x ,
        p0.y ,
        p1.x ,
        p1.y ,
        width as f32,
        color
    ); 
}
pub fn draw_rectangle_pos(
    top_left: Pos,
    size    : Dist,
    color   : Color,
){
    let FPos{ x: left, y:top } = top_left.as_fpos();
    let FDist {x: width, y: height} = size.as_fdist() ;

    draw_rectangle(
        left  + 0.5,
        top  + 0.5,
        width ,
        height  ,
        color
    );
}

pub fn draw_rectangle_lines_pos(
    top_left: Pos,
    size    : Dist,
    thickness : f32,
    color   : Color,
){
    let FPos{ x: left, y:top } = top_left.as_fpos();
    let FDist {x: width, y: height} = size.as_fdist();

    draw_rectangle_lines(
        left ,
        top,
        width ,
        height ,
        thickness, 
        color
    );
}
pub fn draw_circle_pos(center: &Pos,  radius: f32, color: Color){
    let center = center.as_fpos().to_array();
    draw_circle(center[0], center[1], radius, color);
}
pub fn draw_arc_pos(center: &Pos, radius: f32, limits: (f32, f32), thickness: f32, color: Color){
    let center = center.as_fpos().to_array();
    let sides =
        ((limits.0 - limits.1).abs() * (radius.abs() / 30.0) * 20.0)
        .pipe(|val| val.clamp_max(64.0) as u8 ) ;

    draw_arc(center[0], center[1], sides, radius,  limits.0, thickness, limits.1, color);
}