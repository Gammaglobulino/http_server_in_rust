use std::collections::HashMap;

struct QueryString<'from_buf>{
    data: HashMap<&'from_buf str,Value<'from_buf>>
}

enum Value<'from_buf>{
    Single(&'from_buf str),
    Multiple(Vec<&'from_buf str>)
}
impl <'from_buf> QueryString<'from_buf>{
    pub fn get(&self,k:&str) -> Option<&Value>{
        self.data.get(k)
    }
}

impl <'from_buf> From<&'from_buf str> for QueryString<'from_buf>{
    fn from(s:&'from_buf str)->Self{
        let mut data=HashMap::new();
        for sub_str in s.split('&'){
            let mut key=sub_str;
            let mut value="";
            if let Some(i)=sub_str.find('='){
                key=&sub_str[..i];
                value=&sub_str[i+1..];
            }
            data.entry(key)
            .or_insert(Value::Single(value));
        } 
        QueryString{data}
    } 
}