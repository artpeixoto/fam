use std::collections::HashMap;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use fam::application::draw::talu::TaluBankDrawingDefns;
use fam::application::draw::cpu::CpuDrawingDefns;
use fam::application::draw::cpu_register::CpuRegisterBankDrawingDefns;
use fam::application::draw::grid_to_screen::{draw_path_grid, GridToScreenMapper};
use fam::application::draw::instruction_memory;
use fam::application::draw::instruction_memory::{InstructionMemoryCurrentPosition, InstructionMemoryDrawingDefns};
use fam::application::draw::path::draw_path;
use fam::application::draw::port::{PortColorIndex, PortDrawingDefns, PortSignalDirection, SignalType};
use fam::application::draw::pos::{dist, pos, size, Size};
use fam::application::draw::shapes::draw_rectangle_pos;
use fam::application::grid::blocked_point::BlockedPoints;
use fam::application::grid::component::{ComponentGridData, DrawableComponent};
use fam::application::grid::cpu::{CpuGridDefns};
use fam::application::grid::grid_limits::GridLimits;
use fam::application::grid::path::Path;
use fam::application::grid::pos::grid_pos;
use fam::application::simulation::talu::{TaluBank, TaluOperation, TALU_COUNT};
use fam::application::simulation::controller::Controller;
use fam::application::simulation::cpu_registers::{CpuRegisterBank, REGISTER_COUNT};
use fam::application::simulation::simulation::Cpu;
use fam::application::simulation::instruction::Instruction;
use fam::application::simulation::instruction_reader::InstructionMemory;
use fam::application::simulation::main_memory::MainMemory;
use fam::word::Word;

// arch name: STruCC
//  Spatially distributed and Structured Computation and Control

fn main(){
    macroquad::Window::new("FAM Simulator", amain()); 
}
async fn amain() {
    let program = vec![
        Instruction::SetTaluConfig {
            talu_addr    : 2,
            talu_config  : TaluOperation::Add {
                activation_input: 2 ,
                data_input_1: 0,
                data_input_0: 1,
                data_output_0: 3,
                flags_output: None,
                activation_output: None,
            }
        };
        128
    ];

    let data = vec![];

    let screen_size = size(1600, 900);

    let grid_limits = GridLimits::new(u16vec2(screen_size.x as u16 / 2 , screen_size.y as u16 / 2));
        
    let grid_to_screen_mapper = GridToScreenMapper::new(
        &grid_limits,
        Rect::new(0_f32, 0_f32, screen_size.x as f32, screen_size.y as f32),
    );

    let paths = Vec::new();

    let cpu = build_full_cpu(program, data, screen_size, &grid_to_screen_mapper);
    
    let app = Application{
        cpu,
        paths,
        screen_size,
        grid_to_screen_mapper,
        grid_limits,
    };



    // let instruction_memory_calculate_defns=
    //     cpu.instruction_memory.calculate_defns (
    //
    //
    //     );

    loop{
        clear_background(WHITE);
        app.draw();

        next_frame().await;
    }
}
pub struct FullCpu{
    pub sim             : Cpu,
    pub grid_defns      : CpuGridDefns,
    pub drawing_defns   : CpuDrawingDefns
}
pub struct Application{
    pub cpu: FullCpu,
    pub paths                   : Vec<Path>,
    pub screen_size             : Size,
    pub grid_to_screen_mapper   : GridToScreenMapper,
    pub grid_limits             : GridLimits,

}
impl Application{
    pub fn draw(&self){
        draw_full_cpu(
            &self.cpu,
            &self.grid_to_screen_mapper
        );
        draw_paths(
            &self.paths, 
            &self.grid_to_screen_mapper
        );

    }
}

fn draw_fps(){
    draw_rectangle_pos(pos(0,0), dist(200, 30), BLACK);
    macroquad::prelude::draw_fps();
}
fn draw_paths(
    // cpu: &FullCpu,
    paths: &Vec<Path>,
    // grid_limits: &GridLimits,
    grid_to_screen_mapper: &GridToScreenMapper,
) {
    pub const PATH_COLORS: [Color;7] = [BLUE, RED, GREEN, BROWN, PURPLE, GOLD, MAGENTA];
    //
    // draw_path_grid(
    //     grid_to_screen_mapper,
    //     grid_limits,
    //     &cpu.grid_defns.blocked_points
    // );

    for (ix, path) in paths.iter().enumerate(){
        draw_path(
            path,
            &PATH_COLORS[ix%PATH_COLORS.len()],
            grid_to_screen_mapper,
        )
    }
}

fn draw_full_cpu(
    cpu                     : &FullCpu,
    // grid_limits             : &GridLimits,
    grid_to_screen_mapper   : &GridToScreenMapper,
) {

    // draw_path_grid(
    //     &grid_to_screen_mapper,
    //     &grid_limits,
    //     &BlockedPoints::new()
    // );

    cpu.sim.instruction_memory.draw(
        &InstructionMemoryCurrentPosition(0),
        &cpu.grid_defns.instruction_memory,
        &cpu.drawing_defns.instruction_memory,
        &cpu.drawing_defns.port,
        &grid_to_screen_mapper
    );

    let registers_drawing_state = Box::new([();REGISTER_COUNT]);

    cpu.sim.register_bank.draw(
        &registers_drawing_state,
        &cpu.grid_defns.register_bank,
        &cpu.drawing_defns.register_bank,
        &cpu.drawing_defns.port,
        &grid_to_screen_mapper
    );
    let talu_bank_drawing_state = Box::new([(); TALU_COUNT]);
    cpu.sim.talu_bank.draw(
        &talu_bank_drawing_state,
        &cpu.grid_defns.talu_bank,
        &cpu.drawing_defns.talu_bank,
        &cpu.drawing_defns.port,
        &grid_to_screen_mapper
    );
}

fn build_full_cpu(
    program                 : Vec<Instruction>,
    data                    : Vec<Word>,
    screen_size             : Size,
    grid_to_screen_mapper   : &GridToScreenMapper,
) -> FullCpu {

    let mut main_memory = MainMemory::new(data);

    let registers = CpuRegisterBank::new();

    let instruction_memory = InstructionMemory::new(program.clone());

    let talus = TaluBank::new(&mut  main_memory);

    let controller = Controller::new(
        &instruction_memory,
    );

    let cpu = {
        Cpu {
            main_memory,
            talu_bank: talus,
            register_bank: registers,
            controller,
            instruction_memory,
            connections: Default::default()
        }
    };


    let port_drawing_data = PortDrawingDefns {
        base        : 6,
        line_len    : 4,
        arrow_height: 8,
        line_width  : 1,
        color_defn: Box::new(|color| -> Color{
            match color{
                PortColorIndex::Deactivated => {DARKGRAY},
                PortColorIndex::Active(SignalType::Data, PortSignalDirection::Input) => {BLUE}
                PortColorIndex::Active(SignalType::Data, PortSignalDirection::Output) => {RED}
                PortColorIndex::Active(SignalType::Activation, PortSignalDirection::Input) => {GREEN}
                PortColorIndex::Active(SignalType::Activation, PortSignalDirection::Output) => {YELLOW}
                PortColorIndex::Active(SignalType::TaluSetup, PortSignalDirection::Input) => {VIOLET}
                PortColorIndex::Active(SignalType::TaluSetup, PortSignalDirection::Output) => {BROWN}
            }
        }),
    };

    let register_bank_drawing_data = CpuRegisterBankDrawingDefns {
        size: size(screen_size.x / 2, screen_size.y / 2),
        row_count: 8,
        inner_drawing_defns: Default::default(),
    };


    let talu_bank_drawing_data =
        TaluBankDrawingDefns {
            size: screen_size / 2,
            row_count: 4,
            inner_drawing_defns: Default::default(),
        };

    let instruction_memory_top_left =
        grid_pos(1, 1)  ;

    let register_bank_top_left =
        grid_to_screen_mapper
            .screen_to_nearest_grid_pos( pos(screen_size.x / 3,  screen_size.y / 2)  );

    let talu_bank_top_left =
        grid_to_screen_mapper
            .screen_to_nearest_grid_pos( pos(screen_size.x / 3, 0)  );

    let talu_bank_grid_defns = cpu.talu_bank.calculate_defns(
        talu_bank_top_left,
        &talu_bank_drawing_data,
        &port_drawing_data,
        &grid_to_screen_mapper
    );
    let register_bank_calculated_defns = cpu.register_bank.calculate_defns(
        register_bank_top_left,
        &register_bank_drawing_data,
        &port_drawing_data,
        &grid_to_screen_mapper
    );

    let instruction_mem_drawing_defns = InstructionMemoryDrawingDefns {
        size: pos(screen_size.x/4, screen_size.y)
    };

    let instruction_mem_calculated_defns =
        cpu
        .instruction_memory
        .calculate_defns(
            instruction_memory_top_left,
            &instruction_mem_drawing_defns,
            &port_drawing_data,
            &grid_to_screen_mapper
        );

    let cpu_drawing_defns = CpuDrawingDefns{
        port: port_drawing_data,
        register_bank: register_bank_drawing_data,
        talu_bank: talu_bank_drawing_data,
        instruction_memory: instruction_mem_drawing_defns,
    };

    let all_blocked_points = {
        let mut blocked = talu_bank_grid_defns.blocked_points.clone();
        blocked.add_from(register_bank_calculated_defns.blocked_points());
        blocked.add_from(instruction_mem_calculated_defns.blocked_points());
        blocked
    };

    let cpu_grid_defns = CpuGridDefns{
        talu_bank           : talu_bank_grid_defns,
        register_bank       : register_bank_calculated_defns,
        instruction_memory  : instruction_mem_calculated_defns,
        blocked_points      : all_blocked_points,
        controller          : todo!(),
    };

    FullCpu {
        sim: cpu,
        grid_defns: cpu_grid_defns,
        drawing_defns: cpu_drawing_defns,
    }
}

pub fn calculate_paths(
    cpu             : &Cpu,
    cpu_grid_defns  : &CpuGridDefns,
    grid_limits     : &GridLimits
) -> Vec<Path>{
    todo!();

    // cpu.execute()
    // for a in cpu.
    //     cpu.        
    // } 
}