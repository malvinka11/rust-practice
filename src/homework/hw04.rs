fn main() {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 2 * WIDTH - 1;

    for i in 0..WIDTH {
        for _ in 0..WIDTH - i - 1 {
            print!(" ");
        }
        for _ in 0..2 * i + 1 {
            print!("*");
        }
        println!();
    }

    for i in (0..WIDTH - 1).rev() {
        for _ in 0..WIDTH - i - 1 {
            print!(" ");
        }
        for _ in 0..2 * i + 1 {
            print!("*");
        }
        println!();
    }
}
