use pyo3::prelude::{Python, PyModule, PyResult, pymodule};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct Annotations {
    #[serde(rename(serialize = "Annotation", deserialize = "Annotation"))]
    annotations: Vec<Annotation>
}

#[derive(Debug, Deserialize, Serialize)]
struct Annotation {
    #[serde(rename(serialize = "LineColor", deserialize = "@LineColor"))]
    linecolor: i32,
    #[serde(rename(serialize = "Name", deserialize = "@Name"))]
    name: String,
    #[serde(rename(serialize = "Visible", deserialize = "@Visible"))]
    visible: bool,
    #[serde(rename(serialize = "Regions", deserialize = "Regions"))]
    regions: Regions
}

#[derive(Debug, Deserialize, Serialize)]
struct Regions {
    #[serde(rename(serialize = "Region", deserialize = "Region"))]
    regions: Vec<Region>
}

#[derive(Debug, Deserialize, Serialize)]
struct Region {
    #[serde(rename(serialize = "Type", deserialize = "@Type"))]
    regiontype: String,
    #[serde(rename(serialize = "HasEndcaps", deserialize = "@HasEndcaps"))]
    hasendcaps: bool,
    #[serde(rename(serialize = "NegativeROA", deserialize = "@NegativeROA"))]
    negativeroa: bool,
    #[serde(rename(serialize = "Vertices", deserialize = "Vertices"))]
    vertices: Vertices,
}

#[derive(Debug, Deserialize, Serialize)]
struct Vertices {
    #[serde(rename(serialize = "Coordinates", deserialize = "V"))]
    coordinates: Vec<Coordinate>
}

#[derive(Debug, Deserialize, Serialize)]
struct Coordinate {
    #[serde(rename(serialize = "x", deserialize = "@X"))]
    x: i32,
    #[serde(rename(serialize = "y", deserialize = "@Y"))]
    y: i32,
}


#[pymodule]
fn pyrustplay_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "sendstring")]
    fn sendstring(
        _py: Python,
        mystr: &str
    ) -> PyResult<String> {
        let doc: Annotations = quick_xml::de::from_str(mystr).unwrap();
        let result = format!("{}", serde_json::to_string(&doc).unwrap());
        Ok(result)
    }
    Ok(())
}
