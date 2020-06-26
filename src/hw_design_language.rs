use serde::Serialize;
use serde_json::map::Map;
use serde_json::{json, Value};

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DType {
    Int8 = 0,
    Int16 = 1,
    Int32 = 2,
    Uint8 = 3,
    Uint16 = 4,
    Uint32 = 5,
    Bf16 = 6,
    Fp16 = 7,
    Fp32 = 8,
    Fp64 = 9,
}

pub struct SystolicArrayWeightStationaryParams {
    dtype: DType,
    rows: usize,
    cols: usize,
}

pub enum AtomConfig {
    SystolicArrayWeightStationary(SystolicArrayWeightStationaryParams),
}

pub struct Atom {
    name: String,
    config: AtomConfig,
}

pub struct HardwareDesign {
    atoms: Vec<Atom>,
}

pub fn design_to_json(design: &HardwareDesign) -> Value {
    Value::Array(
        design
            .atoms
            .iter()
            .map(|atom: &Atom| atom_to_json(atom))
            .collect(),
    )
}

pub fn atom_to_json(atom: &Atom) -> Value {
    let mut map = Map::default();
    map.append(&mut match &atom.config {
        AtomConfig::SystolicArrayWeightStationary(params) => {
            let mut map = Map::default();
            map.insert(
                "atom".to_string(),
                json!("bsg_systolic_array_weight_stationary"),
            );
            map.insert("dtype".to_string(), json!(params.dtype));
            map.insert("rows".to_string(), json!(params.rows));
            map.insert("cols".to_string(), json!(params.cols));
            map
        }
    });
    map.insert("name".to_string(), json!(atom.name));

    Value::Object(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let design = HardwareDesign {
            atoms: vec![
                Atom {
                    name: "multiplier1".to_string(),
                    config: AtomConfig::SystolicArrayWeightStationary(
                        SystolicArrayWeightStationaryParams {
                            dtype: DType::Int8,
                            rows: 16,
                            cols: 16,
                        },
                    ),
                },
                Atom {
                    name: "multiplier2".to_string(),
                    config: AtomConfig::SystolicArrayWeightStationary(
                        SystolicArrayWeightStationaryParams {
                            dtype: DType::Int8,
                            rows: 16,
                            cols: 16,
                        },
                    ),
                },
                Atom {
                    name: "multiplier3".to_string(),
                    config: AtomConfig::SystolicArrayWeightStationary(
                        SystolicArrayWeightStationaryParams {
                            dtype: DType::Int8,
                            rows: 16,
                            cols: 16,
                        },
                    ),
                },
            ],
        };

        assert_eq!(
            design_to_json(&design),
            json!(
                [
                    { "name" : "multiplier1",
                       "atom" : "bsg_systolic_array_weight_stationary",
                       "dtype" : "int8",
                       "cols" : 16,
                       "rows" : 16,
                    },
                    { "name" : "multiplier2",
                       "atom" : "bsg_systolic_array_weight_stationary",
                       "dtype" : "int8",
                       "cols" : 16,
                       "rows" : 16,
                    },
                    { "name" : "multiplier3",
                       "atom" : "bsg_systolic_array_weight_stationary",
                       "dtype" : "int8",
                       "cols" : 16,
                       "rows" : 16,
                    },
                ]
            )
        );
    }
}
