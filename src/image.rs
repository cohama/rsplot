extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;

use self::gtk::prelude::*;
use self::gtk::{Image, Window, WindowType};
use self::gdk_pixbuf::Pixbuf;

pub struct ImageViewer {
    window: Window,
    // image: Image,
}

struct PixbufWrapper<'a>(&'a Pixbuf);

impl <'a> Into<Option<&'a Pixbuf>> for PixbufWrapper<'a> {
    fn into(self) -> Option<&'a Pixbuf> {
        Some(self.0)
    }
}

impl ImageViewer {
    pub fn from_data(data: Vec<u8>) -> ImageViewer {
        let _ = gtk::init();
        let window = Window::new(WindowType::Toplevel);
        window.set_title("rsplot");
        window.set_name("mywindow");

        let css = gtk::CssProvider::new();
        css.load_from_data(r#"
            #mywindow {
                background-color: #ffffff;
            }
        "#).unwrap();
        gtk::StyleContext::add_provider_for_screen(&gdk::Screen::get_default().unwrap(), &css, gtk::STYLE_PROVIDER_PRIORITY_USER);

        let pb = Pixbuf::new_from_vec(data, 0, false, 8, 28, 28, 84);
        let pbw = PixbufWrapper(&pb);
        println!("{:?}", pb.get_colorspace());

        window.set_default_size(pb.get_width(), pb.get_height());

        let image = Image::new_from_pixbuf(pbw);
        window.add(&image);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        ImageViewer {
            window,
            // image,
        }
    }

    pub fn show(self) {
        self.window.show_all();
        let w = self.window.get_window().unwrap();
        w.set_override_redirect(true);
        w.move_(10, 10);
        w.show();

        gtk::main();
    }
}
