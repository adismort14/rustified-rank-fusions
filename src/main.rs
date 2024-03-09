// use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};

mod utils;
use utils::{f64_to_string, string_to_f64};

mod data_struct;
use data_struct::{Run, Runs, DocRankingVec, push_and_sort};

// mod comb_sum;
// use comb_sum::{comb_sum};

// Single run is of the form as below:
// run_hashmap = {
//     "q_1": {
//         "d_1": 1.5,
//         "d_2": 2.6,
//     },
//     "q_2": {
//         "d_3": 2.8,
//         "d_2": 1.2,
//         "d_5": 3.1,
//     },
// }

// The RRF function needs to take a list of runs and output a single run.

// The first task is to sort all the Run inside the Runs.
// Done using BTrees.
// nvm. it will sort based on the keys and not the values as intended. Will use HashMap to reduce the TC and make a custom function for sorting.
// The data structure `Run` is of the form => Run: HashMap<String,HashMap<String,f64>> ; OUTDATED
// How did I forget that HashMaps are inherently unordered. So, even if I insert in a sorted order, they might not return the same order.
// I am again thinking of using BTreeMaps and just switch the keys and the values.
// This is the final structure of a run => Run: BTreeMap<String,BTreeMap<f64, String>> ;
// Ok, turns out f64 does not have the Ord trait..ARGHHHH
// Another way I could come up with is round these f64 to 2 decimal places and then convert to string. Strings can be ordered. I am a genius hehehe.

// Takes a single query of a single run and calculates the ranks of the docs.


// Takes the doc ranking for a single query and modifies the scores using reciprocal ranking and returns the hashmap of (reciprocal_score_as_string, doc_id_string)
fn rrf_score(single_query: &DocRankingVec, k: usize) -> DocRankingVec {
    let mut ind_computed_rank: DocRankingVec = DocRankingVec::new();
    let mut reciprocal_rank: f64 = 0.0;
    for (i, (doc_id,_)) in single_query.iter().enumerate() {
        reciprocal_rank = 1.0 / ((i as f64) + 1.0 + (k as f64));
        push_and_sort(&mut ind_computed_rank,doc_id.clone(), reciprocal_rank);
    }

    ind_computed_rank
}

// Takes a single run and calculates the ranks of all the documents in all the queries. This is currently the main function to call
fn rrf_score_parallel(run_object: &Run, k: usize) -> Run {
    let combined_result: BTreeMap<String, DocRankingVec> = run_object
        .qid_mapping
        .iter()
        .map(|(q_id, single_query)| ( q_id.clone(),rrf_score(single_query, k)))
        .collect();

    Run {
        qid_mapping: combined_result,
    }
}

// Another issue is that there is no way to initialse a Vector with a certain size with all the elements of some value of Type run.

// This is the public function which takes Runs and k as the input and returns the final combined Run.
pub fn rrf(runs_object: Runs, k: usize) -> Run {
    let mut runs_object_returned: Runs = Runs::new_with_cap(runs_object.len());
    for runInstance in runs_object.runs.iter() {
        let temp_run = rrf_score_parallel(runInstance, k);
        runs_object_returned.runs.push(temp_run);
    }
    return comb_sum(&mut runs_object_returned);
}

fn main() {
    // Dummy data for testing with a single run
    let run_hashmap: HashMap<String, HashMap<String, f64>> = [
        (
            String::from("q_1"),
            [("d_1", 1.5), ("d_2", 2.6)]
                .iter()
                .map(|(k, v)| (String::from(*k), *v))
                .collect(),
        ),
        (
            String::from("q_2"),
            [("d_3", 2.8), ("d_2", 1.2), ("d_5", 3.1)]
                .iter()
                .map(|(k, v)| (String::from(*k), *v))
                .collect(),
        ),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<String, HashMap<String, f64>>>();

    // Best empirical and default value of k according to the research paper
    let k = 60;

    let combined_results = rrf_score_parallel(&run_hashmap, k);

    print!("{:?}", combined_results);
}
