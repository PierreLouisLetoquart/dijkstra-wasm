use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use std::collections::{BTreeMap, BTreeSet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct GraphWrapper {
    graph: BTreeMap<u32, BTreeMap<u32, OrderedFloat<f64>>>,
}

#[derive(Serialize, Deserialize)]
pub struct DijkstraResult {
    vertex: u32,
    predecessor: Option<PredecessorInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct PredecessorInfo {
    predecessor: u32,
    distance: f64,
}

#[wasm_bindgen]
impl GraphWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            graph: BTreeMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_edge(&mut self, from: u32, to: u32, weight: f64) {
        let ordered_weight = OrderedFloat(weight); // Convert f64 to OrderedFloat
        self.graph.entry(from).or_insert_with(BTreeMap::new);
        self.graph.entry(to).or_insert_with(BTreeMap::new);
        self.graph
            .get_mut(&from)
            .unwrap()
            .insert(to, ordered_weight);
    }

    #[wasm_bindgen]
    pub fn dijkstra(&self, start: u32) -> JsValue {
        let mut ans = BTreeMap::new();
        let mut prio = BTreeSet::new();

        ans.insert(start, None);

        if let Some(start_edges) = self.graph.get(&start) {
            for (new, weight) in start_edges {
                ans.insert(*new, Some((start, *weight)));
                prio.insert((*weight, *new));
            }
        }

        while let Some((path_weight, vertex)) = prio.pop_first() {
            if let Some(vertex_edges) = self.graph.get(&vertex) {
                for (next, weight) in vertex_edges {
                    let new_weight = path_weight + *weight; // OrderedFloat supports Add
                    match ans.get(next) {
                        Some(Some((_, dist_next))) if new_weight >= *dist_next => {}
                        Some(None) => {}
                        _ => {
                            if let Some(Some((_, prev_weight))) =
                                ans.insert(*next, Some((vertex, new_weight)))
                            {
                                prio.remove(&(prev_weight, *next));
                            }
                            prio.insert((new_weight, *next));
                        }
                    }
                }
            }
        }

        let serializable_result: Vec<DijkstraResult> = ans
            .into_iter()
            .map(|(vertex, predecessor)| DijkstraResult {
                vertex,
                predecessor: predecessor.map(|(pred_vertex, dist)| PredecessorInfo {
                    predecessor: pred_vertex,
                    distance: dist.into_inner(), // Convert back to f64
                }),
            })
            .collect();

        serde_wasm_bindgen::to_value(&serializable_result).unwrap_or(JsValue::NULL)
    }
}
