use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum LanguageMenuAction {
    SelectEnglish,
    SelectChinese,
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct LanguageMenu {
    #[deref]
    view: View,
}

impl Widget for LanguageMenu {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for LanguageMenu {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self.view.button(id!(english_option)).clicked(&actions) {
            cx.widget_action(self.widget_uid(), &scope.path, LanguageMenuAction::SelectEnglish);
        }
        if self.view.button(id!(chinese_option)).clicked(&actions) {
            cx.widget_action(self.widget_uid(), &scope.path, LanguageMenuAction::SelectChinese);
        }
    }
}
