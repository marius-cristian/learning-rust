use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    println!("HELLO");
    // panic!("crash and burn");
    let _v = vec![1, 2, 3];
    // _v[99];

    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let _f1 = File::open("hello1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello1.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let _t = read_username_from_file();
    let _t2 = read_username_from_file2();
    let _t3 = read_username_from_file3();
    let _t4 = read_username_from_file4();

    // let f = File::open("hello.txt")?;
    // main doest have return type Result | Option<T, E>
    // thus the aboce use of ? operator does not
    // compile

    let _m2 = main2();
}

fn main2() -> Result<(), Box<dyn Error>> {
    let _f = File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello1.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello2.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello3.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello4.txt")
}
