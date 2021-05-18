use super::server::Handler;
use super::http::{Request,Response,StatusCode,HttpMethods};

pub struct Websitehandler;

impl Handler for Websitehandler{
    fn handle_request(&mut self,request: &Request)-> Response{
        match request.method(){
            HttpMethods::GET => {
                match request.path(){
                    "/" => Response::new(StatusCode::Ok,Some("<h1> You're homie babe</h1>".to_string())),
                    "/hello" => Response::new(StatusCode::Ok,Some("<h1> Hello mate!</h1>".to_string())),    
                    _ => Response::new(StatusCode::NotFound,None),
                         
                }
            }
            _ => Response::new(StatusCode::NotFound,None)
        }

    }    
}