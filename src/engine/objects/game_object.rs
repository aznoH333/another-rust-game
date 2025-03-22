pub struct GameObject {
    components: Vec<Box<ObjectComponent>>
}


impl GameObject {
    pub fn get_component_by_index(&mut self, index: i32) -> &mut ObjectComponent{
        return self.game_objects.get_mut(index);
    }

    pub fn get_component_by_id(&mut self, component_identifier: &str){
        return self.game_objects.iter().find(|x| x.downcast::<ObjectComponent>().unwrap().get_component_id == component_identifier).unwrap();
    }

}