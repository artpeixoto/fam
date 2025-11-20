use crate::application::simulation::talu::{CmpOp, TALU_COUNT, TaluAddress, TaluState};
use crate::application::direction::Direction;
use crate::application::direction::Axis::Vertical;
use crate::application::draw::component_bank::{ComponentBankDrawingDefn, ComponentBankGridData};
use crate::application::draw::grid_to_screen::GridToScreenMapper;
use crate::application::draw::port::{draw_port, PortDrawingDefns, PortGridDefns};
use crate::application::draw::pos::{dist, pos, size, ScreenUnit, Size};
use crate::application::draw::shapes::{draw_circle_pos, draw_rectangle_pos};
use crate::application::draw::text::{draw_text_line_tiny, draw_title};
use crate::application::grid::talu::{TaluGridDefns, TaluPortsGridDefns};
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::component::{DrawableComponent, PortDataContainer, PortName, SimpleComponentGridDefns};
use crate::application::grid::pos::{grid_pos, GridPos};
use crate::application::grid::rect::grid_rect;
use crate::application::simulation::talu::{TaluCore, TaluOperation, TaluPortName, TaluPortsDefns};
use crate::tools::used_in::UsedIn;
use itertools::Itertools;
use macroquad::color::{BLACK, GRAY, GREEN, LIGHTGRAY, WHITE, YELLOW};
use std::marker::PhantomData;
use std::ops::Index;


#[derive(Clone, PartialEq, Eq)]
pub struct TaluDrawingDefns {
    pub full_size: Size,
    pub header_height: ScreenUnit,
}


impl Default for TaluDrawingDefns {
    fn default() -> Self {
        Self {
            full_size: size(60, 60),
            header_height: 12
        }
    }
}

impl DrawableComponent for TaluCore {
    type DrawingState = ();
    type DrawingDefn = TaluDrawingDefns;
    type PortName = TaluPortName;
    type PortDataContainer = TaluPortsDefns;
    type PortGridDataContainer = TaluPortsGridDefns;
    type ComponentCalculatedDefns =
        SimpleComponentGridDefns<
            Self::PortName,
            Self::PortDataContainer,
            Self::PortGridDataContainer
        >;

    fn calculate_defns(
        self             : &Self,
        grid_position    : GridPos,
        drawing_data     : &Self::DrawingDefn,
        port_drawing_data: &PortDrawingDefns,
        grid_to_screen   : &GridToScreenMapper,
    ) -> Self::ComponentCalculatedDefns {

        let talu_ports_info = self.get_ports_info();
        let pos = grid_position;
        let talu_grid_size = grid_to_screen.screen_to_grid_size(
            drawing_data.full_size,
        );

        let talu_ports_grid_info =
            {   // draw ports
                let ports_start =
                    (pos
                        + grid_to_screen.screen_to_grid_size(
                        size(0, drawing_data.header_height)
                    ));

                let ports_available_grid_size =
                    grid_to_screen.screen_to_grid_size(
                        drawing_data.full_size
                            - size(0, drawing_data.header_height)
                    );

                let top_y = ports_start.y;
                let y_count = 5;
                let delta_y = ports_available_grid_size.y / y_count;

                let ys =
                    (0..y_count as i16)
                        .into_iter()
                        .map(|i| top_y + (i * delta_y))
                        .collect_vec();

                let left_x = ports_start.x;
                let right_x = ports_start.x + ports_available_grid_size.x;

                let talu_ports_grid_info = TaluPortsGridDefns {
                    // setup_in        : PortGridInfo {
                    //     position: grid_pos(left_x, ys[0] ),
                    //     direction: Direction::Left,
                    // },

                    data_in_0: PortGridDefns {
                        position: grid_pos(left_x, ys[1]),
                        direction: Direction::Left,
                    },

                    data_in_1: PortGridDefns {
                        position: grid_pos(left_x, ys[2]),
                        direction: Direction::Left,
                    },

                    activation_in: PortGridDefns {
                        position: grid_pos(left_x, ys[3]),
                        direction: Direction::Left,
                    },

                    data_out_0: PortGridDefns {
                        position: grid_pos(right_x, ys[1]),
                        direction: Direction::Right,
                    },
                    data_out_1: PortGridDefns {
                        position: grid_pos(right_x, ys[2]),
                        direction: Direction::Right,
                    },
                    activation_out: PortGridDefns {
                        position: grid_pos(right_x, ys[3]),
                        direction: Direction::Right,
                    },
                    setup_in: PortGridDefns {
                        position: grid_pos(right_x, ys[4]),
                        direction: Direction::Right,
                    },
                };

                for port_name in TaluPortName::all_port_names() {
                    let port_info = talu_ports_info.get_for_port(&port_name);
                    let port_grid_info = talu_ports_grid_info.get_for_port(&port_name);

                    draw_port(
                        port_info,
                        port_grid_info,
                        port_drawing_data,
                        grid_to_screen,
                    );
                }

                talu_ports_grid_info
            };
        let talu_grid_rect = grid_rect(pos, talu_grid_size);
        let blocked = BlockedPoints::new_from_blocked_inner_rect(talu_grid_rect.clone());

        SimpleComponentGridDefns {
            grid_rect: talu_grid_rect,
            blocked_points: blocked,
            ports_data: talu_ports_info,
            ports_grid_data: talu_ports_grid_info,
            _phantom: PhantomData {},
        }
    }

    fn draw(
        &self,
        drawing_state       : &Self::DrawingState,
        calculated_defns    : &SimpleComponentGridDefns<
            Self::PortName, 
            Self::PortDataContainer, 
            Self::PortGridDataContainer
        >,
        talu_drawing_data    : &Self::DrawingDefn,
        port_drawing_data   : &PortDrawingDefns,
        grid_to_screen      : &GridToScreenMapper,
    ) {
        let mut cursor =
            grid_to_screen
            .get_cursor_for_region(
                calculated_defns.grid_rect.top_left,
                calculated_defns.grid_rect.size,
            )
            .moved_for_port(
                Direction::Left,
                port_drawing_data,
            )
            .moved_for_port(
                Direction::Right,
                port_drawing_data,
            )
            .with_padding(0, 2)
            ;


        { // boundary frame
            draw_rectangle_pos(
                cursor.top_left(),
                cursor.remaining_size(),
                LIGHTGRAY,
            );
        }

        { // title
            let mut cursor = cursor.split(talu_drawing_data.header_height, Vertical);

            draw_rectangle_pos(
                cursor.top_left(),
                cursor.remaining_size(),
                BLACK,
            );

            cursor.pad(1, 1);

            draw_text_line_tiny(
                &format!("TALU {:2x}", self.addr),
                (cursor.top_left() + pos(2, 4)),
                1,
                WHITE,
            );
        }

        let talu_op = self.operation;

        { // status text
            cursor.go(dist(2, 2));
            let radius = 4;
            let circle_color = match self.state{
                TaluState::JustProcessed => GREEN,
                TaluState::Closing => YELLOW,
                TaluState::Done => GRAY,
            };
            let center =cursor.top_left() +  Direction::Right * radius + Direction::Down * radius ; 
            draw_circle_pos(
                &center , 
                radius as f32, circle_color
            );
        }

        { // draw operation text
            let cursor =
                cursor
                .after_going(cursor.remaining_size() / 2)
                .after_going(dist(-15, -5));

            let operation_text = {
                match talu_op {
                    TaluOperation::Mov{..} => {"MOV"},
                    TaluOperation::NoOp => { "NOP" }
                    TaluOperation::Cmp { op, .. } => match op {
                        CmpOp::LessThan => "LT",
                        CmpOp::LessThanOrEq => "LE",
                        CmpOp::GreaterThan => "GT",
                        CmpOp::GreaterThanOrEq => "GE",
                        CmpOp::Eq => "EQ",
                        CmpOp::NotEq => "NEQ",
                    },
                    // TaluOperation::Mov { .. } => {"MOV"}
                    TaluOperation::Latch { .. } => { "LAT" }
                    TaluOperation::Not { .. } => { "NOT" }
                    TaluOperation::And { .. } => { "AND" }
                    TaluOperation::Or { .. } => { "OR" }
                    TaluOperation::Xor { .. } => { "XOR" }
                    TaluOperation::ShiftLeft { .. } => { "SHL"}
                    TaluOperation::ShiftRight { .. } => { "SHR" }
                    TaluOperation::SelectPart { .. } => { "SEL" }
                    TaluOperation::Add { .. } => { "ADD" }
                    TaluOperation::Sub { .. } => { "SUB" }
                    TaluOperation::Mul { .. } => { "MUL" }
                    TaluOperation::Div { .. } => { "DIV" }
                    TaluOperation::Rem { .. } => { "REM" }
                    TaluOperation::Neg { .. } => { "NEG" }
                    TaluOperation::ReadFromMem { .. } => { "READ" }
                    TaluOperation::WriteToMem { .. } => { "WRIT" }
                }
            };

            draw_title(operation_text, cursor.top_left(), 1, BLACK);

        }

        { // draw ports
            for port_name in TaluPortName::all_port_names() {
                let port_data = calculated_defns.ports_data.get_for_port(&port_name);
                let port_grid_data  = calculated_defns.ports_grid_data.get_for_port(&port_name);

                draw_port(
                   port_data,
                   port_grid_data,
                   port_drawing_data,
                   grid_to_screen
                ) ;
            }
        }
    }
}

pub type TaluBankDrawingDefns = ComponentBankDrawingDefn<TaluDrawingDefns>;
pub type TaluBankGridDefns = ComponentBankGridData<TaluCore, { TALU_COUNT }>;

