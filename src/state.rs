use std::path::PathBuf;
/// State is the application state containing image paths and the current image index
pub struct State {
    pub image_paths: Vec<PathBuf>,
    pub current_image_idx: usize,
}

impl State {
    pub fn num_images(&self) -> usize {
        self.image_paths.len()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            image_paths: Vec::new(),
            current_image_idx: 0,
        }
    }
}
