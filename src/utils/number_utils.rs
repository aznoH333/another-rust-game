pub fn gravitate_number(current: f32, target: f32, speed: f32) -> f32 {
    if (current - target).abs() < speed {
        return target;
    }

    return current - ((current - target).signum() * speed);
}

pub fn error_signum(value: f32, error_margin: f32) -> f32{
    if value.abs() - error_margin < 0.0{
        return 0.0;
    }

    return value.signum();
}