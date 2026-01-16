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

#[derive(Clone, Debug, DefaultNone)]
pub enum ImageViewerAction {
    PrevImage,
    NextImage,
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct ImageViewer {
    #[deref]
    view: View,
    #[live]
    current_image_path: String,
    #[live]
    image_loaded: bool,
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
        // Navigation is handled by app, just process button clicks if needed
        // For now, we don't need special action handling
    }
}

impl ImageViewer {
    pub fn set_image_path(&mut self, image_path: &str) {
        self.current_image_path = image_path.to_string();
        self.image_loaded = true;
    }

    pub fn load_current_image(&mut self, cx: &mut Cx) {
        if self.current_image_path.is_empty() {
            self.view
                .label(id!(image_info))
                .set_text(cx, "No image specified");
            return;
        }

        let path = Path::new(&self.current_image_path);
        let filename = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        self.view
            .label(id!(image_info))
            .set_text(cx, &format!("Image: {}", filename,));

        // Note: In Makepad, loading images dynamically from paths is more complex.
        // For now, we'll just display a message indicating which image should be shown.
        // Proper dynamic image loading requires pre-registering images or using different approach.
    }

    pub fn init(&mut self, cx: &mut Cx) {
        log!("Init method called");
        // No need to load images here, app will control which image to show
        log!("ImageViewer initialized");
    }
}
