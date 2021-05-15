use std::env;
use algos_rust::merge_sort;

fn main() {
    let mut args = env::args();

    args.next();

    let str_array = args.next().expect("First argument is required");

    let mut array = vec![];

    for num in str_array.split(',').collect::<Vec<_>>() {
        let num: i32 = match num.parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        array.push(num);
    }

    println!("sorted {:?}", merge_sort(&array));
}
