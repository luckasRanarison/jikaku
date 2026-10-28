use jikaku::{RawPoint, Stroke};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = matchKanji)]
pub fn match_kanji(strokes: JsValue) -> Result<Vec<u32>, JsError> {
    let strokes: Vec<Vec<[f32; 2]>> = serde_wasm_bindgen::from_value(strokes)
        .map_err(|error| JsError::new(&error.to_string()))?;

    let strokes: Vec<Stroke<RawPoint>> = strokes
        .into_iter()
        .map(|points| Stroke {
            points: points.into_iter().map(|[x, y]| RawPoint { x, y }).collect(),
        })
        .collect();

    Ok(jikaku::match_kanji(&strokes)
        .into_iter()
        .filter(|&id| id != u32::MAX)
        .collect())
}
