fn main() {
    // println!("{}", UnPrintable(3150)); // エラー
    println!("{:?}", DebugPrintable(3150));

    let name = "Peter";
    let age = 25;
    let peter = Person { name, age };

    println!("{:?}", peter);
}

#[warn(dead_code)]
struct UnPrintable(i32); // std::fmtのフォーマット用トレイトが実装されていないため、プリントできない

// deriveアトリビュートを使うことでfmt::Debugトレイトをderive出来る
// プリント可能になる
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}
