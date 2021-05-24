use super::server::Handler;
use super::http::{Request,Response,StatusCode,HttpMethods};
use std::fs;


pub struct Websitehandler{
    public_path:String
}
impl Websitehandler{
    pub fn new(public_path:String)->Self{
        Self{public_path}
    }
    pub fn read_file(&self,file_path:&str)->Option<String>{
        let path=format!("{}/{}",self.public_path,file_path);
        fs::read_to_string(path).ok()    
    }
}

impl Handler for Websitehandler{
    fn handle_request(&mut self,request: &Request)-> Response{
        match request.method(){
            HttpMethods::GET => {
                match request.path(){
                    "/" => Response::new(StatusCode::Ok,self.read_file("index.html")),
                    "/hello" => Response::new(StatusCode::Ok,self.read_file("home.html")),    
                    path => match self.read_file(path){
                        Some(contents) => Response::new(StatusCode::Ok,Some(contents)),
                        None => Response::new(StatusCode::NotFound,None),
                    }
                         
                }
            }
            _ => Response::new(StatusCode::NotFound,None)
        }

    }    
}