#![warn(clippy::all, clippy::pedantic)]
use crate::gui::{ Script, Gui };
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
            if let ESC = self.key {
                gui.enter_normal_mode();
                gui.hide_cmd_box();
            }
        } else if self.ctrl && !self.shift {
            // Ctrl
            match self.key {
                Q => gtk::main_quit(),
                W => gui.close_tab(),
                N => gui.new_tab("about:blank"),
                T => {
                    let uri = match &CONFIG.homepage {
                        Some(c) => c,
                        None => "https://duckduckgo.com",
                    };
                    gui.new_tab(uri);
                },
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
                    J | DOWN => gui.run_script(Script::Down),
                    K | UP => gui.run_script(Script::Up),
                    H | LEFT => gui.run_script(Script::Left),
                    L | RIGHT => gui.run_script(Script::Right),
                    Y => gui.copy_url(),
                    PAGE_UP => gui.run_script(Script::PageUp),
                    PAGE_DOWN => gui.run_script(Script::PageDown),
                    HOME => gui.run_script(Script::Top),
                    END => gui.run_script(Script::Bottom),
                    _ => {}
                }
            } else if self.ctrl && !self.shift {
                // Ctrl
                match self.key {
                    R => gui.go_forward(),
                    F => gui.run_script(Script::PageDown),
                    B => gui.run_script(Script::PageUp),
                    D => gui.run_script(Script::HalfPageDown),
                    U => gui.run_script(Script::HalfPageUp),
                    _ => {}
                }
            } else if !self.ctrl && self.shift {
                // Shift
                match self.key {
                    L => gui.next_tab(),
                    H => gui.prev_tab(),
                    J => gui.run_script(Script::Bottom),
                    K => gui.run_script(Script::Top),
                    _ => {}
                }
            } else if self.ctrl && self.shift {
                // Ctrl-Shift
                match self.key {
                    J => gui.run_script(Script::Bottom),
                    K => gui.run_script(Script::Top),
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
                I if !self.ctrl && !self.shift => gui.enter_insert_mode(),
                COLON if !self.ctrl && self.shift => gui.get_cmd_empty(),
                _ => {}
            }
        }
    }
}
