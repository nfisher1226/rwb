use clap::{crate_version, load_yaml, App};
//use url::{Url, Host, Position};
extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate url;
extern crate webkit2gtk;
use crate::glib::clone;
use crate::gdk::ModifierType;
use crate::gtk::{
    prelude::*, ContainerExt, EntryExt, Inhibit, NotebookExt, WidgetExt,
};
use crate::gtk::Orientation::Vertical;
use crate::gtk::WindowType::Toplevel;
use crate::url::Url;
use crate::webkit2gtk::{ LoadEvent, WebViewExt };

use std::rc::Rc;

mod keys;
use crate::keys::Key;

pub struct Gui {
    pub window: gtk::Window,
    pub notebook: gtk::Notebook,
    pub command_box: gtk::Entry,
}

fn parse_url(url: &str) -> String {
    let url_split = url.split(':').collect::<Vec<&str>>();
    if url_split.len() < 1 {
        return String::from("about:blank")
    }
    match url_split[0] {
        "http" => url.to_string(),
        "https" => url.to_string(),
        "file" => url.to_string(),
        "ftp" => url.to_string(),
        "about" => url.to_string(),
        _ => format!("http://{}", url),
    }
}

fn get_tab_label(uri: &str) -> String {
    let url = match Url::parse(uri) {
        Ok(c) => c,
        Err(_) => return uri.to_string(),
    };
    match url.host_str() {
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
        }
    }

    fn new_tab(&self, uri: &str) {
        let web_view = webkit2gtk::WebView::new();
        web_view.show();
        self.notebook.add(&web_view);
        let tab = self.notebook.page_num(&web_view);
        web_view.load_uri(&parse_url(uri));
        let host = get_tab_label(&uri);
        self.notebook.set_tab_label_text(&web_view, &host);
        self.hide_cmd_box();
        let window = self.window.clone();
        let notebook = self.notebook.clone();
        web_view.connect_load_changed(clone!(@weak web_view, @weak notebook  => move |_,_load_event| {
            if let Some(uri) = web_view.get_uri() {
                let uri = uri.to_string();
                let uri = if uri.len() > 50 {
                    format!("{}... ", &uri[..50])
                } else {
                    format!("{} ", uri)
                };
                let host = get_tab_label(&uri);
                notebook.set_tab_label_text(&web_view, &host);
                if tab == notebook.get_current_page() {
                    if let Some(title) = web_view.get_title() {
                        window.set_title(&format!("RWB - {}", &title));
                    } else {
                        window.set_title(&format!("RWB - {}", &uri));
                    }
                }
            }
        }));
    }

    fn close_tab(&self) {
        let current_tab = self.notebook.get_current_page();
        let widget = match self.notebook.get_nth_page(current_tab) {
            Some(c) => c,
            None => return,
        };
        self.notebook.remove(&widget);
    }

    fn next_tab(&self) {
        self.notebook.next_page();
    }

    fn prev_tab(&self) {
        self.notebook.prev_page();
    }

    fn get_cmd(&self) {
        self.command_box.show();
        self.command_box.set_text(":open ");
        self.command_box.grab_focus();
        self.command_box.set_position(6);
    }

    fn get_cmd_new(&self) {
        self.command_box.show();
        self.command_box.set_text(":open_new ");
        self.command_box.grab_focus();
        self.command_box.set_position(10);
    }

    fn parse_cmd(&self) {
        let cmd_string = self.command_box
            .get_text()
            .to_string();
        let cmd_string: Vec<&str> = cmd_string.split_whitespace().collect();
        let cmd = cmd_string[0];
        let uri = if cmd_string.len() <=1 {
            "about:blank"
        } else {
            cmd_string[1]
        };
        match cmd {
            ":open" => self.load_uri(uri),
            ":open_new" => self.new_tab(uri),
            _ => {},
        }
    }

    fn hide_cmd_box(&self) {
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
        if widget.clone().upcast::<gtk::Widget>().is::<webkit2gtk::WebView>() {
            Some(widget.clone().downcast::<webkit2gtk::WebView>().unwrap())
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
        if widget.clone().upcast::<gtk::Widget>().is::<webkit2gtk::WebView>() {
            Some(widget.clone().downcast::<webkit2gtk::WebView>().unwrap())
        } else {
            None
        }
    }

    fn load_uri(&self, uri: &str) {
        let parsed_uri = parse_url(uri);
        let tab = self.get_current_tab().unwrap();
        if let Some(web_view) = self.get_webview_for_nth(tab) {
            web_view.load_uri(&parsed_uri);
            let host = get_tab_label(&uri);
            self.notebook.set_tab_label_text(&web_view, &host);
            self.set_window_title();
            self.hide_cmd_box();
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

    fn reload_page(&self) {
        if let Some(web_view) = self.get_current_webview() {
            web_view.reload();
        }
    }

    fn go_back(&self) {
        if let Some(web_view) = self.get_current_webview() {
            if web_view.can_go_back() {
                web_view.go_back();
                self.set_current_tab_title();
                self.set_window_title();
            }
        }
    }

    fn go_forward(&self) {
        if let Some(web_view) = self.get_current_webview() {
            if web_view.can_go_forward() {
                web_view.go_forward();
                self.set_current_tab_title();
                self.set_window_title();
            }
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).version(crate_version!()).get_matches();
    let uri = matches.value_of("URI").unwrap();

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let gui = Rc::new(Gui::new());

    let vbox = gtk::Box::new(Vertical, 0);
    vbox.pack_start(&gui.notebook, true, true, 0);
    vbox.pack_start(&gui.command_box, false, false, 0);

    gui.window.set_default_geometry(800, 600);
    gui.window.add(&vbox);
    gui.window.show_all();
    gui.command_box.hide();
    gui.new_tab(&uri);
    gui.set_window_title();

    gui.command_box.connect_activate(clone!(@weak gui => move |_| {
        gui.parse_cmd();
    }));

    gui.notebook.connect_switch_page(clone!(@weak gui => move |_,_,page| {
        if let Some(webview) = gui.get_webview_for_nth(page) {
            if let Some(title) = webview.get_title() {
                gui.window.set_title(&format!("RWB - {}", &title));
            }
        }
    }));

    let clone = gui.clone();
    gui.window.connect_key_release_event(move |_, gdk| {
        let key = Key {
            key: gdk.get_keycode().unwrap(),
            ctrl: gdk.get_state().contains(ModifierType::CONTROL_MASK),
            shift: gdk.get_state().contains(ModifierType::SHIFT_MASK),
        };
        key.process_keypress(clone.clone());
        Inhibit(false)
    });

    gtk::main()


}
