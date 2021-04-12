
    use std::net::TcpListener;
    use std::io::Read;
    use crate::http::Request;
    use std::convert::TryFrom;

    pub struct Server {
        ip_addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Server {
            Server { ip_addr: addr }
        }
        pub fn run(self) {
            println!("running on: {}", self.ip_addr);
            let listener=TcpListener::bind(&self.ip_addr).unwrap();
            loop{
                match listener.accept(){
                    Ok((mut stream,_))=>{
                        let mut buffer=[0;1024];
                        match stream.read(&mut buffer){
                            Ok(_)=>{
                                println!("Data received from request: {}",String::from_utf8_lossy(&buffer));
                                match Request::try_from(&buffer[..]){
                                    Ok(request) =>{},
                                    Err(e)=>println!("Failed to convert the buffer:{}",e),
                                }
                            },
                            Err(e)=> println!("Failed to receive data from the buffer {}",e),
                        }

                    },
                    Err(e) => println!("{}",e),
                }
            }
        }
    }