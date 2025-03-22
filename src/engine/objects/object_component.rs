pub trait ObjectComponent{
    pub fn get_component_id(&self) -> &str;
    pub fn update(this: &mut GameObject, component_index: i32);
}