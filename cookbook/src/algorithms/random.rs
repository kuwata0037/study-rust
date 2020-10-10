#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    fn generate_random_numbers() {
        let mut rng = rand::thread_rng();

        let n1: u8 = rng.gen();
        let n2: u16 = rng.gen();
        println!("Random u8: {}", n1);
        println!("Random u16: {}", n2);
        println!("Random u32: {}", rng.gen::<u32>());
        println!("Random i32: {}", rng.gen::<i32>());
        println!("Random float: {}", rng.gen::<f64>());
    }

    #[test]
    fn generate_random_numbers_within_a_range() {
        let mut rng = rand::thread_rng();
        println!("Integer: {}", rng.gen_range(0, 10));
        println!("Float: {}", rng.gen_range(0.0, 10.0));
    }

    #[test]
    fn generate_random_numbers_with_given_distribution() -> Result<(), rand_distr::NormalError> {
        use rand_distr::{Distribution, Normal};

        let mut rng = rand::thread_rng();
        let normal = Normal::new(2.0, 3.0)?;
        let v = normal.sample(&mut rng);
        println!("{} is from a N(2, 9) distribution", v);
        Ok(())
    }

    #[test]
    fn generate_random_values_of_a_custom_type() {
        use rand::distributions::{Distribution, Standard};

        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Distribution<Point> for Standard {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
                let (rand_x, rand_y) = rng.gen();
                Point {
                    x: rand_x,
                    y: rand_y,
                }
            }
        }

        let mut rng = rand::thread_rng();
        let rand_point = rng.gen::<Point>();
        println!("Random Point: {:?}", rand_point);
    }

    #[test]
    fn create_random_passwords_from_a_set_of_alphanumeric_characters() {
        use rand::distributions::Alphanumeric;

        let rand_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();
        println!("{}", rand_string);
    }

    #[test]
    fn create_random_passwords_from_a_set_of_user_defined_characters() {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        const PASSWORD_LEN: usize = 30;

        let mut rng = rand::thread_rng();
        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        println!("{:?}", password);
    }
}
