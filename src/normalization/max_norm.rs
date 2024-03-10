// use num_traits::Float;
use rayon::prelude::*;

use crate::data_struct::{DocRankingVec, Run, push_and_sort};
use crate::common::{extract_scores,safe_max};

// Make it use only the ones we need
// use crate::common::*;


// LOW LEVEL FUNCTIONS ========================================================
fn _max_norm(results: &DocRankingVec) -> DocRankingVec {
    let scores: Vec<_> = extract_scores(results);
    let max_score = safe_max(&scores);

    let denominator = max_score.max(1e-9);

    let mut normalized_results = DocRankingVec::new();

    for (doc_id, score) in results {
        let normalized_score = score.clone() / denominator;
        push_and_sort(&mut normalized_results, doc_id.clone(), normalized_score);
    }

    normalized_results
}

fn _max_norm_parallel(run: &Run) -> Run {
    let q_ids: Vec<_> = run.qid_mapping.keys().cloned().collect();
    let normalized_run = q_ids
        .par_iter()
        .map(|q_id| {
            let normalized_results = _max_norm(run.qid_mapping.get(q_id).unwrap());
            (q_id.clone(), normalized_results)
        })
        .collect();

    Run {
        qid_mapping: normalized_run,
    }
}

// HIGH LEVEL FUNCTIONS =======================================================
pub fn max_norm(run: &Run) -> Run {
    let normalized_run = _max_norm_parallel(run);
    normalized_run
}
