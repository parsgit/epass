pub struct Pass {
    password: String,
}

impl Pass {
    // pub fn new() -> Self {
    //     Pass {
    //         password: String::new(),
    //     }
    // }

    pub fn set_pass(&mut self, password: String) {
        self.password = password;
    }

    fn decode(&self) {}

    fn encode(&self) {}

    pub fn show_pass() {
        // println!("pass: {}", Pass::password);
    }
}
