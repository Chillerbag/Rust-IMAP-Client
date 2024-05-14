

pub(crate) fn remove_start(start:&str,string:String) -> Result<String,String> {
    if !string.starts_with(start) {return Err(format!("String didn't start with \"{}\" full string is: {}", start,string));}
    let (_,rest) = string.split_at(start.len());
    Ok(rest.to_string())
}

pub(crate) trait DecodeProtocol {
    fn can_parse(s:String) -> bool;
    
    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized;
}