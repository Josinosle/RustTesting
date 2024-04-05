fn main() {

    let p1 = Point {x: 10f32, y: 11f32, z: 12f32 };
    let p2 = Point {x: 12f32, y: 24f32, z: 15f32 };

    let vector = Vector {origin: p1, head: p2};

    let temp_magnitude1:f32 = vector.length();
    let temp_magnitude2:f32 = vector.squared_length();

    let normal_vector:Vector = vector.normalise();
    let fast_normal_vector:Vector = vector.fast_normalise();

    let normal_vector_length:f32 = normal_vector.length();
    let fast_normal_length:f32 = fast_normal_vector.length();

    println!("{}", temp_magnitude1);
    println!("{}", temp_magnitude2);
    println!("{}", normal_vector_length);
    println!("{}", fast_normal_length)

}

struct Point { x: f32, y: f32, z: f32 }

struct Vector { origin: Point, head: Point}

impl Vector {
    fn length (&self) -> f32{
        let Point { x: x1, y: y1, z: z1} = self.origin;
        let Point { x: x2, y: y2, z: z2} = self.head;

        let x : f32 = x2 - x1;
        let y : f32 = y2 - y1;
        let z : f32 = z2 - z1;

        f32::powf((x * x) + (y * y) + (z * z), 0.5)
    }

    fn squared_length (&self) -> f32{
        let Point { x: x1, y: y1, z: z1} = self.origin;
        let Point { x: x2, y: y2, z: z2} = self.head;

        let x : f32 = x2 - x1;
        let y : f32 = y2 - y1;
        let z : f32 = z2 - z1;

        (x * x) + (y * y) + (z * z)
    }

    fn normalise (&self) -> Vector{
        let Point { x: x1, y: y1, z: z1} = self.origin;
        let Point { x: x2, y: y2, z: z2} = self.head;

        let inverse_length : f32 = 1.0 / (&self).length();

        Vector {
            origin:
            Point {x: x1, y: y1, z: z1},
            head:
            Point {
                x: ((x2 - x1) * inverse_length) + x1,
                y: ((y2 - y1) * inverse_length) + y1,
                z: ((z2 - z1) * inverse_length) + z1,
            }}
    }

    fn fast_normalise (&self) -> Vector{
        let Point { x: x1, y: y1, z: z1} = self.origin;
        let Point { x: x2, y: y2, z: z2} = self.head;

        let temp_length_squared:f32 = self.squared_length();

        let inverse_length : f32 = Vector::fast_inv_sqrt(temp_length_squared);

        Vector {
            origin:
                Point {x: x1, y: y1, z: z1},
            head:
                Point {
                x: ((x2 - x1) * inverse_length) + x1,
                y: ((y2 - y1) * inverse_length) + y1,
                z: ((z2 - z1) * inverse_length) + z1,
        }}
    }

    fn fast_inv_sqrt(x: f32) -> f32 {
        let i = x.to_bits();
        let i = 0x5f3759df - (i >> 1);
        let y = f32::from_bits(i);

        y * (1.5 - 0.5 * x * y * y)
    }
}


