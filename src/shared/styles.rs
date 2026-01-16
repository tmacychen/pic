use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // Color palette
    pub COLOR_BG = vec4(0.102, 0.102, 0.180, 1.0)
    pub COLOR_BG_LIGHT = vec4(0.086, 0.129, 0.243, 1.0)
    pub COLOR_PRIMARY = vec4(0.059, 0.204, 0.376, 1.0)
    pub COLOR_ACCENT = vec4(0.914, 0.271, 0.376, 1.0)
    pub COLOR_TEXT = vec4(0.918, 0.918, 0.918, 1.0)
    pub COLOR_TEXT_DIM = vec4(0.533, 0.533, 0.533, 1.0)

    // Spacing
    pub SPACING_SM = 8.0
    pub SPACING_MD = 10.0
    pub SPACING_LG = 24.0

    // Border radius
    pub RADIUS_SM = 4.0
    pub RADIUS_MD = 8.0
    pub RADIUS_LG = 16.0

    // Common text styles
    pub TextRegular = <Label> {
        draw_text: {
            text_style: { font_size: 14.0 }
            color: (COLOR_TEXT)
        }
    }

    pub TextTitle = <Label> {
        draw_text: {
            text_style: { font_size: 24.0 }
            color: (COLOR_TEXT)
        }
    }

    pub TextSubtitle = <Label> {
        draw_text: {
            text_style: { font_size: 18.0 }
            color: (COLOR_TEXT_DIM)
        }
    }

    // Common button style
    pub PrimaryButton = <Button> {
        draw_bg: {
            color: (COLOR_ACCENT)
            border_radius: (RADIUS_SM)
        }
        draw_text: {
            color: #ffffff
        }
    }
}
