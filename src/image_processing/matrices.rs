use std::ops::{Add, Mul};

type Mat3x3 = [f32; 9];
const PRONATOPIA: Mat3x3 = [
    0.17404, 0.56889, 0.05816, 0.42437, 1.50127, 0.22068, 0.05111, 0.22773, 0.77322,
];
const DEUTERANOPIA: Mat3x3 = [
    0.14545, 0.2981, 0.04356, 0.42978, 0.88608, 0.19094, 0.05606, 0.17844, 0.77106,
];

const TRINATOPIA: Mat3x3 = [
    0.12336, 0.33577, 0.04572, 0.32244, 1.08953, 0.20425, 0.05905, 0.94174, 0.87713,
];

const BLUE_CONE_ACHROMATOPSIA: Mat3x3 = [
    0.01775, 0.10945, 0.87262, 0.01775, 0.10945, 0.87262, 0.01775, 0.10945, 0.87262,
];

const ACHROMATOPSIA: Mat3x3 = [
    0.212656, 0.715158, 0.072186, 0.212656, 0.715158, 0.072186, 0.212656, 0.715158, 0.072186,
];

pub const MATRICES: [Mat3x3; 5] = [
    PRONATOPIA,
    DEUTERANOPIA,
    TRINATOPIA,
    BLUE_CONE_ACHROMATOPSIA,
    ACHROMATOPSIA,
];

#[derive(Debug)]
pub struct Kernel<T> {
    matrix: [T; 9],
}

impl<T> Kernel<T> {
    pub fn new(matrix: [T; 9]) -> Self {
        Kernel { matrix }
    }
}

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

impl<T: std::convert::From<u8>> From<[u8;4]> for Vec3<T>{
    fn from(tu: [u8;4]) -> Self  {
        Vec3::new([T::from(tu[0]), T::from(tu[1]), T::from(tu[2])])
    }
}

/// matoperations for 3x3 matrices
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
        Vec3::new([self.cont[0] * r, self.cont[1] * g, self.cont[2] * b])
    }
    fn apply(&self, f: impl Fn(T) -> T) -> Self {
        Vec3::new([f(self.cont[0]), f(self.cont[1]), f(self.cont[2])])
    }
}
