use crate::i18n::{I18n, Language};
use crate::state::State;
use makepad_widgets::*;
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
                    flow: Down
                    show_bg: true
                    draw_bg: { color: (COLOR_BG) }

                    // Add top padding to avoid  window controls button
                    top_spacer = <View> {
                        width: Fill, height: 32
                        show_bg: false
                    }

                    menu_bar = <MenuBar> {}

                    iv = <ImageViewer> {}
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
struct App {
    #[live]
    ui: WidgetRef,
    #[live]
    placeholder: LiveDependency,
    #[rust]
    state: State,
    #[rust]
    language: Language,
    #[rust]
    language_menu_visible: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ui: WidgetRef::default(),
            placeholder: LiveDependency::default(),
            state: State::default(),
            language: Language::English,
            language_menu_visible: false,
        }
    }
}

impl App {
    fn load_image_paths(&mut self, cx: &mut Cx, dir: &Path) {
        self.state.image_paths.clear();

        for entry in dir.read_dir().unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() {
                self.state.image_paths.push(path);
            }
        }
        log!("Loaded {:?} images", self.state.image_paths);
        self.set_current_image(cx, 0);
    }

    fn on_file_menu_clicked(&mut self) {
        let menu_text = I18n::get("file", self.language);
        log!("File menu clicked: {}", menu_text);
    }

    fn toggle_language_menu(&mut self, cx: &mut Cx) {
        self.language_menu_visible = !self.language_menu_visible;
        self.ui.view(id!(menu_bar.language_menu)).set_visible(cx, self.language_menu_visible);
    }

    fn set_language(&mut self, cx: &mut Cx, lang: Language) {
        self.language = lang;
        
        // Update file menu text
        let file_menu_text = I18n::get("file", self.language);
        self.ui.button(id!(menu_bar.file_menu)).set_text(cx, file_menu_text);
        
        // Update language button text
        let language_text = I18n::get("language", self.language);
        self.ui.button(id!(menu_bar.language_menu_button)).set_text(cx, language_text);
        
        // Hide language menu
        self.ui.view(id!(menu_bar.language_menu)).set_visible(cx, false);
        
        log!("Language changed to: {:?}", lang);
        self.ui.redraw(cx);
    }

    fn set_current_image(&mut self, cx: &mut Cx, image_idx: usize) {
        self.state.current_image_idx = image_idx;
        let image = self.ui.image(id!(image_container.current_image));
        let image_index = self.ui.label(id!(image_index));
        let image_info = self.ui.label(id!(image_info));
        if let Some(path) = self.state.image_paths.get(image_idx) {
            log!("Current image {} , path: {:?}", image_idx, path.display());
            image.load_image_file_by_path_async(cx, &path).unwrap();
            image_index.set_text(cx, image_idx.to_string().as_str());
            image_info.set_text(cx, path.display().to_string().as_str());
        } else {
            let placeholder = self.placeholder.as_str();
            image.load_image_dep_by_path(cx, placeholder).unwrap();
            image_info.set_text(cx, "No image selected");
        }

        self.ui.redraw(cx);
    }

    fn go_to_previous_image(&mut self, cx: &mut Cx) {
        if self.state.current_image_idx > 0 {
            self.set_current_image(cx, self.state.current_image_idx - 1);
        }
    }

    fn go_to_next_image(&mut self, cx: &mut Cx) {
        if self.state.current_image_idx + 1 < self.state.num_images() {
            self.set_current_image(cx, self.state.current_image_idx + 1);
        }
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::shared::live_design(cx);
        crate::image_viewer::live_design(cx);
    }
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.language = Language::current();
        let file_menu_text = I18n::get("file", self.language);
        self.ui.button(id!(menu_bar.file_menu)).set_text(cx, file_menu_text);
        
        let language_text = I18n::get("language", self.language);
        self.ui.button(id!(menu_bar.language_menu_button)).set_text(cx, language_text);
        
        self.load_image_paths(cx, "./img".as_ref());
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        let mut scope = Scope::with_data(&mut self.state);
        self.ui.handle_event(cx, event, &mut scope);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(menu_bar.file_menu)).clicked(&actions) {
            self.on_file_menu_clicked();
        }

        if self.ui.button(id!(menu_bar.language_menu_button)).clicked(&actions) {
            self.toggle_language_menu(cx);
        }

        if self.ui.button(id!(menu_bar.language_menu.english_option)).clicked(&actions) {
            self.set_language(cx, Language::English);
        }

        if self.ui.button(id!(menu_bar.language_menu.chinese_option)).clicked(&actions) {
            self.set_language(cx, Language::Chinese);
        }

        if self.ui.button(id!(prev_button)).clicked(&actions) {
            self.go_to_previous_image(cx);
        }
        if self.ui.button(id!(next_button)).clicked(&actions) {
            self.go_to_next_image(cx);
        }

        if let Some(event) = self.ui.view(id!(image_container)).key_down(&actions) {
            match event.key_code {
                KeyCode::Escape => {
                    self.ui
                        .page_flip(id!(page_flip))
                        .set_active_page(cx, live_id!(image_browser));
                }
                KeyCode::ArrowLeft => self.go_to_previous_image(cx),
                KeyCode::ArrowRight => self.go_to_next_image(cx),
                _ => {}
            }
        }
    }
}
