use super::method::*;
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display,Result as FmtResult,Formatter,Debug};
use super::QueryString;
use std::str;

#[derive(Debug)]
pub struct Request<'from_buf> {
    path: &'from_buf str,
    query: Option<QueryString<'from_buf>>,
    method: HttpMethods,
}
impl<'from_buf> Request<'from_buf>{
    pub fn path(&self) -> &'from_buf str{
        &self.path
    }
    pub fn method(&self)-> &HttpMethods{
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString>{
        self.query.as_ref()
    }
}

// GET /search?name=abc&sort=1 HTTP/1.1 \r\n
impl <'from_buf> TryFrom<&'from_buf [u8]> for Request<'from_buf>{
    type Error=ParseError;
    fn try_from(buf: &'from_buf [u8]) -> Result<Request<'from_buf>, Self::Error>{
        let request=str::from_utf8(buf)?;
        let (method,request)=get_next_world(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path,request)=get_next_world(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,_)=get_next_world(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }
        let method:HttpMethods=method.parse()?;
        let mut query_string=None;

        if let Some(i)=path.find('?'){
            query_string=Some(QueryString::from(&path[i+1 ..])); //everything before ?
            path=&path[..i]; //everyting after ?
        }
        Ok(Self{
            path,
            query:query_string,
            method,
        })

     
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
