use std::io;

fn main() {
    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Wrong");

    let temp: f32 = temp.trim().parse().expect("Wrong");

    let temp = (temp - 32.0) * 5.0 / 9.0;

    println!("{}", &temp);
}
