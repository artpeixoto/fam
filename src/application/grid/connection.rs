use itertools::Itertools;
use crate::application::draw::port::{PortDefns, PortGridDefns};
use crate::application::grid::pos::{grid_pos, GridPos};
use crate::application::simulation::cpu_registers::{CpuRegisterAddress, CpuRegisterPortName};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ConnectionEndpoint{
    pub id : GridPos,
    pub pos: GridPos,
}

pub enum PortId{

}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ConnectionEndpointPair(
    pub ConnectionEndpoint,
    pub ConnectionEndpoint,
);

impl ConnectionEndpointPair{
    pub fn new(p0: ConnectionEndpoint, p1: ConnectionEndpoint) -> Self{
        let (first, second) = {
            let cmp_val = |p: &ConnectionEndpoint| {
                (p.pos.x, p.pos.y)
            };
            if cmp_val(&p0) >= cmp_val(&p1) {
                (p0, p1)
            }  else {
                (p1, p0)
            }
        };

        Self(
            first,
            second
        )
    }
}