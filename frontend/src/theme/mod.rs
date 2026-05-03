use std::rc::Rc;

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone)]
pub struct ThemeContext {
    pub mode: ThemeMode,
    pub toggle_theme: Rc<dyn Fn()>,
}

impl ThemeContext {
    pub fn new(mode: ThemeMode, toggle_theme: Rc<dyn Fn()>) -> Self {
        Self { mode, toggle_theme }
    }

    pub fn is_dark(&self) -> bool {
        self.mode == ThemeMode::Dark
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
    }
}
