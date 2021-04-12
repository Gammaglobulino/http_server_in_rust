use super::method::*;
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display,Result as FmtResult,Formatter,Debug};
use std::str;

pub struct Request {
    path: String,
    query: Option<String>,
    method: HttpMethods,
}

//GET / HTTP/1.1
impl TryFrom<&[u8]> for Request{
    type Error=ParseError;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{
        let request=str::from_utf8(buf)?;
        let (method,request)=get_next_world(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path,request)=get_next_world(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,_)=get_next_world(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }
        let method:HttpMethods=method.parse()?;
        let mut query_string=None;
        match path.find('?'){
            Some(i) => {
                query_string=Some(&path[i+1 ..]);
                path=&path[..i];
            },
            None=>{},
        }

        unimplemented!()
    }
}

fn get_next_world(request:&str) -> Option<(&str,&str)>{
    for (i,c) in request.chars().enumerate(){
        if c==' ' || c=='\r'{
            return Some((&request[..i],&request[i+1..]));
        }
    }
    None
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f,"{}",self.message())
    }


}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self)-> &str{
        match self{
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f,"{}",self.message())
    }
}

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self{
        ParseError::InvalidMethod
    }
}
impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self{
        ParseError::InvalidEncoding
    }
}
impl Error for ParseError{

}
