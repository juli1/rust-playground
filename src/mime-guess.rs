use std::env;
extern crate mime;
use mime::Mime;

fn find_mimetype (filename : &String) -> Mime{
    let parts : Vec<&str> = filename.split('.').collect();
    let res = match parts.last() {
            Some(v) =>
                match *v {
                    "png" => mime::IMAGE_PNG,
                    "jpg" => mime::IMAGE_JPEG,
                    "json" => mime::APPLICATION_JSON,
                    &_ => mime::TEXT_PLAIN,
                },
            None => mime::TEXT_PLAIN,
        };
    return res;
}

fn main () {
    if env::args().len() != 2 {
        println!("Specify the filename to guess");
        std::process::exit(1);
    }
    let filename = env::args().nth(1).unwrap();

    let mimetype = find_mimetype(&filename);
    println!("mimetype for filename {:?}: {:?}", filename, mimetype);
}
