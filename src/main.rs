mod min_sketch;

use crate::min_sketch::CountMinSketch;

fn main() {
    let mut cms = CountMinSketch::new(4, 3);

    cms.insert("192.168.0.1");
    cms.insert("192.168.0.1");
    let count = cms.count("192.168.0.1");
    println!("{:?}", count);
    println!("{}", cms);
}
