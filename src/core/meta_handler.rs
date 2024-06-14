use std::collections::HashMap;
use core::hash::Hash;

#[derive(Clone)]
pub struct MetaHandler<T> {
    id : usize,
    id_2_obj : HashMap<usize, T>,
    obj_2_id : HashMap<T, usize>,
    attr : HashMap <usize, HashMap<String, String>>
}

impl <T> MetaHandler <T>
where
    T: Eq + Hash + Clone,
{
    pub fn new()->Self {
        MetaHandler{
            id : 0 ,
            id_2_obj : HashMap::new(),
            obj_2_id : HashMap::new(),
            attr : HashMap::new(),
        }
    }

    pub fn add_object(&mut self, obj: T, attributes: Option<HashMap<String, String>>) -> usize {
        // Controlla se l'oggetto è già presente
        if let Some(&existing_id) = self.obj_2_id.get(&obj) {
            return existing_id;
        }

        let obj_id = self.id;
        self.id += 1;

        self.id_2_obj.insert(obj_id, obj.clone());
        self.obj_2_id.insert(obj, obj_id);

        if let Some(attrs) = attributes {
            self.attr.insert(obj_id, attrs);
        }

        obj_id
    }

    pub fn get_object_by_id(&self, obj_id: usize) -> Option<&T> {
        self.id_2_obj.get(&obj_id)
    }

    pub fn get_id_by_object(&self, obj: &T) -> Option<&usize> {
        self.obj_2_id.get(obj)
    }

    pub fn get_attributes(&self, obj_id: usize) -> Option<&HashMap<String, String>> {
        self.attr.get(&obj_id)
    }

    pub fn set_attributes(&mut self, obj: &T, attr: HashMap<String, String>) {
        if let Some(&obj_id) = self.get_id_by_object(obj) {
            self.attr.insert(obj_id, attr);
        }
    }

    pub fn add_attribute(&mut self, obj: &T, attr: String, value: String) {
        if let Some(&obj_id) = self.get_id_by_object(obj) {
            let entry = self.attr.entry(obj_id).or_insert_with(HashMap::new);
            entry.insert(attr, value);
        }
    }

    pub fn remove_attribute(&mut self, obj: &T, attribute: &str) -> Option<String> {
        if let Some(&obj_id) = self.get_id_by_object(obj) {
            if let Some(entry) = self.attr.get_mut(&obj_id) {
                return entry.remove(attribute);
            }
        }
        None
    }

    pub fn remove_object(&mut self, obj: &T) -> Option<(T, HashMap<String, String>)> {
        if let Some(obj_id) = self.obj_2_id.remove(obj) {
            self.id_2_obj.remove(&obj_id);
            let attributes = self.attr.remove(&obj_id);
            Some((obj.clone(), attributes.unwrap_or_else(HashMap::new)))
        } else {
            None
        }
    }

}