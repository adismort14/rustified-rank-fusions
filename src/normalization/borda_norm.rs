// use num_traits::Float;
use rayon::prelude::*;

use crate::data_struct::{push_and_sort, DocRankingVec, Run, Runs};

// Make it use only the ones we need
// use crate::common::*;

fn get_candidates(runs: &Runs) -> Vec<Vec<String>> {
    let mut candidates = Vec::new();

    if let Some(first_run) = runs.runs.get(0) {
        for q_id in first_run.qid_mapping.keys() {
            let mut new_candidates = Vec::new();
            for run in &runs.runs {
                if let Some(doc_ranking_vec) = run.get_doc_ranking(q_id) {
                    for (doc_id, _) in doc_ranking_vec {
                        new_candidates.push(doc_id.clone());
                    }
                }
            }

            if !new_candidates.is_empty() {
                candidates.push(new_candidates);
            } else {
                // Fixes the case when no runs have docs for a given query
                candidates.push(Vec::new());
            }
        }
    }

    candidates
}

fn _borda_norm(results: &DocRankingVec, candidates_inner_vec: &Vec<String>) -> DocRankingVec {
    let doc_ids: Vec<_> = results.iter().map(|(doc_id, _)| doc_id.clone()).collect();
    let n_results = results.len();
    let n_candidates = candidates_inner_vec.len();

    let mut normalized_results = DocRankingVec::new();

    for doc_id in candidates_inner_vec {
        if let Some(index) = doc_ids.iter().position(|id| id == doc_id) {
            push_and_sort(&mut normalized_results,doc_id.clone(), 1.0 - (index as f64 / n_candidates as f64));
        } else {
            push_and_sort(&mut normalized_results,doc_id.clone(), 0.5 - ((n_results - 1) as f64 / (2.0 * n_candidates as f64)));
        }
    }

    normalized_results
}

fn _borda_norm_parallel(run: &Run, candidates: &Vec<Vec<String>>) -> Run {
    let q_ids: Vec<_> = run.qid_mapping.keys().cloned().collect();
    let normalized_run = q_ids
    .par_iter()
    .map(|q_id| {
        let q_index = q_ids.iter().position(|id| *id == *q_id).unwrap();
        let normalized_results = _borda_norm(run.qid_mapping.get(q_id).unwrap(), &candidates[q_index]);
        (q_id.clone(), normalized_results)
    })
    .collect();


    Run {
        qid_mapping: normalized_run,
    }
}


pub fn borda_norm(runs: &Runs) -> Runs {
    let candidates = get_candidates(runs);

    let mut normalized_runs = Runs::new();
    for run in &runs.runs {
        let normalized_run = _borda_norm_parallel(&run, &candidates);
        normalized_runs.runs.push(normalized_run);
    }

    normalized_runs
}

