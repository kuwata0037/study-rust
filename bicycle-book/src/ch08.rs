#[cfg(test)]
mod tests {
    struct CartesianCoord {
        x: f64,
        y: f64,
    }

    struct PolarCoord {
        r: f64,
        theta: f64,
    }

    trait Coordinates {
        fn into_cartesian(self) -> CartesianCoord;
        fn from_cartesian(cart: CartesianCoord) -> Self;
    }

    impl Coordinates for CartesianCoord {
        fn into_cartesian(self) -> CartesianCoord {
            self
        }

        fn from_cartesian(cart: CartesianCoord) -> Self {
            cart
        }
    }

    impl Coordinates for PolarCoord {
        fn into_cartesian(self) -> CartesianCoord {
            CartesianCoord {
                x: self.r * self.theta.cos(),
                y: self.r * self.theta.sin(),
            }
        }

        fn from_cartesian(cart: CartesianCoord) -> Self {
            Self {
                r: (cart.x.powi(2) + cart.y.powi(2)).sqrt(),
                theta: (cart.y / cart.x).atan(),
            }
        }
    }

    impl Coordinates for (f64, f64) {
        fn into_cartesian(self) -> CartesianCoord {
            CartesianCoord {
                x: self.0,
                y: self.1,
            }
        }

        fn from_cartesian(cart: CartesianCoord) -> Self {
            (cart.x, cart.y)
        }
    }
    fn to_string<P: Coordinates>(point: P) -> String {
        let p = point.into_cartesian();
        format!("({:.8}, {:.8})", p.x, p.y)
    }

    struct Matrix([[f64; 2]; 2]);

    trait LinearTransform: Coordinates {
        fn transform(self, matrix: &Matrix) -> Self;
    }

    impl LinearTransform for CartesianCoord {
        fn transform(mut self, matrix: &Matrix) -> Self {
            let x = self.x;
            let y = self.y;
            let m = matrix.0;
            self.x = m[0][0] * x + m[0][1] * y;
            self.y = m[1][0] * x + m[1][1] * y;
            self
        }
    }

    trait Dimension {
        const DIMENSION: u32;
    }

    impl Dimension for CartesianCoord {
        const DIMENSION: u32 = 2;
    }

    #[test]
    fn test_trait_basics() {
        let point = (1.0, 1.0);

        let cartesian = point.into_cartesian();
        approx::assert_abs_diff_eq!(cartesian.x, 1.0);
        approx::assert_abs_diff_eq!(cartesian.y, 1.0);

        let polar = PolarCoord::from_cartesian(cartesian);
        assert_eq!(format!("{:.8}", polar.r), "1.41421356");
        assert_eq!(format!("{:.8}", polar.theta), "0.78539816");

        assert_eq!(to_string(polar), "(1.00000000, 1.00000000)");

        assert_eq!(CartesianCoord::DIMENSION, 2);
    }
}
