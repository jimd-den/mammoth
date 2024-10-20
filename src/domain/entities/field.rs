pub enum FieldType {
    Text,
    Number,
    Date,
    // ... other field types as needed
}

pub struct Field {
    pub field_type: FieldType,
    pub label: String,
}
