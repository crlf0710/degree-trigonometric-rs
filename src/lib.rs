pub trait DegreeTrigonomeric {
    fn sin_degree(self) -> Self;
    fn cos_degree(self) -> Self;
    fn atan2_degree(self, other: Self) -> Self;
}

const SIN_N90_4: &[isize] = &[0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0];

fn sin_n90(n: isize) -> isize {
    SIN_N90_4[(n + 4) as usize]
}

macro_rules! impl_degree_trigonomeric {
    ($t: ty, $pi: expr) => {
        impl DegreeTrigonomeric for $t {
            fn sin_degree(self) -> Self {
                let v = self % 360.0;
                let x = v / 90.0;
                if x == x.floor() {
                    sin_n90(x as isize) as Self
                } else {
                    (v * $pi / 180.0).sin()
                }
            }

            fn cos_degree(self) -> Self {
                (90.0 - self).sin_degree()
            }

            fn atan2_degree(self, other: Self) -> Self {
                let v = self.atan2(other);
                v * 180.0 / $pi
            }
        }
    };
}

impl_degree_trigonomeric!(f32, std::f32::consts::PI);
impl_degree_trigonomeric!(f64, std::f64::consts::PI);

#[test]
fn test_sin_cos() {
    assert_eq!(0.0f32.sin_degree(), 0.0);
    assert_eq!(45.0f32.sin_degree(), std::f32::consts::SQRT_2/2.0);
    assert_eq!(90.0f32.sin_degree(), 1.0);
    assert_eq!(180.0f32.sin_degree(), 0.0);
    assert_eq!(270.0f32.sin_degree(), -1.0);
    assert_eq!(360.0f32.sin_degree(), 0.0);

    assert_eq!(0.0f32.cos_degree(), 1.0);
    assert_eq!(45.0f32.cos_degree(), std::f32::consts::SQRT_2/2.0);
    assert_eq!(60.0f32.cos_degree(), 0.5);
    assert_eq!(90.0f32.cos_degree(), 0.0);
}