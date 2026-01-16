use makepad_widgets::*;

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
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::shared::live_design(cx);
        crate::image_viewer::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        // Initialize the app
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle global actions
        let _ = (cx, actions);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());

        // We initialize the image viewer differently
    }
}
