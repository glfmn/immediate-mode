//! Color type and default themes

pub use theme::*;

/// RGBA Color as hex
#[derive(Copy, Clone, Debug)]
pub struct Color(pub u32);

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
    /// Text color and color of foreground elements like lines
    pub fg: Color,
    /// Text color of disabled foreground elements
    pub fg_disabled: Color,
    /// Background color
    pub bg: Color,
    /// Color of borders
    pub border: Color,
    /// Color of an element like a button
    pub element: Color,
    /// Color of an item (like a button) currently being interacted with
    pub hot: Color,
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
        bg: theme::dark::BG,
        border: theme::dark::BG2,
        element: theme::dark::BLUE,
        selected: theme::dark::BRIGHT_BLUE,
        hover: theme::dark::AQUA,
        hot: theme::dark::BRIGHT_AQUA,
    };

    /// Default light theme for UI
    pub const LIGHT: Self = Theme {
        fg: theme::light::FG,
        fg_disabled: theme::light::FG2,
        bg: theme::light::BG,
        border: theme::light::BG2,
        element: theme::light::BLUE,
        selected: theme::light::BRIGHT_BLUE,
        hover: theme::light::AQUA,
        hot: theme::light::BRIGHT_AQUA,
    };
}

/// The theme themes
pub mod theme {
    #![allow(clippy::unreadable_literal)]
    #![allow(missing_docs)]

    use super::Color;

    pub const BLACK: Color = Color(0x282828FF);
    pub const WHITE: Color = Color(0xFBF1C7FF);
    pub const RED: Color = Color(0xCC241DFF);
    pub const GREEN: Color = Color(0x98971AFF);
    pub const YELLOW: Color = Color(0xD79921FF);
    pub const BLUE: Color = Color(0x458588FF);
    pub const PURPLE: Color = Color(0xB16286FF);
    pub const AQUA: Color = Color(0x689D6AFF);
    pub const ORANGE: Color = Color(0xD65D0EFF);
    pub const GRAY: Color = Color(0x928374FF);

    /// The dark version of the theme theme
    pub mod dark {
        use crate::color::theme as base;
        use crate::color::Color;

        // Background (dark grey and black) colors

        pub const BG: Color = base::BLACK;
        pub const BGH: Color = Color(0x1D2021FF);
        pub const BGS: Color = Color(0x32302FFF);
        pub const BG1: Color = Color(0x3C3836FF);
        pub const BG2: Color = Color(0x504945FF);
        pub const BG3: Color = Color(0x665C54FF);
        pub const BG4: Color = Color(0x7C6F64FF);

        // Foreground (white and light gray) colors

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

    pub mod light {
        use crate::color::theme as base;
        use crate::color::theme::dark;
        use crate::color::Color;

        // Background (white and light gray) colors

        pub const BG: Color = base::WHITE;
        pub const BGS: Color = Color(0xF2E5BCFF);
        pub const BGH: Color = Color(0xF9F5D7FF);
        pub const BG1: Color = dark::FG1;
        pub const BG2: Color = dark::FG2;
        pub const BG3: Color = dark::FG3;
        pub const BG4: Color = dark::FG4;

        // Foreground (black and gray) colors

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
