use jikaku::{RawPoint, Stroke};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Candidate {
    pub id: u32,
    pub distance: f64,
}

#[wasm_bindgen(js_name = matchCharacter)]
pub fn match_character(strokes: JsValue) -> Result<Vec<Candidate>, JsError> {
    let strokes: Vec<Vec<[f32; 2]>> = serde_wasm_bindgen::from_value(strokes)
        .map_err(|error| JsError::new(&error.to_string()))?;

    let strokes: Vec<Stroke<RawPoint>> = strokes
        .into_iter()
        .map(|points| Stroke {
            points: points.into_iter().map(|[x, y]| RawPoint { x, y }).collect(),
        })
        .collect();

    Ok(jikaku::match_character(&strokes)
        .into_iter()
        .filter(|c| c.id != u32::MAX)
        .map(|c| Candidate {
            id: c.id,
            distance: c.distance,
        })
        .collect())
}
