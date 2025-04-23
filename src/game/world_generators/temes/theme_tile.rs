pub struct ThemeTile{
    texture_name: String,
    chance_to_appear: i32,
}

impl ThemeTile {
    pub fn new(name: &str, chance_to_appear: i32) -> ThemeTile {
        return ThemeTile { texture_name: name.to_string(), chance_to_appear };
    }

    pub fn get_texture_name(&self) -> &String {
        return &self.texture_name;
    }

    pub fn get_chance_to_appear(&self) -> i32 {
        return self.chance_to_appear;
    }
}