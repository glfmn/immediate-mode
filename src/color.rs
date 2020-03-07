//! Color type and default themes

/// RGBA Color as hex
#[derive(Copy, Clone, Debug)]
pub struct Color(pub u32);

impl Color {
    /// Set the alpha of a color
    ///
    /// ```
    /// # use immediate_mode::Color;
    /// let color = Color(0xFF_FF_FF_FF);
    /// let alpha: [u8; 4] = color.alpha(0x00).into();
    /// assert_eq!(alpha, [255, 255, 255, 0]);
    /// ```
    pub const fn alpha(self, alpha: u8) -> Color {
        Color((self.0 & 0xFF_FF_FF_00) | alpha as u32)
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        // TODO: determine better method
        [
            ((self.0 & 0xFF_00_00_00) >> 24) as u8,
            ((self.0 & 0x00_FF_00_00) >> 16) as u8,
            ((self.0 & 0x00_00_FF_00) >> 8) as u8,
            (self.0 & 0x00_00_00_FF) as u8,
        ]
    }
}

#[test]
fn test_color_u8_conversion() {
    let color: [u8; 4] = Color(0xFF_00_FF_00).into();
    assert_eq!(color, [255, 0, 255, 0]);
    let color: [u8; 4] = Color(0x00_FF_00_FF).into();
    assert_eq!(color, [0, 255, 0, 255]);
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        // TODO: determine better method
        [
            ((self.0 & 0xFF_00_00_00) >> 24) as f32 / 255.0,
            ((self.0 & 0x00_FF_00_00) >> 16) as f32 / 255.0,
            ((self.0 & 0x00_00_FF_00) >> 8) as f32 / 255.0,
            (self.0 & 0x00_00_00_FF) as f32 / 255.0,
        ]
    }
}

#[test]
fn test_color_f32_conversion() {
    let color: [f32; 4] = Color(0xFF_00_FF_00).into();
    assert_eq!(color, [1.0, 0.0, 1.0, 0.0]);
    let color: [f32; 4] = Color(0x00_FF_00_FF).into();
    assert_eq!(color, [0.0, 1.0, 0.0, 1.0]);
}

/// Colors used in the UI
pub struct Theme {
    /// Text color and default color of foreground elements like lines
    pub fg: Color,
    /// Disabled text and foreground elements like lines
    pub fg_disabled: Color,
    /// Text and elements inside selected items like tabs
    pub fg_selected: Color,
    /// Background color
    pub bg: Color,
    /// Background for child regions
    pub bg_child: Color,
    /// Selected text background color
    pub bg_highlight: Color,
    /// More transparent background color for items which appear over other content
    pub bg_overlay: Color,
    /// Color of borders
    pub border: Color,
    /// Color of an element like a button
    pub element: Color,
    /// Color of an item (like a button) currently being interacted with
    pub active: Color,
    /// Color of an item that has been selected
    pub selected: Color,
    /// Color of an item under the mouse or currently tabbed to
    pub hover: Color,
}

impl Theme {
    /// Default dark theme for UI
    pub const DARK: Self = Theme {
        fg: theme::dark::FG,
        fg_disabled: theme::dark::FG2,
        fg_selected: theme::dark::BG,
        bg: theme::dark::BG,
        bg_child: theme::dark::BG1,
        bg_highlight: theme::dark::BG3,
        bg_overlay: theme::dark::BG2.alpha(100u8),
        border: theme::dark::BG4,
        element: theme::dark::BLUE,
        selected: theme::dark::BLUE,
        hover: theme::dark::AQUA,
        active: theme::dark::BRIGHT_AQUA,
    };

    /// Default light theme for UI
    pub const LIGHT: Self = Theme {
        fg: theme::light::FG,
        fg_disabled: theme::light::FG2,
        fg_selected: theme::light::BG,
        bg: theme::light::BG,
        bg_child: theme::light::BG1,
        bg_highlight: theme::light::BG3,
        bg_overlay: theme::light::BG2.alpha(100u8),
        border: theme::light::BG4,
        element: theme::light::BLUE,
        selected: theme::light::BLUE,
        hover: theme::light::AQUA,
        active: theme::light::BRIGHT_AQUA,
    };
}

/// Colors for the default theme
///
/// Includes:
/// - Base color pallete of white, black, primary and secondary colors
/// - Light and dark theme specific range of foreground and background colors
/// - Light and dark theme specific "Bright" colors
pub mod theme {
    #![allow(clippy::unreadable_literal)]
    #![allow(missing_docs)]

    use super::Color;

    pub const BLACK: Color = Color(0x282828FF);
    pub const GRAY: Color = Color(0x928374FF);
    pub const WHITE: Color = Color(0xFBF1C7FF);
    pub const RED: Color = Color(0xCC241DFF);
    pub const ORANGE: Color = Color(0xD65D0EFF);
    pub const YELLOW: Color = Color(0xD79921FF);
    pub const GREEN: Color = Color(0x98971AFF);
    pub const AQUA: Color = Color(0x689D6AFF);
    pub const BLUE: Color = Color(0x458588FF);
    pub const PURPLE: Color = Color(0xB16286FF);

    /// Dark theme colors for the default theme
    pub mod dark {
        use crate::color::theme as base;
        use crate::color::Color;

        // Background (dark grey and black) colors

        /// Dark background color
        pub const BG: Color = base::BLACK;
        /// High contrast background color
        pub const BGH: Color = Color(0x1D2021FF);
        /// Soft contrast background color
        pub const BGS: Color = Color(0x32302FFF);
        pub const BG1: Color = Color(0x3C3836FF);
        pub const BG2: Color = Color(0x504945FF);
        pub const BG3: Color = Color(0x665C54FF);
        pub const BG4: Color = Color(0x7C6F64FF);

        // Foreground (white and light gray) colors

        /// Light color for text and foreground elements
        pub const FG: Color = base::WHITE;
        pub const FG1: Color = Color(0xEBDBB2FF);
        pub const FG2: Color = Color(0xD5C4A1FF);
        pub const FG3: Color = Color(0xBDAE93FF);
        pub const FG4: Color = Color(0xA89984FF);

        // Primary and Secondary colors

        pub const RED: Color = base::RED;
        pub const BRIGHT_RED: Color = Color(0xFB4934FF);
        pub const GREEN: Color = base::GREEN;
        pub const BRIGHT_GREEN: Color = Color(0xB8BB26FF);
        pub const YELLOW: Color = base::YELLOW;
        pub const BRIGHT_YELLOW: Color = Color(0xFABD2FFF);
        pub const BLUE: Color = base::BLUE;
        pub const BRIGHT_BLUE: Color = Color(0x83A598FF);
        pub const PURPLE: Color = base::PURPLE;
        pub const BRIGHT_PURPLE: Color = Color(0xD3868BFF);
        pub const AQUA: Color = base::AQUA;
        pub const BRIGHT_AQUA: Color = Color(0x8EC07CFF);
        pub const ORANGE: Color = base::ORANGE;
        pub const BRIGHT_ORANGE: Color = Color(0xFE8019FF);
        pub const GRAY: Color = base::GRAY;
        pub const BRIGHT_GRAY: Color = Color(0x928374FF);
    }

    /// Light theme colors for the default theme
    pub mod light {
        use crate::color::theme as base;
        use crate::color::theme::dark;
        use crate::color::Color;

        // Background (white and light gray) colors

        /// Light background color
        pub const BG: Color = base::WHITE;
        /// High contrast background color
        pub const BGH: Color = Color(0xF9F5D7FF);
        /// Soft contrast background color
        pub const BGS: Color = Color(0xF2E5BCFF);
        pub const BG1: Color = dark::FG1;
        pub const BG2: Color = dark::FG2;
        pub const BG3: Color = dark::FG3;
        pub const BG4: Color = dark::FG4;

        // Foreground (black and gray) colors

        /// Dark color for text and foreground elements
        pub const FG: Color = dark::BG;
        pub const FG1: Color = dark::BG1;
        pub const FG2: Color = dark::BG2;
        pub const FG3: Color = dark::BG3;
        pub const FG4: Color = dark::BG4;

        // Primary and Secondary colors

        pub const RED: Color = base::RED;
        pub const BRIGHT_RED: Color = Color(0x9D0006FF);
        pub const GREEN: Color = base::GREEN;
        pub const BRIGHT_GREEN: Color = Color(0x79740EFF);
        pub const YELLOW: Color = base::YELLOW;
        pub const BRIGHT_YELLOW: Color = Color(0xB57614FF);
        pub const BLUE: Color = base::BLUE;
        pub const BRIGHT_BLUE: Color = Color(0x076678FF);
        pub const PURPLE: Color = base::PURPLE;
        pub const BRIGHT_PURPLE: Color = Color(0x8F3F71FF);
        pub const AQUA: Color = base::AQUA;
        pub const BRIGHT_AQUA: Color = Color(0x427B58FF);
        pub const GRAY: Color = Color(0x7C6F64FF);
        pub const BRIGHT_GRAY: Color = base::GRAY;
        pub const ORANGE: Color = base::ORANGE;
        pub const BRIGHT_ORANGE: Color = Color(0xAF3A03FF);
    }
}
