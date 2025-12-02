use ratatui_core::style::{Color, Style};

/// Color scheme for menu rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuTheme {
    /// Style for the menu bar background and text.
    pub menu_bar: Style,
    /// Style for a focused menu title in the menu bar.
    pub menu_bar_focused: Style,
    /// Style for dropdown background and borders.
    pub dropdown: Style,
    /// Style for dropdown borders specifically.
    pub dropdown_border: Style,
    /// Style for normal menu items.
    pub item: Style,
    /// Style for focused/selected menu items.
    pub item_focused: Style,
    /// Style for disabled menu items.
    pub item_disabled: Style,
    /// Style for separators.
    pub separator: Style,
    /// Style for submenu indicators (arrow).
    pub submenu_indicator: Style,
}

impl MenuTheme {
    /// Creates a classic MS-DOS edit.com style theme (cyan/blue).
    pub fn classic() -> Self {
        Self {
            menu_bar: Style::default().bg(Color::Cyan).fg(Color::Black),
            menu_bar_focused: Style::default().bg(Color::Blue).fg(Color::White),
            dropdown: Style::default().bg(Color::Blue).fg(Color::White),
            dropdown_border: Style::default().bg(Color::Blue).fg(Color::White),
            item: Style::default().bg(Color::Blue).fg(Color::White),
            item_focused: Style::default().bg(Color::White).fg(Color::Black),
            item_disabled: Style::default().bg(Color::Blue).fg(Color::DarkGray),
            separator: Style::default().bg(Color::Blue).fg(Color::White),
            submenu_indicator: Style::default().bg(Color::Blue).fg(Color::White),
        }
    }

    /// Creates a modern dark theme.
    pub fn dark() -> Self {
        Self {
            menu_bar: Style::default().bg(Color::Rgb(45, 45, 45)).fg(Color::White),
            menu_bar_focused: Style::default()
                .bg(Color::Rgb(70, 130, 180))
                .fg(Color::White),
            dropdown: Style::default().bg(Color::Rgb(60, 60, 60)).fg(Color::White),
            dropdown_border: Style::default().bg(Color::Rgb(60, 60, 60)).fg(Color::White),
            item: Style::default().bg(Color::Rgb(60, 60, 60)).fg(Color::White),
            item_focused: Style::default()
                .bg(Color::Rgb(70, 130, 180))
                .fg(Color::White),
            item_disabled: Style::default()
                .bg(Color::Rgb(60, 60, 60))
                .fg(Color::Rgb(120, 120, 120)),
            separator: Style::default().bg(Color::Rgb(60, 60, 60)).fg(Color::White),
            submenu_indicator: Style::default().bg(Color::Rgb(60, 60, 60)).fg(Color::White),
        }
    }

    /// Creates a light modern theme.
    pub fn light() -> Self {
        Self {
            menu_bar: Style::default()
                .bg(Color::Rgb(248, 248, 248))
                .fg(Color::Black),
            menu_bar_focused: Style::default()
                .bg(Color::Rgb(70, 130, 180))
                .fg(Color::White),
            dropdown: Style::default().bg(Color::White).fg(Color::Black),
            dropdown_border: Style::default()
                .bg(Color::Rgb(200, 200, 200))
                .fg(Color::Black),
            item: Style::default()
                .bg(Color::Rgb(200, 200, 200))
                .fg(Color::Black),
            item_focused: Style::default()
                .bg(Color::Rgb(70, 130, 180))
                .fg(Color::White),
            item_disabled: Style::default()
                .bg(Color::White)
                .fg(Color::Rgb(150, 150, 150)),
            separator: Style::default()
                .bg(Color::Rgb(200, 200, 200))
                .fg(Color::Black),
            submenu_indicator: Style::default().bg(Color::White).fg(Color::Black),
        }
    }

    /// Creates a vibrant terminal theme with bright colors.
    pub fn terminal() -> Self {
        Self {
            menu_bar: Style::default().bg(Color::Black).fg(Color::Green),
            menu_bar_focused: Style::default().bg(Color::Green).fg(Color::Black),
            dropdown: Style::default().bg(Color::Black).fg(Color::Green),
            dropdown_border: Style::default().bg(Color::Black).fg(Color::Green),
            item: Style::default().bg(Color::Black).fg(Color::Green),
            item_focused: Style::default().bg(Color::Green).fg(Color::Black),
            item_disabled: Style::default().bg(Color::Black).fg(Color::DarkGray),
            separator: Style::default().bg(Color::Black).fg(Color::Green),
            submenu_indicator: Style::default().bg(Color::Black).fg(Color::Green),
        }
    }

    /// Creates an authentic Turbo Vision theme (gray/cyan like Borland IDE).
    pub fn turbo_vision() -> Self {
        Self {
            // Gray menu bar like original Turbo Vision
            menu_bar: Style::default().bg(Color::Gray).fg(Color::Black),
            menu_bar_focused: Style::default().bg(Color::Green).fg(Color::Black),
            // Cyan dropdown like original
            dropdown: Style::default().bg(Color::Cyan).fg(Color::Black),
            dropdown_border: Style::default().bg(Color::Cyan).fg(Color::Black),
            item: Style::default().bg(Color::Cyan).fg(Color::Black),
            item_focused: Style::default().bg(Color::Green).fg(Color::Black),
            item_disabled: Style::default().bg(Color::Cyan).fg(Color::DarkGray),
            separator: Style::default().bg(Color::Cyan).fg(Color::Black),
            submenu_indicator: Style::default().bg(Color::Cyan).fg(Color::Black),
        }
    }

    /// Creates a Catppuccin Mocha inspired theme.
    pub fn catppuccin() -> Self {
        Self {
            menu_bar: Style::default()
                .bg(Color::Rgb(30, 30, 46))    // Base
                .fg(Color::Rgb(205, 214, 244)), // Text
            menu_bar_focused: Style::default()
                .bg(Color::Rgb(137, 180, 250))  // Blue
                .fg(Color::Rgb(30, 30, 46)),    // Base
            dropdown: Style::default()
                .bg(Color::Rgb(49, 50, 68))     // Surface0
                .fg(Color::Rgb(205, 214, 244)), // Text
            dropdown_border: Style::default()
                .bg(Color::Rgb(49, 50, 68))
                .fg(Color::Rgb(137, 180, 250)), // Blue
            item: Style::default()
                .bg(Color::Rgb(49, 50, 68))
                .fg(Color::Rgb(205, 214, 244)),
            item_focused: Style::default()
                .bg(Color::Rgb(137, 180, 250))  // Blue
                .fg(Color::Rgb(30, 30, 46)),
            item_disabled: Style::default()
                .bg(Color::Rgb(49, 50, 68))
                .fg(Color::Rgb(108, 112, 134)), // Overlay0
            separator: Style::default()
                .bg(Color::Rgb(49, 50, 68))
                .fg(Color::Rgb(88, 91, 112)),   // Surface2
            submenu_indicator: Style::default()
                .bg(Color::Rgb(49, 50, 68))
                .fg(Color::Rgb(180, 190, 254)), // Lavender
        }
    }

    /// Creates a Nord inspired theme.
    pub fn nord() -> Self {
        Self {
            menu_bar: Style::default()
                .bg(Color::Rgb(46, 52, 64))     // Nord0
                .fg(Color::Rgb(236, 239, 244)), // Nord6
            menu_bar_focused: Style::default()
                .bg(Color::Rgb(136, 192, 208))  // Nord8
                .fg(Color::Rgb(46, 52, 64)),
            dropdown: Style::default()
                .bg(Color::Rgb(59, 66, 82))     // Nord1
                .fg(Color::Rgb(236, 239, 244)),
            dropdown_border: Style::default()
                .bg(Color::Rgb(59, 66, 82))
                .fg(Color::Rgb(136, 192, 208)),
            item: Style::default()
                .bg(Color::Rgb(59, 66, 82))
                .fg(Color::Rgb(236, 239, 244)),
            item_focused: Style::default()
                .bg(Color::Rgb(136, 192, 208))
                .fg(Color::Rgb(46, 52, 64)),
            item_disabled: Style::default()
                .bg(Color::Rgb(59, 66, 82))
                .fg(Color::Rgb(76, 86, 106)),   // Nord3
            separator: Style::default()
                .bg(Color::Rgb(59, 66, 82))
                .fg(Color::Rgb(76, 86, 106)),
            submenu_indicator: Style::default()
                .bg(Color::Rgb(59, 66, 82))
                .fg(Color::Rgb(129, 161, 193)), // Nord9
        }
    }

    /// Creates a Gruvbox Dark inspired theme.
    pub fn gruvbox() -> Self {
        Self {
            menu_bar: Style::default()
                .bg(Color::Rgb(40, 40, 40))     // bg0
                .fg(Color::Rgb(235, 219, 178)), // fg
            menu_bar_focused: Style::default()
                .bg(Color::Rgb(215, 153, 33))   // yellow
                .fg(Color::Rgb(40, 40, 40)),
            dropdown: Style::default()
                .bg(Color::Rgb(60, 56, 54))     // bg1
                .fg(Color::Rgb(235, 219, 178)),
            dropdown_border: Style::default()
                .bg(Color::Rgb(60, 56, 54))
                .fg(Color::Rgb(215, 153, 33)),
            item: Style::default()
                .bg(Color::Rgb(60, 56, 54))
                .fg(Color::Rgb(235, 219, 178)),
            item_focused: Style::default()
                .bg(Color::Rgb(215, 153, 33))
                .fg(Color::Rgb(40, 40, 40)),
            item_disabled: Style::default()
                .bg(Color::Rgb(60, 56, 54))
                .fg(Color::Rgb(146, 131, 116)), // gray
            separator: Style::default()
                .bg(Color::Rgb(60, 56, 54))
                .fg(Color::Rgb(102, 92, 84)),   // bg3
            submenu_indicator: Style::default()
                .bg(Color::Rgb(60, 56, 54))
                .fg(Color::Rgb(250, 189, 47)),  // bright yellow
        }
    }

    /// Creates a Solarized Dark inspired theme.
    pub fn solarized() -> Self {
        Self {
            menu_bar: Style::default()
                .bg(Color::Rgb(0, 43, 54))      // base03
                .fg(Color::Rgb(131, 148, 150)), // base0
            menu_bar_focused: Style::default()
                .bg(Color::Rgb(38, 139, 210))   // blue
                .fg(Color::Rgb(253, 246, 227)), // base3
            dropdown: Style::default()
                .bg(Color::Rgb(7, 54, 66))      // base02
                .fg(Color::Rgb(147, 161, 161)), // base1
            dropdown_border: Style::default()
                .bg(Color::Rgb(7, 54, 66))
                .fg(Color::Rgb(38, 139, 210)),
            item: Style::default()
                .bg(Color::Rgb(7, 54, 66))
                .fg(Color::Rgb(147, 161, 161)),
            item_focused: Style::default()
                .bg(Color::Rgb(38, 139, 210))
                .fg(Color::Rgb(253, 246, 227)),
            item_disabled: Style::default()
                .bg(Color::Rgb(7, 54, 66))
                .fg(Color::Rgb(88, 110, 117)),  // base01
            separator: Style::default()
                .bg(Color::Rgb(7, 54, 66))
                .fg(Color::Rgb(88, 110, 117)),
            submenu_indicator: Style::default()
                .bg(Color::Rgb(7, 54, 66))
                .fg(Color::Rgb(42, 161, 152)),  // cyan
        }
    }
}

impl Default for MenuTheme {
    fn default() -> Self {
        Self::classic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_presets_have_different_colors() {
        let classic = MenuTheme::classic();
        let dark = MenuTheme::dark();
        let light = MenuTheme::light();
        let terminal = MenuTheme::terminal();

        // Ensure themes are actually different
        assert_ne!(classic.menu_bar, dark.menu_bar);
        assert_ne!(classic.menu_bar, light.menu_bar);
        assert_ne!(classic.menu_bar, terminal.menu_bar);
        assert_ne!(dark.menu_bar, light.menu_bar);
    }

    #[test]
    fn default_theme_is_classic() {
        assert_eq!(MenuTheme::default(), MenuTheme::classic());
    }
}
