use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};

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

// Takes a single query of a single run and calculates the ranks of the docs.

#[derive(Debug)]
struct Run {
    data: BTreeMap<String, BTreeMap<f64, String>>,
}

impl Run {
    fn new() -> Self {
        Run {
            data: BTreeMap::new(),
        }
    }

    fn insert(&mut self, outer_key: String, mut sorted_inner_data: BTreeMap<f64, String>) {
        self.data.insert(outer_key, sorted_inner_data);
    }
}

// Runs = List[Run]

#[derive(Debug)]
struct Runs {
    runs: Vec<Run>,
}

impl Runs {
    fn new() -> Self {
        Runs { runs: Vec::new() }
    }

    fn insert(&mut self, insert_run: Run){
        self.runs.push(insert_run);
    }
}

fn rrf_score(single_query: &HashMap<String, f64>, k: usize) -> HashMap<String, f64> {
    let mut ind_computed_rank: HashMap<String, f64> = HashMap::new();
    for (i, (doc_id, _)) in single_query.iter().enumerate() {
        ind_computed_rank.insert(doc_id.clone(), 1.0 / ((i as f64) + 1.0 + (k as f64)));
    }

    ind_computed_rank
}

// Takes a single run and calculates the ranks of all the documents in all the queries. This is currently the main function to call
fn rrf_score_parallel(
    run_object: &Run,
    k: usize,
) -> Run {
    let combined_result: Run = run_object.data
        .par_iter()
        .map(|(q_id, single_query)| (q_id.clone(), rrf_score(single_query, k))).collect();

    combined_result
}

fn rrf(runs_object:Runs, k:usize)-> Run {
    let dummy_run=Run::new();
    let mut runs_object_returned:Runs=Runs::new();
    for (i,runInstance) in runs_object.runs.iter().enumerate(){
        let mut temp_run=Run::new();
        temp_run.data=rrf_score_parallel(runInstance.data, k);
        runs_object_returned.runs[i]=temp_run;
    }
    // return comb_sum(runs_object_returned);
    dummy_run
}

fn comb_sum_parallel(run: HashMap<String, HashMap<String, f64>>) {}

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
