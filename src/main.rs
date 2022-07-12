use std::fmt;
use std::mem;
use reqwest::Client;
mod gg;
mod results;
mod pdf;

#[derive(Debug)]
struct MinMax(i64, i64);
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

struct List(Vec<i32>);

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

#[derive(Debug)]
struct Structure(i32);

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}


impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}i", self.real, self.imag)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec: &Vec<i32> = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", count, v)?;
        }

        write!(f, "]")
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c: char = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c: char = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{0:02X}{1:02X}{2:02X}",
            self.red, self.green, self.blue
        )
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[allow(dead_code)]
fn printing() {
    println!("Hello, world!");
    let x: i32 = 5;
    println!("Is `x` 10 or 100? x = {}", x);
    println!("{0} this is {1}, {1} this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!(
        "{} of {:x} people know binary, the other half doesn't",
        3, 15
    );
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:0>width$}", number = 190, width = 6);

    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:0>width$}");
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("This struct {}", Structure(5));

    // let name = "Peter";
    // let age = 27;
    // let p = Person { name, age };
    // println!("{:#?}", p);

    let minmax = MinMax(19, 14);
    println!("{}", minmax);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

#[allow(dead_code)]
fn implementations() {
    let list = List(vec![1, 2, 3]);
    println!("{}", list);
    let city_a = City {
        name: "Las Vegas",
        lat: 30.0,
        lon: -40.0,
    };
    println!("{}", city_a);
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", *color);
    }
}

fn analyze_slice(slice: &[i32]) {
    println!("The first element is {:?}", slice.first());
    println!("The last element is {:?}", slice.last());
    println!("Length = {}", slice.len());
}

#[allow(dead_code)]
fn prims() {
    let logical: bool = true;
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);

    let mut integer = 77;
    print!("{}", integer);
    integer = 12;
    println!("{}", integer);
    println!("true AND false is {}", true && false);
    println!("One million is written as {}", 1_000_000u32);
    println!("{}", logical);

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}", (a, b, c, d));
    println!("{:?}", reverse((a, d)));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("{:?}", xs);
    println!("length of array is {}", ys.len());

    println!("array occupies {} bytes", mem::size_of_val(&ys));

    analyze_slice(&xs[1..3]);
}

#[allow(dead_code)]
fn intro() {
    let x = 'ℤ';
    {
        let x = '😻';
        println!("The value of x in the inner scope is: {x}");
        println!("{:?}", mem::size_of_val(&x));
    }

    println!("The value of x is: {x}");
    gg::guessing_game()
}

#[tokio::main]
#[allow(dead_code)]
async fn file_download() {
    results::download_file(
        &Client::new(),
        "https://mega.nz/file/JFgBmCiJ#-B6zCMG0KIdwJA-nl913w4NI9w2utqh2DIUheC8Vys0",
        "test.txt",
    )
    .await
    .unwrap_or_else(|e| {
        println!("Error: {}", e);
    });
}

fn main(){
    println!("{:?}", pdf::get_text("test.pdf"));
}
