use core::marker::PhantomData;
use std::{boxed::Box, collections::HashMap, fmt, hash::Hash, vec::Vec};
use crate::{visit::{GraphBase, GraphProp, GraphRef, IntoNeighbors, IntoEdgeReferences, IntoEdges}, Directed, EdgeType};

#[derive(Clone, Copy)]
pub struct NodeInsertion<N, NodeIx>{
    value: N,
    index: NodeIx
}

#[derive(Clone, Copy)]
pub struct NodeUpdate<N, NodeIx>{
    new_value: N,
    index: NodeIx
}

#[derive(Clone, Copy)]
pub struct NodeRemoval<N, NodeIx>{
    old_value: N,
    index: NodeIx,
}

#[derive(Clone, Copy)]
pub struct EdgeInsertion<E, NodeIx, EdgeIx>{
    value: E,
    index: EdgeIx,
    from: NodeIx,
    to: NodeIx
}

#[derive(Clone, Copy)]
pub struct EdgeUpdate<E, EdgeIx>{
    new_value: E,
    index: EdgeIx,
}

#[derive(Clone, Copy)]
pub struct EdgeRemoval<E, NodeIx, EdgeIx>{
    old_value: E,
    index: EdgeIx,
    from: NodeIx,
    to: NodeIx
}

#[derive(Clone, Copy)]
pub enum GraphDelta<N, E, NodeIx, EdgeIx>{
    NodeInsertion(NodeInsertion<N, NodeIx>),
    NodeUpdate(NodeUpdate<N, NodeIx>),
    NodeRemoval(NodeRemoval<N, NodeIx>),
    EdgeInsertion(EdgeInsertion<E, NodeIx, EdgeIx>),
    EdgeUpdate(EdgeUpdate<E, EdgeIx>),
    EdgeRemoval(EdgeRemoval<E, NodeIx, EdgeIx>)
}

pub type DefaultIx = u32;

// pub trait IndexType: Copy + Default + Hash + Ord + fmt::Debug{}

/// A Graph type which supports layering alterations over a base graph type, where the base graph supports various graph traits.
/// The base graph is a lifetimed immutable reference, so as to allow sharing as widely as possible.  
pub struct BaseDeltaGraph<'g, G: 'g, N, E> where G: GraphBase + GraphProp {
    base_graph: &'g G,
    deltas: Vec<GraphDelta<N, E, G::NodeId, G::NodeId>>,
    // _phantom: PhantomData<Ty>,
}

impl<'g, G: 'g, N, E> BaseDeltaGraph<'g, G, N, E> where G: GraphBase + GraphProp{
    fn new(graph: &'g G) -> Self{
        BaseDeltaGraph{
            base_graph: graph,
            deltas: Vec::new(),
            // _phantom: PhantomData,
        }
    }
}

impl<'g, G: 'g, N, E> GraphBase for BaseDeltaGraph<'g, G, N, E>
where 
G: GraphBase + GraphProp,
{
    #[doc = r" edge identifier"]
type EdgeId = G::EdgeId;

    #[doc = r" node identifier"]
type NodeId = G::NodeId;
}

impl<'g, G: 'g, N, E> GraphProp for BaseDeltaGraph<'g, G, N, E>
where 
G: GraphBase + GraphProp,
{
    #[doc = r" The kind of edges in the graph."]
type EdgeType = G::EdgeType;

    fn is_directed(&self) -> bool {
        self.base_graph.is_directed()
    }
}

pub struct BaseDeltaGraphNeighbors<'g, G: 'g> where
G: IntoEdges
{
    base_edges: G::Edges,
    node_removals: HashMap<G::NodeId, ()>,
    edge_insertions: HashMap<G::EdgeId, G::NodeId>, //EdgeID, Target Node ID
    edge_removals: HashMap<G::EdgeId, G::NodeId>, //EdgeID, Target Node ID
    _phantom_ref: PhantomData<&'g ()>
}

// impl<'g, G: 'g, NodeIx, Ty> BaseDeltaGraphNeighbors<'g, G, NodeIx, Ty>{
//     fn new(G::Neighbors base, )
// }

impl<'g, G: 'g> Iterator for BaseDeltaGraphNeighbors<'g, G> where
G: IntoEdges
{
    type Item = G::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'g, 'a, G: 'g, N, E: 'a> IntoNeighbors for &'a BaseDeltaGraph<'g, G, N, E> 
where G: IntoEdges + GraphBase + GraphProp,
G::NodeId: Hash + Eq, G::EdgeId: Hash + Eq
{

    

    type Neighbors = BaseDeltaGraphNeighbors<'g, G>;

    #[doc = r" Return an iterator of the neighbors of node `a`."]
    fn neighbors(self,a:Self::NodeId) -> Self::Neighbors {
        let base_edges = self.base_graph.edges(a);
        let mut node_removals: HashMap<Self::NodeId, ()> = HashMap::new();
        let mut edge_insertions: HashMap<G::EdgeId, G::NodeId> = HashMap::new();
        let mut edge_removals: HashMap<G::EdgeId, G::NodeId> = HashMap::new();
        // let mut delta_origin: Option<Self::NodeId> = None;

        for delta in self.deltas.iter(){
            match delta{
                GraphDelta::EdgeInsertion(edge_insertion) => {
                    if edge_insertion.from == a {
                        additions.insert(edge_insertion.to, ());
                        invalidations.remove(&edge_insertion.to);
                    }
                    else if edge_insertion.to == a && !self.base_graph.is_directed() {
                        additions.insert(edge_insertion.from, ());
                        invalidations.remove(&edge_insertion.from);
                    }

                },
                GraphDelta::NodeInsertion(node_insertion) => (),
                GraphDelta::NodeUpdate(node_update) => (),
                GraphDelta::NodeRemoval(node_removal) => {
                    invalidations.insert(node_removal.index, ());
                },
                GraphDelta::EdgeUpdate(edge_update) => (),
                GraphDelta::EdgeRemoval(edge_removal) => {
                    if(edge_removal.from == a){
                        invalidations.insert(edge_removal.to, ());
                        additions.remove(&edge_removal.to);
                    }
                    else if(edge_removal.to == a && !self.base_graph.is_directed()){
                        invalidations.insert(edge_removal.from, ());
                        additions.remove(&edge_removal.from);
                    }

                },
            }
        }

        Self::Neighbors{
            base_edges,
            invalidations,
            additions,
            _phantom_ref: PhantomData,
        }
    }
}