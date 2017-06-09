extern crate gtk;
extern crate gdk;
extern crate cairo;

use self::gtk::prelude::*;
use self::gtk::{DrawingArea, Window, WindowType};


pub struct Plot {
    window: Window,
    drawing_area: DrawingArea,
    points: Vec<Vec<(f64, f64)>>,
    funcs: Vec<Box<Fn(f64) -> f64>>,
    xaxis: Axis,
    yaxis: Axis
}

impl Plot {
    pub fn new() -> Plot {
        let _ = gtk::init();
        let window = Window::new(WindowType::Toplevel);
        window.set_title("rsplot");
        window.set_name("mywindow");
        window.set_default_size(600, 400);

        let css = gtk::CssProvider::new();
        css.load_from_data(r#"
            #mywindow {
                background-color: #ffffff;
            }
        "#).unwrap();
        gtk::StyleContext::add_provider_for_screen(&gdk::Screen::get_default().unwrap(), &css, gtk::STYLE_PROVIDER_PRIORITY_USER);

        let drawing_area = DrawingArea::new();
        window.add(&drawing_area);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let xaxis = Axis {
            from: -20.0, to: 40.0, cross_at: 0.0
        };
        let yaxis = Axis {
            from: -10.0, to: 20.0, cross_at: 0.0
        };

        Plot {
            window,
            drawing_area,
            points: vec![],
            funcs: vec![],
            xaxis,
            yaxis,
        }
    }

    pub fn add_points<I: IntoIterator<Item=(f64, f64)>>(&mut self, points: I) {
        self.points.push(points.into_iter().collect())
    }

    pub fn add_func<F: Fn(f64) -> f64 + 'static>(&mut self, f: F) {
        self.funcs.push(Box::new(f))
    }

    pub fn set_xrange(&mut self, from: f64, to: f64) {
        self.xaxis.from = from;
        self.xaxis.to = to;
    }

    pub fn set_yrange(&mut self, from: f64, to: f64) {
        self.yaxis.from = from;
        self.yaxis.to = to;
    }

    pub fn show(self) {
        let Plot {window, drawing_area, points, funcs, xaxis, yaxis} = self;
        // let xa = self.xaxis;
        // let ya = self.yaxis;
        // let pss = self.points.clone();
        // let fs = self.funcs.clone();
        // let window = self.window.clone();
        drawing_area.connect_draw(move |w, c| {
            let width = w.get_allocated_width();
            let height = w.get_allocated_height();

            // let xaxis: &Axis = &xa.borrow();
            // let yaxis: &Axis = &ya.borrow();
            let drawer = CairoDrawer::new(width, height, &xaxis, &yaxis, c);
            drawer.draw_axes();

            // let pss: &Vec<_> = &pss.borrow();
            // use std::ops::Deref;
            // let pss: &Vec<_> = self.points.borrow().deref();
            let pss: &Vec<_> = points.as_ref();
            for ps in pss {
                drawer.draw_points(ps);
            }

            let ref fs: Vec<_> = funcs;
            for f in fs {
                let precision = width;
                let points = (0..precision+1).map(|i| {
                    (xaxis.to - xaxis.from) / (precision as f64) * i as f64 + xaxis.from
                }).map(|x| (x, f(x)))
                  .collect::<Vec<(f64, f64)>>();
                drawer.draw_line(&points);
            }



            Inhibit(true)
        });

        // ::std::mem::forget(self);

        window.show_all();
        let w = window.get_window().unwrap();
        w.set_override_redirect(true);
        w.move_(10, 10);
        w.show();

        gtk::main();
    }
}

#[derive(Debug)]
struct Axis {
    from: f64,
    to: f64,
    // label: Option<String>,
    cross_at: f64
}

struct DrawingConf {
    axis_color: (f64, f64, f64),
    plot_color: (f64, f64, f64),
    line_color: (f64, f64, f64),
    axis_margin_px: i32
}

static DEFAULT_DRAWING_CONF: DrawingConf = DrawingConf {
    axis_color: (0.0, 0.0, 0.0),
    plot_color: (0.3, 0.7, 0.6),
    line_color: (0.3, 0.5, 0.9),
    axis_margin_px: 10
};

struct CairoDrawer<'a, 'b> {
    width: i32,
    height: i32,
    xaxis: &'a Axis,
    yaxis: &'a Axis,
    context: &'a cairo::Context,
    conf: &'b DrawingConf
}

impl <'a, 'b> CairoDrawer<'a, 'b> {

    pub fn new(width: i32, height: i32, xaxis: &'a Axis, yaxis: &'a Axis, context: &'a cairo::Context) -> CairoDrawer<'a, 'static> {
        CairoDrawer {
            width,
            height,
            xaxis,
            yaxis,
            context,
            conf: &DEFAULT_DRAWING_CONF
        }
    }

    fn f2p(&self, x: f64, y: f64) -> (f64, f64) {
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

    pub fn draw_points(&self, points: &Vec<(f64, f64)>) {
        let (r, g, b) = self.conf.plot_color;
        self.context.set_source_rgb(r, g, b);

        for p in points {
            let (x, y) = self.f2p(p.0, p.1);
            self.context.new_path();
            self.context.arc(x, y, 5.0, 0.0, 2.0 * ::std::f64::consts::PI);
            self.context.fill();
        }
    }

    pub fn draw_line(&self, points: &Vec<(f64, f64)>) {
        let (r, g, b) = self.conf.line_color;
        self.context.set_source_rgb(r, g, b);

        // self.context.new_path();
        for p in points {
            let (x, y) = self.f2p(p.0, p.1);
            self.context.line_to(x, y);
        }
        self.context.stroke();
    }
}



