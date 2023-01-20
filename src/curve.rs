fn parameterized_logistic(value: f32, max_value: f32, mid_point: f32, steepness: f32) -> f32 {
    max_value / (1.0 + (-steepness * (value - mid_point)).exp())
}

pub fn logistic(value: f32, steepness: f32) -> f32 {
    parameterized_logistic(value, 1.0, 0.5, steepness)
}

pub fn linear(value: f32, max: f32) -> f32 {
    value / max
}

pub fn quadratic(value: f32, max: f32, exponent: f32) -> f32 {
    linear(value, max).powf(exponent)
}

pub enum CurveType {
    Linear,
    Quadratic(f32),
}
pub struct CurvePiece {
    pub max_value: f32,
    pub max_result: f32,
    pub curve_type: CurveType,
}

pub fn piece_wise(value: f32, pieces: &Vec<CurvePiece>) -> f32 {
    let mut previous_max_value = 0.0;
    for piece in pieces {
        if value > previous_max_value && value <= piece.max_value {
            let normalized_value = value / piece.max_value;
            return match piece.curve_type {
                CurveType::Linear => linear(normalized_value, 1.0),
                CurveType::Quadratic(exponent) => quadratic(normalized_value, 1.0, exponent),
            } * piece.max_result;
        }
        previous_max_value = piece.max_value;
    }
    0.0
}

pub fn invert(value: f32) -> f32 {
    1.0 - value
}

#[cfg(test)]
mod tests {
    use super::*;
    fn round_to_precision(value: f32, precision: u32) -> f32 {
        let multiplier = 10_i32.pow(precision) as f32;
        (value * multiplier).round() / multiplier
    }

    #[test]
    fn invert_inverts_a_value_between_zero_and_one() {
        let result = invert(0.9);
        assert_eq!(round_to_precision(result, 4), 0.1);
    }

    #[test]
    fn linear_returns_a_weight_for_a_value_given_a_max() {
        let result = linear(0.5, 1.0);
        assert_eq!(round_to_precision(result, 4), 0.5);
    }

    #[test]
    fn quadratic_returns_a_weight_for_a_given_max_and_exponent() {
        let result = quadratic(0.5, 1.0, 0.5);
        assert_eq!(round_to_precision(result, 4), 0.7071);
    }

    #[test]
    fn logistic_returns_a_weight_for_a_given_steepness() {
        let result = logistic(0.75, 1.0);
        assert_eq!(round_to_precision(result, 4), 0.5622);
    }

    #[test]
    fn logistic_has_a_stable_midpoint() {
        let result = logistic(0.5, 0.75);
        assert_eq!(round_to_precision(result, 4), 0.5);
    }

    #[test]
    fn piece_wise_returns_a_weight_for_a_given_set_of_pieces_with_linear() {
        let pieces = vec![
            CurvePiece {
                max_value: 0.25,
                max_result: 0.2,
                curve_type: CurveType::Linear,
            },
            CurvePiece {
                max_value: 0.75,
                max_result: 0.8,
                curve_type: CurveType::Linear,
            },
            CurvePiece {
                max_value: 1.0,
                max_result: 1.0,
                curve_type: CurveType::Linear,
            },
        ];
        let result = piece_wise(0.6, &pieces);
        assert_eq!(round_to_precision(result, 4), 0.64);
    }

    #[test]
    fn piece_wise_returns_a_weight_for_a_given_set_of_pieces_with_quadratic() {
        let pieces = vec![
            CurvePiece {
                max_value: 0.25,
                max_result: 0.2,
                curve_type: CurveType::Quadratic(0.2),
            },
            CurvePiece {
                max_value: 0.75,
                max_result: 0.8,
                curve_type: CurveType::Quadratic(1.0),
            },
            CurvePiece {
                max_value: 1.0,
                max_result: 1.0,
                curve_type: CurveType::Quadratic(1.5),
            },
        ];
        let result = piece_wise(0.5, &pieces);
        assert_eq!(round_to_precision(result, 4), 0.5333);
    }
}
