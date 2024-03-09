use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};

mod utils;
use utils::{string_to_f64,f64_to_string};

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

#[derive(Debug)]
struct Run {
    data: BTreeMap<String, BTreeMap<String, String>>,
}

// So using the data structures above we have a guarantee that the Run will also be sorted in q_ids as well as the ranking values of the documents.
// I now need to create the required wrappers to convert a given a simple dictionary into a Run.

impl Run {
    fn new() -> Self {
        Run {
            data: BTreeMap::new(),
        }
    }

    // It will get sorted auto.
    fn insert(&mut self, outer_key: String, inner_data: BTreeMap<String, String>) {
        self.data.insert(outer_key, inner_data);
    }

    // fn convert_to_run(input: HashMap<String, HashMap<String, f64>>) -> Run {
    //     let mut run = Run::new();

    //     for (outer_key, inner_map) in input {
    //         let inner_data: BTreeMap<String, String> = inner_map
    //             .into_iter()
    //             .map(|(k, v)| (v.to_string(),k))
    //             .collect();
    //         run.insert(outer_key, inner_data);
    //     }

    //     run
    // }

    fn from_hashmap_to_run(input: HashMap<String, HashMap<String, f64>>) -> Self {
        let mut run = Run::new();

        for (outer_key, inner_map) in input {
            let inner_data: BTreeMap<String, String> = inner_map
                .into_iter()
                .map(|(k, v)| (v.to_string(), k))
                .collect();
            run.insert(outer_key, inner_data);
        }

        run
    }
}

// Runs = List[Run]

// TODO: Check for passing by reference whereever we can.

#[derive(Debug)]
struct Runs {
    runs: Vec<Run>,
}

impl Runs {
    fn new() -> Self {
        Runs { runs: Vec::new() }
    }

    fn new_with_cap(size: usize) -> Self {
        Runs {
            runs: Vec::with_capacity(size),
        }
    }

    fn insert(&mut self, insert_run: Run) {
        self.runs.push(insert_run);
    }

    fn from_list_of_hashmaps_runs(input: Vec<HashMap<String, HashMap<String, f64>>>) -> Self {
        let mut runs = Runs::new();

        for inner_map in input {
            let run = Run::from_hashmap_to_run(inner_map);
            runs.insert(run);
        }

        runs
    }

    fn len(&self) -> usize {
        self.runs.len()
    }
}

fn rrf_score(single_query: &BTreeMap<String, String>, k: usize) -> BTreeMap<String, String> {
    let mut ind_computed_rank: BTreeMap<String, String> = BTreeMap::new();
    let mut calc_rank:f64=0.0;
    for (i, (_, doc_id)) in single_query.iter().enumerate() {
        calc_rank=(1.0 / ((i as f64) + 1.0 + (k as f64)));
        ind_computed_rank.insert( f64_to_string(calc_rank),doc_id.clone());
    }

    ind_computed_rank
}

// Takes a single run and calculates the ranks of all the documents in all the queries. This is currently the main function to call
// fn rrf_score_parallel(run_object: &Run, k: usize) -> Run {
//     let combined_result: Run = run_object
//         .data
//         .par_iter()
//         .map(|(q_id, single_query)| (q_id.clone(), rrf_score(single_query, k)))
//         .collect();

//     combined_result
// }

fn rrf_score_parallel(run_object: &Run, k: usize) -> Run {
    let combined_result: BTreeMap<String, HashMap<String, String>> = run_object
        .data
        .par_iter()
        .map(|(q_id, single_query)| (rrf_score(single_query, k),q_id.clone()))
        .collect();

    Run { data: combined_result }
}

// Another issue is that there is no way to initialse a Vector with a certain size with all the elements of some value of Type run.
fn rrf(runs_object: Runs, k: usize) -> Run {
    let dummy_run = Run::new();
    // let mut runs_object_returned: Runs = Runs::new_with_cap(runs_object.len());
    let mut runs_object_returned: Runs = Runs::new_with_cap(runs_object.len());
    for runInstance in runs_object.runs.iter() {
        let mut temp_run = Run::new();
        temp_run.data = rrf_score_parallel(runInstance, k);
        runs_object_returned.runs.push(temp_run);
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
