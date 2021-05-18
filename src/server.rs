
    use crate::http::{Request,Response,StatusCode,ParseError};
    use std::net::TcpListener;
    use std::io::{Read,Write};
    use std::convert::TryFrom;

    pub trait Handler{
        fn handle_request(&mut self, request: &Request) ->Response;
        fn handle_bad_request(&mut self,e:&ParseError) ->Response{
            println!("Filed to parse request: {}",e);
            Response::new(StatusCode::BadRequest, None)
        }
    }
    pub struct Server {
        ip_addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Server {
            Server { ip_addr: addr }
        }
        pub fn run(self, mut handler:impl Handler) {
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
                                    Ok(request) => handler.handle_request(&request),
                                    Err(e)=> handler.handle_bad_request(&e)
        
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