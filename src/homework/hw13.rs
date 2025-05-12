use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();

    for rect in xs {
        let x1 = rect.a.x.min(rect.b.x);
        let x2 = rect.a.x.max(rect.b.x);
        let y1 = rect.a.y.min(rect.b.y);
        let y2 = rect.a.y.max(rect.b.y);

        for x in x1..x2 {
            for y in y1..y2 {
                occupied.insert((x, y));
            }
        }
    }

    occupied.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 1, y: 1 },
            b: Point { x: 4, y: 7 }, // 3*6 = 18
        },
        Rectangle {
            a: Point { x: 3, y: 5 },
            b: Point { x: 13, y: 7 }, // 10*2 = 20
        },
        Rectangle {
            a: Point { x: 6, y: 1 },
            b: Point { x: 10, y: 9 }, // 4*8 = 32
        },
    ]
}

fn main() {
    let data = test_data();
    let occupied_area = area_occupied(&data);
    println!("Occupied area: {}", occupied_area); // Очікуємо: 60
}
