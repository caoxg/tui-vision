use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

use super::{MenuBar, MenuItem};

/// Direction for submenu navigation.
#[derive(Debug, Clone, Copy)]
enum SubmenuNavDirection {
    Up,
    Down,
}

/// Result of handling a key event in the menu system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuEventResult {
    /// Event was not handled by the menu system.
    NotHandled,
    /// Event was handled but no specific action occurred.
    Handled,
    /// A menu was opened.
    MenuOpened { menu_index: usize },
    /// A menu was closed.
    MenuClosed,
    /// Navigation occurred within the menu.
    NavigationChanged,
    /// A menu item was selected.
    ItemSelected { command: String },
    /// A submenu was opened.
    SubmenuOpened { submenu_label: String },
    /// A submenu was closed.
    SubmenuClosed { submenu_label: String },
}

impl MenuBar {
    /// Handles a keyboard event for the menu system.
    ///
    /// Returns a `MenuEventResult` indicating what happened as a result of the key press.
    /// The caller can use this to update status messages, handle commands, etc.
    pub fn handle_key_event(&mut self, key: KeyEvent) -> MenuEventResult {
        match key.code {
            // Menu activation with Alt/Ctrl key combinations
            KeyCode::Char(c)
                if key.modifiers.contains(KeyModifiers::ALT)
                    || key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                self.handle_menu_hotkey(c)
            }

            // Arrow key navigation
            KeyCode::Left => self.handle_left_arrow(),
            KeyCode::Right => self.handle_right_arrow(),
            KeyCode::Down => self.handle_down_arrow(),
            KeyCode::Up => self.handle_up_arrow(),

            // Enter to select item or open submenu
            KeyCode::Enter => self.handle_enter(),

            // Escape to close menu
            KeyCode::Esc => self.handle_escape(),

            // Tab navigation
            KeyCode::Tab => self.handle_tab(key.modifiers.contains(KeyModifiers::SHIFT)),

            // Space bar to activate menu system
            KeyCode::Char(' ') => self.handle_space(),

            // Direct hotkey access (without modifiers)
            KeyCode::Char(c) => self.handle_item_hotkey(c),

            _ => MenuEventResult::NotHandled,
        }
    }

    /// Handles a mouse event for the menu system.
    ///
    /// Returns a `MenuEventResult` indicating what happened as a result of the mouse action.
    /// The caller can use this to update status messages, handle commands, etc.
    ///
    /// Note: Menu title and item positions must be updated during rendering for this to work.
    /// The positions are cached in `menu_title_areas`, `dropdown_area`, and `dropdown_item_areas`.
    pub fn handle_mouse_event(&mut self, mouse: MouseEvent) -> MenuEventResult {
        let x = mouse.column;
        let y = mouse.row;

        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => self.handle_mouse_click(x, y),
            MouseEventKind::Moved => self.handle_mouse_move(x, y),
            _ => MenuEventResult::NotHandled,
        }
    }

    /// Handles a mouse click at the given position.
    fn handle_mouse_click(&mut self, x: u16, y: u16) -> MenuEventResult {
        // First, check if clicked on a menu title in the menu bar
        for (index, area) in self.menu_title_areas.iter().enumerate() {
            if x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height {
                // Toggle menu: if already open, close it; otherwise open it
                if self.opened_menu == Some(index) {
                    self.close_menu();
                    return MenuEventResult::MenuClosed;
                } else {
                    self.open_menu(index);
                    return MenuEventResult::MenuOpened { menu_index: index };
                }
            }
        }

        // Check if clicked on a dropdown item
        if self.has_open_menu() {
            // Check if click is within the dropdown area
            if let Some(dropdown) = self.dropdown_area {
                if x >= dropdown.x
                    && x < dropdown.x + dropdown.width
                    && y >= dropdown.y
                    && y < dropdown.y + dropdown.height
                {
                    // Find which item was clicked (immutable borrow)
                    let clicked_index = self
                        .dropdown_item_areas
                        .iter()
                        .enumerate()
                        .find(|(_, item_area)| {
                            y >= item_area.y && y < item_area.y + item_area.height
                        })
                        .map(|(index, _)| index);

                    // Now handle the clicked item (mutable borrow)
                    if let Some(index) = clicked_index {
                        if let Some(menu) = self.opened_menu_mut() {
                            // Skip separators
                            if matches!(menu.items.get(index), Some(MenuItem::Separator(_))) {
                                return MenuEventResult::Handled;
                            }

                            menu.focused_item = Some(index);

                            // Now handle selection
                            if let Some(item) = menu.items.get_mut(index) {
                                match item {
                                    MenuItem::Action(action) => {
                                        let command = action.command.to_string();
                                        self.close_menu();
                                        return MenuEventResult::ItemSelected { command };
                                    }
                                    MenuItem::SubMenu(submenu) => {
                                        submenu.is_open = !submenu.is_open;
                                        if submenu.is_open {
                                            submenu.focused_item = submenu
                                                .items
                                                .iter()
                                                .position(|item| {
                                                    !matches!(item, MenuItem::Separator(_))
                                                });
                                            return MenuEventResult::SubmenuOpened {
                                                submenu_label: submenu.label.clone(),
                                            };
                                        } else {
                                            submenu.focused_item = None;
                                            return MenuEventResult::SubmenuClosed {
                                                submenu_label: submenu.label.clone(),
                                            };
                                        }
                                    }
                                    MenuItem::Separator(_) => {
                                        return MenuEventResult::Handled;
                                    }
                                }
                            }
                        }
                    }
                    return MenuEventResult::Handled;
                }
            }

            // Clicked outside the dropdown - close the menu
            self.close_menu();
            return MenuEventResult::MenuClosed;
        }

        MenuEventResult::NotHandled
    }

    /// Handles mouse movement for hover effects.
    fn handle_mouse_move(&mut self, x: u16, y: u16) -> MenuEventResult {
        // Check if hovering over a menu title (immutable borrow)
        let hovered_menu_index = self
            .menu_title_areas
            .iter()
            .enumerate()
            .find(|(_, area)| {
                x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
            })
            .map(|(index, _)| index);

        // Update hover state for menu titles
        let hover_changed = self.hovered_menu != hovered_menu_index;
        self.hovered_menu = hovered_menu_index;

        // If a menu is open and we're hovering over a different menu title, switch to it
        if let Some(index) = hovered_menu_index {
            if self.has_open_menu() && self.opened_menu != Some(index) {
                self.open_menu(index);
                return MenuEventResult::MenuOpened { menu_index: index };
            }
            if hover_changed {
                return MenuEventResult::NavigationChanged;
            }
            return MenuEventResult::Handled;
        }

        // If no menu is open, just report navigation change for hover highlight
        if !self.has_open_menu() {
            if hover_changed {
                return MenuEventResult::NavigationChanged;
            }
            return MenuEventResult::NotHandled;
        }

        // Check if hovering over dropdown items
        if let Some(dropdown) = self.dropdown_area {
            if x >= dropdown.x
                && x < dropdown.x + dropdown.width
                && y >= dropdown.y
                && y < dropdown.y + dropdown.height
            {
                // Find which item is being hovered (immutable borrow)
                let hovered_item_index = self
                    .dropdown_item_areas
                    .iter()
                    .enumerate()
                    .find(|(_, item_area)| {
                        y >= item_area.y && y < item_area.y + item_area.height
                    })
                    .map(|(index, _)| index);

                // Now handle the hovered item (mutable borrow)
                if let Some(index) = hovered_item_index {
                    if let Some(menu) = self.opened_menu_mut() {
                        // Skip separators for focus
                        if matches!(menu.items.get(index), Some(MenuItem::Separator(_))) {
                            return MenuEventResult::Handled;
                        }

                        if menu.focused_item != Some(index) {
                            menu.focused_item = Some(index);
                            return MenuEventResult::NavigationChanged;
                        }
                    }
                    return MenuEventResult::Handled;
                }
            }
        }

        MenuEventResult::NotHandled
    }

    /// Helper to get the currently focused submenu if any.
    fn get_focused_submenu_mut(&mut self) -> Option<&mut super::SubMenuItem> {
        let menu = self.opened_menu_mut()?;
        let focused_index = menu.focused_item?;
        match menu.items.get_mut(focused_index)? {
            MenuItem::SubMenu(submenu) => Some(submenu),
            _ => None,
        }
    }

    /// Helper to check if we're currently in an open submenu.
    fn is_in_open_submenu(&self) -> bool {
        if let Some(menu) = self.opened_menu() {
            if let Some(focused_index) = menu.focused_item {
                if let Some(MenuItem::SubMenu(submenu)) = menu.items.get(focused_index) {
                    return submenu.is_open;
                }
            }
        }
        false
    }

    /// Helper to navigate within a submenu.
    fn navigate_submenu(&mut self, direction: SubmenuNavDirection) -> MenuEventResult {
        let submenu = match self.get_focused_submenu_mut() {
            Some(submenu) if submenu.is_open => submenu,
            _ => return MenuEventResult::NotHandled,
        };

        match direction {
            SubmenuNavDirection::Down => {
                if let Some(current) = submenu.focused_item {
                    let next =
                        submenu
                            .items
                            .iter()
                            .enumerate()
                            .skip(current + 1)
                            .find_map(|(i, item)| {
                                if !matches!(item, MenuItem::Separator(_)) {
                                    Some(i)
                                } else {
                                    None
                                }
                            });
                    submenu.focused_item = next.or(submenu.focused_item);
                } else {
                    submenu.focused_item = submenu
                        .items
                        .iter()
                        .position(|item| !matches!(item, MenuItem::Separator(_)));
                }
            }
            SubmenuNavDirection::Up => {
                if let Some(current) = submenu.focused_item {
                    if current > 0 {
                        let prev = submenu
                            .items
                            .iter()
                            .enumerate()
                            .take(current)
                            .rev()
                            .find_map(|(i, item)| {
                                if !matches!(item, MenuItem::Separator(_)) {
                                    Some(i)
                                } else {
                                    None
                                }
                            });
                        submenu.focused_item = prev.or(submenu.focused_item);
                    }
                }
            }
        }
        MenuEventResult::NavigationChanged
    }

    /// Helper to handle submenu item selection.
    fn handle_submenu_item_selection(&mut self) -> Option<MenuEventResult> {
        let submenu = self.get_focused_submenu_mut()?;
        if !submenu.is_open {
            return None;
        }

        let submenu_focused = submenu.focused_item?;
        let submenu_item = submenu.items.get(submenu_focused)?;

        match submenu_item {
            MenuItem::Action(action) => {
                let command = action.command.to_string();
                self.close_menu();
                Some(MenuEventResult::ItemSelected { command })
            }
            _ => Some(MenuEventResult::NotHandled),
        }
    }

    fn handle_menu_hotkey(&mut self, hotkey: char) -> MenuEventResult {
        // Check for menu hotkeys (case insensitive)
        for (index, menu) in self.menus.iter().enumerate() {
            if let Some(menu_hotkey) = menu.hotkey {
                if menu_hotkey.to_ascii_lowercase() == hotkey.to_ascii_lowercase() {
                    self.open_menu(index);
                    return MenuEventResult::MenuOpened { menu_index: index };
                }
            }
        }
        MenuEventResult::NotHandled
    }

    fn handle_left_arrow(&mut self) -> MenuEventResult {
        // Check if we're in a submenu and should close it
        if let Some(submenu) = self.get_focused_submenu_mut() {
            if submenu.is_open {
                submenu.is_open = false;
                submenu.focused_item = None;
                return MenuEventResult::SubmenuClosed {
                    submenu_label: submenu.label.clone(),
                };
            }
        }

        // Move to previous menu if we have an open menu
        if self.has_open_menu() {
            self.open_previous_menu();
            MenuEventResult::NavigationChanged
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_right_arrow(&mut self) -> MenuEventResult {
        // Check if current focused item is a submenu
        if let Some(submenu) = self.get_focused_submenu_mut() {
            if !submenu.is_open {
                // Open the submenu
                submenu.is_open = true;
                submenu.focused_item = submenu
                    .items
                    .iter()
                    .position(|item| !matches!(item, MenuItem::Separator(_)));

                return MenuEventResult::SubmenuOpened {
                    submenu_label: submenu.label.clone(),
                };
            }
        }

        // Move to next menu if we have an open menu
        if self.has_open_menu() {
            self.open_next_menu();
            MenuEventResult::NavigationChanged
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_down_arrow(&mut self) -> MenuEventResult {
        // Try to navigate within submenu first
        if self.is_in_open_submenu() {
            return self.navigate_submenu(SubmenuNavDirection::Down);
        }

        // Regular menu navigation
        if let Some(menu) = self.opened_menu_mut() {
            menu.focus_next_item();
            MenuEventResult::NavigationChanged
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_up_arrow(&mut self) -> MenuEventResult {
        // Try to navigate within submenu first
        if self.is_in_open_submenu() {
            return self.navigate_submenu(SubmenuNavDirection::Up);
        }

        // Regular menu navigation
        if let Some(menu) = self.opened_menu_mut() {
            menu.focus_previous_item();
            MenuEventResult::NavigationChanged
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_enter(&mut self) -> MenuEventResult {
        // First check if we're selecting a submenu item
        if let Some(result) = self.handle_submenu_item_selection() {
            return result;
        }

        // Handle main menu items
        let menu = match self.opened_menu_mut() {
            Some(menu) => menu,
            None => return MenuEventResult::NotHandled,
        };

        let focused_index = match menu.focused_item {
            Some(index) => index,
            None => return MenuEventResult::NotHandled,
        };

        let item = match menu.items.get_mut(focused_index) {
            Some(item) => item,
            None => return MenuEventResult::NotHandled,
        };

        match item {
            MenuItem::Action(action) => {
                let command = action.command.to_string();
                self.close_menu();
                MenuEventResult::ItemSelected { command }
            }
            MenuItem::SubMenu(submenu) => {
                submenu.is_open = !submenu.is_open;
                if submenu.is_open {
                    submenu.focused_item = submenu
                        .items
                        .iter()
                        .position(|item| !matches!(item, MenuItem::Separator(_)));
                    MenuEventResult::SubmenuOpened {
                        submenu_label: submenu.label.clone(),
                    }
                } else {
                    submenu.focused_item = None;
                    MenuEventResult::SubmenuClosed {
                        submenu_label: submenu.label.clone(),
                    }
                }
            }
            MenuItem::Separator(_) => MenuEventResult::NotHandled,
        }
    }

    fn handle_escape(&mut self) -> MenuEventResult {
        if self.has_open_menu() {
            self.close_menu();
            MenuEventResult::MenuClosed
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_tab(&mut self, shift_pressed: bool) -> MenuEventResult {
        if shift_pressed {
            self.open_previous_menu();
        } else {
            self.open_next_menu();
        }
        MenuEventResult::NavigationChanged
    }

    fn handle_space(&mut self) -> MenuEventResult {
        if !self.has_open_menu() {
            self.open_menu(0);
            MenuEventResult::MenuOpened { menu_index: 0 }
        } else {
            MenuEventResult::NotHandled
        }
    }

    fn handle_item_hotkey(&mut self, hotkey: char) -> MenuEventResult {
        if let Some(menu) = self.opened_menu_mut() {
            // Check for item hotkeys within open menu (case insensitive)
            if let Some(index) = find_item_by_hotkey(menu, hotkey) {
                menu.focused_item = Some(index);
                if let Some(item) = menu.get_focused_item() {
                    match item {
                        MenuItem::Action(action) => {
                            let command = action.command.to_string();
                            self.close_menu();
                            MenuEventResult::ItemSelected { command }
                        }
                        MenuItem::SubMenu(_) => MenuEventResult::NavigationChanged,
                        _ => MenuEventResult::NotHandled,
                    }
                } else {
                    MenuEventResult::NotHandled
                }
            } else {
                MenuEventResult::NotHandled
            }
        } else {
            // Check for menu hotkeys (case insensitive)
            for (index, menu) in self.menus.iter().enumerate() {
                if let Some(menu_hotkey) = menu.hotkey {
                    if menu_hotkey.to_ascii_lowercase() == hotkey.to_ascii_lowercase() {
                        self.open_menu(index);
                        return MenuEventResult::MenuOpened { menu_index: index };
                    }
                }
            }
            MenuEventResult::NotHandled
        }
    }
}

/// Finds a menu item by its hotkey character (case insensitive).
fn find_item_by_hotkey(menu: &super::Menu, hotkey: char) -> Option<usize> {
    menu.items.iter().position(|item| {
        if let Some(item_hotkey) = item.hotkey() {
            item_hotkey.to_ascii_lowercase() == hotkey.to_ascii_lowercase()
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{item, menu, menu_bar};

    fn create_test_menu_bar() -> MenuBar {
        menu_bar![
            menu![
                "File",
                'F',
                item![action: "New", command: "file.new", hotkey: 'N'],
                item![action: "Open", command: "file.open", hotkey: 'O'],
                item![submenu: "Export", items: [
                    item![action: "PDF", command: "file.export.pdf", hotkey: 'P'],
                    item![action: "HTML", command: "file.export.html", hotkey: 'H']
                ], hotkey: 'E'],
            ],
            menu![
                "Edit",
                'E',
                item![action: "Undo", command: "edit.undo", hotkey: 'U'],
                item![action: "Redo", command: "edit.redo", hotkey: 'R'],
            ]
        ]
    }

    #[test]
    fn menu_hotkey_with_alt_opens_menu() {
        let mut menu_bar = create_test_menu_bar();
        let key = KeyEvent::new(KeyCode::Char('f'), KeyModifiers::ALT);

        let result = menu_bar.handle_key_event(key);

        assert_eq!(result, MenuEventResult::MenuOpened { menu_index: 0 });
        assert!(menu_bar.has_open_menu());
        assert_eq!(menu_bar.opened_menu, Some(0));
    }

    #[test]
    fn item_hotkey_selects_action() {
        let mut menu_bar = create_test_menu_bar();
        menu_bar.open_menu(0); // Open File menu

        let key = KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE);
        let result = menu_bar.handle_key_event(key);

        assert_eq!(
            result,
            MenuEventResult::ItemSelected {
                command: "file.new".to_string()
            }
        );
        assert!(!menu_bar.has_open_menu());
    }

    #[test]
    fn arrow_keys_navigate_menus() {
        let mut menu_bar = create_test_menu_bar();
        menu_bar.open_menu(0); // Open File menu

        let result = menu_bar.handle_key_event(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        assert_eq!(result, MenuEventResult::NavigationChanged);
        assert_eq!(menu_bar.opened_menu, Some(1)); // Should move to Edit menu
    }

    #[test]
    fn escape_closes_menu() {
        let mut menu_bar = create_test_menu_bar();
        menu_bar.open_menu(0);

        let result = menu_bar.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(result, MenuEventResult::MenuClosed);
        assert!(!menu_bar.has_open_menu());
    }

    #[test]
    fn enter_opens_submenu() {
        let mut menu_bar = create_test_menu_bar();
        menu_bar.open_menu(0);

        // Focus the Export submenu (index 2)
        if let Some(menu) = menu_bar.opened_menu_mut() {
            menu.focused_item = Some(2);
        }

        let result = menu_bar.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(
            result,
            MenuEventResult::SubmenuOpened {
                submenu_label: "Export".to_string()
            }
        );

        // Check that submenu is open
        if let Some(menu) = menu_bar.opened_menu() {
            if let Some(MenuItem::SubMenu(submenu)) = menu.items.get(2) {
                assert!(submenu.is_open);
            } else {
                panic!("Expected submenu at index 2");
            }
        }
    }

    #[test]
    fn space_activates_menu_system() {
        let mut menu_bar = create_test_menu_bar();

        let result =
            menu_bar.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
        assert_eq!(result, MenuEventResult::MenuOpened { menu_index: 0 });
        assert!(menu_bar.has_open_menu());
    }

    #[test]
    fn submenu_item_selection_triggers_item_selected_event() {
        use crate::{item, menu, menu_bar};

        let mut menu_bar = menu_bar![menu![
            "View",
            'V',
            item![submenu: "Theme", items: [
                item![action: "Dark Theme", command: "view.theme.dark", hotkey: 'D'],
                item![action: "Light Theme", command: "view.theme.light", hotkey: 'L']
            ], hotkey: 'T'],
        ]];

        // Open the View menu
        menu_bar.open_menu(0);

        // The submenu should be focused by default, press Enter to open it
        let result = menu_bar.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        // Debug: let's check what we actually got
        match &result {
            MenuEventResult::SubmenuOpened { submenu_label } => {
                assert_eq!(submenu_label, "Theme");
            }
            other => {
                panic!("Expected SubmenuOpened, got: {other:?}");
            }
        }

        // Navigate down to the first theme option (should be focused already)
        let menu = menu_bar.opened_menu().unwrap();
        let submenu = match &menu.items[0] {
            MenuItem::SubMenu(submenu) => submenu,
            _ => panic!("Expected submenu"),
        };
        assert!(submenu.is_open);
        assert_eq!(submenu.focused_item, Some(0)); // Should focus "Dark Theme"

        // Press Enter to select the Dark Theme option
        let result = menu_bar.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        match &result {
            MenuEventResult::ItemSelected { command } => {
                assert_eq!(command, "view.theme.dark");
            }
            other => {
                panic!("Expected ItemSelected, got: {other:?}");
            }
        }

        // Menu should be closed after selection
        assert!(!menu_bar.has_open_menu());
    }
}
