use makepad_widgets::Cx;

pub mod styles;
pub mod widgets;

pub fn live_design(cx: &mut Cx) {
    self::styles::live_design(cx);
    self::widgets::live_design(cx);
}
