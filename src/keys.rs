use crate::gui::Gui;
use std::rc::Rc;

const ESC: u16 = 9;
const TAB: u16 = 23;
const Q: u16 = 24;
const W: u16 = 25;
//const E: u16 = 26
const R: u16 = 27;
const T: u16 = 28;
//const Y: u16 = 29;
//const U: u16 = 30;
//const I: u16 = 31;
const O: u16 = 32;
//const P: u16 = 33;
//const A: u16 = 38;
//const S: u16 = 39;
//const D: u16 = 40;
//const F: u16 = 41;
//const G: u16 = 42;
const H: u16 = 43;
const J: u16 = 44;
const K: u16 = 45;
const L: u16 = 46;
//const Z: u16 = 52;
//const X: u16 = 53;
//const C: u16 = 54;
//const V: u16 = 55;
//const B: u16 = 56;
const N: u16 = 57;
//const M: u16 = 58;

pub struct Key {
    pub key: u16,
    pub ctrl: bool,
    pub shift: bool,
}

impl Key {
    pub fn process_keypress(&self, gui: Rc<Gui>) {
        if self.ctrl && !self.shift {          // Ctrl
            match self.key {
                Q => gtk::main_quit(),
                W => gui.close_tab(),
                R => gui.reload_page(),
                N => gui.new_tab("about:blank"),
                O => gui.get_cmd(),
                H => gui.go_back(),
                L => gui.go_forward(),
                T => gui.new_tab("http://google.com"),
                TAB => gui.next_tab(),
                _ => {},
            }
        } else if self.ctrl && self.shift {     // Ctrl-Shift
            match self.key {
                O => gui.get_cmd_new(),
                J => gui.next_tab(),
                K => gui.prev_tab(),
                _ => {},
            }
        } else if !self.ctrl && !self.shift {   // No modifiers
            match self.key {
                ESC => gui.hide_cmd_box(),
                _ => {},
            }
        }
    }
}
