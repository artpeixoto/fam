use std::hash::Hash;
use std::marker::PhantomData;
use wgpu::naga::FastHashMap;
use crate::application::draw::grid_to_screen::GridScreenTransformer;
use crate::application::draw::port::{PortDrawingDefns, PortGridDefns, PortDefns};
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::pos::GridPos;
use crate::application::grid::rect::GridRect;

pub trait DrawableComponent {
    type DrawingState;
    type DrawingDefn;
    type PortName: PortName;
    type PortDataContainer
        : PortDataContainer<Self::PortName, PortDefns>;
    type PortGridDataContainer
        : PortDataContainer<Self::PortName, PortGridDefns>;

    type ComponentCalculatedDefns:  ComponentCalculatedDefns<
        PortName=Self::PortName,
        PortDataContainer=Self::PortDataContainer,
        PortGridDataContainer=Self::PortGridDataContainer,
    > ;
    fn calculate_defns(
        &self,
        grid_pos         : GridPos,
        drawing_info     : &Self::DrawingDefn,
        port_drawing_info: &PortDrawingDefns,
        grid_to_screen   : &GridScreenTransformer,
    ) -> Self::ComponentCalculatedDefns;

    fn draw(
        &self,
        drawing_state    : &Self::DrawingState,
        grid_defns       : &Self::ComponentCalculatedDefns,
        drawing_defns    : &Self::DrawingDefn,
        port_drawing_info: &PortDrawingDefns,
        grid_to_screen   : &GridScreenTransformer,
    );
}

pub trait PortName: Sized + Hash + Eq + Clone {
    fn all_port_names() -> Vec<Self>;

    /// Name must be 5 characters or fewer
    fn small_name(&self) -> &str;
}

pub trait ComponentCalculatedDefns
{
    type PortName: PortName;
    type PortDataContainer: PortDataContainer<Self::PortName, PortDefns>;
    type PortGridDataContainer: PortDataContainer<Self::PortName, PortGridDefns>;
    fn grid_rect(&self) -> GridRect;
    fn blocked_points(&self) -> &BlockedPoints;
    fn ports_data(&self) -> &Self::PortDataContainer;
    fn ports_grid_data (&self) -> &Self::PortGridDataContainer;
}

impl<N, P, G>
    ComponentCalculatedDefns
    for SimpleComponentGridData<N, P, G>
where
    N: PortName,
    P: PortDataContainer<N, PortDefns>,
    G: PortDataContainer<N, PortGridDefns>,
{
    type PortName = N; 
    type PortDataContainer = P;
    type PortGridDataContainer = G;

    fn grid_rect(&self) -> GridRect {
        self.grid_rect.clone()
    }

    fn blocked_points(&self) -> &BlockedPoints {
        &self.blocked_points
    }

    fn ports_data(&self) -> &P {
        &self.ports_data
    }

    fn ports_grid_data(&self) -> &G {
        &self.ports_grid_data
    }
}

pub struct SimpleComponentGridData<N, P, G,>
where
    N: PortName,
    P: PortDataContainer<N, PortDefns>,
    G: PortDataContainer<N, PortGridDefns>,
{
    pub grid_rect       : GridRect,
    pub blocked_points  : BlockedPoints,
    pub ports_data      : P,
    pub ports_grid_data : G,
    pub _phantom        : PhantomData<N>,
}

pub trait PortDataContainer< N: PortName, P, >{
    fn get_for_port(&self, port_name: &N) -> &P;
}


impl<N: PortName, P> PortDataContainer<N, P> for FastHashMap<N, P>{
    fn get_for_port(&self, port_name: &N) -> &P {
        &self[port_name]
    }
}
