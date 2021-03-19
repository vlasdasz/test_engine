// Rust

#![allow(dead_code)]

extern crate rustc_serialize;
use rustc_serialize::json;

mod gm;
mod gl;

use gm::Point;
use gl::GL;
use crate::gm::Size;


fn main() {
    let point = Point { x: 5.0, y: 5.0 };
    let new_point = Point::new();
    println!(
        "The area of the rectangle is {} square pixels. A {}",
        point.to_string(),
        new_point.to_string()
    );

    println!("{}", point.normalized().length());

    let encoded = json::encode(&point).unwrap();

    println!("{}", encoded);

    GL::init(Size { width: 500.0, height: 500.0 });

}