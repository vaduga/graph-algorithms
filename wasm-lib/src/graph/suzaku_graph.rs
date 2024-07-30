use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use serde::{Serialize, Deserialize};

use hypergraph::{Hypergraph, VertexIndex};
use std::fmt::{Display, Formatter};

use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};
use hypergraph::errors::HypergraphError;
use js_sys::{Array, Float64Array};

// Function to convert a Float64Array to Coords
fn js_array_to_coords(array: &Float64Array) -> Coords {
    let lon = array.get_index(0);
    let lat = array.get_index(1);
    Coords { lon, lat }
}

// Function to convert Coords to a Float64Array
fn coords_to_js_array(coords: &Coords) -> Float64Array {
    let array = Float64Array::new_with_length(2);
    array.set_index(0, coords.lon);
    array.set_index(1, coords.lat);
    array
}

// Function to convert HypergraphError to JsValue
fn convert_error_to_js_value(error: HypergraphError<Node, Relation>) -> JsValue {
    JsValue::from_str(&format!("Error adding vertex: {:?}", error))
}


// Define the Coords struct
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coords {
    pub lon: f64,
    pub lat: f64,
}

// Implement PartialEq for Coords
impl PartialEq for Coords {
    fn eq(&self, other: &Self) -> bool {
        self.lon.to_bits() == other.lon.to_bits() && self.lat.to_bits() == other.lat.to_bits()
    }
}

// Implement Eq for Coords
impl Eq for Coords {}

// Implement Hash for Coords
impl Hash for Coords {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lon.to_bits().hash(state);
        self.lat.to_bits().hash(state);
    }
}

// Define the Node struct
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Node {
    id: usize,
    coords: Coords,
    pub thr_id: Option<i32>,  // Use Option<i32> to handle undefined values
}

// Implement PartialEq for Node
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.coords == other.coords && self.thr_id == other.thr_id
    }
}

// Implement Eq for Node
impl Eq for Node {}

// Implement Hash for Node
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.coords.hash(state);
        self.thr_id.hash(state);
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Handle the Option<i32> with proper formatting
        write!(
            f,
            "Node {} {:?} {}",
            self.id,
            self.coords,
            match self.thr_id {
                Some(id) => id.to_string(),
                None => "No threshold ID".to_string(),
            }
        )
    }
}

#[wasm_bindgen]
impl Node {
    pub fn new(id: usize, coords: Coords, thr_id: Option<i32>) -> Node {
        Node { id, coords, thr_id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn to_string(&self) -> String {
        format!("Node {}", self.id)
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
pub struct GraphWrapper {
    graph: Hypergraph<Node, Relation>,
    people: HashMap<usize, (String, i32)>,
    relations: HashMap<usize, (String, usize)>
}

// Internal function to load vertex coordinates
impl GraphWrapper {
    pub fn load_places(&self) -> Vec<Vec<f64>> {
        let count = self.graph.count_vertices();
        let mut coords = Vec::with_capacity(count);

        for i in 0..count {
            let vertex_index = VertexIndex(i);
            match self.graph.get_vertex_weight(vertex_index) {
                Ok(node) => {
                    coords.push(vec![node.coords.lon, node.coords.lat]);
                },
                Err(_) => {
                    coords.push(vec![]); // Handle vertex retrieval failure gracefully
                }
            }
        }

        coords
    }
}


#[wasm_bindgen]
impl GraphWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GraphWrapper {
        let graph = Hypergraph::<Node, Relation>::new();
        GraphWrapper {
            graph,
            people: HashMap::new(),
            relations: HashMap::new(),
        }
    }

    // Create a vertex
    #[wasm_bindgen]
    pub fn create_vertex(&mut self, id: usize, coords_array: Float64Array, thr_id: Option<i32>) -> Result<u32, JsValue> {
        // Convert Float64Array to Coords
        let coords = js_array_to_coords(&coords_array);

        // Create a new Node with the given id, coordinates, and thread ID
        let temp_person = Node { id, coords, thr_id };

        // Try to add the vertex to the hypergraph
        match self.graph.add_vertex(temp_person) {
            Ok(vertex_index) => Ok(vertex_index.0 as u32), // Convert VertexIndex to u32
            Err(e) => Err(convert_error_to_js_value(e)),
        }
    }


    #[wasm_bindgen]
    pub fn get_vertex_weight(&self, vertex_index: u32) -> Result<JsValue, JsValue> {
        // Convert u32 to VertexIndex if needed
        let vertex_index = VertexIndex(vertex_index as usize);

        // Retrieve the vertex weight from the hypergraph
        match self.graph.get_vertex_weight(vertex_index) {
            Ok(weight) => {
                // Convert weight to JsValue
                let js_value = to_value(&weight).map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))?;
                Ok(js_value)
            },
            Err(e) => Err(JsValue::from_str(&format!("Error retrieving vertex: {:?}", e))),
        }
    }

    #[wasm_bindgen]
    pub fn get_all_vertex_coords(&self) -> JsValue {
        // Call the internal function to get the coordinates
        let coords_vec = self.load_places();

        // Convert Vec<Vec<f64>> to js_sys::Array
        let js_array = Array::new();
        for coords in coords_vec {
            let inner_array = Array::new();
            for value in coords {
                inner_array.push(&JsValue::from_f64(value));
            }
            js_array.push(&inner_array.into());
        }

        // Return the JavaScript array
        js_array.into()
    }



    #[wasm_bindgen]
    pub fn graph_clear(&mut self) -> Result<(), JsValue> {
        // Clear the hypergraph
        // Example: Assuming self.graph has a method `clear` or similar
        self.graph.clear(); // Replace with the actual method or logic to clear your hypergraph

        // Return Ok() to indicate success
        Ok(())
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
