use rayon::prelude::*;
use std::collections::HashMap;

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

// The data structure `Run` is of the form => Run: HashMap<String,HashMap<String,f64>> ;

fn rrf_score(single_qrel: &HashMap<String, f64>, k: usize) -> HashMap<String, f64> {
    let mut ind_computed_rank: HashMap<String, f64> = HashMap::new();
    for (i, (doc_id, _)) in single_qrel.iter().enumerate() {
        ind_computed_rank.insert(doc_id.clone(), 1.0 / ((i as f64) + 1.0 + (k as f64)));
    }

    ind_computed_rank
}

// fn _rrf_score_parallel(comb_qrel: &HashMap<String, HashMap<String, f64>>, k: usize) -> HashMap<String,HashMap<String, f64>> {
    
//     let mut combined_result:HashMap<String,HashMap<String, f64>>=HashMap::new();
    
//     // let q_id:Vec<&String>=combQrel.keys().collect();
//     // println!("{:?}",q_id);

//     for (q_id, single_qrel) in comb_qrel.iter() {
//         combined_result.insert(q_id.clone(), rrf_score(single_qrel, k));
//     }
//     // for i in 0..q_id.len(){
//     //     combined_result.insert(rrf_score(combQrel[q_id[i]], k));
//     // }
//     // for (qrelId, singleQrel) in combQrel.keys().par_iter() {
//     //     rrf_score(singleQrel, k);
//     // }
//     combined_result
// }

fn rrf_score_parallel(comb_qrel: &HashMap<String, HashMap<String, f64>>, k: usize) -> HashMap<String, HashMap<String, f64>> {
    // Create a parallel iterator over the query IDs and their associated single_qrel
    let combined_result: HashMap<String, HashMap<String, f64>> = comb_qrel.par_iter()
        .map(|(q_id, single_qrel)| (q_id.clone(), rrf_score(single_qrel, k)))
        .collect();

    combined_result
}


fn main() {
    // Dummy data for testing
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
    .collect();

    // Best empirical and default value of k according to the research paper
    let k = 60;

    let combined_results = rrf_score_parallel(&run_hashmap, k);

    for (q_id, result) in &combined_results {
        println!("Query ID: {}", q_id);
        for (doc_id, score) in result {
            println!("Document ID: {}, Score: {}", doc_id, score);
        }
        println!(); 
    }
}
