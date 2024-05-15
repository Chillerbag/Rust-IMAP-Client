use crate::helpers::lexicon::rfc2045::*;

use super::general::{remove_start, DecodeProtocol};


impl DecodeProtocol for Field {
    fn can_parse(s:String) -> bool {
        s.split("\r\n").next().unwrap_or("").contains(":")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let mut rest = s;
        let mut field_body = None;
        let (rs, field_name) = FieldName::parse_new(rest)?;
        let rs = remove_start(":", rs)?;
        rest = rs;
        if FieldBody::can_parse(rest.to_string()) {
            let (rs,part) = FieldBody::parse_new(rest)?;
            rest = rs;
            field_body = Some(part);
        }
        let rest = remove_start("\r\n", rest)?;
        Ok((rest,Field { field_body, field_name}))
    }
}


impl DecodeProtocol for FieldBody {
    fn can_parse(s:String) -> bool {
        FieldBodyContents::can_parse(s)
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let mut next_field_body = None;
        let (mut rest,field_body_contents) = FieldBodyContents::parse_new(s)?;
        if rest.starts_with("\r\n ") {
            rest = remove_start("\r\n ", rest)?;
            let (rs,part) = FieldBody::parse_new(rest)?;
            rest =rs;
            next_field_body = Some(Box::new(part));
        }
        else if rest.starts_with("\r\n	") {
            rest = remove_start("\r\n	", rest)?;
            let (rs,part) = FieldBody::parse_new(rest)?;
            rest =rs;
            next_field_body = Some(Box::new(part));
        }

        Ok((rest,FieldBody {field_body_contents,next_field_body}))
    }
}

impl DecodeProtocol for FieldBodyContents {
    fn can_parse(s:String) -> bool {
        !s.starts_with("\r\n")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let rest =s;
        let invalid_chars = "\r\n";
        let (chars,rest) = rest.split_at(rest.chars().position(|c| invalid_chars.contains(c)).unwrap_or(rest.len()));
        Ok((rest.to_string(),FieldBodyContents {ascii:chars.to_string()}))
        
    }
}


impl DecodeProtocol for FieldName {
    fn can_parse(s:String) -> bool {
        !s.starts_with(": ")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        //TODO:CTLs
        let rest =s;
        let invalid_chars = ": ";
        let (chars,rest) = rest.split_at(rest.chars().position(|c| invalid_chars.contains(c)).unwrap_or(rest.len()));
        Ok((rest.to_string(),FieldName {chars:chars.to_lowercase().to_string()}))
        

    }
}
