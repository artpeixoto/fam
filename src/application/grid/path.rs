use std::collections::{BTreeMap, VecDeque};
use std::ops::{Deref, Not};
use std::rc::{Rc, Weak};
use either::Either;
use itertools::Itertools;
use wgpu::naga::{FastHashMap, FastHashSet};
use crate::application::connection::CpuConnection;
use crate::application::direction::Direction;
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::grid_limits::GridLimits;
use crate::application::grid::movement::GridMovement;
use crate::application::grid::pos::GridPos;

pub type Paths = FastHashMap<CpuConnection, Path>;

#[derive(Debug, Eq, PartialEq,  Clone) ]
pub enum InvalidPointReason{
    OutOfBounds,
    Blocked,
    InAnotherPath,
}

#[derive(Debug, Eq, PartialEq,  Clone) ]
pub enum PathSearchingFailure{
    InvalidStartingPoint(InvalidPointReason),
    InvalidEndingPoint(InvalidPointReason),
    NoPathFound
}

pub fn find_path_a_star(
    from            : &GridPos,
    to              : &GridPos,
    connection      : &CpuConnection, 
    existing_paths  : &Paths,
    blocked_points  : &BlockedPoints,
    grid_bounds     : &GridLimits,
) ->  Result<Path, PathSearchingFailure>{
    // first check everything.
    // if !grid_bounds.contains_point(from) {return }
    let mut moves_analyzed = 0_u64;

    let mut is_move_available = {
        let lines_used_by_other_paths =
            existing_paths
            .iter()
            .filter(|&(path_conn, path)| {
                // connection.first() != path_conn.
                path_conn.first() != connection.first() && 
                path_conn.first() != connection.second() &&
                path_conn.second() != connection.first() &&
                path_conn.second() != connection.second() 
            })
            .flat_map(|path| path.1.into_iter())
            .map(|movement| movement.line )
            .collect::<FastHashSet<_>>();



        move |movement: &GridMovement, visited_points: &FastHashSet<GridPos>| -> bool {
            // println!("analysing movement {} {:?} -{:?}-> {:?}", moves_analyzed, movement.starting_point, movement.move_dir, movement.destination_point);
            moves_analyzed +=1;
            let line = &movement.line;

            let line_is_in_bounds = grid_bounds.contains_line(line);
            if  !line_is_in_bounds {return false}
            // println!("\tline is in bounds: {:?}", line_is_in_bounds);

            let line_is_not_blocked = line.points().into_iter().all(|p| blocked_points.point_is_available(&p));
            // println!("\tline is not blocked: {:?}", line_is_not_blocked);
            if !line_is_not_blocked {return false}

            let line_is_available = !lines_used_by_other_paths.contains(line);
            // println!("\tline is available: {:?}", line_is_available);
            if !line_is_available {return false}

            let point_has_not_been_visited = visited_points.contains(&movement.destination_point).not();
            // println!("\tpoint_has_not_been_visited: {:?}", line_is_available);
            if !point_has_not_been_visited {return false}
            // let movement_target_has_not_been_visited =
            //     node_walker
            //         .into_iter()
            //         .map(|node|{
            //             node.position.clone()
            //         })
            //         .all(|l|
            //              &movement.destination_point != &l
            //         );
            //
            // println!("\tpoint has not been visited: {:?}", movement_target_has_not_been_visited);
            // if !movement_target_has_not_been_visited {return false}



            true
        }
    };

    let estimate_cost = |start: &GridPos, end: &GridPos| -> Cost{
        start.x.abs_diff(end.x) + start.y.abs_diff(end.y)
    };
    pub type Cost = u16;

    struct SearchNodeParentConn{
        parent  : Weak<SearchNode>,
        move_dir: Direction,
    }

    struct SearchNode {
        parent_conn              : Option<SearchNodeParentConn>,
        position                 : GridPos,
        estimated_remaining_cost : Cost,
        accumulated_cost         : Cost,
    }
    struct SearchNodeParentIterator<'a>{
        child: Option<Either<&'a SearchNode, Rc<SearchNode>>>
    }
    enum SearchNodeParentIteratorChildRef<'a>{
        Ref(&'a SearchNode),
        Rc(Rc<SearchNode>),
    }
    impl<'a> Deref for SearchNodeParentIteratorChildRef<'a>{
        type Target = SearchNode;

        fn deref(&self) -> &Self::Target {
            match self{
                &SearchNodeParentIteratorChildRef::Ref(a) => &*a,
                SearchNodeParentIteratorChildRef::Rc(a) => a.deref(),
            }
        }
    }
    impl<'a> Iterator for SearchNodeParentIterator<'a>{
        type Item = SearchNodeParentIteratorChildRef<'a>;
        fn next(&mut self) -> Option<Self::Item> {
            let child = self.child.take()?;
            let child = match child{
                Either::Left(a)  => SearchNodeParentIteratorChildRef::Ref(a),
                Either::Right(a) => SearchNodeParentIteratorChildRef::Rc(a),
            };
            if let Some(parent) = child.parent_conn.as_ref(){
                self.child = Some(Either::Right(parent.parent.upgrade().unwrap()));
            }
            Some(child)
        }
    }

    impl<'a> SearchNodeParentIterator<'a>{
        pub fn new(node: &'a SearchNode) -> Self{
            Self{
                child: Some(Either::Left(node)),
            }
        }

    }

    impl SearchNode {
        pub fn full_cost(&self) -> Cost{
            self.accumulated_cost + self.estimated_remaining_cost
        }

        pub fn iter_parents(&self) -> SearchNodeParentIterator {
            SearchNodeParentIterator::new(self)
        }
        pub fn get_path(&self) -> Path{
            let mut starting_point = self.position.clone();
            let movements = {
                let mut movements =
                    self
                        .iter_parents()
                        .flat_map(|node| {
                            starting_point = node.position.clone();
                            Some(node.parent_conn.as_ref()?.move_dir.clone())
                        })
                        .collect_vec();
                movements.reverse();
                movements
            };

            Path{
                starting_point,
                movements
            }
        }
    }

    pub struct Frontier(
        BTreeMap<Cost, VecDeque<SearchNode>>
    );

    impl Frontier{
        fn add(&mut self, node: SearchNode) {
            let full_cost = node.full_cost();
            self.0.entry(full_cost).or_default().push_front(node);
        }
        fn take_next(&mut self) -> Option<SearchNode> {
            let (&cost, stack) = self.0.iter_mut().next()?;
            let next = stack.pop_back().unwrap();

            if stack. is_empty(){
                self.0.remove(&cost);
            }
            Some(next)
        }
    }

    fn get_next_moves(node: &SearchNode) -> Vec<GridMovement>{
        if let Some(parent_conn)  = node.parent_conn.as_ref(){
            let dir = parent_conn.move_dir.clone();

            [dir.clone(), dir.rotate_cw(), dir.rotate_ccw() ]
                .into_iter()
                .map(|dir| node.position + dir )
                .collect_vec()

        } else {
            node.position.all_moves()
        }
    }

    let mut visited_points = FastHashSet::default();
    let mut opened = Vec::new();
    let mut frontier = Frontier(BTreeMap::new());

    frontier.add(
        SearchNode {
            parent_conn: None,
            position: *from,
            estimated_remaining_cost: estimate_cost(from, to),
            accumulated_cost: 0,
        }
    );
    loop{
        let Some(node) = frontier.take_next() else {return Err(PathSearchingFailure::NoPathFound)};
        let node = Rc::new(node);
        opened.push(node.clone());

        if &node.position == to{
            return Ok(node.get_path())
        }

        visited_points.insert(node.position);

        // open the node


        let moves = get_next_moves(&node);

        for m in moves{
            if !is_move_available(&m, &visited_points){ continue }
            let position = m.destination_point;
            let estimated_remaining_cost = estimate_cost(&m.destination_point, to);
            let accumulated_cost  = node.accumulated_cost + 1;
            let new_node = SearchNode{
                parent_conn: Some(SearchNodeParentConn{
                    parent: Rc::downgrade(&node),
                    move_dir: m.move_dir,
                }),

                position,
                estimated_remaining_cost,
                accumulated_cost
            };
            visited_points.insert(position);
            frontier.add(new_node);
        }
    }
}

pub struct Path{
    starting_point  : GridPos,
    movements       : Vec<Direction>,
}
impl Path{
    pub fn walk(&self) -> PathWalker{
        PathWalker{
            path: &self,
            current_movement_ix: 0,
            current_pos: self.starting_point.clone(),
        }
    }
}

impl<'a> IntoIterator for &'a Path{
    type Item = GridMovement;
    type IntoIter = PathWalker<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.walk()
    }
}


#[derive(Clone)]
pub struct PathWalker<'a>{
    path: &'a Path,
    current_movement_ix : usize,
    current_pos         : GridPos,
}
impl Iterator  for PathWalker<'_> {
    type Item = GridMovement;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_movement_ix < self.path.movements.len()  {
            let current_move= &self.path.movements[self.current_movement_ix];
            let current_move_result = self.current_pos.go(*current_move);
            self.current_pos = current_move_result.destination_point;
            self.current_movement_ix += 1;
            Some(current_move_result)
        } else {
            None
        }
    }
}

