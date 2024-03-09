use std::collections::{HashMap, BTreeMap};

// // So using the data structures above we have a guarantee that the Run will also be sorted in both q_ids as well as the ranking values of the documents.
// // I now need to create the required wrappers to convert a given a simple dictionary into a Run.

type DocId = String;
type RankingScore = f64;
type DocRankingVec = Vec<(DocId, RankingScore)>;
type QidMapping = BTreeMap<String, DocRankingVec>;

#[derive(Debug)]
pub struct Run {
    pub qid_mapping: QidMapping,
}

impl Run {
    /// Instantiate a new Run object.
    pub fn new()-> Self{
        Run{
            qid_mapping:QidMapping::new(),
        }
    }

    /// Add a new association given a q_id, doc_id and the ranking
    pub fn add_association(&mut self, qid: String, doc_id: DocId, float_value: RankingScore) {
        let doc_ranking = self.qid_mapping.entry(qid).or_insert_with(Vec::new);
        doc_ranking.push((doc_id, float_value));
        doc_ranking.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    }

    /// Function to retrieve DocRankingVec for a specific qid
    pub fn get_doc_ranking(&self, qid: &str) -> Option<&DocRankingVec> {
        self.qid_mapping.get(qid)
    }

    pub fn create_run_from_dict(input: HashMap<String, HashMap<String, f64>>) -> Run{
        let mut dummy_run=Run::new();
        for (q_id_iter,rank_tuple) in input{
            let mut doc_ranking_vec_iter=DocRankingVec::new();
            for (doc_id_iter, rank_iter) in rank_tuple{
                doc_ranking_vec_iter.push((doc_id_iter,rank_iter));
                doc_ranking_vec_iter.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            }
            dummy_run.qid_mapping.insert(q_id_iter, doc_ranking_vec_iter);
        }
        dummy_run
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
            let run = Run::create_run_from_dict(inner_map);
            runs.insert(run);
        }

        runs
    }

    pub fn len(&self) -> usize {
        self.runs.len()
    }
}


// Dummy sample data to test
// fn main() {
//     // Define sample data
//     let mut sample_data = HashMap::new();
//     let mut inner_map1 = HashMap::new();
//     inner_map1.insert("doc1".to_string(), 1.5);
//     inner_map1.insert("doc2".to_string(), 0.8);
//     sample_data.insert("qid1".to_string(), inner_map1);

//     // Create Run instance from sample data
//     let run1 = Run::create_run_from_dict(sample_data);

//     let mut sample_data = HashMap::new();
//     let mut inner_map1 = HashMap::new();
//     inner_map1.insert("doc3".to_string(), 12.0);
//     inner_map1.insert("doc2".to_string(), 0.01);
//     let mut inner_map2 = HashMap::new();
//     inner_map2.insert("doc3".to_string(), 12.0);
//     inner_map2.insert("doc2".to_string(), 0.01);
//     sample_data.insert("qid4".to_string(), inner_map1);
//     sample_data.insert("qid1".to_string(), inner_map2);

//     // Create Run instance from sample data
//     let run2 = Run::create_run_from_dict(sample_data);

//     // Create Runs collection and insert the Run instance
//     let mut runs_collection = Runs::new();
//     runs_collection.insert(run1);
//     runs_collection.insert(run2);

//     // Display the content of Runs collection
//     println!("Runs collection: {:?}", runs_collection);
// }
