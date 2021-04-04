#![warn(clippy::all, clippy::pedantic)]
use crate::gdk::ModifierType;
use crate::glib::clone;
use crate::gtk::Orientation::Vertical;
use crate::gtk::WindowType::Toplevel;
use crate::gtk::{prelude::*, ContainerExt, EntryExt, Inhibit, NotebookExt, WidgetExt};
use crate::url::Url;
use crate::webkit2gtk::{CookieManagerExt, LoadEvent, WebContext, WebContextExt, WebViewExt};
use getopts::Matches;
use webkit2gtk::CookiePersistentStorage::Text;

use std::cell::RefCell;
use std::process;
use std::rc::Rc;

use crate::command;
use crate::command::Command;
use crate::CONFIG;
use crate::CONFIGDIR;
use crate::keys::Key;

pub struct Gui {
    pub window: gtk::Window,
    pub notebook: gtk::Notebook,
    pub command_box: gtk::Entry,
    pub webcontext: webkit2gtk::WebContext,
    pub mode: RefCell<String>,
}

fn get_tab_label(uri: &str) -> String {
    let parsed_uri = match Url::parse(uri) {
        Ok(c) => c,
        Err(_) => return uri.to_string(),
    };
    match parsed_uri.host_str() {
        Some(c) => c.to_string(),
        None => uri.to_string(),
    }
}

impl Gui {
    fn new() -> Gui {
        Gui {
            window: gtk::Window::new(Toplevel),
            notebook: gtk::Notebook::new(),
            command_box: gtk::Entry::new(),
            webcontext: if let Some(c) = WebContext::get_default() {
                c
            } else {
                eprintln!("Unable to get default web context, exiting now.");
                process::exit(1);
            },
            mode: RefCell::new(String::from("normal")),
        }
    }

    pub fn new_tab(&self, uri: &str) {
        let web_view = webkit2gtk::WebView::with_context(&self.webcontext);
        web_view.show();
        self.notebook.add(&web_view);
        web_view.load_uri(&command::parse_url(uri));
        let host = get_tab_label(&uri);
        self.notebook.set_tab_label_text(&web_view, &host);
        self.notebook.set_tab_reorderable(&web_view, true);
        self.hide_cmd_box();
        /*web_view.connect_create(clone!(@weak gui |_,action| {
            if let Some(request) = action.get_request() {
                println!("{}", request);
            }
        }));*/
        let notebook = self.notebook.clone();
        let window = self.window.clone();
        web_view.connect_load_changed(clone!(@weak web_view => move |_,load_event| {
            if let Some(uri) = web_view.get_uri() {
                let uri = uri.to_string();
                let uri = if uri.len() > 50 {
                    format!("{}... ", &uri[..50])
                } else {
                    format!("{} ", uri)
                };
                let host = get_tab_label(&uri);
                notebook.set_tab_label_text(&web_view, &host);
                if notebook.page_num(&web_view) == notebook.get_current_page() {
                    if let Some(title) = web_view.get_title() {
                        window.set_title(&format!("RWB - {}", &title));
                    } else {
                        window.set_title(&format!("RWB - {}", &uri));
                    }
                }
            }
            if  load_event == LoadEvent::Finished {
                let cancellable = gio::Cancellable::new();
                let script = include_str!("scripts/disable_forms.js");
                web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                    Ok(_) => {}
                    Err(error) => println!("{}", error),
                });
            }
        }));
    }

    pub fn close_tab(&self) {
        let current_tab = self.notebook.get_current_page();
        let widget = match self.notebook.get_nth_page(current_tab) {
            Some(c) => c,
            None => return,
        };
        self.notebook.remove(&widget);
    }

    pub fn next_tab(&self) {
        self.notebook.next_page();
    }

    pub fn prev_tab(&self) {
        self.notebook.prev_page();
    }

    pub fn get_cmd(&self) {
        self.enter_cmd_mode();
        self.command_box.show();
        self.command_box.set_text(":open ");
        self.command_box.grab_focus();
        self.command_box.set_position(6);
    }

    pub fn get_cmd_new(&self) {
        self.enter_cmd_mode();
        self.command_box.show();
        self.command_box.set_text(":open_new ");
        self.command_box.grab_focus();
        self.command_box.set_position(10);
    }

    pub fn get_cmd_empty(&self) {
        self.enter_cmd_mode();
        self.command_box.show();
        self.command_box.set_text(":");
        self.command_box.grab_focus();
        self.command_box.set_position(2);
    }

    pub fn enter_cmd_mode(&self) {
        if self.mode.borrow().as_str() == "insert" {
            let uri = match self.get_current_uri() {
                Some(c) => c,
                None => String::from("unknown"),
            };
            if let Some(current_web_view) = self.get_current_webview() {
                self.notebook
                    .set_tab_label_text(&current_web_view, &get_tab_label(&uri));
            }
        }
        self.mode.replace(String::from("command"));
    }

    pub fn enter_normal_mode(&self) {
        self.mode.replace(String::from("normal"));
        let uri = match self.get_current_uri() {
            Some(c) => c,
            None => String::from("unknown"),
        };
        if let Some(current_web_view) = self.get_current_webview() {
            self.notebook
                .set_tab_label_text(&current_web_view, &get_tab_label(&uri));
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/disable_forms.js");
            current_web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    pub fn enter_insert_mode(&self) {
        self.mode.replace(String::from("insert"));
        let uri = match self.get_current_uri() {
            Some(c) => c,
            None => String::from("unknown"),
        };
        let label_text = format!(
            "<span foreground=\"white\" background=\"green\">{} [insert]</span>",
            get_tab_label(&uri)
        );
        let tab_label = gtk::Label::new(None);
        tab_label.set_markup(&label_text);
        if let Some(current_web_view) = self.get_current_webview() {
            self.notebook
                .set_tab_label(&current_web_view, Some(&tab_label));
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/enable_forms.js");
            current_web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    fn parse_cmd(&self) {
        let cmd_string = self.command_box.get_text().to_string();
        let command: Command = Command::new(cmd_string);
        match command.command.as_str() {
            ":open" => self.load_uri(&command.uri),
            ":open_new" => self.new_tab(&command.uri),
            _ => {}
        }
        self.enter_normal_mode();
    }

    pub fn hide_cmd_box(&self) {
        self.command_box.set_text("");
        self.command_box.hide();
    }

    fn get_current_tab(&self) -> Option<u32> {
        self.notebook.get_current_page()
    }

    fn get_webview_for_nth(&self, tab: u32) -> Option<webkit2gtk::WebView> {
        let widget = match self.notebook.get_nth_page(Some(tab)) {
            Some(c) => c,
            None => return None,
        };
        if widget
            .clone()
            .upcast::<gtk::Widget>()
            .is::<webkit2gtk::WebView>()
        {
            Some(widget.downcast::<webkit2gtk::WebView>().unwrap())
        } else {
            None
        }
    }

    fn get_current_webview(&self) -> Option<webkit2gtk::WebView> {
        let current_tab = self.notebook.get_current_page();
        let widget = match self.notebook.get_nth_page(current_tab) {
            Some(c) => c,
            None => return None,
        };
        if widget
            .clone()
            .upcast::<gtk::Widget>()
            .is::<webkit2gtk::WebView>()
        {
            Some(widget.downcast::<webkit2gtk::WebView>().unwrap())
        } else {
            None
        }
    }

    fn get_current_uri(&self) -> Option<String> {
        if let Some(web_view) = self.get_current_webview() {
            if let Some(uri) = web_view.get_uri() {
                Some(String::from(uri))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn load_uri(&self, uri: &str) {
        let parsed_uri = command::parse_url(uri);
        let tab = self.get_current_tab().unwrap();
        if let Some(web_view) = self.get_webview_for_nth(tab) {
            web_view.load_uri(&parsed_uri);
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/disable_forms.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
            let host = get_tab_label(&uri);
            self.notebook.set_tab_label_text(&web_view, &host);
            self.set_window_title();
            self.hide_cmd_box();
            self.enter_normal_mode();
        }
    }

    fn set_window_title(&self) {
        if let Some(webview) = self.get_current_webview() {
            if let Some(title) = webview.get_title() {
                self.window.set_title(&format!("RWB - {}", &title));
            }
        }
    }

    fn set_current_tab_title(&self) {
        if let Some(web_view) = self.get_current_webview() {
            if let Some(uri) = web_view.get_uri() {
                let host = get_tab_label(&uri);
                self.notebook.set_tab_label_text(&web_view, &host);
            }
        }
    }

    pub fn reload_page(&self) {
        if let Some(web_view) = self.get_current_webview() {
            web_view.reload();
        }
    }

    pub fn go_back(&self) {
        if let Some(web_view) = self.get_current_webview() {
            if web_view.can_go_back() {
                web_view.go_back();
                self.set_current_tab_title();
                self.set_window_title();
            }
        }
    }

    pub fn go_forward(&self) {
        if let Some(web_view) = self.get_current_webview() {
            if web_view.can_go_forward() {
                web_view.go_forward();
                self.set_current_tab_title();
                self.set_window_title();
            }
        }
    }

    pub fn copy_url(&self) {
        if let Some(web_view) = self.get_current_webview() {
            if let Some(uri) = web_view.get_uri() {
                let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
                clipboard.set_text(&uri);
            }
        }
    }

    pub fn scroll_down(&self) {
        if let Some(web_view) = self.get_current_webview() {
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/scroll_down.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    pub fn scroll_up(&self) {
        if let Some(web_view) = self.get_current_webview() {
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/scroll_up.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    pub fn scroll_right(&self) {
        if let Some(web_view) = self.get_current_webview() {
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/scroll_right.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    pub fn scroll_left(&self) {
        if let Some(web_view) = self.get_current_webview() {
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/scroll_left.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    pub fn scroll_page_down(&self) {
        if let Some(web_view) = self.get_current_webview() {
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/scroll_page_down.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    pub fn scroll_page_up(&self) {
        if let Some(web_view) = self.get_current_webview() {
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/scroll_page_up.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    pub fn scroll_half_page_down(&self) {
        if let Some(web_view) = self.get_current_webview() {
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/scroll_half_page_down.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    pub fn scroll_half_page_up(&self) {
        if let Some(web_view) = self.get_current_webview() {
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/scroll_half_page_up.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    pub fn scroll_bottom(&self) {
        if let Some(web_view) = self.get_current_webview() {
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/scroll_bottom.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    pub fn scroll_top(&self) {
        if let Some(web_view) = self.get_current_webview() {
            let cancellable = gio::Cancellable::new();
            let script = include_str!("scripts/scroll_top.js");
            web_view.run_javascript(&script, Some(&cancellable), |result| match result {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            });
        }
    }

    /// By default cookies are not saved across sessions. This behavior can be
    /// changed if desired by simply creating ~/.config/rwb/cookies. If present,
    /// this will be set up as the persistent storage location. Reversing is as
    /// simple as deleting the file.
    fn setup_cookies_storage(&self) {
        let mut cookiesfile = CONFIGDIR.clone();
        cookiesfile.push("cookies");
        if cookiesfile.exists() {
            if let Ok(md) = cookiesfile.metadata() {
                if ! md.permissions().readonly() {
                    let cookiemgr = if let Some(c) = self.webcontext.get_cookie_manager() {
                        c
                    } else {
                        eprintln!("Unable to get cookie manager, exiting");
                        process::exit(1);
                    };
                    cookiemgr.set_persistent_storage(cookiesfile.to_str().unwrap(), Text);
                }
            }
        }
    }
}

pub fn run(args: Matches) {
    let uri = if args.free.len() == 1 {
        &args.free[0]
    } else {
        match &CONFIG.homepage {
            Some(c) => c,
            None => "https://duckduckgo.com",
        }
    };
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let gui = Rc::new(Gui::new());

    if ! args.opt_present("p") {
        gui.setup_cookies_storage();
    }

    let vbox = gtk::Box::new(Vertical, 0);
    gui.notebook.set_scrollable(true);
    gui.notebook.set_property_enable_popup(true);
    gui.notebook.set_show_tabs(false);
    vbox.pack_start(&gui.notebook, true, true, 0);
    vbox.pack_start(&gui.command_box, false, false, 0);

    gui.window.set_default_geometry(800, 600);
    gui.window.add(&vbox);
    gui.window.show_all();
    gui.command_box.hide();

    gui.new_tab(&uri);
    gui.set_window_title();

    gui.command_box
        .connect_activate(clone!(@weak gui => move |_| {
            gui.enter_normal_mode();
            gui.parse_cmd();
        }));

    gui.notebook
        .connect_switch_page(clone!(@weak gui => move |_,_,page| {
            gui.enter_normal_mode();
            if let Some(web_view) = gui.get_webview_for_nth(page) {
                if let Some(title) = web_view.get_title() {
                    gui.window.set_title(&format!("RWB - {}", &title));
                }
            }
            let pages = gui.notebook.get_n_pages();
            for page in 0..pages {
                if let Some(web_view) = gui.get_webview_for_nth(page) {
                    if let Some(uri) = web_view.get_uri() {
                        let host = get_tab_label(&uri);
                        gui.notebook.set_tab_label_text(&web_view, &host);
                    }
                }
            }
        }));

    gui.notebook
        .connect_page_removed(clone!(@weak gui => move |_,_,_| {
            match gui.notebook.get_children().len() {
                0 => gtk::main_quit(),
                1 => gui.notebook.set_show_tabs(false),
                _ => gui.notebook.set_show_tabs(true),
            };
        }));

    gui.notebook
        .connect_page_added(clone!(@weak gui => move |_,_,_| {
            if gui.notebook.get_children().len() > 1 {
                gui.notebook.set_show_tabs(true);
            }
        }));

    let clone = gui.clone();
    gui.window.connect_key_press_event(move |_, gdk| {
        let key = Key {
            key: gdk.get_keycode().unwrap(),
            ctrl: gdk.get_state().contains(ModifierType::CONTROL_MASK),
            shift: gdk.get_state().contains(ModifierType::SHIFT_MASK),
        };
        key.process_keypress(clone.clone());
        Inhibit(false)
    });

    let clone = gui.clone();
    gui.window.connect_key_release_event(move |_, gdk| {
        let key = Key {
            key: gdk.get_keycode().unwrap(),
            ctrl: gdk.get_state().contains(ModifierType::CONTROL_MASK),
            shift: gdk.get_state().contains(ModifierType::SHIFT_MASK),
        };
        key.process_keyrelease(clone.clone());
        Inhibit(false)
    });

    gui.window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main()
}
