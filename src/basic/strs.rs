use ansi_term::Colour;

pub fn inf(a: &str) -> String {
    let b = format!("{} {}",
            Colour::Blue.bold().paint("i"),
            Colour::White.paint(a)
    );
    b
}
macro_rules! inf{
    ($a:expr) => {
        println!("{}", inf($a));
    };
}

pub fn err(a: &str) -> String {
    let b = format!("{} {}",
            Colour::Red.bold().paint("e"),
            Colour::White.paint(a)
    );
    b
}
macro_rules! err{
    ($a:expr) => {
        println!("{}", err($a));
    };
}