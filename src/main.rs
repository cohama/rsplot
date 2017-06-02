extern crate gtk;
extern crate gdk;
extern crate cairo;

use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_name("mywindow");
    window.set_default_size(350, 70);

    let css = gtk::CssProvider::new();
    css.load_from_data(r#"
#mywindow {
    background-color: #ffffff;
}
    "#).unwrap();
    gtk::StyleContext::add_provider_for_screen(&gdk::Screen::get_default().unwrap(), &css, gtk::STYLE_PROVIDER_PRIORITY_USER);

    let drawingarea = DrawingArea::new();
    drawingarea.set_name("plot-area");
    // let stylecontext = drawingarea.get_style_context().unwrap();
    // stylecontext.add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_USER);
    drawingarea.connect_draw(|w, c| {
        let width = w.get_allocated_width();
        let height = w.get_allocated_height();

        let xaxis = Axis {
            from: -20.0, to: 40.0, label: Some("x axis".to_string()), cross_at: 0.0
        };
        let yaxis = Axis {
            from: -10.0, to: 20.0, label: Some("y axis".to_string()), cross_at: 0.0
        };
        let drawer = CairoDrawer::new(width, height, c);
        drawer.draw_axes(xaxis, yaxis);

        Inhibit(true)
    });
    window.add(&drawingarea);

    window.show_all();
    let w = window.get_window().unwrap();
    w.set_override_redirect(true);
    w.move_(10, 10);
    w.resize(300, 400);
    w.show();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

struct Axis {
    from: f64,
    to: f64,
    label: Option<String>,
    cross_at: f64
}

struct DrawingConf {
    axis_color: (f64, f64, f64),
    axis_margin_px: i32
}

static DEFAULT_DRAWING_CONF: DrawingConf = DrawingConf {
    axis_color: (0.0, 0.0, 0.0),
    axis_margin_px: 10
};

struct CairoDrawer<'a, 'b> {
    width: i32,
    height: i32,
    context: &'a cairo::Context,
    conf: &'b DrawingConf
}

impl <'a, 'b> CairoDrawer<'a, 'b> {
    pub fn new(width: i32, height: i32, context: &'a cairo::Context) -> CairoDrawer<'a, 'static> {
        CairoDrawer {width, height, context, conf: &DEFAULT_DRAWING_CONF}
    }

    pub fn draw_axes(&self, xax: Axis, yax: Axis) {
        let (r, g, b) = self.conf.axis_color;
        self.context.set_source_rgb(r, g, b);
        let h_range = xax.to - xax.from;
        let v_range = yax.to - yax.from;

        let f2p = |x, y| {
            let (w, h) = (self.width as f64, self.height as f64);
            let m = self.conf.axis_margin_px as f64;
            let (xt, yt) = (xax.to, yax.to);
            let (xf, yf) = (xax.from, yax.from);
            ((w - 2.0*m) / (xt - xf) * (x - xf) + m,
             (h - 2.0*m) / (yt - yf) * (yt - y) + m)
        };

        let (hx0, hy0) = f2p(xax.from, yax.cross_at);
        let (hx1, hy1) = f2p(xax.to, yax.cross_at);
        let (vx0, vy0) = f2p(xax.cross_at, yax.from);
        let (vx1, vy1) = f2p(xax.cross_at, yax.to);

        self.context.move_to(hx0, hy0);
        self.context.line_to(hx1, hy1);

        self.context.move_to(vx0, vy0);
        self.context.line_to(vx1, vy1);

        println!("{:?}", (hx0, hy0));


        self.context.stroke();
    }
}



