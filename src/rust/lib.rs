use pyo3::prelude::*;

#[pymodule]
fn pyrustplay_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "sendstring")]
    fn sendstring(
        _py: Python,
        mystr: &str
    ) -> PyResult<bool> {
        let mut result = false;
        if mystr.len()>5 {
            result = true;
        }
        Ok(result)
    }
    Ok(())
}
