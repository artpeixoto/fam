use std::marker::PhantomData;
use itertools::Itertools;
use macroquad::color::{BLACK, BLUE, DARKBROWN, RED, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::draw_rectangle;
use wgpu::hal::DynCommandEncoder;
use crate::application::direction::Direction;
use crate::application::draw::component_bank::{ComponentBankDrawingDefn, ComponentBankPortDataContainer, ComponentBankPortName};
use crate::application::draw::cursor::RectCursor;
use crate::application::draw::grid_to_screen::GridScreenTransformer;
use crate::application::draw::port::{draw_port, PortDrawingDefns, PortGridDefns};
use crate::application::draw::text::{draw_text_line_normal, draw_text_line_tiny, normal_font, tiny_font};
use crate::application::draw::text::title::draw_title;
use crate::application::grid;
use crate::application::grid::cpu_register::{ CpuRegisterPortsGridData};
use crate::application::grid::pos::{grid_dist, grid_pos, grid_size, GridPos, GridSize};
use crate::application::draw::pos::{Pos, ScreenUnit, Size, dist, pos, size};
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::component::{SimpleComponentGridData, DrawableComponent};
use crate::application::grid::rect::{grid_rect, GridRect};
use crate::application::simulation::cpu_registers::{CpuRegister, CpuRegisterBank, CpuRegisterPortName, CpuRegisterPortsData, REGISTER_COUNT};

#[derive(Clone, PartialEq, Eq, Debug, Hash,)]
pub struct CpuRegisterDrawingDefn {
    pub size: Size,
}
impl Default for CpuRegisterDrawingDefn {
    fn default() -> Self {
        Self{size: size(60, 30),}
    }
}
pub type CpuRegisterBankDrawingDefns = ComponentBankDrawingDefn<CpuRegisterDrawingDefn>;
pub type CpuRegisterBankPortName    = ComponentBankPortName<CpuRegisterPortName, REGISTER_COUNT>;

impl DrawableComponent for CpuRegister{
    type DrawingState = ();
    type DrawingDefn = CpuRegisterDrawingDefn;
    type PortName = CpuRegisterPortName;
    type PortDataContainer = CpuRegisterPortsData;
    type PortGridDataContainer = CpuRegisterPortsGridData;
    type ComponentCalculatedDefns =
        SimpleComponentGridData<
            CpuRegisterPortName,
            CpuRegisterPortsData,
            CpuRegisterPortsGridData
        >;

    fn calculate_defns(
        &self,
        reg_grid_info: GridPos,
        reg_drawing_info: &Self::DrawingDefn,
        port_drawing_info: &PortDrawingDefns,
        grid_screen_transform: &GridScreenTransformer
    ) -> SimpleComponentGridData<Self::PortName, Self::PortDataContainer, Self::PortGridDataContainer>
    {
        let reg_grid_rect = {
            let reg_grid_pos = reg_grid_info;
            let reg_grid_size = grid_screen_transform.screen_to_grid_size(reg_drawing_info.size);
            grid_rect(reg_grid_pos, reg_grid_size)
        };

        let reg_ports_grid_info  = {
            let port_width = port_drawing_info.base as ScreenUnit ;

            let port_grid_separation = grid_screen_transform.screen_to_grid_size(size(port_width, 0)).x + 1;
            
            let y       = reg_grid_rect.top_left.y - 1;
            let x_right = reg_grid_rect.top_left.x + reg_grid_rect.size.x - 1;
            CpuRegisterPortsGridData {
                input : PortGridDefns {
                    position    : grid_pos(x_right - port_grid_separation, y),
                    direction   : Direction::Up,
                },
                output: PortGridDefns {
                    position : grid_pos(x_right , y),
                    direction: Direction::Up,
                },
            }
        };

        let blocked_points = BlockedPoints::new_from_blocked_rect(reg_grid_rect.clone());

        SimpleComponentGridData {
            grid_rect: reg_grid_rect,
            blocked_points,
            ports_data: self.ports_info().clone(),
            ports_grid_data: reg_ports_grid_info,
            _phantom: PhantomData{},
        }
    }

    fn draw(
        &self,
        _           : &Self::DrawingState,
        grid_data   : &SimpleComponentGridData<Self::PortName, Self::PortDataContainer, Self::PortGridDataContainer>,
        drawing_data        : &Self::DrawingDefn,
        port_drawing_info   : &PortDrawingDefns,
        grid_to_screen_info : &GridScreenTransformer
    ) {

        let SimpleComponentGridData {
            grid_rect: reg_grid_rect,
            blocked_points,
            ports_grid_data:reg_ports_grid_info,
            ports_data,
            ..
        } = grid_data;

        let cursor =
            grid_to_screen_info
            .get_cursor_for_region(
                reg_grid_rect.top_left,
                reg_grid_rect.size
            );


        let full_port_height = port_drawing_info.full_len();

        { // draw ports
            for port_name in CpuRegisterPortName::iter_ports(){
                let port_info  = &ports_data[port_name];
                let port_grid_info = &reg_ports_grid_info[port_name];
                draw_port(
                    port_info,
                    port_grid_info,
                    port_drawing_info,
                    grid_to_screen_info,
                )
            }
        }

        { // draw inner square
            let mut cursor = cursor.after_advancing( dist(2, full_port_height));

            // draw base rectangle
            draw_rectangle(
                cursor.top_left().x as f32,
                cursor.top_left().y as f32,
                cursor.remaining_size().x as f32,
                cursor.remaining_size().y as f32,
                DARKBROWN
            );

            // draw INDEX
            const INDEX_FONT_SIZE: i32 = tiny_font::DIMS.full_height() as i32;

            draw_rectangle(
                cursor.top_left().x as f32,
                (cursor.top_left().y - INDEX_FONT_SIZE - 2) as f32,
                (cursor.remaining_size().x as f32 * 0.3), // ????
                (INDEX_FONT_SIZE + 2) as f32,
                BLACK.with_alpha(0.3)
            );

            draw_text_line_tiny(
                &format!("{:X}", self.address),
                pos(
                    cursor.top_left().x  + 2,
                    cursor.top_left().y  - INDEX_FONT_SIZE
                ),
                1,
                BLACK
            );

            cursor.pad(2, 2);

            { // DRAW VALUE

                let cursor = cursor.after_advancing(cursor.remaining_size().with_x(0)/2);

                draw_text_line_normal(
                    &format!("{:X}", self.value),
                    (cursor.top_left() - dist(0, normal_font::DIMS.full_height() as i32 / 2) ),
                    1,
                    WHITE
                );
            }
        }

    }
}
pub type CpuRegisterGridData =
    SimpleComponentGridData<
        CpuRegisterPortName,
        CpuRegisterPortsData,
        CpuRegisterPortsGridData
    >;

pub type CpuRegisterBankGridData = <CpuRegisterBank as
DrawableComponent>::ComponentCalculatedDefns;