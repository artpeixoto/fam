use std::marker::PhantomData;

use itertools::Itertools;
use macroquad::color::{BLACK, DARKBROWN, WHITE};
use macroquad::math::ivec2;
use macroquad::shapes::draw_rectangle;
use wgpu::naga::FastHashMap;
use crate::application::direction::{self, Direction, Axis};
use crate::application::draw::cursor::RectCursor;
use crate::application::grid::component::{ComponentCalculatedDefns, DrawableComponent, PortDataContainer, PortName, SimpleComponentGridData};
use crate::application::draw::grid_to_screen::GridScreenTransformer;
use crate::application::draw::port::{PortDefns, PortDrawingDefns, PortGridDefns, PortSignalDirection, SignalType, draw_port};
use crate::application::draw::pos::Size;
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::controller::{ControllerPortsData, ControllerPortsGridData};
use crate::application::grid::pos::{grid_pos, grid_size, GridPos};
use crate::application::grid::rect::{grid_rect, GridRect};
use crate::application::simulation::component_bank::ComponentBank;
use crate::application::simulation::controller::{Controller, ControllerExecutionState, ControllerPortName};
use crate::application::simulation::instruction::HorizontalDir;
use crate::application::simulation::talu::TaluState;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ControllerDrawingDefns{
	pub size: Size 
}

impl Default for ControllerDrawingDefns{
	fn default() -> Self {
		Self { size: Size::new(100, 100) }
	}
}

impl DrawableComponent for Controller{
	type DrawingState = ();

	type DrawingDefn = ControllerDrawingDefns;

	type PortName = ControllerPortName;

	type PortDataContainer = ControllerPortsData
		;

	type PortGridDataContainer = ControllerPortsGridData
		;

	type ComponentCalculatedDefns  = SimpleComponentGridData<
		ControllerPortName,
		ControllerPortsData,
		ControllerPortsGridData
	>;

	fn calculate_defns(
		&self,
		top_left         : GridPos,
		drawing_info     : &Self::DrawingDefn,
		port_drawing_info: &PortDrawingDefns,
		grid_to_screen   : &GridScreenTransformer,
	) -> Self::ComponentCalculatedDefns {
        let grid_rect = {
            let reg_grid_size = grid_to_screen.screen_to_grid_size(drawing_info.size);
            grid_rect(top_left, reg_grid_size)
        };
		let ports_data = ControllerPortsData::from_iter([
			(	ControllerPortName::ProgramCounterReader,
				PortDefns{
					active	   	: true,
					signal_dir	: PortSignalDirection::Input,
					signal_type	: crate::application::draw::port::SignalType::Data,	
				}
			)	,
			(	ControllerPortName::ProgramCounterWriter,
				PortDefns{
					active	   	: true,
					signal_dir	: PortSignalDirection::Output,
					signal_type	: SignalType::Data,	
				}
			),
			(	ControllerPortName::RegisterReader,
				PortDefns{
					active	   	: true,
					signal_dir	: PortSignalDirection::Input,
					signal_type	: SignalType::Data,	
				}
			),
			(	ControllerPortName::RegisterWriter,
				PortDefns{
					active	   	: true,
					signal_dir	: PortSignalDirection::Output,
					signal_type	: SignalType::Data,
				}
			),
			(	ControllerPortName::TaluConfigWriter,
				PortDefns{
					active	   	: true,
					signal_dir	: PortSignalDirection::Output,
					signal_type	: SignalType::TaluSetup,	
				}
			),
			(	ControllerPortName::MainMemoryReader,
				PortDefns{
					active	   	: true,
					signal_dir	: PortSignalDirection::Input,
					signal_type	: SignalType::Data
				}
			),
		]);
        let ports_grid_data  = {
            // let y       = grid_rect.+ grid_rect.size - 1;
            // let x_right = grid_rect.top_left.x + grid_rect.size.x - 1;
			let x_right = grid_rect.pos(Direction::Right) + 1;
			let x_left 	 = grid_rect.pos(Direction::Left) - 1 ;
			let y_delta = 4;
			let y_bottom = grid_rect.pos(Direction::Down) - 1;

            ControllerPortsGridData::from_iter([
				(   ControllerPortName::ProgramCounterReader, 
					PortGridDefns {
						position  : grid_pos(x_right, y_bottom),
						direction : Direction::Right,
					}
				),
				(   ControllerPortName::ProgramCounterWriter, 
					PortGridDefns {
						position: grid_pos(x_right, y_bottom - y_delta),
						direction : Direction::Right,
					}
				),
				(   ControllerPortName::RegisterReader, 
					PortGridDefns {
						position: grid_pos(x_right, y_bottom - 2*y_delta),
						direction : Direction::Right,
					}
				),
				(   ControllerPortName::RegisterWriter, 
					PortGridDefns {
						position: grid_pos(x_right, y_bottom - 3*y_delta),
						direction : Direction::Right,
					}
				),
				(   ControllerPortName::TaluConfigWriter, 
					PortGridDefns {
						position  : grid_pos(x_right, y_bottom - 4*y_delta),
						direction : Direction::Right,
					}
				),				
				(   ControllerPortName::MainMemoryReader, 
					PortGridDefns {
						position: grid_pos(x_left, y_bottom),
						direction: Direction::Left,
					}
				),
			])
       	};

        let blocked_points = 
			BlockedPoints::new_from_blocked_rect(grid_rect.clone());

        SimpleComponentGridData {
            grid_rect,
            blocked_points,
            ports_data,
            ports_grid_data,
            _phantom		: PhantomData,
        }
	}

	fn draw(
		&self,
		drawing_state    : &Self::DrawingState,
		grid_defns       : &Self::ComponentCalculatedDefns,
		drawing_defns    : &Self::DrawingDefn,
		port_drawing_info: &PortDrawingDefns,
		grid_to_screen   : &GridScreenTransformer,
	) {
		let SimpleComponentGridData{
			grid_rect,
			ports_data,
			ports_grid_data,
			..
		} = grid_defns;

		let mut cursor = 
			grid_to_screen
			.get_cursor_for_region(grid_rect.top_left, grid_rect.size)
			.moved_for_port(Direction::Left, port_drawing_info)
			.moved_for_port(Direction::Right, port_drawing_info)
			;

		cursor.draw_rect_lines(BLACK, 1.);
		cursor.pad(1, 1).draw_rect(WHITE);

		cursor.split(24, direction::Axis::Horizontal)
		 	.after_padding(2, 2)
			.draw_text_line("CONTROLLER", super::text::TextStyle::Normal, 1, BLACK);
		
        { // draw ports
            for port_name in ControllerPortName::all_port_names(){
                let port_info  = &ports_data[&port_name];
                let port_grid_info = &ports_grid_data[&port_name];
                draw_port(
                    port_info,
                    port_grid_info,
                    port_drawing_info,
                    grid_to_screen,
                )
            }
        }


		
	}
}
// pub 
// #[derive(Clone, PartialEq, Eq, Debug, Hash,)]
// pub struct CpuRegisterDrawingDefn {
//     pub size: Size,
// }
// impl Default for CpuRegisterDrawingDefn {
//     fn default() -> Self {

//         Self{size: size(50, 30),}

//     }
// }
// pub type CpuRegisterBankDrawingDefns = ComponentBankDrawingDefn<CpuRegisterDrawingDefn>;
// pub type CpuRegisterBankPortName    = ComponentBankPortName<CpuRegisterPortName, REGISTER_COUNT>;

// impl DrawableComponent for CpuRegister{
//     type DrawingState = ();
//     type DrawingDefn = CpuRegisterDrawingDefn;
//     type PortName = CpuRegisterPortName;
//     type PortDataContainer = CpuRegisterPortsData;
//     type PortGridDataContainer = CpuRegisterPortsGridData;
//     type ComponentCalculatedDefns =
//         SimpleComponentGridDefns<
//             CpuRegisterPortName,
//             CpuRegisterPortsData,
//             CpuRegisterPortsGridData
//         >;

//     fn calculate_defns(
//         &self,
//         reg_grid_info: GridPos,
//         reg_drawing_info: &Self::DrawingDefn,
//         port_drawing_info: &PortDrawingDefns,
//         grid_to_screen_info: &GridToScreenMapper
//     ) -> SimpleComponentGridDefns<Self::PortName, Self::PortDataContainer, Self::PortGridDataContainer>
//     {
//         let reg_grid_rect = {
//             let reg_grid_pos = reg_grid_info;
//             let reg_grid_size = grid_to_screen_info.screen_to_grid_size(reg_drawing_info.size);
//             grid_rect(reg_grid_pos, reg_grid_size)
//         };

//         let reg_ports_grid_info  = {
//             let y       = reg_grid_rect.top_left.y - 1;
//             let x_right = reg_grid_rect.top_left.x + reg_grid_rect.size.x - 1;
//             CpuRegisterPortsGridData {
//                 input : PortGridDefns {
//                     position    : grid_pos(x_right - 4, y),
//                     direction   : Direction::Up,
//                 },
//                 output: PortGridDefns {
//                     position : grid_pos(x_right , y),
//                     direction: Direction::Up,
//                 },
//             }
//         };

//         let blocked_points = BlockedPoints::new_from_blocked_inner_rect(reg_grid_rect.clone());

//         SimpleComponentGridDefns {
//             grid_rect: reg_grid_rect,
//             blocked_points,
//             ports_data: self.ports_info().clone(),
//             ports_grid_data: reg_ports_grid_info,
//             _phantom: PhantomData{},
//         }
//     }

//     fn draw(
//         &self,
//         _           : &Self::DrawingState,
//         grid_data   : &SimpleComponentGridDefns<Self::PortName, Self::PortDataContainer, Self::PortGridDataContainer>,
//         drawing_data        : &Self::DrawingDefn,
//         port_drawing_info   : &PortDrawingDefns,
//         grid_to_screen_info : &GridToScreenMapper
//     ) {
//         let SimpleComponentGridDefns {
//             grid_rect: reg_grid_rect,
//             blocked_points,
//             ports_grid_data:reg_ports_grid_info,
//             ports_data,
//             ..
//         } = grid_data;

//         let cursor =
//             grid_to_screen_info
//             .get_cursor_for_region(
//                 reg_grid_rect.top_left,
//                 reg_grid_rect.size
//             );


//         let full_port_height = port_drawing_info.full_len();

//         { // draw ports
//             for port_name in CpuRegisterPortName::iter_ports(){
//                 let port_info  = &ports_data[port_name];
//                 let port_grid_info = &reg_ports_grid_info[port_name];
//                 draw_port(
//                     port_info,
//                     port_grid_info,
//                     port_drawing_info,
//                     grid_to_screen_info,
//                 )
//             }
//         }

//         { // draw inner square
//             let mut cursor = cursor.after_going( dist(2, full_port_height));

//             // draw base rectangle
//             draw_rectangle(
//                 cursor.top_left().x as f32,
//                 cursor.top_left().y as f32,
//                 cursor.remaining_size().x as f32,
//                 cursor.remaining_size().y as f32,
//                 DARKBROWN
//             );

//             // draw INDEX
//             const INDEX_FONT_SIZE: i32 = tiny_font::DIMS.full_height() as i32;

//             draw_rectangle(
//                 cursor.top_left().x as f32,
//                 (cursor.top_left().y - INDEX_FONT_SIZE - 2) as f32,
//                 (cursor.remaining_size().x as f32 * 0.3), // ????
//                 (INDEX_FONT_SIZE + 2) as f32,
//                 BLACK.with_alpha(0.3)
//             );

//             draw_text_line_tiny(
//                 &format!("{:X}", self.address),
//                 pos(
//                     cursor.top_left().x  + 2,
//                     cursor.top_left().y  - INDEX_FONT_SIZE
//                 ),
//                 1,
//                 BLACK
//             );

//             cursor.pad(2, 2);

//             { // DRAW VALUE

//                 let cursor = cursor.after_going(cursor.remaining_size().with_x(0)/2);

//                 draw_text_line_normal(
//                     &format!("{:X}", self.value),
//                     (cursor.top_left() - dist(0, normal_font::DIMS.full_height() as i32 / 2) ),
//                     1,
//                     WHITE
//                 );
//             }
//         }

//     }
// }
// pub type CpuRegisterGridDefns =
//     SimpleComponentGridDefns<
//         CpuRegisterPortName,
//         CpuRegisterPortsData,
//         CpuRegisterPortsGridData
//     >;

// pub type CpuRegisterBankGridDefns = <CpuRegisterBank as
// DrawableComponent>::ComponentCalculatedDefns;