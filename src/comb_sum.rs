use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};

use crate::{data_struct::{push_and_sort, DocRankingVec}, utils};
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

// fn _comb_sum(results: Vec<BTreeMap<String, String>>) -> DocRankingVec {
//     let mut combined_results: BTreeMap<String, String> = create_empty_results_dict();

//     // Calculate the sum of values for each doc_id across all maps in results
//     for doc_id in results.iter().flat_map(|r| r.keys()).cloned() {
//         // Calculate the sum of values for the current doc_id
//         let sum = results.iter()
//             .filter_map(|r| r.get(&doc_id))  // Filter maps to get only values for the current doc_id
//             .filter_map(|v| v.parse::<f64>().ok())  // Convert values to f64
//             .sum::<f64>();  // Sum up the values
        
//         // Insert the doc_id and its corresponding sum into combined_results
//         combined_results.entry(doc_id.clone()).or_insert(f64_to_string(sum));
//     }

//     combined_results
// }

// fn comb_sum_parallel(combined_run: &mut Run, run: &Run) {
//     for (q_id, inner_map) in &run.data {
//         let combined_inner_map = combined_run.data.entry(q_id.clone()).or_insert_with(BTreeMap::new);
//         for (doc_id, value) in inner_map {
//             *combined_inner_map.entry(doc_id.clone()).or_insert(0.0) += value;
//         }
//     }
// }

fn _comb_sum_parallel(combined_output_run: &mut Run, input_runs: &Runs) {
    // // Assuming Run struct has a data field of type BTreeMap<String, Vec<(doc_id,rank)>
    // let q_ids: Vec<&String> = input_runs.runs[0].qid_mapping.keys().collect();

    // //TODO: Make this parallel
    // for i in 0..q_ids.len(){
    //     let mut q_id=q_ids[i];
    //     for j in 0..input_runs.len(){
            
    //     }
    //     combined_output_run.qid_mapping.insert(q_id.clone(), _comb_sum(input_runs));
    // }
    for (q_id, _) in &input_runs.runs[0].qid_mapping {
        let mut doc_ranking_vec = Vec::new();
        
        // Iterate over each document ID for the current query ID
        for (doc_id, _) in input_runs.runs[0].qid_mapping.get(q_id).unwrap() {
            let mut rank_sum = 0.0;

            // Iterate over each Run instance to sum ranks for the current document ID
            for run in &input_runs.runs {
                // If the current Run instance contains the document ID, sum its rank
                if let Some(doc_ranking) = run.qid_mapping.get(q_id) {
                    if let Some(rank) = doc_ranking.iter().find(|(id, _)| id == doc_id).map(|(_, rank)| rank) {
                        rank_sum += *rank;
                    }
                }
            }
            
            // Push the summed rank for the current document ID into the vector
            push_and_sort(&mut doc_ranking_vec,doc_id.clone(), rank_sum);
        }
        
        // Insert the vector of summed ranks into the final Run instance
        combined_output_run.qid_mapping.insert(q_id.clone(), doc_ranking_vec);
    }
}

pub fn comb_sum(runs: &Runs) -> Run {
    let mut combined_run = Run::new();
    _comb_sum_parallel(&mut combined_run,runs);
    // No extra sort function will be required as it will be already sorted.
    combined_run
}