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

    // Menu bar
    pub MenuBar = <View> {
        width: Fill, height: Fit
        show_bg: true
        draw_bg: {
            color: (COLOR_BG_LIGHT)
        }
        flow: Right
        padding: (SPACING_SM)
        spacing: (SPACING_SM)

        file_menu = <Button> {
            width: Fit, height: Fit
            draw_bg: {
                color: (COLOR_BG_LIGHT)
            }
            draw_text: {
                color: (COLOR_TEXT)
                text_style: { font_size: 14.0 }
            }
            text: "File"
        }

        filler = <View> {
            width: Fill, height: Fit
        }

        language_menu_button = <Button> {
            width: Fit, height: Fit
            draw_bg: {
                color: (COLOR_BG_LIGHT)
            }
            draw_text: {
                color: (COLOR_TEXT)
                text_style: { font_size: 14.0 }
            }
            text: "Language"
        }

        language_menu = <View> {
            visible: false
            width: 120, height: Fit
            show_bg: true
            draw_bg: {
                color: (COLOR_BG_LIGHT)
            }
            flow: Down
            padding: (SPACING_SM)
            spacing: (SPACING_SM)

            english_option = <Button> {
                width: Fill, height: Fit
                draw_bg: {
                    color: (COLOR_BG_LIGHT)
                }
                draw_text: {
                    color: (COLOR_TEXT)
                    text_style: { font_size: 12.0 }
                }
                text: "English"
            }

            chinese_option = <Button> {
                width: Fill, height: Fit
                draw_bg: {
                    color: (COLOR_BG_LIGHT)
                }
                draw_text: {
                    color: (COLOR_TEXT)
                    text_style: { font_size: 12.0 }
                }
                text: "中文"
            }
        }
    }
}
