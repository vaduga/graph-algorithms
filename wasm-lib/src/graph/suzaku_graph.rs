use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::prelude::*;
//use web_sys::js_sys::{Int32Array, Uint32Array};


use hypergraph::{Hypergraph}; //HyperedgeIndex, VertexIndex
use std::fmt::{Display, Formatter};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Person {
    id: usize,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Person {}", self.id)
    }
}

#[wasm_bindgen]
impl Person {
    pub fn new(id: usize) -> Person {
        Person { id }
    }
    pub fn id(&self) -> usize {
        self.id
    }
    // Method to get the display string for the person
    pub fn to_string(&self) -> String {
        format!("Person {}", self.id)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Relation {
    id: usize,
}

impl Into<usize> for Relation {
    fn into(self) -> usize {
        self.id
    }
}

#[wasm_bindgen]
impl Relation {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn to_string(&self) -> String {
        format!("Relation {}", self.id)
    }
}

impl Display for Relation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Relation {}", self.id)
    }
}


#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct PersonRecord {
    name: String,
    value: i32,
}

#[wasm_bindgen]
impl PersonRecord {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct VertexWithRecord {
    new_node: Person,
    record: PersonRecord,
}

#[wasm_bindgen]
impl VertexWithRecord {
    #[wasm_bindgen(getter)]
    pub fn new_node(&self) -> Person {
        self.new_node.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn record(&self) -> PersonRecord {
        self.record.clone()
    }
}


#[wasm_bindgen]
pub struct GraphWrapper {
    graph: Hypergraph<Person, Relation>,
    people: HashMap<usize, (String, i32)>,
    relations: HashMap<usize, (String, usize)>,
    next_person_id: usize,
    next_relation_id: usize,
}

impl GraphWrapper {
    // Method to return all vertices
    pub fn _all_vertices(&self) -> Vec<String> {
        self.people.values().map(|(name, _)| name.clone()).collect()
    }

}

#[wasm_bindgen]
impl GraphWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GraphWrapper {
        let graph = Hypergraph::<Person, Relation>::new();
        GraphWrapper {
            graph,
            people: HashMap::new(),
            relations: HashMap::new(),
            next_person_id: 0,
            next_relation_id: 0,
        }
    }

    // Create a vertex
    pub fn create_vertex(&mut self, name: String, value: i32) -> VertexWithRecord {
        let id = self.next_person_id;
        self.next_person_id += 1;
        let new_node = Person { id };
        self.graph.add_vertex(new_node);
        self.people.insert(id, (name.clone().into(), value));

        let record_tuple = self.people.get(&id).unwrap().clone();
        let record = PersonRecord {
            name: record_tuple.0.into(),
            value: record_tuple.1,
        };


        VertexWithRecord {
            new_node,
            record,
        }
    }

    // pub fn to_string(&self) -> String {
    //     let mut text = String::new();
    //
    //     for node in self.graph.node_indices() {
    //         let node_data = self.graph.node_weight(node).unwrap();
    //         text.push_str(&format!("Node {}: {:?}\n", node.index(), node_data));
    //     }
    //
    //     for edge in self.graph.edge_indices() {
    //         let (source, destination) = self.graph.edge_endpoints(edge).unwrap();
    //         text.push_str(&format!("Edge {} -> {}\n", source.index(), destination.index()));
    //     }
    //
    //     text
    // }

    // pub fn len(&self) -> u32 {
    //     self.graph.node_count() as u32
    // }

    // Return MyVertexData instead of usize


    // pub fn neighbors(&self, node_id: usize) -> Uint32Array {
    //     let node_index = NodeIndex::<u32>::new(node_id);
    //
    //     let node_indexes: Vec<u32> = self
    //         ._neighbors(node_index)
    //         .map(|node| node.index() as u32)
    //         .collect();
    //
    //     Uint32Array::from(&node_indexes[..])
    // }

    // pub fn delete_vertex(&mut self, node_id: usize) {
    //     let node_index = NodeIndex::<u32>::new(node_id);
    //     self.graph.remove_node(node_index);
    // }
    //
    // pub fn delete_edge(&mut self, edge_id: usize) {
    //     let edge_index = petgraph::graph::EdgeIndex::<u32>::new(edge_id);
    //     self.graph.remove_edge(edge_index);
    // }
    //
    // pub fn edge(
    //     &self,
    //     first_node_id: usize,
    //     second_node_id: usize,
    // ) -> Result<Option<u32>, String> {
    //     let first_node = NodeIndex::<u32>::new(first_node_id);
    //     let second_node = NodeIndex::<u32>::new(second_node_id);
    //
    //     let first_to_second = self
    //         .graph
    //         .edges_connecting(first_node, second_node)
    //         .map(|edge| edge.id().index() as u32);
    //
    //     let second_to_first = self
    //         .graph
    //         .edges_connecting(second_node, first_node)
    //         .map(|edge| edge.id().index() as u32);
    //
    //     let all_edges: Vec<u32> = first_to_second.chain(second_to_first).collect();
    //
    //     if all_edges.len() > 1 {
    //         return Err(format!(
    //             "An error was logged because there exists more than one edge between {first_node_id} and {second_node_id}"
    //         ));
    //     }
    //
    //     if all_edges.is_empty() {
    //         return Ok(None);
    //     }
    //
    //     Ok(Some(all_edges[0]))
    // }

    // pub fn edge_directed(
    //     &self,
    //     first_node_id: usize,
    //     second_node_id: usize,
    // ) -> Result<Option<u32>, String> {
    //     let first_node = NodeIndex::<u32>::new(first_node_id);
    //     let second_node = NodeIndex::<u32>::new(second_node_id);
    //
    //     let edges: Vec<u32> = self
    //         .graph
    //         .edges_connecting(first_node, second_node)
    //         .map(|edge| edge.id().index() as u32)
    //         .collect();
    //
    //     if edges.len() > 1 {
    //         return Err(format!(
    //             "An error was logged because there exists more than one edge between {first_node_id} and {second_node_id}"
    //         ));
    //     }
    //
    //     if edges.is_empty() {
    //         return Ok(None);
    //     }
    //
    //     Ok(Some(edges[0]))
    // }
    //
    // pub fn adjacent_edges(&self, node_id: usize) -> Int32Array {
    //     let node_index = NodeIndex::<u32>::new(node_id);
    //
    //     let outgoing_edges = self
    //         .graph
    //         .edges_directed(node_index, Incoming)
    //         .map(|edge| edge.id().index() as i32);
    //
    //     let edge_indexes = self
    //         .graph
    //         .edges_directed(node_index, Outgoing)
    //         .map(|edge| edge.id().index() as i32);
    //
    //     let all_edges: Vec<i32> = outgoing_edges.chain(edge_indexes).collect();
    //
    //     Int32Array::from(&all_edges[..])
    // }
    //
    // pub fn create_edge(
    //     &mut self,
    //     source_node_id: usize,
    //     destination_node_id: usize,
    //     weight: Option<u32>,
    // ) -> Result<usize, String> {
    //     let source_node_index = NodeIndex::<u32>::new(source_node_id);
    //     let destination_node_index = NodeIndex::<u32>::new(destination_node_id);
    //
    //     let new_edge = self.graph.update_edge(
    //         source_node_index,
    //         destination_node_index,
    //         weight.unwrap_or(1),
    //     );
    //
    //     Ok(new_edge.index())
    // }
}
