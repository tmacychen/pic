use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::shared::styles::*;

    // Card container
    pub Card = <RoundedView> {
        width: Fill, height: Fit
        padding: (SPACING_MD)
        show_bg: true
        draw_bg: {
            color: (COLOR_BG_LIGHT)
            border_radius: (RADIUS_MD)
        }
    }

    // Horizontal divider
    pub Divider = <View> {
        width: Fill, height: 1
        show_bg: true
        draw_bg: { color: (COLOR_TEXT_DIM) }
    }

    // Icon button
    pub IconButton = <Button> {
        width: 40, height: 40
    }
}
