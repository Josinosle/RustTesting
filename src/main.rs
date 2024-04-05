use crate::vector_geometry::Vector;
use crate::vector_geometry::DisplacementVector;

mod vector_geometry;

fn main() {

    let p1 = DisplacementVector {
        x: 10f64,
        y: 11f64,
        z: 12f64
    };
    let p2 = DisplacementVector {
        x: 12f64,
        y: 24f64,
        z: 15f64
    };

    let vector = Vector::new (p1, p2);

    let normal_vector:Vector = vector.normalise();
    let fast_normal_vector:Vector = vector.fast_normalise();

    let normal_vector_length = normal_vector.length();
    let fast_normal_length = fast_normal_vector.length();

    println!("Normal maths vector normalisation\n{normal_vector_length}\nFast inverse square root normalisation\n{fast_normal_length}");
}




