fn print_tree(triangle_count: usize) {
    let max_width = triangle_count * 2 + 1;
    let mut lines: Vec<String> = Vec::new();

    for t in 1..=triangle_count {
        for i in 1..=t {
            let stars = 2 * i - 1;
            let spaces = (max_width - stars) / 2;
            let line = " ".repeat(spaces) + &"*".repeat(stars);
            lines.push(line);
        }
    }

    let trunk = " ".repeat((max_width - 1) / 2) + "*";
    lines.push(trunk);

    lines.iter().for_each(|line| println!("{}", line));
}

fn main() {
    let triangle_count = 5; 
    print_tree(triangle_count);
}
