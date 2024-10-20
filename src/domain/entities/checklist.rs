pub struct Checklist {
    pub name: String,
    pub items: Vec<String>,
}

impl Checklist {
    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item_id: usize) -> Result<(), String> {
        if item_id < self.items.len() {
            self.items.remove(item_id);
            Ok(())
        } else {
            Err("Invalid item ID".to_string())
        }
    }
}
