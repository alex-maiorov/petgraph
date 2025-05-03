
pub struct NodeInsertion<N, Ix>{
    value: N,
    index: Ix
}

pub struct NodeUpdate<N, Ix>{
    new_value: N,
    index: Ix
}

pub struct NodeRemoval<N, Ix>{
    old_value: N,
    index: Ix,
}

pub struct EdgeInsertion<E, NodeIx, EdgeIx>{
    value: E,
    index: EdgeIx,
    from: NodeIx,
    to: NodeIx
}

pub struct EdgeUpdate<E, Ix>{
    new_value: E,
    index: Ix,
}

pub struct EdgeRemoval<E, Ix>{
    old_value: E,
    index: Ix,
}

pub enum GraphDelta<N, E, NodeIx, EdgeIx>{
    NodeInsertion(NodeInsertion<N, NodeIx>),
    NodeUpdate(NodeUpdate<N, NodeIx>),
    NodeRemoval(NodeRemoval<N, NodeIx>),
    EdgeInsertion(EdgeInsertion<E, NodeIx, EdgeIx>),
    EdgeUpdate(EdgeUpdate<E, EdgeIx>),
    EdgeRemoval(EdgeRemoval<E, EdgeIx>)
}

pub struct BaseDeltaGraph<G, N, E, Ty = Directed, NodeIx = DefaultIx, EdgeIx = DefaultIx>{
    base_graph: G,
    deltas: Vec<GraphDelta<N, E, NodeIx, EdgeIx>>
}

impl<G, N, E, Ty, Ix> BaseDeltaGraph<G, N, E, Ty, Ix> {

}
