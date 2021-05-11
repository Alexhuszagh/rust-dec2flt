use dec2flt::{black_box, parse};

pub fn main() {
    black_box(parse::<f64>("1.2345").unwrap());
}
