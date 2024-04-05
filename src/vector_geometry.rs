use std::mem::transmute;
pub(crate) struct DisplacementVector { pub(crate) x: f64, pub(crate) y: f64, pub(crate) z: f64 }

pub(crate) struct Vector { pub(crate) origin: DisplacementVector, pub(crate) head: DisplacementVector }

impl Vector {

    pub(crate) fn length (&self) -> f64{
        let DisplacementVector { x: x1, y: y1, z: z1} = self.origin;
        let DisplacementVector { x: x2, y: y2, z: z2} = self.head;

        let x : f64 = x2 - x1;
        let y : f64 = y2 - y1;
        let z : f64 = z2 - z1;

        f64::powf((x * x) + (y * y) + (z * z), 0.5)
    }

    fn squared_length (&self) -> f64{
        let DisplacementVector { x: x1, y: y1, z: z1} = self.origin;
        let DisplacementVector { x: x2, y: y2, z: z2} = self.head;

        let x : f64 = x2 - x1;
        let y : f64 = y2 - y1;
        let z : f64 = z2 - z1;

        (x * x) + (y * y) + (z * z)
    }

    pub(crate) fn normalise (&self) -> Vector{
        let inverse_length : f64 = 1.0 / (&self).length();

        Self::normalised_vector(&self,inverse_length)
    }

    pub(crate) fn fast_normalise (&self) -> Vector{
        let temp_length_squared:f64 = self.squared_length();

        let inverse_length : f64 = Vector::fast_inv_sqrt(temp_length_squared);

        Self::normalised_vector(&self,inverse_length)
    }

    fn normalise_axis (axis1: f64, axis2: f64, inverse_length: f64) -> f64 {
        ((axis2 - axis1) * inverse_length) + axis1
    }

    fn normalised_vector (&self, inverse_length: f64) -> Vector {
        let DisplacementVector { x: x1, y: y1, z: z1} = self.origin;
        let DisplacementVector { x: x2, y: y2, z: z2} = self.head;

        Vector {
            origin:
            DisplacementVector {x: x1, y: y1, z: z1},
            head:
            DisplacementVector {
                x: Self::normalise_axis(x1,x2,inverse_length),
                y: Self::normalise_axis(y1,y2,inverse_length),
                z: Self::normalise_axis(z1,z2,inverse_length)
            }}
    }

    fn fast_inv_sqrt(x: f64) -> f64 {
        let mut y = x;
        let x2 = y * 0.5;
        let mut i: u64 = unsafe { transmute(y) };
        i = 0x5fe6eb50c7b537a9 - (i >> 1);

        y = unsafe { transmute(i) };
        y = y * (1.5 - (x2 * y * y));

        y
    }
}