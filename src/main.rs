mod lib;
use lib::Stamp;

fn main() {
    let mut stamp = Stamp::new(vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ]);
    let stamp2 = Stamp::new(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]);
    stamp.stamp(&stamp2, 1, 1);
    dbg!(&stamp);
    let match_stamp = Stamp::new(vec![vec![0, 0], vec![1, 1], vec![1, 1]]);
    let matches = stamp.find(&match_stamp);
    dbg!(matches);
}
