use core::num;
use std::{env, fs::read_to_string};

fn read_lines(path:&str)-> Vec<String>{
    read_to_string(path).unwrap() // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn listed_file_content(path:&str)->(Vec<char>, Vec<f32>){
    let lines: Vec<String> = read_lines(path);
    let mut ascii: Vec<char> = vec![];
    let mut brightness: Vec<f32> = vec![];
    for text in lines {
        let fisrs_char = &text[..1];
        let number = &text[2..];
        let number = number.parse::<f32>().unwrap();
        let val = fisrs_char.chars().take(1).last().unwrap();
        println!("{val}: {number}");
        ascii.push(val);
        brightness.push(number);
    }
    (ascii, brightness)

}