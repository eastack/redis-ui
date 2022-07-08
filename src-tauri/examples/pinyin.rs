use pinyin::{Pinyin, ToPinyin};

fn main() {
    let hz = "EaStack 老大哥在看着你";
    let py: Vec<&str> = hz.to_pinyin().flatten().map(Pinyin::first_letter).collect();

    println!("{} ==2py==> {:?}", hz, py);
}
