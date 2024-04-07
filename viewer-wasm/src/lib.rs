use safetensors::{Dtype, SafeTensorError, SafeTensors};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub fn deserialize(data: &[u8]) -> JsValue {
    match deserialize_rs(data) {
        Ok(data) => to_value(&data).unwrap(),
        Err(err) => JsValue::from_str(&format!("{:?}", err)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CustomTensorView {
    dtype: Dtype,
    shape: Vec<usize>,
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SafeTensorInfo {
    names: Vec<String>,
    tensors: Vec<(String, CustomTensorView)>
}

fn deserialize_rs(data: &[u8]) -> Result<SafeTensorInfo, SafeTensorError> {
    let tensors_data = SafeTensors::deserialize(&data)?;
    let names = tensors_data.names();
    let tensors = tensors_data.tensors();
    // self.read_safetensors("", &tensors
    
    Ok(SafeTensorInfo {
        names: names.iter().map(|x| x.to_string()).collect(),
        tensors: tensors.iter().map(|(name, tensor)| {
            let dtype = tensor.dtype();
            let shape = tensor.shape();
            let data = tensor.data();
            (name.to_string(), CustomTensorView {
                dtype,
                shape: shape.to_vec(),
                data: data.to_vec(),
            })
        }).collect()
    })
}
