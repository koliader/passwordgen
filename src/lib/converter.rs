pub struct Converter {}

impl Converter {
    pub fn convert_arr_to_string(&self, vec: &Vec<String>) -> String {
        let mut res = String::new();
        for v in vec.iter() {
            res.push_str(format!("{}\n", v).as_str())
        }
        res
    }
}