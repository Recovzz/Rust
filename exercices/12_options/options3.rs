// 
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), // Utilisation de `ref` pour créer une référence au lieu de déplacer la valeur
        _ => panic!("no match!"),
    } 
    y; // Fix without deleting this line.
}
