use std::fmt::{self, Display, Formatter};

/*
* 文字列のフォーマットのされ方はフォーマット文字列{}で決まる
* {}の指定方法によって対応するトレイトが呼び出される({}はDisplay、{:b}は２進数表現など)
* 各指定方法と対応するフォーマット用のトレイト一覧
* https://doc.rust-lang.org/std/fmt/#formatting-traits
*/

struct City {
    name: &'static str,
    lat: f32, // 緯度
    lon: f32, // 経度
} // 経度

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lat >= 0.0 { 'E' } else { 'W' };

        // バッファーf にフォーマットされた文字列を書き込む
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

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
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
        println!("{:?}", *color);
    }

    // 演習
    println!("演習");
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
        println!("{}", *color);
    }
}

// 演習
// Colorにfmt::Displayトレイトを実装する

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            // 引数の順番で結果が変わる
            // 右から順に適用されるっぽい
            "RGB ({r}, {g}, {b}) 0x{r:>02X}{g:>02X}{b:>02X}", // Xで大文字
            r = self.red,
            g = self.green,
            b = self.blue
        )
    }
}
