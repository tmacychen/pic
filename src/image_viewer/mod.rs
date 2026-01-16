use makepad_widgets::Cx;

pub mod image_gallery;
pub mod image_viewer;

pub fn live_design(cx: &mut Cx) {
    self::image_gallery::live_design(cx);
    self::image_viewer::live_design(cx);
}
