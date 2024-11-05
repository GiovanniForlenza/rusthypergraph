use std::collections::HashMap;
use core::hash::Hash;
use std::fmt::Display;

#[derive(Clone)]
pub struct MetaHandler<T> {
    id : usize,
    id_2_obj : HashMap<usize, T>,
    obj_2_id : HashMap<T, usize>,
    attr : HashMap <usize, HashMap<String, String>>
}

impl <T> MetaHandler <T>
where
    T: Eq + Hash + Clone + Display,
{
    pub fn new()->Self {
        MetaHandler{
            id : 0 ,
            id_2_obj : HashMap::new(),
            obj_2_id : HashMap::new(),
            attr : HashMap::new(),
        }
    }

    pub fn add_obj(
        &mut self, 
        obj: T, 
        obj_type: Option<String>, 
        attributes: Option<HashMap<String, String>>
    ) -> usize {
        
        let obj_clone = obj.clone(); 
    
        if let Some(&existing_id) = self.obj_2_id.get(&obj_clone) {
            return existing_id;
        }
    
        let obj_id = self.id;
        self.id += 1;
    
        self.id_2_obj.insert(obj_id, obj_clone.clone());
        self.obj_2_id.insert(obj_clone.clone(), obj_id);
    
        let mut combined_attributes = HashMap::new();
    
        if let Some(t) = obj_type {
            combined_attributes.insert("type".to_string(), t);
        }
    
        combined_attributes.insert("name".to_string(), obj_clone.to_string());
    
        if let Some(attrs) = attributes {
            combined_attributes.extend(attrs);
        }
    
        self.attr.insert(obj_id, combined_attributes);
    
        obj_id
    }

    pub fn get_id(&self, obj: &T) -> Result<usize, String> {
        self.obj_2_id.get(obj).cloned().ok_or_else(|| format!("No object {}.", obj))
    }


    pub fn set_attr(&mut self, obj: &T, new_attr: HashMap<String, String>) -> Result<(), String> {
        let id = self.get_id(obj)?;
        let attributes = self.attr.entry(id).or_insert_with(HashMap::new);
        for (key, value) in new_attr {
            attributes.insert(key, value);
        }
        Ok(())
    }

    pub fn get_attr(&self, obj: &T) -> Result<&HashMap<String, String>, String> {
        let idx = self.get_id(obj)?;
        self.attr.get(&idx).ok_or_else(|| format!("No object {}.", obj))
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

    pub fn set_attributes_by_id(&mut self, obj_id: usize, attr: HashMap<String, String>) {
        self.attr.insert(obj_id, attr);
    }

    pub fn remove_object(&mut self, obj: &T) -> Result<(), String> {
        if let Some(idx) = self.obj_2_id.remove(obj) {
            self.id_2_obj.remove(&idx);
            self.attr.remove(&idx);
            Ok(())
        } else {
            Err(format!("No object {}.", obj))
        }
    }

}