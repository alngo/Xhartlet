#[derive(Default)]
pub struct User {
    id: u64,
    name: String,
}

impl User {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
        }
    }

    pub fn get_id(&self) -> &u64 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
