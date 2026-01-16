use makepad_widgets::*;
use std::fs;
use std::path::Path;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;

    pub ImageViewer = {{ImageViewer}} {
        width: Fill,
        height: Fill,
        show_bg: true,
        draw_bg: {
            color: (COLOR_BG)
        }

        flow: Down,
        padding: (SPACING_MD),
        spacing: (SPACING_MD),

        image_container = <View> {
            width: Fill,
            height: Fill,
            align: {x: 0.5, y: 0.5},

            current_image = <Image> {
                width: Fit,
                height: Fit,
                fit: Horizontal,
            }
        }

        controls = <Card> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: (SPACING_SM),
            padding: (SPACING_SM),

            prev_button = <Button> {
                text: "<"
                width: 60,
                height: 40,
            }

            image_info = <Label> {
                width: Fill,
                height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                    color: (COLOR_TEXT)
                }
                text: "No image loaded"
            }

            next_button = <Button> {
                text: ">"
                width: 60,
                height: 40,
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ImageViewer {
    #[deref]
    view: View,
    #[rust]
    image_paths: Vec<String>,
    #[rust]
    current_index: usize,
}

impl Widget for ImageViewer {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ImageViewer {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // Previous button clicked
        if self.view.button(id!(prev_button)).clicked(&actions) {
            if !self.image_paths.is_empty() {
                if self.current_index > 0 {
                    self.current_index -= 1;
                } else {
                    self.current_index = self.image_paths.len() - 1; // Wrap around to last image
                }
                self.load_current_image(cx);
            }
        }

        // Next button clicked
        if self.view.button(id!(next_button)).clicked(&actions) {
            if !self.image_paths.is_empty() {
                if self.current_index < self.image_paths.len() - 1 {
                    self.current_index += 1;
                } else {
                    self.current_index = 0; // Wrap around to first image
                }
                self.load_current_image(cx);
            }
        }
    }
}

impl ImageViewer {
    pub fn load_images_from_directory(&mut self, directory: &str) {
        let mut image_paths = Vec::new();
        let img_dir = Path::new(directory);

        if img_dir.exists() && img_dir.is_dir() {
            for entry in fs::read_dir(img_dir).unwrap() {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        let ext = path
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_lowercase())
                            .unwrap_or_default();

                        // Check if the file is an image
                        if matches!(
                            ext.as_str(),
                            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tga" | "tiff" | "webp"
                        ) {
                            image_paths.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        image_paths.sort(); // Sort alphabetically for consistent ordering
        self.image_paths = image_paths;
        self.current_index = 0;
    }

    pub fn load_current_image(&mut self, cx: &mut Cx) {
        if self.image_paths.is_empty() {
            self.view
                .label(id!(image_info))
                .set_text(cx, "No images found");
            return;
        }

        let current_path = &self.image_paths[self.current_index];
        let filename = Path::new(current_path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        self.view.label(id!(image_info)).set_text(
            cx,
            &format!(
                "{} ({}/{})",
                filename,
                self.current_index + 1,
                self.image_paths.len()
            ),
        );

        // Note: In Makepad, loading images dynamically from paths is more complex.
        // For now, we'll just display a message indicating which image should be shown.
        // Proper dynamic image loading requires pre-registering images or using different approach.
        self.view.label(id!(image_info)).set_text(
            cx,
            &format!(
                "Image: {} ({}/{})",
                filename,
                self.current_index + 1,
                self.image_paths.len()
            ),
        );
    }

    pub fn init(&mut self, cx: &mut Cx) {
        self.load_images_from_directory("./img"); // Load images from ./img directory
        self.load_current_image(cx);
    }
}
