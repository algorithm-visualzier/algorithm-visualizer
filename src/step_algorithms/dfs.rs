use std::collections::VecDeque;

use petgraph::graph::NodeIndex;
use petgraph::Direction;

use crate::graph::node::NodeState;
use crate::graph::Graph;
use crate::step_algorithms::algorithm::{Algorithm, AlgorithmStep, EdgeStep, NodeStep};
use crate::step_algorithms::timer::Timer;

pub struct Dfs {
    steps: VecDeque<AlgorithmStep>,
    timer: Timer,
    start_idx: NodeIndex,
}

impl Algorithm for Dfs {
    fn start_idx(&self) -> NodeIndex {
        self.start_idx
    }

    fn timer(&self) -> &Timer {
        &self.timer
    }

    fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }

    fn steps(&self) -> &VecDeque<AlgorithmStep> {
        &self.steps
    }

    fn steps_mut(&mut self) -> &mut VecDeque<AlgorithmStep> {
        &mut self.steps
    }

    fn run_algorithm(&mut self, graph: &mut Graph) {
        self.reset_algorithm(graph);
        self.dfs(graph);
    }
}

impl Dfs {
    pub fn new(start_idx: NodeIndex) -> Dfs {
        Dfs {
            steps: VecDeque::new(),
            timer: Timer::new(1., true),
            start_idx,
        }
    }

    fn dfs(&mut self, graph: &mut Graph) {
        self.dfs_helper(graph, self.start_idx());
    }

    fn dfs_helper(&mut self, graph: &mut Graph, node_index: NodeIndex) {
        self.add_step(AlgorithmStep::Node(NodeStep::new(
            node_index,
            NodeState::Queued,
        )));

        if let Some(node) = graph.node_weight_mut(node_index) {
            node.set_state(NodeState::Queued)
        }

        let mut walker = graph
            .neighbors_directed(node_index, Direction::Outgoing)
            .detach();

        while let Some((edge_idx, other_node_idx)) = walker.next(graph) {
            if let Some(other_state) = graph
                .node_weight(other_node_idx)
                .map(|node| node.get_state())
            {
                if matches!(other_state, NodeState::NotVisited) {
                    self.add_step(AlgorithmStep::Edge(EdgeStep::new(edge_idx)));
                    self.dfs_helper(graph, other_node_idx);
                }
            }
        }

        self.add_step(AlgorithmStep::Node(NodeStep::new(
            node_index,
            NodeState::Visited,
        )));

        if let Some(node) = graph.node_weight_mut(node_index) {
            node.set_state(NodeState::Visited)
        }
    }

    pub fn show_dfs(&mut self, graph: &mut Graph) {
        

        
    }
}
