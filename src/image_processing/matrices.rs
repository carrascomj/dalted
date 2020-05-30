use std::ops::{Add, Mul};

type Mat3x3 = [f32; 9];
type Vec3x3 = [f32; 3];
const PRONATOPIA: Mat3x3 = [
    0.17056, 0.82944, -0.0, 0.17056, 0.82944, 0.0, -0.00452, 0.00452, 1.0,
];
const DEUTERANOPIA: Mat3x3 = [
    0.33066, 0.66934, -0.0, 0.33066, 0.66934, 0.0, -0.02786, 0.02786, 1.0,
];

const TRINATOPIA: Mat3x3 = [
    1.0, 0.1274, -0.1274, -0.0, 0.87391, 0.12609, 0.0, 0.87391, 0.12609,
];

const BLUE_CONE_ACHROMATOPSIA: Vec3x3 = [0.01775, 0.10945, 0.87262];

const ACHROMATOPSIA: Vec3x3 = [0.212_656, 0.715_158, 0.072_186];

/// Matrices to be applied to a linear RGB vector to simulate color blindness.
/// Each matrix `M` is $M = T  S T^{-1}$ where T is the linear transformation from
/// linear RGB (0,1) to LMS and S is the color blindness filter.
pub const MATRICES: [Mat3x3; 3] = [PRONATOPIA, DEUTERANOPIA, TRINATOPIA];

/// Vectors to be applied to a linear RGB vector to simulate color blindness.
/// Since they perform a gray scale conversion, they produce 1 point (broadcasted to three for
/// consistency).
/// Each vector `M` is $M = T  S T^{-1}$ where T is the linear transformation from
/// linear RGB (0,1) to LMS and S is the color blindness filter.
pub const VECTORS: [Vec3x3; 2] = [BLUE_CONE_ACHROMATOPSIA, ACHROMATOPSIA];

/// Kernel is a (stacked) square 3x3 matrix. Simple interface to apply color filters.
/// K is used by pipe_transform as an f32
///
/// # Examples
/// ```rust
/// use dalted::image_processing::Kernel
///
/// let ex_k = Kernel::new([1u8, 2, 3, 4, 5, 6, 7, 8, 9]);
/// let ex_s = Kernel::new([9u8, 2, 3, 0, 5, 6, 1, 8, 9]);
/// assert_eq!(
///     Kernel::new([12, 36, 42, 42, 81, 96, 72, 126, 150]).matrix,
///     (ex_k * ex_s).matrix
/// );
/// ```
#[derive(Debug)]
pub struct Kernel<T> {
    matrix: [T; 9],
}

impl<T> Kernel<T> {
    pub fn new(matrix: [T; 9]) -> Self {
        Kernel { matrix }
    }
}

// Dot product
impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul for Kernel<T> {
    type Output = Kernel<T>;

    fn mul(self, rhs: Kernel<T>) -> Self {
        let rhs = rhs.matrix;
        Kernel::<T>::new([
            self.matrix[0] * rhs[0] + self.matrix[1] * rhs[3] + self.matrix[2] * rhs[6],
            self.matrix[0] * rhs[1] + self.matrix[1] * rhs[4] + self.matrix[2] * rhs[7],
            self.matrix[0] * rhs[2] + self.matrix[1] * rhs[5] + self.matrix[2] * rhs[8],
            self.matrix[3] * rhs[0] + self.matrix[4] * rhs[3] + self.matrix[5] * rhs[6],
            self.matrix[3] * rhs[1] + self.matrix[4] * rhs[4] + self.matrix[5] * rhs[7],
            self.matrix[3] * rhs[2] + self.matrix[4] * rhs[5] + self.matrix[5] * rhs[8],
            self.matrix[6] * rhs[0] + self.matrix[7] * rhs[3] + self.matrix[8] * rhs[6],
            self.matrix[6] * rhs[1] + self.matrix[7] * rhs[4] + self.matrix[8] * rhs[7],
            self.matrix[6] * rhs[2] + self.matrix[7] * rhs[5] + self.matrix[8] * rhs[8],
        ])
    }
}

/// Vec3 is a (stacked) 3-lenght matrix. Simple interface to apply color filters.
#[derive(Debug)]
pub struct Vec3<T> {
    cont: [T; 3],
}

impl<T> Vec3<T> {
    pub fn new(cont: [T; 3]) -> Self {
        Vec3 { cont }
    }
    pub fn cont(self) -> [T; 3] {
        self.cont
    }
}

impl<T: std::convert::From<u8>> From<[u8; 4]> for Vec3<T> {
    fn from(tu: [u8; 4]) -> Self {
        Vec3::new([T::from(tu[0]), T::from(tu[1]), T::from(tu[2])])
    }
}

/// Operations for 3x3 matrices and 3x vectors
pub trait Matops3<T: Copy + Mul<T, Output = T>> {
    fn vecmul(&self, vec: Vec3<T>) -> Vec3<T>;
    fn apply(&self, f: impl Fn(T) -> T) -> Self;
}

impl<T: Copy + Mul<T, Output = T> + Add<T, Output = T>> Matops3<T> for Kernel<T> {
    fn vecmul(&self, vector: Vec3<T>) -> Vec3<T> {
        let [r, g, b] = vector.cont;
        Vec3::new([
            self.matrix[0] * r + self.matrix[1] * g + self.matrix[2] * b,
            self.matrix[3] * r + self.matrix[4] * g + self.matrix[5] * b,
            self.matrix[6] * r + self.matrix[7] * g + self.matrix[8] * b,
        ])
    }
    fn apply(&self, f: impl Fn(T) -> T) -> Self {
        Kernel::new([
            f(self.matrix[0]),
            f(self.matrix[1]),
            f(self.matrix[2]),
            f(self.matrix[3]),
            f(self.matrix[4]),
            f(self.matrix[5]),
            f(self.matrix[6]),
            f(self.matrix[7]),
            f(self.matrix[8]),
        ])
    }
}

impl<T: Copy + Mul<T, Output = T> + Add<T, Output = T>> Matops3<T> for Vec3<T> {
    fn vecmul(&self, vector: Vec3<T>) -> Vec3<T> {
        let [r, g, b] = vector.cont;
        let gray = self.cont[0] * r + self.cont[1] * g + self.cont[2] * b;
        Vec3::new([gray, gray, gray])
    }
    fn apply(&self, f: impl Fn(T) -> T) -> Self {
        Vec3::new([f(self.cont[0]), f(self.cont[1]), f(self.cont[2])])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product() {
        let ex_k = Kernel::new([1u8, 2, 3, 4, 5, 6, 7, 8, 9]);
        let ex_s = Kernel::new([9u8, 2, 3, 0, 5, 6, 1, 8, 9]);
        assert_eq!(
            Kernel::new([12, 36, 42, 42, 81, 96, 72, 126, 150]).matrix,
            (ex_k * ex_s).matrix
        );
    }

    #[test]
    fn hadamart_product() {
        let ex_k = Vec3::new([1u8, 2, 3]);
        let ex_s = Vec3::new([9u8, 2, 3]);
        assert_eq!(Vec3::new([9, 4, 9]).cont, (ex_k.vecmul(ex_s)).cont);
    }

    #[test]
    fn dot_product_kernel_x_vec() {
        let ex_k = Kernel::new([1f32, 2., 3., 4., 5., 6., 7., 8., 9.]);
        assert_eq!(
            Vec3::new([36., 84., 132.]).cont,
            ex_k.vecmul(Vec3::new([5., 2., 9.])).cont
        );
    }

    #[test]
    fn apply_mul() {
        let ex_k = Kernel::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]);
        assert_eq!(
            Kernel::new([5., 10., 15., 20.0, 25.0, 30.0, 35.0, 40.0, 45.0]).matrix,
            ex_k.apply(|x| x * 5.).matrix
        );
    }
}
