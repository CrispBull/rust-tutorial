#![allow(dead_code)] // allows unused functions

//use std::thread;

// useful to suppress unused functions being used
fn main() {
    let sums = 23 + 45;
    println!("Sum of values is {}", sums);
    //greet_world();
    //parse_csv();
    //dangling_pointer_example();
    data_race_example();
    buffer_overflow_error();
    mutate_iterator_while_iterating()
}


fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grűb Gott!";
    let igbo = "Awunla";
    let groups = [southern_germany, igbo];
    for group in groups.iter() {
        println!("{}", group)
    }
}

/**
 Processes a csv string into.

 */
fn parse_csv() {
    let penguin_data = "
    common name, length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    iordland penguin, 60
    Invalid, data
    ";
    println!("{}", penguin_data);
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 { // skips header
            continue;
        }

        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length)
        }
    }
}

#[derive(Debug)]// allows the println! to print debug print which is needed when we print with {:?}
enum Cereal {
    Barley, Millet, Rice, Rye, Spelt, Wheat
}

/*
Dangling pointers are live references to data that has become invalid over the course of the
program.
*/
fn dangling_pointer_example() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    //drop(grains);
    println!("{:?}", grains);
}

/*
Data race condition happens due to inability to determine how a program behaves due to changing
external factors
 */
fn data_race_example() {
    // let mut data = 100;
    // thread::spawn(|| { data = 500; });
    // thread::spawn(|| { data = 1000; });
    // println!("{}", data)
}

/*
This error occurs when you attempt to access items in memory that do not exist or are illegal,
this is similar to an index out of bounds exception in kotlin. This is a runtime exception here too
 */
fn buffer_overflow_error() {
    let fruits = vec!['f', 'o', 'b'];
    let buffer_overflow = fruits[4];
    assert_eq!(buffer_overflow, 'g')
}

// Results in a compile time error because we're trying to modify while iterating same object
fn mutate_iterator_while_iterating() {
    let mut letters = vec![
        'f', 'o', 'b'
    ];

    for letter in letters {
        println!("{}", letter);
        //letters.push(letter.clone()); // this would fail to compile
    }
}
