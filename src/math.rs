use num::Float;

// Outputs the roots to a degree 2 polynomial,
// specified in terms of the coefficients: ax^2 + bx + c
// Returns None if there are no real roots
pub fn find_roots<T: Float + Copy>(a: T, b: T, c: T) -> Option<(T, T)> {
    let b_sq = b * b;
    let four_a_c = T::from(4).unwrap() * a * c;
    if four_a_c > b_sq {
        return None;    // No real roots
    }
    let sqrt_term = (b_sq - four_a_c).sqrt();
    let two_a = T::from(2).unwrap() * a;
    Some( ((-b + sqrt_term)/two_a, (-b - sqrt_term)/two_a) )
}