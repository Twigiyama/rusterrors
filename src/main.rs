use std::fs;
use std::io;

fn main() {
    let result = fs::read_to_string("the_ultimate_question.txt");
    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("Nobody knows the ultimate questions"),
            io::ErrorKind::PermissionDenied => String::from("Permission Denied"),
            _ => panic!("Another type of error {:?}", error)
        }
    };

    println!("content is {:?}", contents);

    fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
        let mut s1 = fs::read_to_string(f1)?;

        let s2 = match fs::read_to_string(f2) {
            Ok(s) => s,
            Err(error) => return Err(error)
        };

        s1.push('\n');
        s1.push_str(&s2);

        Ok(s1)
    }

    let result = read_and_combine("planets.txt", "dwarf_planets.txt");
    match result {
        Ok(s) => println!("result is ...\n{}", s),
        Err(e) => println!("There was an error: {}", e)
    };
}

