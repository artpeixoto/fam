use itertools::Itertools;
use macroquad::color::BLACK;
use wgpu::naga::FastHashMap;
use crate::application::direction::Axis;
use crate::application::draw::cursor::RectCursor;
use crate::application::draw::text::{TextStyle, draw_text_pos};
use crate::application::grid::component::{DrawableComponent, PortDataContainer, PortName, ComponentGridData};
use crate::application::draw::grid_to_screen::GridToScreenMapper;
use crate::application::draw::port::{PortDefns, PortDrawingDefns, PortGridDefns};
use crate::application::draw::pos::Size;
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::pos::{grid_pos, grid_size, GridPos};
use crate::application::grid::rect::{grid_rect, GridRect};
use crate::application::simulation::component_bank::ComponentBank;

#[derive(Clone, PartialEq, Eq)]
pub struct ComponentBankDrawingDefn<CompDrawingDefn> {
    pub name        : String, 
    pub size        : Size,
    pub row_count   : usize,
    pub inner_drawing_defns: CompDrawingDefn,
}

impl<CompDrawingDefn: Default> Default for ComponentBankDrawingDefn<CompDrawingDefn> {
    fn default() -> Self {
        Self {
            name: "Component Bank".to_string(),
            size: Size::new(1920 / 2, 1080 / 2),
            row_count: 4,
            inner_drawing_defns: CompDrawingDefn::default(),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash)]
pub struct ComponentBankPortName<CompPortName, const COMP_COUNT: usize>{
    pub comp_addr: usize ,
    pub port_name: CompPortName
}

impl<CompPortName: PortName, const COMP_COUNT: usize>
    PortName
    for ComponentBankPortName<CompPortName, COMP_COUNT>
{
    fn all_port_names() -> Vec<Self> {
        (0..COMP_COUNT)
        .into_iter()
        .map(|address| {
            CompPortName
            ::all_port_names()
            .into_iter()
            .map(move |port_name|{ComponentBankPortName {
                comp_addr: address,
                port_name: port_name.clone()}
            })
        })
        .flatten()
        .collect_vec()
    }

    fn small_name(&self) -> &str {
        self.port_name.small_name()
    }
}

pub struct ComponentBankPortDataContainer<CompPortName: PortName, Data, const COMP_COUNT: usize>{
    pub elements: FastHashMap<ComponentBankPortName<CompPortName, COMP_COUNT>, Data>
}

impl<CompPortName, Data, const  COMP_COUNT: usize>
    PortDataContainer<ComponentBankPortName<CompPortName, COMP_COUNT>, Data>
    for ComponentBankPortDataContainer<CompPortName, Data, COMP_COUNT>
where
    CompPortName: PortName
{
    fn get_for_port(&self, port_name: &ComponentBankPortName<CompPortName, COMP_COUNT>) -> &Data {
        self.elements.get(port_name).unwrap()
    }
}

pub struct ComponentBankGridData<
    InnerComp: DrawableComponent,
    const COMP_COUNT: usize
>
{
    pub title_pos       : GridPos,
    pub grid_rect       : GridRect,
    pub blocked_points  : BlockedPoints,
    pub ports_data      : ComponentBankPortDataContainer<InnerComp::PortName, PortDefns, COMP_COUNT>,
    pub ports_grid_data : ComponentBankPortDataContainer<InnerComp::PortName, PortGridDefns, COMP_COUNT>,
    pub comp_grid_datas : Box<[InnerComp::ComponentCalculatedDefns; COMP_COUNT]>,
}
impl<InnerComp, const COMP_COUNT: usize>
    ComponentGridData for ComponentBankGridData<InnerComp, COMP_COUNT>
where
    InnerComp: DrawableComponent,
{
    type PortName = ComponentBankPortName<InnerComp::PortName, COMP_COUNT>;
    type PortDataContainer = ComponentBankPortDataContainer<InnerComp::PortName, PortDefns, COMP_COUNT>;
    type PortGridDataContainer = ComponentBankPortDataContainer<InnerComp::PortName, PortGridDefns,
        COMP_COUNT>;

    fn grid_rect(&self) -> GridRect {
       self.grid_rect .clone()
    }

    fn blocked_points(&self) -> &BlockedPoints {
        &self.blocked_points
    }

    fn ports_data(&self) -> &Self::PortDataContainer {
        &self.ports_data
    }

    fn ports_grid_data(&self) -> &Self::PortGridDataContainer {
        &self.ports_grid_data
    }
}

impl<
    InnerComp,
    const COMP_COUNT: usize
>
    DrawableComponent
    for ComponentBank<InnerComp, COMP_COUNT>
where
    InnerComp: DrawableComponent,
    InnerComp::PortName: PortName,
{
    type DrawingState = Box<[InnerComp::DrawingState; COMP_COUNT]>;
    type DrawingDefn = ComponentBankDrawingDefn<InnerComp::DrawingDefn>;
    type PortName = ComponentBankPortName<InnerComp::PortName, COMP_COUNT>;
    type PortDataContainer = ComponentBankPortDataContainer<InnerComp::PortName, PortDefns, COMP_COUNT>;
    type PortGridDataContainer = ComponentBankPortDataContainer<InnerComp::PortName, PortGridDefns, COMP_COUNT>;
    type ComponentCalculatedDefns = ComponentBankGridData<InnerComp, COMP_COUNT>;

    fn calculate_defns(
        &self,
        grid_top_left: GridPos,
        drawing_data: &Self::DrawingDefn,
        port_drawing_data: &PortDrawingDefns,
        grid_to_screen_mapper: &GridToScreenMapper
    ) -> Self::ComponentCalculatedDefns {
        let mut cursor = RectCursor::new(grid_to_screen_mapper.grid_to_screen_pos(grid_top_left), drawing_data.size);

        let title_height = TextStyle::W_I_D_E.get_dims().full_height() * 2 + 4;

        let title_cursor =  cursor.split(title_height, Axis::Vertical).with_padding(0, 2);
        let title_pos = grid_to_screen_mapper.screen_to_nearest_grid_pos(title_cursor.top_left());

        let grid_top_left = grid_to_screen_mapper.screen_to_nearest_grid_pos(cursor.top_left());
        let full_grid_size = grid_to_screen_mapper.screen_to_grid_size(cursor.remaining_size());
        let col_count = COMP_COUNT / drawing_data.row_count;

        let inner_grid_size =
            self
                .components[0]
                .calculate_defns(
                    grid_top_left,
                    &drawing_data.inner_drawing_defns,
                    port_drawing_data,
                    grid_to_screen_mapper
                )
                .grid_rect()
                .size;

        let remaining_grid_size = full_grid_size - grid_size(
            (inner_grid_size.x * col_count as i16),
            (inner_grid_size.y * drawing_data.row_count as i16),
        );

        let grid_spacing = grid_size(
            remaining_grid_size.x / (col_count as i16 + 1),
            remaining_grid_size.y / (drawing_data.row_count as i16 + 1),
        );
        let mut port_data = FastHashMap::default();
        let mut port_grid_data = FastHashMap::default();
        let mut blocked_points = BlockedPoints::new();
        let mut inner_components_grid_datas = Vec::new();
        for iy in 0..drawing_data.row_count {
            for ix in 0..col_count {
                let grid_x = grid_top_left.x + grid_spacing.x + (ix as i16 * (grid_spacing.x +
                    inner_grid_size.x));

                let grid_y = grid_top_left.y + grid_spacing.y + (iy as i16 * (grid_spacing.y +
                    inner_grid_size.y));

                let inner_comp_pos = grid_pos(grid_x, grid_y);

                let inner_comp_addr = iy * drawing_data.row_count + ix;

                let inner_comp = &self.components[inner_comp_addr];

                let inner_comp_grid_data = inner_comp.calculate_defns(
                    inner_comp_pos,
                    &drawing_data.inner_drawing_defns,
                    &port_drawing_data,
                    &grid_to_screen_mapper
                );


                blocked_points.add_from(inner_comp_grid_data.blocked_points());

                for inner_port_name in InnerComp::PortName::all_port_names() {
                    let cur_port_name: Self::PortName  = ComponentBankPortName{
                        comp_addr: inner_comp_addr,
                        port_name: inner_port_name.clone(),
                    };
                    port_data.insert(
                        cur_port_name.clone(),
                        inner_comp_grid_data.ports_data().get_for_port(&inner_port_name).clone()
                    );
                    port_grid_data.insert(
                        cur_port_name.clone(),
                        inner_comp_grid_data.ports_grid_data().get_for_port(&inner_port_name).clone()
                    );
                }

                inner_components_grid_datas.push(inner_comp_grid_data);
            }
        }

        let port_data = ComponentBankPortDataContainer{
            elements: port_data
        };
        let port_grid_data = ComponentBankPortDataContainer{
            elements: port_grid_data
        };
        let inner_components_grid_datas = inner_components_grid_datas.into_boxed_slice()
            .into_array().unwrap();
        ComponentBankGridData{
            title_pos: title_pos,
            grid_rect: grid_rect(grid_top_left, full_grid_size),
            blocked_points,
            ports_data: port_data,
            ports_grid_data: port_grid_data,
            comp_grid_datas: inner_components_grid_datas,
        }
    }

    fn draw(
        &self,
        drawing_state           : &Self::DrawingState,
        calculated_defns        : &Self::ComponentCalculatedDefns,
        drawing_defns           : &Self::DrawingDefn,
        port_drawing_defns      : &PortDrawingDefns,
        grid_to_screen_mapper   : &GridToScreenMapper
    ) {
        draw_text_pos(
            &drawing_defns.name, 
            grid_to_screen_mapper.grid_to_screen_pos(calculated_defns.title_pos), 
            TextStyle::W_I_D_E,
            2, 
            BLACK
        );

        for addr in 0..COMP_COUNT{
            let cur_comp = &self.components[addr];
            
            let cur_comp_grid_data = &calculated_defns.comp_grid_datas[addr];
            
            cur_comp.draw(
                &drawing_state[addr],
                cur_comp_grid_data,
                &drawing_defns.inner_drawing_defns,
                port_drawing_defns,
                grid_to_screen_mapper
            );
        }
    }
}