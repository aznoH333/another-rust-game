
pub fn get_texture_with_index(texture_name: &str, index: i32) -> String{
    let mut base = texture_name.to_owned();
    let zero_padding = (index as f32).log10() as i32;
    base.push_str("_");
    for _ in 0..3-zero_padding {
        base.push_str("0");
    }
    base.push_str(index.to_string().as_str());
    return base;
}