use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;

    pub ImageGallery = {{ImageGallery}} {
        width: Fill,
        height: Fill,
        show_bg: true,
        draw_bg: {
            color: (COLOR_BG)
        }

        flow: Down,
        padding: (SPACING_MD),
        spacing: (SPACING_MD),

        header = <Card> {
            width: Fill,
            height: Fit,
            padding: (SPACING_MD),

            title = <TextTitle> {
                text: "Image Gallery"
            }
        }

        gallery_view = <ScrollXYView> {
            width: Fill,
            height: Fill,

            grid = <Grid> {
                cell_width: 120,
                cell_height: 120,
                flow: Right,
                spacing: 10,

                template: <View> {
                    width: 120,
                    height: 120,

                    thumb = <Image> {
                        width: Fill,
                        height: Fill,
                        fit: Both,
                    }

                    overlay = <View> {
                        width: Fill,
                        height: 30,
                        align: {x: 0.5, y: 0.5},
                        show_bg: true,
                        draw_bg: { color: #00000088 },

                        label = <Label> {
                            width: Fill,
                            height: Fit,
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR> { font_size: 10.0 }
                                color: (COLOR_TEXT)
                            }
                            text: ""
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ImageGallery {
    #[deref]
    view: View,
    #[rust]
    _image_paths: Vec<String>,
}

impl Widget for ImageGallery {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
