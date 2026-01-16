use makepad_widgets::*;
use std::fs;
use std::path::Path;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::image_viewer::image_viewer::ImageViewer;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: { title: "Image Viewer", inner_size: vec2(1024, 768) }
                body = <View> {
                    width: Fill, height: Fill
                    show_bg: true
                    draw_bg: { color: (COLOR_BG) }

                    <ImageViewer> {}
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    image_paths: Vec<String>,
    #[rust]
    current_index: usize,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::shared::live_design(cx);
        crate::image_viewer::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // Initialize the app
        self.load_images_from_directory("./img");
        self.update_image_display(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle global actions
        // Check for navigation actions from ImageViewer
        for action in actions {
            if let Some(crate::image_viewer::image_viewer::ImageViewerAction::PrevImage) =
                action.as_widget_action().cast()
            {
                self.previous_image(cx);
            } else if let Some(crate::image_viewer::image_viewer::ImageViewerAction::NextImage) =
                action.as_widget_action().cast()
            {
                self.next_image(cx);
            }
        }

        // Handle other global actions
        let _ = (cx, actions);
    }
}

impl App {
    fn load_images_from_directory(&mut self, directory: &str) {
        let mut image_paths = Vec::new();
        let img_dir = Path::new(directory);

        if img_dir.exists() && img_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(img_dir) {
                for entry in entries {
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
        }

        image_paths.sort(); // Sort alphabetically for consistent ordering
        self.image_paths = image_paths;
        self.current_index = 0;

        log!(
            "Loaded {} images from {}",
            self.image_paths.len(),
            directory
        );
    }

    fn previous_image(&mut self, cx: &mut Cx) {
        if !self.image_paths.is_empty() {
            if self.current_index > 0 {
                self.current_index -= 1;
            } else {
                self.current_index = self.image_paths.len() - 1; // Wrap around to last image
            }
            self.update_image_display(cx);
        }
    }

    fn next_image(&mut self, cx: &mut Cx) {
        if !self.image_paths.is_empty() {
            if self.current_index < self.image_paths.len() - 1 {
                self.current_index += 1;
            } else {
                self.current_index = 0; // Wrap around to first image
            }
            self.update_image_display(cx);
        }
    }

    fn update_image_display(&mut self, cx: &mut Cx) {
        if self.image_paths.is_empty() {
            return;
        }

        let current_path = &self.image_paths[self.current_index];

        // Update the ImageViewer with the current image path
        // We'll use apply_over to update the image path
        self.ui.widget(id!(ImageViewer)).apply_over(
            cx,
            live! {
                current_image_path: (current_path)
            },
        );

        log!(
            "Displaying image: {} ({} of {})",
            Path::new(current_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy(),
            self.current_index + 1,
            self.image_paths.len()
        );
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());

        // Initialize image viewer after UI is set up
        if let Event::Draw(_) = event {
            let _image_viewer_ref = self.ui.widget(id!(ImageViewer));
            // We'll initialize in the image viewer's own draw method instead
        }
    }
}
