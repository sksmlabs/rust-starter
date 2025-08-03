// Tutorial Link: https://www.youtube.com/watch?v=zlK4f7gjXhk

#![allow(unused)]

#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang = "rust";
    println!("hello {}", lang);
    println!("hello {} {}", lang, lang);
    println!("hello {lang}");

    let x = 2;
    // index represent position of inputs
    println!("{0} x {0} = {1}", x, x * x);

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.8.3".to_string(),
    };
    // Print using debug feature
    println!("{:?}", lang);
    // Print using some line breaks
    println!("{:#?}", lang);
}