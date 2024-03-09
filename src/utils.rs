pub fn string_to_f64(input: &str) -> Result<f64, std::num::ParseFloatError> {
    input.parse()
}

pub fn f64_to_string(input: f64) -> String {
    input.to_string()
}
