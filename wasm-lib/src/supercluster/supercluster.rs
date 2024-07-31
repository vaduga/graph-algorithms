// supercluster

use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use supercluster_rs::{Supercluster, ClusterInfo as ExternalClusterInfo, ClusterId, SuperclusterBuilder, SuperclusterOptions, ClusterInfo};
use crate::graph::suzaku_graph::GraphWrapper;
use geo::{Centroid, ConvexHull, CoordsIter};
use gloo_console::log;

use geo_types::{MultiPoint, Point, Polygon};
use js_sys::Array;

#[derive(Serialize)]
pub struct GeometryResult {
    pub centroid: CenterCoordinates,
    pub convex_hull: Vec<Vec<f64>>,
}



#[derive(Serialize)]
pub struct CenterCoordinates {
    pub center_x: f64,
    pub center_y: f64,
}



#[derive(Serialize, Clone)]
pub struct ClusterInfoWrapper {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub cluster: bool,
    pub point_count: usize,
    pub get_origin_idx: usize,
    pub get_origin_zoom: usize
}

impl ClusterInfoWrapper {
    pub fn from_external(cluster_info: ExternalClusterInfo) -> Self {
        Self {
            id: cluster_info.id().as_usize(),  // Convert ClusterId to usize
            x: cluster_info.x(),
            y: cluster_info.y(),
            cluster: cluster_info.is_cluster(),
            get_origin_idx: cluster_info.id().get_origin_idx(0),
            get_origin_zoom:  cluster_info.id().get_origin_zoom(0),
            point_count: cluster_info.count(),
        }
    }
}



#[wasm_bindgen]
pub struct SuperclusterWrapper {
    supercluster: Supercluster,
    points: Vec<(f64, f64)>,
    options: SuperclusterOptions
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

        SuperclusterWrapper { supercluster, points: vec![], options: Default::default() }
    }

    #[wasm_bindgen]
    pub fn get_clusters(&self, min_lng: f64, min_lat: f64, max_lng: f64, max_lat: f64, zoom: usize) -> JsValue {
        let external_clusters = self.supercluster.get_clusters(min_lng, min_lat, max_lng, max_lat, zoom);

        let wrappers: Vec<ClusterInfoWrapper> = external_clusters.into_iter()
            .map(ClusterInfoWrapper::from_external)
            .collect();

        to_value(&wrappers).unwrap()
    }



    #[wasm_bindgen]
    pub fn get_cluster_expansion_zoom(&self, cluster_id: usize) -> Result<usize, JsValue> {
        // Convert usize to ClusterId
        let cl_id = ClusterId::new_source_id(cluster_id);

        match self.supercluster.get_cluster_expansion_zoom(    cl_id   ) {
            Ok(exp_zoom) => Ok(exp_zoom),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
        
    }

    #[wasm_bindgen]
    pub fn get_expand_coords(&self, cluster_id: usize, origin_idx: usize, origin_zoom: usize) -> JsValue {
        // Convert usize to ClusterId

        let cl_id = ClusterId::new_source_id(cluster_id);


        // Fetch leaves with a default limit of usize::MAX (infinity) and offset of 0
        match self.supercluster.get_leaves(cl_id, Some(usize::MAX), Some(0)) {
            Ok(leaves) => {
                // Convert ClusterInfo to geo::Point
                let points: Vec<Point<f64>> = leaves.into_iter()
                    .map(|info| Point::new(info.x(), info.y()))
                    .collect();

                // Create MultiPoint from the points
                let multi_point = MultiPoint::from(points.clone());

                // Compute the convex hull
                let convex_hull: Polygon<f64> = multi_point.convex_hull();
                let hull_coords: Vec<Vec<f64>> = convex_hull.exterior().coords_iter()
                    .map(|coord| vec![coord.x, coord.y])
                    .collect();


                // Log the convex hull coordinates for debugging
                log!(&format!("Convex Hull Coordinates: {:?}", hull_coords));


                // Compute the centroid
                let centroid = multi_point.centroid().unwrap_or(Point::new(0.0, 0.0));
                let center_coords = CenterCoordinates {
                    center_x: centroid.x(),
                    center_y: centroid.y(),
                };



                // Create the result struct
                let result = GeometryResult {
                    centroid: center_coords,
                    convex_hull: hull_coords,
                };

                // Convert GeometryResult to JsValue
                to_value(&result).unwrap()
            }
            Err(e) => JsValue::from_str(&e.to_string()),
        }
    }

    #[wasm_bindgen]
    pub fn get_children_cluster_ids(&self, cluster_id: usize) -> JsValue {
        // Convert usize to ClusterId
        let cl_id = ClusterId::new_source_id(cluster_id);

        // Vector to collect cluster IDs
        let mut cluster_ids: Vec<usize> = Vec::new();

        // Recursive function to collect cluster IDs
        fn collect_cluster_ids(wrapper: &SuperclusterWrapper, cl_id: ClusterId, cluster_ids: &mut Vec<usize>) {
            if let Ok(children) = wrapper.supercluster.get_children(cl_id) {
                for info in children {
                    if info.is_cluster() {
                        cluster_ids.push(info.id().as_usize());
                        // Recursively collect children of this cluster
                        collect_cluster_ids(wrapper, ClusterId::new_source_id(info.id().as_usize()), cluster_ids);
                    }
                }
            }
        }

        // Start collecting cluster IDs from the initial cluster
        collect_cluster_ids(self, cl_id, &mut cluster_ids);

        // Convert Vec<usize> to js_sys::Array
        let js_array = Array::new();
        for id in cluster_ids {
            js_array.push(&JsValue::from_f64(id as f64));
        }

        js_array.into()
    }


}






