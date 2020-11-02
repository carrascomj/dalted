type Mat3p = [f32; 9];

const PRONATOPIA: Mat3p = [
    0.17056, 0.17056, 0.00452, 0.82944, 0.82944, 0.0042, 0., 0., 1.,
];
const DEUTERANOPIA: Mat3p = [
    0.33066, 0.33066, -0.02786, 0.66934, 0.66934, 0.02786, 0., 0., 1.,
];
const TRINATOPIA: Mat3p = [
    1.0, 0.0, 0.0, 0.1274, 0.87391, 0.87391, -0.1274, 0.12609, 0.12609,
];

// this last two are redundant and should be vectors, but it would need more
// thinking (maybe an Either enum) to work in `color_filter` and I am tired.
// Anyways, it makes it a bit less performant but a bit more readable.
const BLUE_CONE_ACHROMATOPSIA: Mat3p = [
    0.01775, 0.01775, 0.01775, 0.10945, 0.10945, 0.10945, 0.87262, 0.87262, 0.87262,
];
const ACHROMATOPSIA: Mat3p = [
    0.212_656, 0.212_656, 0.212_656, 0.715_158, 0.715_158, 0.715_158, 0.072_186, 0.072_186,
    0.072_186,
];

/// Matrices to be applied to a linear RGB vector to simulate color blindness.
/// Each matrix `M` is $M = T  S T^{-1}$ where T is the linear transformation from
/// linear RGB (0,1) to LMS and S is the color blindness filter.
pub const MATRICES: [Mat3p; 5] = [
    PRONATOPIA,
    DEUTERANOPIA,
    TRINATOPIA,
    BLUE_CONE_ACHROMATOPSIA,
    ACHROMATOPSIA,
];
