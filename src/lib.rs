use std::path::PathBuf;

use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
#[allow(unused)]
use rten_tensor::prelude::*;
use ext_php_rs::prelude::*;



/// OcrEngine class for PHP
#[php_class]
pub struct PhpOcrEngine {
    engine: OcrEngine,
}

#[php_impl]
impl PhpOcrEngine {
    /// Create a new OCR engine instance
    pub fn __construct(detection_model_path: String, rec_model_path: String) -> PhpResult<PhpOcrEngine> {
        let detection_model_path_buff = PathBuf::from(detection_model_path.clone());
        let rec_model_path_buff = PathBuf::from(rec_model_path.clone());
        let recognition_model;
        let detection_model;

        match Model::load_file(detection_model_path_buff) {
            Ok(model) => {
                detection_model = Some(model);
            },
            Err(e) => return Err(PhpException::default(
                format!("Failed to load detection model from {}: {}", detection_model_path, e)
            ))
        }
        match Model::load_file(rec_model_path_buff) {
            Ok(model) => {
                recognition_model = Some(model);
            },
            Err(e) => return Err(PhpException::default(
                format!("Failed to load detection model from {}: {}", rec_model_path, e)
            ))
        }

        match OcrEngine::new(OcrEngineParams {
    detection_model,
    recognition_model,
    ..OcrEngineParams::default()}) {
            Ok(engine) => {
                Ok(PhpOcrEngine {engine})
            },
            Err(e) => panic!("OCR Engine construction error: {}", e)
        }
    }


    /// Process an image and return the recognized text
    pub fn process_image(&self, image_path: String) -> PhpResult<String> {
        let engine = &self.engine;

        // Read image using image-rs library
        let img = match image::open(&image_path) {
            Ok(img) => img.into_rgb8(),
            Err(e) => return Err(PhpException::default(
                format!("Failed to open image: {}", e)
            )),
        };

        // Apply standard image pre-processing
        let img_source = match ImageSource::from_bytes(img.as_raw(), img.dimensions()) {
            Ok(source) => source,
            Err(e) => return Err(PhpException::default(
                format!("Failed to process image: {}", e)
            )),
        };

        let ocr_input = match engine.prepare_input(img_source) {
            Ok(input) => input,
            Err(e) => return Err(PhpException::default(
                format!("Failed to prepare input: {}", e)
            )),
        };

        // Get the text from the image
        match engine.get_text(&ocr_input) {
            Ok(text) => Ok(text),
            Err(e) => Err(PhpException::default(
                format!("Failed to recognize text: {}", e)
            )),
        }
    }


}

// Register the PhpOcrEngine class with the PHP extension
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<PhpOcrEngine>()
}
