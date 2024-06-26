// use num_traits::Float;
use rayon::prelude::*;

use crate::normalization::common::{extract_scores, vec_mean, vec_stdev};
use crate::data_struct::{push_and_sort, DocRankingVec, Run};

// Make it use only the ones we need
// use crate::common::*;

// LOW LEVEL FUNCTIONS ========================================================
fn _zmuv_norm(results: &DocRankingVec) -> DocRankingVec {
    let scores = extract_scores(results);
    let mean_score = vec_mean(&scores);
    let stdev_score = vec_stdev(&scores);

    let denominator = stdev_score.max(1e-9);

    let mut normalized_results = DocRankingVec::new();

    for (doc_id, score) in results {
        let normalized_score = (score.clone() - mean_score) / denominator;
        push_and_sort(&mut normalized_results, doc_id.clone(), normalized_score);
    }

    normalized_results
}

fn _zmuv_norm_parallel(run: &Run) -> Run {
    let q_ids: Vec<_> = run.qid_mapping.keys().cloned().collect();
    let normalized_run = q_ids
        .par_iter()
        .map(|q_id| {
            let normalized_results = _zmuv_norm(run.qid_mapping.get(q_id).unwrap());
            (q_id.clone(), normalized_results)
        })
        .collect();

    Run {
        qid_mapping: normalized_run,
    }
}

// HIGH LEVEL FUNCTIONS =======================================================
pub fn zmuv_norm(run: &Run) -> Run {
    let normalized_run = _zmuv_norm_parallel(run);
    normalized_run
}
