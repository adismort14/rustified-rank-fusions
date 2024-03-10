use crate::data_struct::DocRankingVec;

pub fn extract_scores(input_doc_vec: &DocRankingVec)->Vec<f64>{
    let mut extracted_scores_vec:Vec<f64>=Vec::with_capacity(input_doc_vec.len());
    for (i,dict) in input_doc_vec.iter().enumerate(){
        extracted_scores_vec[i]=dict.1;
    }
    extracted_scores_vec
}

pub fn safe_max(input_score_vec: &Vec<f64>) -> f64 {
    if !input_score_vec.is_empty() {
        // The DocRankingVec is already reverse sorted and so is the extracted_score_vec and so is the input_score_vec
        return input_score_vec[0];
    }
    0.0
}

pub fn safe_min(input_score_vec: &Vec<f64>) -> f64 {
    if !input_score_vec.is_empty() {
        // The DocRankingVec is already reverse sorted and so is the extracted_score_vec and so is the input_score_vec
        return input_score_vec[input_score_vec.len() - 1];
    }
    0.0
}

pub fn sum_vec(input_score_vec: &Vec<f64>) -> f64{
    let sumVal: f64 = input_score_vec.iter().sum();
    sumVal
}

pub fn vec_mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

fn variance(data: &[f64]) -> f64 {
    let mu = vec_mean(data);
    let sq_diff_sum: f64 = data.iter().map(|x| (x - mu).powi(2)).sum();
    sq_diff_sum / (data.len() - 1) as f64
}

pub fn vec_stdev(data: &[f64]) -> f64 {
    variance(data).sqrt()
}