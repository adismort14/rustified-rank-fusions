use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};

use crate::utils;
use utils::{f64_to_string, string_to_f64};

use crate::data_struct;
use data_struct::{Run,Runs};

// fn _comb_sum(results: Vec<BTreeMap<String, f64>>) -> BTreeMap<String, f64> {
//     let mut combined_results = create_empty_results_dict();

//     for res in &results {
//         for doc_id in res.keys() {
//             let doc_id = to_unicode(doc_id);
//             if !combined_results.contains_key(&doc_id) {
//                 let sum: f64 = results.iter().map(|res| res.get(&doc_id).unwrap_or(&0.0)).sum();
//                 combined_results.insert(doc_id.clone(), sum);
//             }
//         }
//     }

//     combined_results
// }

fn create_empty_results_dict() -> BTreeMap<String, String> {
    BTreeMap::new()
}

// fn _comb_sum(results: Vec<BTreeMap<String, String>>) -> BTreeMap<String, String> {
//     let mut combined_results: BTreeMap<String, String> = create_empty_results_dict();
//
//     for res in results.iter() {
//         for (_, doc_id) in res.iter() {
//             if !combined_results.values().any(|v| v == doc_id) {
//                 combined_results.insert(
//                     results.iter().map(|r| r.get(doc_id).cloned().unwrap_or_default()).sum(),
//                     doc_id.clone(),
//                 );
//             }
//         }
//     }
//
//     combined_results
// }

fn _comb_sum(results: Vec<BTreeMap<String, String>>) -> BTreeMap<String, String> {
    let mut combined_results: BTreeMap<String, String> = create_empty_results_dict();

    // Calculate the sum of values for each doc_id across all maps in results
    for doc_id in results.iter().flat_map(|r| r.keys()).cloned() {
        // Calculate the sum of values for the current doc_id
        let sum = results.iter()
            .filter_map(|r| r.get(&doc_id))  // Filter maps to get only values for the current doc_id
            .filter_map(|v| v.parse::<f64>().ok())  // Convert values to f64
            .sum::<f64>();  // Sum up the values
        
        // Insert the doc_id and its corresponding sum into combined_results
        combined_results.entry(doc_id.clone()).or_insert(f64_to_string(sum));
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

fn _comb_sum_parallel(combined_run: &mut Run, modified_runs: &Runs) {
    // Assuming Run struct has a data field of type BTreeMap<String, BTreeMap<String, String>>
    let q_ids: Vec<&String> = modified_runs.runs[0].q_rank_map.keys().collect();

    //TODO: Make this parallel
    for i in 0..q_ids.len(){
        let mut q_id=q_ids[i];
        
    }
    // for (q_id, inner_map) in &run.q_rank_map {
    //     let combined_inner_map = combined_run.q_rank_map.entry(q_id.clone()).or_insert_with(BTreeMap::new);
    //     for (value, doc_id) in inner_map {
    //         let f64_value: f64 = value.parse().unwrap_or(0.0);
    //         *combined_inner_map.entry(f64_value.to_string()).or_insert_with(|| doc_id.clone()) = doc_id.clone();
    //     }
    // }
}

pub fn comb_sum(runs: &Runs) -> Run {
    let mut combined_run = Run::new();
    _comb_sum_parallel(&mut combined_run,runs);
    // The sort function won't be required as it will be already sorted.
    // combined_run.sort();
    combined_run
}