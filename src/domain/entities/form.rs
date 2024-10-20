use crate::domain::entities::field::Field;

pub struct Form {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Form {
    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn remove_field(&mut self, field_id: usize) -> Result<(), String> {
        if field_id < self.fields.len() {
            self.fields.remove(field_id);
            Ok(())
        } else {
            Err("Invalid field ID".to_string())
        }
    }
}
