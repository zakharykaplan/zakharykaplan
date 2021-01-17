use Languages::*;
use Tools::*;

fn main() {
    let zakhary = Person {
        about: Contact {
            name: "Zakhary Kaplan".to_string(),
            email: "zakharykaplan@gmail.com".to_string(),
            country: "🇨🇦".to_string(),
        },
        education: Degree {
            school: "University of Toronto".to_string(),
            discipline: "Computer (Hard|Soft)ware Engineering".to_string(),
            level: Level::Bachelors,
            grad: 2023,
        },
        work: Developer {
            languages: vec![C, Cpp, Python, Rust, Verilog],
            tools: vec![Arduino, RaspberryPi, Fpga, Docker, GnuMake],
            os: "*nix".to_string(),
            editor: "Vim".to_string(),
        },
    };

    println!("{:?}", zakhary);
}

#[derive(Debug)]
enum Languages {
    C,
    Cpp,
    Python,
    Rust,
    Verilog,
}

#[derive(Debug)]
enum Tools {
    Arduino,
    RaspberryPi,
    Fpga,
    Docker,
    GnuMake,
}

#[derive(Debug)]
struct Person {
    about: Contact,
    education: Degree,
    work: Developer,
}

#[derive(Debug)]
struct Contact {
    name: String,
    email: String,
    country: String,
}

#[derive(Debug)]
enum Level {
    Bachelors,
}

#[derive(Debug)]
struct Degree {
    school: String,
    level: Level,
    discipline: String,
    grad: u32,
}

#[derive(Debug)]
struct Developer {
    languages: Vec<Languages>,
    tools: Vec<Tools>,
    os: String,
    editor: String,
}
