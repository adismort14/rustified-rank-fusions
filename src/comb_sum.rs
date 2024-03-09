use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};

use crate::utils;
use utils::{f64_to_string, string_to_f64};

use crate::data_struct;
use data_struct::{Run,Runs};

fn _comb_sum(results: Vec<BTreeMap<String, f64>>) -> BTreeMap<String, f64> {
    let mut combined_results = create_empty_results_dict();

    for res in &results {
        for doc_id in res.keys() {
            let doc_id = to_unicode(doc_id);
            if !combined_results.contains_key(&doc_id) {
                let sum: f64 = results.iter().map(|res| res.get(&doc_id).unwrap_or(&0.0)).sum();
                combined_results.insert(doc_id.clone(), sum);
            }
        }
    }

    combined_results
}

// fn comb_sum_parallel(combined_run: &mut Run, run: &Run) {
//     for (q_id, inner_map) in &run.data {
//         let combined_inner_map = combined_run.data.entry(q_id.clone()).or_insert_with(BTreeMap::new);
//         for (doc_id, value) in inner_map {
//             *combined_inner_map.entry(doc_id.clone()).or_insert(0.0) += value;
//         }
//     }
// }

fn _comb_sum_parallel(combined_run: &mut Run, runs: &Runs) {
    // Assuming Run struct has a data field of type BTreeMap<String, BTreeMap<String, f64>>
    for (q_id, inner_map) in &run.q_rank_map {
        let combined_inner_map = combined_run.q_rank_map.entry(q_id.clone()).or_insert_with(BTreeMap::new);
        for (value, doc_id) in inner_map {
            let f64_value: f64 = value.parse().unwrap_or(0.0);
            *combined_inner_map.entry(f64_value.to_string()).or_insert_with(|| doc_id.clone()) = doc_id.clone();
        }
    }
}

pub fn comb_sum(runs: &Runs) -> Run {
    let mut combined_run = Run::new();
    _comb_sum_parallel(&mut combined_run,runs);
    // The sort function won't be required as it will be already sorted.
    // combined_run.sort();
    combined_run
}