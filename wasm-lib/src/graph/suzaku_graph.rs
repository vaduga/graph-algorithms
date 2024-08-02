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
use crate::supercluster::SuperclusterWrapper;
use gloo_console::log;
use hypergraph::iterator::HypergraphIterator;

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
    pub is_node: bool, // New field to indicate if it's a node or just a set of coordinates
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
    pub fn new(id: usize, coords: Coords, thr_id: Option<i32>, is_node: bool) -> Node {
        Node { id, coords, thr_id, is_node }
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

    relations: HashMap<usize, (String, usize)>
}

// Internal function to load vertex coordinates
impl GraphWrapper {
    pub fn load_places(&self) -> Vec<(f64, f64)> {

        let count = self.graph.count_vertices();
        let mut coords = Vec::with_capacity(count);

        for i in 0..count {
            let vertex_index = VertexIndex(i);
            match self.graph.get_vertex_weight(vertex_index) {
                Ok(node) => {
                    if node.is_node {
                        // Only push coordinates if `is_node` is true
                        coords.push((node.coords.lon, node.coords.lat)); // Assuming coords are (lon, lat)
                    } else {
                        // Optionally push an empty vector or handle non-node cases
                        // coords.push(vec![]);
                    }
                },
                Err(_) => {
                    //coords.push((vec![])); // Handle vertex retrieval failure gracefully
                }
            }
        }

        coords
    }


    pub fn load_threshold_ids(&self) -> Vec<usize> {

        let count = self.graph.count_vertices();
        let mut thr_ids = Vec::with_capacity(count);

        for i in 0..count {
            let vertex_index = VertexIndex(i);
            match self.graph.get_vertex_weight(vertex_index) {
                Ok(node) => {
                    if node.is_node {
                        if let Some(thr_id) = node.thr_id {
                            // Only push thr_id if `is_node` is true and thr_id is Some
                            thr_ids.push(thr_id as usize);
                        }
                    } else {

                    }
                },
                Err(_) => {
                    //coords.push((vec![])); // Handle vertex retrieval failure gracefully
                }
            }
        }

        thr_ids
    }



}


#[wasm_bindgen]
impl GraphWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GraphWrapper {
        let graph = Hypergraph::<Node, Relation>::new();

        GraphWrapper {
            graph,
            relations: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn create_supercluster(&self, max_zoom: usize, radius: f64) -> SuperclusterWrapper {
        log!("creating supercluster in rust");
        SuperclusterWrapper::new(self, max_zoom, radius)
    }

    // Create a vertex
    #[wasm_bindgen]
    pub fn create_vertex(&mut self, id: usize, coords_array: Float64Array, thr_id: Option<i32>, is_node: bool) -> Result<u32, JsValue> {
        // Convert Float64Array to Coords
        let coords = js_array_to_coords(&coords_array);

        // Create a new Node with the given id, coordinates, and thread ID
        let new_node = Node { id, coords, thr_id, is_node };

        // Try to add the vertex to the hypergraph
        match self.graph.add_vertex(new_node) {
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
        for (x, y) in coords_vec {
            let inner_array = Array::new();
            inner_array.push(&JsValue::from_f64(x));
            inner_array.push(&JsValue::from_f64(y));
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


}
