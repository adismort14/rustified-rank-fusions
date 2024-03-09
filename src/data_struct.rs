use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub struct Run {
    pub q_rank_map: BTreeMap<String, BTreeMap<String, String>>,
}

// So using the data structures above we have a guarantee that the Run will also be sorted in both q_ids as well as the ranking values of the documents.
// I now need to create the required wrappers to convert a given a simple dictionary into a Run.

impl Run {
    pub fn new() -> Self {
        Run {
            q_rank_map: BTreeMap::new(),
        }
    }

    // It will get sorted auto.
    pub fn insert(&mut self, outer_key: String, inner_data: BTreeMap<String, String>) {
        self.q_rank_map.insert(outer_key, inner_data);
    }

    pub fn from_hashmap_to_run(input: HashMap<String, HashMap<String, f64>>) -> Run {
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
pub struct Runs {
    pub runs: Vec<Run>,
}

impl Runs {
    pub fn new() -> Self {
        Runs { runs: Vec::new() }
    }

    // This implementation was created to initialise during compile time? I dont think that is how it works.
    pub fn new_with_cap(size: usize) -> Self {
        Runs {
            runs: Vec::with_capacity(size),
        }
    }

    pub fn insert(&mut self, insert_run: Run) {
        self.runs.push(insert_run);
    }

    pub fn from_list_of_hashmaps_to_runs(input: Vec<HashMap<String, HashMap<String, f64>>>) -> Self {
        let mut runs = Runs::new();

        for inner_map in input {
            let run = Run::from_hashmap_to_run(inner_map);
            runs.insert(run);
        }

        runs
    }

    pub fn len(&self) -> usize {
        self.runs.len()
    }
}
