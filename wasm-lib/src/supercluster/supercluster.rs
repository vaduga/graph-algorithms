// supercluster

use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use supercluster_rs::{Supercluster, ClusterInfo as ExternalClusterInfo, ClusterId, SuperclusterBuilder, SuperclusterOptions};
use crate::graph::suzaku_graph::GraphWrapper;

// Assuming you already have this defined
#[derive(Serialize, Clone)]
pub struct ClusterInfoWrapper {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub cluster: bool,
    pub point_count: usize,
}

impl ClusterInfoWrapper {
    pub fn from_external(cluster_info: ExternalClusterInfo) -> Self {
        Self {
            id: cluster_info.id().as_usize(),  // Convert ClusterId to usize
            x: cluster_info.x(),
            y: cluster_info.y(),
            cluster: cluster_info.is_cluster(),
            point_count: cluster_info.count(),
        }
    }
}

#[wasm_bindgen]
pub struct SuperclusterWrapper {
    supercluster: Supercluster,
}

#[wasm_bindgen]
impl SuperclusterWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(graph: &GraphWrapper, max_zoom: usize, radius: f64) -> SuperclusterWrapper {
        let coords = graph.load_places();
        let options = SuperclusterOptions {
            max_zoom,
            radius,
            ..Default::default()
        };
        let mut builder = SuperclusterBuilder::new_with_options(coords.len(), options);
        for coord in coords {
            builder.add(coord[0], coord[1]);
        }
        let supercluster = builder.finish();

        SuperclusterWrapper { supercluster }
    }

    #[wasm_bindgen]
    pub fn get_clusters(&self, min_lng: f64, min_lat: f64, max_lng: f64, max_lat: f64, zoom: usize) -> JsValue {
        let external_clusters = self.supercluster.get_clusters(min_lng, min_lat, max_lng, max_lat, zoom);

        let wrappers: Vec<ClusterInfoWrapper> = external_clusters.into_iter()
            .map(ClusterInfoWrapper::from_external)
            .collect();

        to_value(&wrappers).unwrap()
    }

    // #[wasm_bindgen]
    // pub fn get_cluster_expansion_zoom(&self, cluster_id: ClusterId) -> Result<usize, JsValue> {
    //     let mut cluster_id = cluster_id;
    //     let mut expansion_zoom = cluster_id.get_origin_zoom(self.points.len()) - 1;
    //     while expansion_zoom <= self.options.max_zoom {
    //         let children = self.supercluster.get_children(cluster_id).map_err(|e| JsValue::from_str(&e.to_string()))?;
    //         expansion_zoom += 1;
    //         if children.len() != 1 {
    //             break;
    //         }
    //         cluster_id = children[0].id();
    //     }
    //
    //     Ok(expansion_zoom)
    // }
}
