# stamp-rs

This is a very limited two-dimensional array manipulation library. It's specifically made to allow for searching and merging (stamping) two-dimensional arrays. You can create a Stamp which you can then rotate, use to search for a matching stamp inside a stamp, and replace parts of a stamp with another stamp.

## Note
This lib is not yet stable, if you use it please ensure to specify the exact version.

## Usage

```rust
use stamp_rs::Stamp;

let stamp_vec = vec![
    vec![0, 0, 0, 0, 0],
    vec![0, 1, 1, 1, 0],
    vec![0, 1, 1, 1, 0],
    vec![0, 1, 1, 1, 0],
    vec![0, 0, 0, 0, 0]
];

let stamp = Stamp::new(2dArray);

let query_vec = vec![
    vec![0, 0],
    vec![1, 0]
];

let query_stamp = Stamp::new(query_vec);

let replace_vec = vec![
    vec![2, 2],
    vec![2, 2]
];
let replacement_stamp = Stamp::new(replace_vec);

let query_stamp = query_stamp.rotate_90();
let replace = stamp.find(query_stamp).iter().next();
if let Some(coords) = replace {
    stamp.stamp(query_stamp);
}
///
/// 0,0,0,0,0
/// 0,1,1,1,0
/// 0,1,1,1,0
/// 0,1,1,2,2
/// 0,0,0,2,2
/// 
```