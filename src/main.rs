extern crate rsplot;

use rsplot::Plot;

fn main() {
    let mut plot = Plot::new();
    let points = (0..20).map(|x| (3*x - 20) as f64).map(|x| (x, -0.02 * x * x + 8.0)).collect::<Vec<(f64, f64)>>();
    // plot.set_xrange(-100.0, 100.0);
    // plot.set_yrange(-100.0, 100.0);
    plot.add_points(points);
    plot.add_func(|x| f64::powf(1.07, x));
    plot.add_func(|x| -x + 20.0);
    plot.show();
}

