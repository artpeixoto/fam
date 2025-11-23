use crate::application::direction::Axis;
use crate::application::draw::cursor::RectCursor;
use crate::application::draw::grid_to_screen::GridScreenTransformer;
use crate::application::draw::port::{PortDefns, PortDrawingDefns, PortGridDefns};
use crate::application::draw::pos::{Dist, Pos, ScreenUnit, Size, *};
use crate::application::draw::shapes::{draw_line_pos, draw_rectangle_pos};
use crate::application::draw::text::{
    TextStyle, draw_multiline_text_pos, draw_text_pos, draw_title, measure_multiline_text,
    normal_font,
};
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::component::{DrawableComponent, PortName, SimpleComponentGridData};
use crate::application::grid::pos::GridPos;
use crate::application::grid::rect::grid_rect;
use crate::application::simulation::instruction_reader::InstructionMemory;
use macroquad::color::{BLACK, DARKGRAY, GRAY, WHITE};
use std::collections::HashMap;
use std::marker::PhantomData;
use wgpu::naga::FastHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Never {}

pub type InstructionMemoryPortName = Never;
impl PortName for Never {
    fn all_port_names() -> Vec<Self> {
        vec![]
    }

    fn small_name(&self) -> &str {
        panic!("genitals obliterated")
    }
}
pub type InstructionMemoryCurrentPosition = usize;

impl DrawableComponent for InstructionMemory {
    type DrawingState = InstructionMemoryCurrentPosition;
    type DrawingDefn = InstructionMemoryDrawingDefns;
    type PortName = InstructionMemoryPortName;
    type PortDataContainer = FastHashMap<Never, PortDefns>;
    type PortGridDataContainer = FastHashMap<Never, PortGridDefns>;

    type ComponentCalculatedDefns = SimpleComponentGridData<
        Never,
        FastHashMap<Never, PortDefns>,
        FastHashMap<Never, PortGridDefns>,
    >;

    fn calculate_defns(
        &self,
        grid_position: GridPos,
        drawing_info: &Self::DrawingDefn,
        _port_drawing_info: &PortDrawingDefns,
        grid_to_screen: &GridScreenTransformer,
    ) -> Self::ComponentCalculatedDefns {
        let grid_size = grid_to_screen.screen_to_grid_size(drawing_info.size);
        let grid_rect = grid_rect(grid_position, grid_size);
        let blocked_points = BlockedPoints::new_from_blocked_rect(grid_rect.clone());

        SimpleComponentGridData {
            grid_rect,
            blocked_points,
            ports_data: FastHashMap::default(),
            ports_grid_data: FastHashMap::default(),
            _phantom: PhantomData {},
        }
    }

    fn draw(
        &self,
        drawing_state       : &Self::DrawingState,
        grid_data           : &Self::ComponentCalculatedDefns,
        drawing_data        : &Self::DrawingDefn,
        port_drawing_defns  : &PortDrawingDefns,
        grid_to_screen      : &GridScreenTransformer,
    ) {
        let top_left = grid_to_screen.grid_to_screen_pos(grid_data.grid_rect.top_left);
        let size = grid_to_screen.grid_to_screen_size(grid_data.grid_rect.size);

        let mut cursor = RectCursor::new(top_left, size);

        let title_dims = draw_title("Instruction Memory", top_left, 2, BLACK);

        cursor.advance(Dist::new(0, title_dims.height as ScreenUnit * 2 + 2));
        let initial_cursor = cursor.clone();
        {
            let mut current_cell_ix = *drawing_state;
            loop {
                let Some(current_instruction) = self.0.get(current_cell_ix).cloned() else {
                    break;
                };

                let instruction_value_text = format!("{:#?}", current_instruction);
                let font_dims = normal_font::DIMS;

                // draw top line
                draw_line_pos(
                    cursor.top_left(),
                    pos(cursor.right(), cursor.top()),
                    1,
                    BLACK,
                );

                {
                    // draw address
                    let mut cursor = cursor.split(font_dims.full_height() + 4, Axis::Vertical);

                    draw_rectangle_pos(cursor.top_left(), cursor.remaining_size(), DARKGRAY);

                    cursor.pad(0, 2);

                    draw_text_pos(
                        &format!("ADDR {:}", current_cell_ix),
                        cursor.top_left() + dist(2, 2),
                        TextStyle::Normal,
                        1,
                        WHITE,
                    );
                }

                // prepare to draw instruction
                let text_dims =
                    measure_multiline_text(&instruction_value_text, TextStyle::Normal, 1);

                // verify we have enough space to do so
                if text_dims.height as i32 + cursor.top() > cursor.bottom() {
                    break;
                }

                {
                    // draw instruction
                    draw_multiline_text_pos(
                        &instruction_value_text,
                        cursor.top_left() + dist(2, 2),
                        TextStyle::Normal,
                        1,
                        BLACK,
                    );
                }
                cursor.advance(dist(0, text_dims.height as i32 + 4));
                current_cell_ix += 1;
            }
        }

        draw_line_pos(
            initial_cursor.top_left(),
            initial_cursor.bottom_left(),
            1,
            BLACK,
        );

        draw_line_pos(
            initial_cursor.top_right(),
            initial_cursor.bottom_right(),
            1,
            BLACK,
        );
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionMemoryDrawingDefns {
    pub size: Size,
    pub current_pos: InstructionMemoryCurrentPosition,
}

pub type InstructionMemoryGridDefns 
    =  SimpleComponentGridData<
        Never,
        FastHashMap<Never, PortDefns>,
        FastHashMap<Never, PortGridDefns>,
    >;
