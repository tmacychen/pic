use makepad_widgets::*;
use std::path::Path;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;

    LEFT_ARROW = dep("crate://self/resources/left_arrow.svg");
    RIGHT_ARROW = dep("crate://self/resources/right_arrow.svg");
    PLACEHOLDER = dep("crate://self/resources/placeholder.png");

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
                width: Fill,
                height: Fill,
                fit: Biggest,
                source: (PLACEHOLDER)
            }
        }

        controls = <Card> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: (SPACING_SM),
            padding: (SPACING_SM),

            prev_button = <IconButton> {
                draw_icon: {
                    svg_file: (LEFT_ARROW)
                }
            }
            image_index = <TextTitle> {
                width: Fit,
                height: Fit,
                text: "0"
            }
            image_info = <TextTitle> {
                width: Fill,
                height: Fit,
                text: "No image loaded"
            }

            next_button = <IconButton> {
                draw_icon: {
                    svg_file: (RIGHT_ARROW)
                }
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

impl ImageViewer {}
