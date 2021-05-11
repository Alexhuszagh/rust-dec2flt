use dec2flt::black_box;

pub fn main() {
    black_box("1.2345".parse::<f64>().unwrap());
}
