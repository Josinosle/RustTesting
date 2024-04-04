fn main() {
    println!("Hello, world!");
    let boolean_test: bool = true;
    println!("Is rust better than C++");
    println!("{}", boolean_test);

    let p1 = Point {x: 10, y: 11, z: 12};
    let p2 = Point {x: 12, y: 24, z: 15};

    let vector = Vector {origin: p1, head: p2};

    let tempMagnitude1:f32 = vector.length();
    let tempMagnitude2:f32 = vector.sqrd_length();

    println!("{}", tempMagnitude1);
    println!("{}", tempMagnitude2)

}

struct Point { x: i32, y: i32, z: i32 }

struct Vector { origin: Point, head: Point}

impl Vector {
    fn length (&self) -> f32{
        let Point { x: x1, y: y1, z: z1} = self.origin;
        let Point { x: x2, y: y2, z: z2} = self.head;

        let x : i32 = x2 - x1;
        let y : i32 = y2 - y1;
        let z : i32 = z2 - z1;

        f32::powf(((x * x) + (y * y) + (z * z)) as f32, 0.5)
    }

    fn sqrd_length (&self) -> f32{
        let Point { x: x1, y: y1, z: z1} = self.origin;
        let Point { x: x2, y: y2, z: z2} = self.head;

        let x : i32 = x2 - x1;
        let y : i32 = y2 - y1;
        let z : i32 = z2 - z1;

        ((x * x) + (y * y) + (z * z)) as f32
    }
}


