
struct Server{
    ip_addr:String,
}

impl Server{
    fn new(addr:String)->Server{
        Server{
            ip_addr:addr
        }
    }
    fn run(self){
        println!("running on: {}",self.ip_addr);
    }
}
fn main() {
    let server=Server::new(String::from("127.0.0.1:8080"));
    server.run();
}
