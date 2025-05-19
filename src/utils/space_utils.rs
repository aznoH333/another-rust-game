pub struct SpaceUtils{

}

impl SpaceUtils {
    pub fn squares_collide(x1: i32, y1: i32, w1: i32, h1: i32, x2: i32, y2: i32, w2: i32, h2: i32) -> bool {
        return 
            x1 + w1 >= x2 &&
            x1 < x2 + w2 &&
            y1 + h1 >= y2 &&
            y1 < y2 + h2;
    }


    pub fn pythagoras(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        return ((x1 - x2).abs().powf(2.0) + (y1 - y2).abs().powf(2.0)).sqrt();
    }
}
