
    use crate::http::{Request,Response,StatusCode};
    use std::net::TcpListener;
    use std::io::{Read,Write};
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
                                let response= match Request::try_from(&buffer[..]){ //lifetime initiation
                                    Ok(request) =>{
                                        dbg!(request);
                                        Response::new(
                                            StatusCode::Ok,
                                            Some("<h1> it works babe</h1>".to_string()))
                                    },
        
                                    Err(e)=>{
                                        println!("Failed to receive data {}",e);
                                        Response::new(StatusCode::BadRequest,None)
                                    }  
        
                                };
                                if let Err(e)=response.send(&mut stream){
                                    println!("Failed to response {}",e)

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