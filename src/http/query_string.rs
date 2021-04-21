use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'from_buf>{
    data: HashMap<&'from_buf str,Value<'from_buf>>
}
#[derive(Debug,PartialEq)]
pub enum Value<'from_buf>{
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
            .and_modify(|existing: &mut Value| match existing{
                Value::Single(prev_val) =>{
                    *existing=Value::Multiple(vec![prev_val,value])
                },
                Value::Multiple(vec) => vec.push(value),

            })
            .or_insert(Value::Single(value));
        } 
        QueryString{data}
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn query_string_generation_is_ok() {
        let qr_tested:QueryString=QueryString::from("name=abc");
        assert_eq!(qr_tested.get("name").is_some(),true);
        assert_eq!(qr_tested.get("name").unwrap(),&Value::Single("abc"));
    }
    #[test]
    fn query_string_generation_is_not_ok() {
        let qr_tested:QueryString=QueryString::from("name=name=abc");
        assert_eq!(qr_tested.get("name").is_some(),true);
        assert_ne!(qr_tested.get("name").unwrap(),&Value::Single("abc"));
    }

}
