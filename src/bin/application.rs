use fam::application::direction::Axis::{Horizontal, Vertical};
use fam::application::draw::controller::ControllerDrawingDefns;
use fam::application::draw::cpu::CpuDrawingData;
use fam::application::draw::cpu_register::CpuRegisterBankDrawingDefns;
use fam::application::draw::cursor::RectCursor;
use fam::application::draw::grid_to_screen::{GridToScreenMapper, draw_path_grid};
use fam::application::draw::instruction_memory;
use fam::application::draw::instruction_memory::{
    InstructionMemoryCurrentPosition, InstructionMemoryDrawingDefns,
};
use fam::application::draw::path::draw_path;
use fam::application::draw::port::{
    PortColorIndex, PortDrawingDefns, PortSignalDirection, SignalType,
};
use fam::application::draw::pos::{Size, dist, pos, size};
use fam::application::draw::shapes::draw_rectangle_pos;
use fam::application::draw::talu::{TaluBankDrawingDefns, TaluDrawingDefns};
use fam::application::grid::blocked_point::BlockedPoints;
use fam::application::grid::component::{ComponentGridData, DrawableComponent};
use fam::application::grid::controller::ControllerGridDefns;
use fam::application::grid::cpu::CpuGridData;
use fam::application::grid::grid_limits::GridLimits;
use fam::application::grid::path::{Path, Paths};
use fam::application::grid::pos::grid_pos;
use fam::application::simulation::controller::Controller;
use fam::application::simulation::cpu_registers::{CpuRegisterBank, REGISTER_COUNT};
use fam::application::simulation::instruction::Instruction;
use fam::application::simulation::instruction_reader::InstructionMemory;
use fam::application::simulation::main_memory::MainMemory;
use fam::application::simulation::simulation::Cpu;
use fam::application::simulation::talu::{CmpOp, TALU_COUNT, TaluBank, TaluOperation};
use fam::word::Word;
use macroquad::input::get_keys_pressed;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use std::collections::HashMap;
use wgpu::naga::FastHashMap;

fn main() {
    macroquad::Window::new("FAM Simulator", amain());
}

async fn amain() {
    let (program, data) = make_loop_program();

    let screen_size = size(1600, 900);

    let grid_limits = GridLimits::new(u16vec2(screen_size.x as u16 / 4, screen_size.y as u16 / 4));

    let grid_to_screen_mapper = GridToScreenMapper::new(
        &grid_limits,
        Rect::new(0_f32, 0_f32, screen_size.x as f32, screen_size.y as f32),
    );

    let cpu = build_full_cpu(program, data, screen_size, &grid_to_screen_mapper);

    let mut app = Application {
        step: 0,
        cpu,
        screen_size,
        grid_to_screen_mapper,
        grid_limits,
    };

    loop {
        set_window_size(screen_size.x as u32, screen_size.y as u32);
        if get_keys_pressed().contains(&KeyCode::Space) {
            app.step();
        }
        clear_background(WHITE);
        app.draw();

        next_frame().await;
    }
}
pub fn make_loop_program() -> (Vec<Instruction>, Vec<Word>) {
    let source_addr = 0;
    let target_addr = 5;

    let loop_count_reg = 2;
    let start_reg = 3;
    let finished_reg = 4;
    let ix_reg = 5;
    let ix_lt_loop_reg = 6;

    let start_increment_reg = 7;
    let start_inner_reg = 8;
    let source_addr_reg = 9;
    let target_addr_reg = 10;

    let word_reg = 11;
    let word_ready_reg = 12;
    let write_finished_reg = 13;

    let program = Vec::from_iter([
        Instruction::SetLiteral {
            literal: 0,
            register: 0,
        },
        Instruction::SetLiteral {
            literal: 1,
            register: 1,
        },
        Instruction::SetLiteral {
            literal: 5,
            register: 2,
        }, // loop_count
        Instruction::SetLiteral {
            literal: 0,
            register: 3,
        }, // start
        Instruction::SetLiteral {
            literal: 0,
            register: 4,
        }, // finished
        Instruction::SetLiteral {
            literal: 0,
            register: 5,
        }, // i
        Instruction::SetTaluConfig {
            talu_addr: 0,
            talu_config: TaluOperation::Cmp {
                op: CmpOp::LessThan,
                activation_input: 1,
                activation_output: None,
                data_input_0: ix_reg,
                data_input_1: loop_count_reg,
                data_output: ix_lt_loop_reg,
            },
        },
        Instruction::SetTaluConfig {
            talu_addr: 1,
            talu_config: TaluOperation::And {
                data_input_0: start_reg,
                data_input_1: ix_lt_loop_reg,
                activation_input: 1,
                data_output_0: start_inner_reg,
                activation_output: None,
            },
        },
        Instruction::SetTaluConfig {
            talu_addr: 2,
            talu_config: TaluOperation::Add {
                data_input_0: ix_reg,
                data_input_1: 1,
                activation_input: start_inner_reg,

                result_output: ix_reg,
                flags_output: None,
                activation_output: None,
            },
        },
        Instruction::SetLiteral {
            literal: source_addr,
            register: source_addr_reg,
        },
        Instruction::SetLiteral {
            literal: target_addr,
            register: target_addr_reg,
        },
        Instruction::SetTaluConfig {
            talu_addr: 3,
            talu_config: TaluOperation::ReadFromMem {
                activation_input: start_inner_reg,
                data_input_0: source_addr_reg,
                data_output_0: word_reg,
                activation_output: Some(word_ready_reg),
            },
        },
        Instruction::SetTaluConfig {
            talu_addr: 4,
            talu_config: TaluOperation::WriteToMem {
                activation_input: word_ready_reg,
                address_input: target_addr_reg,
                data_input: word_reg,
                activation_output: Some(write_finished_reg),
            },
        },
       Instruction::SetTaluConfig {
            talu_addr: 5,
            talu_config: TaluOperation::Add {
                activation_input: write_finished_reg,
                data_input_0: source_addr_reg,
                data_input_1: 1,
                result_output: source_addr_reg,
                flags_output: None,
                activation_output: None,
            },
        },
        Instruction::SetTaluConfig {
            talu_addr: 6,
            talu_config: TaluOperation::Add {
                activation_input: write_finished_reg,
                data_input_0: target_addr_reg,
                data_input_1: 1,
                result_output: target_addr_reg,
                flags_output: None,
                activation_output: None,
            },
        },
        Instruction::SetTaluConfig {
            talu_addr: 7,
            talu_config: TaluOperation::Mov {
                activation_input: 1,
                value_input: write_finished_reg,
                data_output: start_reg,
                activation_output: None,
            },
        },
 
        Instruction::SetLiteral {
            literal: 1,
            register: 2,
        },
        Instruction::SetLiteral {
            literal: 0,
            register: 2,
        },
        Instruction::WaitForActivationSignal { register_index: 4 }, // 128
    ]);

    let data = vec![1,2,3,4,5,0,0,0,0,0,0];

    (program, data)
}
pub struct FullCpu {
    pub sim: Cpu,
    pub grid: CpuGridData,
    pub drawing: CpuDrawingData,
}

pub struct Application {
    pub step: u32,
    pub cpu: FullCpu,
    pub screen_size: Size,
    pub grid_to_screen_mapper: GridToScreenMapper,
    pub grid_limits: GridLimits,
}
impl Application {
    pub fn draw(&self) {
        draw_full_cpu(&self.cpu, &self.grid_limits, &self.grid_to_screen_mapper);

        draw_paths(&self.cpu.grid.paths, &self.grid_to_screen_mapper);
        // draw_fps();
    }

    pub fn step(&mut self) -> bool {
        println!("stepping");
        let should_continue = self.cpu.sim.execute();
        if let Some(instruction_addr) = self
            .cpu
            .sim
            .controller
            .instruction_reader
            .get_instruction_pos()
        {
            self.cpu.drawing.instruction_memory.current_pos = instruction_addr;
        }
        self.cpu.grid.update_blocked_points();
        self.cpu
            .grid
            .calculate_paths(&self.cpu.sim.connections, &self.grid_limits);
        return should_continue;
    }
}

fn draw_step() {}
fn draw_fps() {
    draw_rectangle_pos(pos(0, 0), dist(200, 30), BLACK);
    macroquad::prelude::draw_fps();
}

fn draw_paths(
    // cpu: &FullCpu,
    paths: &Paths,
    // grid_limits: &GridLimits,
    grid_to_screen_mapper: &GridToScreenMapper,
) {
    pub const PATH_COLORS: [Color; 7] = [BLUE, RED, GREEN, BROWN, PURPLE, GOLD, MAGENTA];
    //
    // draw_path_grid(
    //     grid_to_screen_mapper,
    //     grid_limits,
    //     &cpu.grid_defns.blocked_points
    // );

    for (ix, (conn, path)) in paths.iter().enumerate() {
        draw_path(
            path,
            &PATH_COLORS[ix % PATH_COLORS.len()],
            grid_to_screen_mapper,
        )
    }
}

fn draw_full_cpu(
    cpu: &FullCpu,
    grid_limits: &GridLimits,
    grid_to_screen_mapper: &GridToScreenMapper,
) {
    // draw_path_grid(
    //     &grid_to_screen_mapper,
    //     &grid_limits,
    //     &BlockedPoints::new()
    // );

    cpu.sim.instruction_memory.draw(
        &cpu.drawing.instruction_memory.current_pos,
        &cpu.grid.instruction_memory,
        &cpu.drawing.instruction_memory,
        &cpu.drawing.port,
        &grid_to_screen_mapper,
    );

    let registers_drawing_state = Box::new([(); REGISTER_COUNT]);

    cpu.sim.register_bank.draw(
        &registers_drawing_state,
        &cpu.grid.register_bank,
        &cpu.drawing.register_bank,
        &cpu.drawing.port,
        &grid_to_screen_mapper,
    );

    let talu_bank_drawing_state = Box::new([(); TALU_COUNT]);

    cpu.sim.controller.draw(
        &(),
        &cpu.grid.controller,
        &cpu.drawing.controller,
        &cpu.drawing.port,
        &grid_to_screen_mapper,
    );

    cpu.sim.talu_bank.draw(
        &talu_bank_drawing_state,
        &cpu.grid.talu_bank,
        &cpu.drawing.talu_bank,
        &cpu.drawing.port,
        &grid_to_screen_mapper,
    );
}

fn build_full_cpu(
    program: Vec<Instruction>,
    data: Vec<Word>,
    screen_size: Size,
    grid_to_screen_mapper: &GridToScreenMapper,
) -> FullCpu {
    let mut main_memory = MainMemory::new(data);

    let registers = CpuRegisterBank::new();

    let instruction_memory = InstructionMemory::new(program.clone());

    let talus = TaluBank::new(&mut main_memory);

    let controller = Controller::new(&instruction_memory);

    let cpu = {
        Cpu {
            main_memory,
            talu_bank: talus,
            register_bank: registers,
            controller,
            instruction_memory,
            is_done: false,
            connections: Default::default(),
        }
    };

    let port_drawing_data = PortDrawingDefns {
        base: 6,
        line_len: 4,
        arrow_height: 8,
        line_width: 1,
        color_defn: Box::new(|color| -> Color {
            match color {
                PortColorIndex::Deactivated => DARKGRAY,
                PortColorIndex::Active(SignalType::Data, PortSignalDirection::Input) => BLUE,
                PortColorIndex::Active(SignalType::Data, PortSignalDirection::Output) => RED,
                PortColorIndex::Active(SignalType::Activation, PortSignalDirection::Input) => GREEN,
                PortColorIndex::Active(SignalType::Activation, PortSignalDirection::Output) => {
                    YELLOW
                }
                PortColorIndex::Active(SignalType::TaluSetup, PortSignalDirection::Input) => VIOLET,
                PortColorIndex::Active(SignalType::TaluSetup, PortSignalDirection::Output) => BROWN,
            }
        }),
    };

    let mut cursor = RectCursor::new(pos(0, 0), screen_size);
    cursor.pad(4, 4);

    let memory_cursor = cursor
        .split(cursor.remaining_size().x / 5, Horizontal)
        .with_padding(20, 0);

    let instruction_mem_drawing_defns = InstructionMemoryDrawingDefns {
        current_pos: 0,
        size: memory_cursor.remaining_size(),
    };

    let instruction_mem_calculated_defns = cpu.instruction_memory.calculate_defns(
        grid_to_screen_mapper.screen_to_nearest_grid_pos(memory_cursor.top_left()),
        &instruction_mem_drawing_defns,
        &port_drawing_data,
        &grid_to_screen_mapper,
    );

    let mut top_half_cursor = cursor.split(cursor.remaining_size().y / 2, Vertical);

    let controller_cursor = top_half_cursor.split(140, Horizontal).with_padding(20, 20);

    let controller_drawing_data = ControllerDrawingDefns {
        size: size(controller_cursor.remaining_size().x, 100),
    };

    let controller_grid_data = cpu.controller.calculate_defns(
        grid_to_screen_mapper.screen_to_nearest_grid_pos(controller_cursor.top_left()),
        &controller_drawing_data,
        &port_drawing_data,
        grid_to_screen_mapper,
    );

    let talu_bank_cursor = top_half_cursor.with_padding(40, 0);

    let talu_bank_drawing_data = TaluBankDrawingDefns {
        name: "TALUs".to_string(),
        size: talu_bank_cursor.remaining_size(),
        row_count: 4,
        inner_drawing_defns: Default::default(),
    };

    let talu_bank_grid_defns = cpu.talu_bank.calculate_defns(
        grid_to_screen_mapper.screen_to_nearest_grid_pos(talu_bank_cursor.top_left()),
        &talu_bank_drawing_data,
        &port_drawing_data,
        &grid_to_screen_mapper,
    );

    let bottom_half_cursor = cursor.with_padding(40, 40);

    let register_bank_drawing_data = CpuRegisterBankDrawingDefns {
        name: "Registers".to_string(),
        size: bottom_half_cursor.remaining_size(),
        row_count: 8,
        inner_drawing_defns: Default::default(),
    };

    let register_bank_grid_data = cpu.register_bank.calculate_defns(
        grid_to_screen_mapper.screen_to_nearest_grid_pos(bottom_half_cursor.top_left()),
        &register_bank_drawing_data,
        &port_drawing_data,
        &grid_to_screen_mapper,
    );

    let cpu_drawing_data = CpuDrawingData {
        port: port_drawing_data,
        register_bank: register_bank_drawing_data,
        talu_bank: talu_bank_drawing_data,
        instruction_memory: instruction_mem_drawing_defns,
        controller: controller_drawing_data,
    };

    let all_blocked_points = {
        let mut blocked = talu_bank_grid_defns.blocked_points.clone();
        blocked.add_from(register_bank_grid_data.blocked_points());
        blocked.add_from(instruction_mem_calculated_defns.blocked_points());
        blocked.add_from(controller_grid_data.blocked_points());
        blocked
    };

    let cpu_grid_defns = CpuGridData {
        talu_bank: talu_bank_grid_defns,
        register_bank: register_bank_grid_data,
        instruction_memory: instruction_mem_calculated_defns,
        blocked_points: all_blocked_points,
        controller: controller_grid_data,
        paths: FastHashMap::default(),
    };

    FullCpu {
        sim: cpu,
        grid: cpu_grid_defns,
        drawing: cpu_drawing_data,
    }
}
