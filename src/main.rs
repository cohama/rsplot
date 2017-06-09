extern crate rsplot;
extern crate byteorder;

// use rsplot::Plot;
use rsplot::image::ImageViewer;
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::Read;
use std::io::Cursor;

fn main() {

    let mut file = File::open("/home/cohama/proj/rust/deeplearning/data/mnist/train-images-idx3-ubyte").unwrap();
    let mut header: [u8; 16] = [0; 16];
    let _ = file.read(&mut header);
    println!("{:?}", header);
    let mut reader = Cursor::new(header);
    let magicnum = reader.read_i32::<BigEndian>().unwrap();
    let count = reader.read_i32::<BigEndian>().unwrap();
    let rows = reader.read_i32::<BigEndian>().unwrap();
    let cols = reader.read_i32::<BigEndian>().unwrap();
    println!("{}, {}, {}, {}", magicnum, count, rows, cols);

    let mut buf: Vec<u8> = vec![];
    for b in file.take(784).bytes() {
        let b = b.unwrap();
        buf.push(b);
        buf.push(b);
        buf.push(b);
    }
    println!("{:?}", buf.len());

    let image = ImageViewer::from_data(buf);
    image.show();
}

// fn main() {
//     let mut plot = Plot::new();
//     plot.set_xrange(-6.0, 6.0);
//     plot.set_yrange(0.0, 1.0);
//     plot.add_func(|x| 1.0 / (1.0 + (-x).exp()));
//     plot.show();
// }
