pub fn gravitate_number(current: f32, target: f32, speed: f32) -> f32 {
    if (current - target).abs() < speed {
        return target;
    }

    return current - ((current - target).signum() * speed);
}


// 1 0 1
