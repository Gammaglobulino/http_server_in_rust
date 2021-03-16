
    pub struct Server {
        ip_addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Server {
            Server { ip_addr: addr }
        }
        pub fn run(self) {
            println!("running on: {}", self.ip_addr);
        }
    }