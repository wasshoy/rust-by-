use std::fmt;

struct Structure(i32);

// プリントのためのfmt::Displayを手動で実装できる
// {}でプリントするためにはfmt::Displayトレイとが実装されている必要がある
impl fmt::Display for Structure {
    // fmt::Displayトレイトを実装
    // fmtメソッドを実装
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)] // fmt::Debugを実装
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    // fmt::Displayフォーマットを実装
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("x: {:b}, y:{:b}", self.x as i32, self.y as i32) // 整数に変換して２進数表現にする
        )
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("表示の比較");
    println!("Display: {}", minmax);
    println!("Debug: {}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "big range: {big}\nsmall range: {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("表示の比較");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("look like binary: {:b}", point); // Binaryを実装したためプリント可能

    // 演習
    let complex = Complex {
        real: 3.3,
        image: 7.2,
    };

    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

#[derive(Debug)]
struct Complex {
    real: f32,
    image: f32,
}

impl fmt::Display for Complex {
    // fmt::Displayフォーマットを実装
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.image)
    }
}
