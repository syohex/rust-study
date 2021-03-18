pub struct Sample {
    name: String,
}

impl Sample {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn hello(&self) -> String {
        format!("Hello {}", self.name)
    }
}
