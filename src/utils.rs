pub fn string_to_f64(input: &str) -> Result<f64, std::num::ParseFloatError> {
    input.parse()
}

pub fn f64_to_string(input: &str) -> Result<String, std::num::ParseFloatError> {
    input.parse::<f64>().map(|parsed_float| parsed_float.to_string())
}
