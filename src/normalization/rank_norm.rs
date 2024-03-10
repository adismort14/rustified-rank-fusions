// use num_traits::Float;
use rayon::prelude::*;

use crate::data_struct::{push_and_sort, DocRankingVec, Run};

// Make it use only the ones we need
// use crate::common::*;

// LOW LEVEL FUNCTIONS ========================================================
fn _rank_norm(results: &DocRankingVec) -> DocRankingVec {

    let denominator = results.len();
    let mut normalized_results = DocRankingVec::new();

    for (i,rank_tuple) in results.iter().enumerate() {
        let doc_id=rank_tuple.0.clone();
        let normalized_score = 1.0 - (i as f64/ (denominator as f64));

        // TODO: ideally there should not be any need for push_and_sort in these normalizations. Verify once and if not required, remove it as it is adding unnecessary TC
        push_and_sort(&mut normalized_results, doc_id, normalized_score);
    }

    normalized_results
}

fn _rank_norm_parallel(run: &Run) -> Run {
    let q_ids: Vec<_> = run.qid_mapping.keys().cloned().collect();
    let normalized_run = q_ids
        .par_iter()
        .map(|q_id| {
            let normalized_results = _rank_norm(run.qid_mapping.get(q_id).unwrap());
            (q_id.clone(), normalized_results)
        })
        .collect();

    Run {
        qid_mapping: normalized_run,
    }
}

// HIGH LEVEL FUNCTIONS =======================================================
pub fn rank_norm(run: &Run) -> Run {
    let normalized_run = _rank_norm_parallel(run);
    normalized_run
}
