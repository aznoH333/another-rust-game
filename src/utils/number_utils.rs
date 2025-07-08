use rand::Rng;


pub struct NumberUtils{

}


impl NumberUtils {
    pub fn gravitate_number(current: f32, target: f32, speed: f32) -> f32 {
        if (current - target).signum() > speed.signum() || (current - target).abs() < speed {
            return target;
        }

        return current - ((current - target).signum() * speed.abs());
    }

    pub fn error_signum(value: f32, error_margin: f32) -> f32{
        if value.abs() - error_margin < 0.0{
            return 0.0;
        }

        return value.signum();
    }


    pub fn random_integer(min: i32, max: i32) -> i32{
        let mut rng = rand::rng();

        return rng.random_range(min..max+1);
    }

    pub fn random_float() -> f32 {
        let mut rng = rand::rng();
        return rng.random_range(0.0..1.0);
    }

    pub fn random_float_range(min: f32, max: f32) -> f32 {
        let mut rng = rand::rng();
        return rng.random_range(min..max);
    }

    pub fn random_integer_from_array(array: &[i32]) -> i32 {
        return array[Self::random_integer(0, array.iter().count() as i32 - 1) as usize];
    }



    pub fn random_chance(percentage: i32) -> bool {
        return Self::random_integer(0, 100) < percentage;
    }

    pub fn bool_to_minus_plus(input: bool) -> i32 {
        return input as i32 * 2 - 1;
    }

    pub fn bool_to_minus_plus_f32(input: bool) -> f32 {
        return NumberUtils::bool_to_minus_plus(input) as f32;
    }

    pub fn bool_to_i32(input: bool) -> i32 {
        return input as i32;
    }

    pub fn bool_to_f32(input: bool) -> f32 {
        return NumberUtils::bool_to_i32(input) as f32;
    }
}



