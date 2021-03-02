use crate::gui::Gui;
use crate::CONFIG;
use std::rc::Rc;

const ESC: u16 = 9;
const TAB: u16 = 23;
const COLON: u16 = 47;
const PAGE_UP: u16 = 112;
const PAGE_DOWN: u16 = 117;
const UP: u16 = 111;
const DOWN: u16 = 116;
const LEFT: u16 = 113;
const RIGHT: u16 = 114;
const HOME: u16 = 110;
const END: u16 = 115;
const Q: u16 = 24;
const W: u16 = 25;
//const E: u16 = 26
const R: u16 = 27;
const T: u16 = 28;
const Y: u16 = 29;
const U: u16 = 30;
const I: u16 = 31;
const O: u16 = 32;
//const P: u16 = 33;
//const A: u16 = 38;
//const S: u16 = 39;
const D: u16 = 40;
const F: u16 = 41;
//const G: u16 = 42;
const H: u16 = 43;
const J: u16 = 44;
const K: u16 = 45;
const L: u16 = 46;
//const Z: u16 = 52;
//const X: u16 = 53;
//const C: u16 = 54;
//const V: u16 = 55;
const B: u16 = 56;
const N: u16 = 57;
//const M: u16 = 58;

pub struct Key {
    pub key: u16,
    pub ctrl: bool,
    pub shift: bool,
}

impl Key {
    pub fn process_keypress(&self, gui: Rc<Gui>) {
        let mode = gui.mode.borrow().to_string();
        /* Global - all modes */
        if !self.ctrl && !self.shift {
            // No modifiers
            match self.key {
                ESC => {
                    gui.enter_normal_mode();
                    gui.hide_cmd_box();
                }
                _ => {}
            }
        } else if self.ctrl && !self.shift {
            // Ctrl
            match self.key {
                Q => gtk::main_quit(),
                W => gui.close_tab(),
                N => gui.new_tab("about:blank"),
                T => gui.new_tab(CONFIG.global.get("homepage").unwrap()),
                TAB => gui.next_tab(),
                _ => {}
            }
        }
        /* Normal mode */
        if mode == "normal" {
            if !self.ctrl && !self.shift {
                // No Modifiers
                match self.key {
                    D => gui.close_tab(),
                    R => gui.reload_page(),
                    U => gui.go_back(),
                    I => gui.enter_insert_mode(),
                    J => gui.scroll_down(),
                    K => gui.scroll_up(),
                    H => gui.scroll_left(),
                    L => gui.scroll_right(),
                    Y => gui.copy_url(),
                    PAGE_UP => gui.scroll_page_up(),
                    PAGE_DOWN => gui.scroll_page_down(),
                    UP => gui.scroll_up(),
                    DOWN => gui.scroll_down(),
                    LEFT => gui.scroll_left(),
                    RIGHT => gui.scroll_right(),
                    HOME => gui.scroll_top(),
                    END => gui.scroll_bottom(),
                    _ => {}
                }
            } else if self.ctrl && !self.shift {
                // Ctrl
                match self.key {
                    R => gui.go_forward(),
                    F => gui.scroll_page_down(),
                    B => gui.scroll_page_up(),
                    D => gui.scroll_half_page_down(),
                    U => gui.scroll_half_page_up(),
                    _ => {}
                }
            } else if !self.ctrl && self.shift {
                // Shift
                match self.key {
                    L => gui.next_tab(),
                    H => gui.prev_tab(),
                    J => gui.scroll_bottom(),
                    K => gui.scroll_top(),
                    _ => {}
                }
            } else if self.ctrl && self.shift {
                // Ctrl-Shift
                match self.key {
                    J => gui.scroll_bottom(),
                    K => gui.scroll_top(),
                    _ => {}
                }
            }
        }
    }

    pub fn process_keyrelease(&self, gui: Rc<Gui>) {
        let mode = gui.mode.borrow().to_string();
        /* Normal mode */
        if mode == "normal" {
            match self.key {
                O if !self.ctrl && !self.shift => gui.get_cmd(),
                O if !self.ctrl && self.shift => gui.get_cmd_new(),
                COLON if !self.ctrl && self.shift => gui.get_cmd_empty(),
                _ => {}
            }
        }
    }
}
