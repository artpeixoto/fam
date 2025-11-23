use crate::application::direction::Direction;
use macroquad::prelude::*;
use macroquad::color::Color;
use macroquad::math::Vec2;
use crate::application::draw::grid_to_screen::GridScreenTransformer;
use crate::application::draw::shapes::draw_line_pos;
use crate::application::grid::{pos::GridPos};
use crate::application::draw::pos::{Pos, ToFPosExt};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalType{
    Data,
    Activation,
    TaluSetup,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PortGridDefns {
    pub position    : GridPos,
    pub direction   : Direction,
}


pub struct PortDrawingDefns {
    pub base            : u32,
    pub arrow_height    : u32,
    pub line_len        : u32,
    pub line_width      : u8,
    pub color_defn      : Box<dyn Fn(PortColorIndex) -> Color>,

}
#[derive(Clone, PartialEq, Debug, Eq)]
pub enum PortColorIndex{
    Deactivated,
    Active(SignalType, PortSignalDirection)
}

impl PortDrawingDefns {
    pub const fn full_len(&self) -> i32 {
        (self.line_len + self.arrow_height) as i32
    }
}

#[derive(Clone, Debug, PartialEq,  Eq, Copy )]
pub enum PortSignalDirection {
    Input,
    Output,
}

#[derive(Clone, Debug, PartialEq,  Eq )]
pub struct PortDefns {
    pub active      : bool,
    pub signal_dir  : PortSignalDirection,
    pub signal_type : SignalType
}

pub fn draw_port(
    port              : &PortDefns,
    port_grid_info    : &PortGridDefns,
    port_drawing_info : &PortDrawingDefns,
    grid_to_screen    : &GridScreenTransformer,
) {
    
    let color = {
        let port_color_index = if port.active{
            PortColorIndex::Active(
                port.signal_type,
                port.signal_dir,
            )
        } else {
            PortColorIndex::Deactivated
        };

        (port_drawing_info.color_defn)(port_color_index)
    };

    fn draw_arrow_head( direction: &Direction, pos: &Pos, height: &u32, base: &u32, color: &Color){
        let vertice_point = *pos;
        let base_dir = direction.rotate_ccw();
        let base_center = vertice_point - (direction * *height);
        let base_point_0 = base_center + base_dir * (base / 2);
        let base_point_1 = base_center - base_dir * (base / 2);

        draw_triangle(
            vertice_point.as_fpos(),
            base_point_0.as_fpos(),
            base_point_1.as_fpos(),
            *color
        );
    }

    let connection_pos = grid_to_screen.grid_to_screen_pos(port_grid_info.position);

    match port.signal_dir {
        PortSignalDirection::Output => {
            let arrow_head_top_pos =
                connection_pos +
                    (   (-port_grid_info.direction)
                        * (port_drawing_info.line_len)
                    )
                    ;

            draw_arrow_head(
                &port_grid_info.direction,
                &arrow_head_top_pos,
                &port_drawing_info.arrow_height,
                &port_drawing_info.base,
                &color
            );

            let line_end = connection_pos + (-port_grid_info.direction) * port_drawing_info.line_len ;
            
            draw_line_pos(
                connection_pos,
                line_end,
                port_drawing_info.line_width,
                color,
            )

            // draw_line(connection_pos, );
        }

        PortSignalDirection::Input => {
            let inner_dir = -port_grid_info.direction ;
            let arrow_head_top_pos =
                (-port_grid_info.direction)
                * (port_drawing_info.arrow_height + port_drawing_info.line_len)
                + connection_pos;

            draw_arrow_head(
                &inner_dir,
                &arrow_head_top_pos,
                &port_drawing_info.arrow_height,
                &port_drawing_info.base,
                &color,
            );

            let line_end =
                connection_pos + (-port_grid_info.direction) * port_drawing_info.line_len;

            draw_line_pos(
                connection_pos,
                line_end,
                port_drawing_info.line_width,
                color
            );
        },
    };
}