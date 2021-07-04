use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;
use std::fs;

#[test]
fn example_fn() {
    // let v = vec![12, 123, 424];
    // v[99];

    let f = File::open("./src/p_panic/test.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./src/p_panic/test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file : {:?}", e),
            },
            _other_error => panic!("problem opening the file : {:?}", error),
        }
    };
}

#[test]
fn test_open_file() {
    let _f = File::open("./src/p_panic/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("./src/p_panic/hello.txt").unwrap_or_else(|error| {
                panic!("open and create file error: {:?}", error)
            })
        } else {
            panic!("open file error {:?}", error)
        }
    });
    // let _f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("failed to open file");
}

#[test]
fn read_file() -> Result<String, io::Error> {
    let f = File::open("helle.txt");

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

#[test]
fn read_file_other() -> Result<String, io::Error> {
    let mut f = File::open("helle.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[test]
fn read_file_other_2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("helle.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


#[test]
fn read_file_other_3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

#[allow(dead_code)]
fn throw_panic() {
    panic!("this is a panic")
}
