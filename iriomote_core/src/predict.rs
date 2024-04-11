use flate2::bufread::GzDecoder;
use image::{imageops::FilterType, GenericImageView};
use ndarray::Array;
use ort::{GraphOptimizationLevel, Session};
use std::io::prelude::*;

pub struct IriomoteCore {
    session: Session,
}

impl IriomoteCore {
    pub fn new() -> anyhow::Result<Self> {
        let mut compressed_data = GzDecoder::new(&include_bytes!("./models/iriomote-cats.gz")[..]);
        let mut data: Vec<u8> = Vec::new();
        compressed_data.read_to_end(&mut data)?;
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_memory(&data)?;
        Ok(Self { session })
    }

    pub async fn predict(&self, image_bytes: Vec<u8>) -> anyhow::Result<Option<f32>> {
        let img = image::load_from_memory(&image_bytes)?;
        let img = img.resize_exact(224, 224, FilterType::CatmullRom);
        let mut input = Array::zeros((1, 224, 224, 3));
        for pixel in img.pixels() {
            let x = pixel.0 as _;
            let y = pixel.1 as _;
            let [r, g, b, _] = pixel.2 .0;
            input[[0, y, x, 0]] = (r as f32) / 255.;
            input[[0, y, x, 1]] = (g as f32) / 255.;
            input[[0, y, x, 2]] = (b as f32) / 255.;
        }
        let outputs = self
            .session
            .run_async(ort::inputs!["keras_layer_input" => input]?)?
            .await?;
        let output = outputs[0].try_extract_tensor::<f32>()?;
        if let Some(output) = output.to_slice() {
            return Ok(Some(output[0]));
        }
        return Ok(None);
    }
}
