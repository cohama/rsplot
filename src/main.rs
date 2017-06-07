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
            from: -20.0, to: 40.0, cross_at: 0.0
        };
        let yaxis = Axis {
            from: -10.0, to: 20.0, cross_at: 0.0
        };
        let drawer = CairoDrawer::new(width, height, xaxis, yaxis, c);
        drawer.draw_axes();

        let points = (0..20).map(|x| (3*x - 20) as f64).map(|x| (x, 0.02 * x * x - 8.0));
        drawer.draw_points(points);

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
    // label: Option<String>,
    cross_at: f64
}

struct DrawingConf {
    axis_color: (f64, f64, f64),
    plot_color: (f64, f64, f64),
    axis_margin_px: i32
}

static DEFAULT_DRAWING_CONF: DrawingConf = DrawingConf {
    axis_color: (0.0, 0.0, 0.0),
    plot_color: (0.3, 0.7, 0.6),
    axis_margin_px: 10
};

struct CairoDrawer<'a, 'b> {
    width: i32,
    height: i32,
    xaxis: Axis,
    yaxis: Axis,
    context: &'a cairo::Context,
    conf: &'b DrawingConf
}

impl <'a, 'b> CairoDrawer<'a, 'b> {

    pub fn new(width: i32, height: i32, xaxis: Axis, yaxis: Axis, context: &'a cairo::Context) -> CairoDrawer<'a, 'static> {
        CairoDrawer {
            width,
            height,
            xaxis,
            yaxis,
            context,
            conf: &DEFAULT_DRAWING_CONF
        }
    }

    pub fn f2p(&self, x: f64, y: f64) -> (f64, f64) {
        let (w, h) = (self.width as f64, self.height as f64);
        let m = self.conf.axis_margin_px as f64;
        let (xt, yt) = (self.xaxis.to, self.yaxis.to);
        let (xf, yf) = (self.xaxis.from, self.yaxis.from);
        ((w - 2.0*m) / (xt - xf) * (x - xf) + m,
         (h - 2.0*m) / (yt - yf) * (yt - y) + m)
    }

    pub fn draw_axes(&self) {
        let (r, g, b) = self.conf.axis_color;
        self.context.set_source_rgb(r, g, b);

        let (hx0, hy0) = self.f2p(self.xaxis.from, self.yaxis.cross_at);
        let (hx1, hy1) = self.f2p(self.xaxis.to, self.yaxis.cross_at);
        let (vx0, vy0) = self.f2p(self.xaxis.cross_at, self.yaxis.from);
        let (vx1, vy1) = self.f2p(self.xaxis.cross_at, self.yaxis.to);

        self.context.move_to(hx0, hy0);
        self.context.line_to(hx1, hy1);

        self.context.move_to(vx0, vy0);
        self.context.line_to(vx1, vy1);

        println!("{:?}", (hx0, hy0));


        self.context.stroke();
    }

    pub fn draw_points<I: IntoIterator<Item=(f64, f64)>>(&self, points: I) {
        let (r, g, b) = self.conf.plot_color;
        self.context.set_source_rgb(r, g, b);

        for p in points {
            let (x, y) = self.f2p(p.0, p.1);
            self.context.new_path();
            self.context.arc(x, y, 5.0, 0.0, 2.0 * std::f64::consts::PI);
            self.context.fill();
        }

    }
}



