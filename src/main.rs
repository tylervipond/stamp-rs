mod lib;
use lib::{Stamp, StampPart::Use, QueryStampPart::Is};

fn main() {
    let mut stamp = Stamp::new(vec![
        vec![Use(0), Use(0), Use(0), Use(0), Use(0)],
        vec![Use(0), Use(0), Use(0), Use(0), Use(0)],
        vec![Use(0), Use(0), Use(0), Use(0), Use(0)],
        vec![Use(0), Use(0), Use(0), Use(0), Use(0)],
        vec![Use(0), Use(0), Use(0), Use(0), Use(0)],
    ]);
    let stamp2 = Stamp::new(vec![
        vec![Use(1), Use(1), Use(1)],
        vec![Use(1), Use(1), Use(1)],
        vec![Use(1), Use(1), Use(1)],
    ]);
    stamp.stamp(&stamp2, 1, 1);
    dbg!(&stamp);
    let query_stamp = Stamp::new(vec![
        vec![Is(Box::new([0])), Is(Box::new([0]))],
        vec![Is(Box::new([1])), Is(Box::new([1]))],
        vec![Is(Box::new([1])), Is(Box::new([1]))],
    ]);
    let matches = stamp.find(&query_stamp);
    dbg!(matches);
}
