use super::lexicon::*;


pub(crate) fn parse_response(s:String) -> Result<Response,String> {
    let (remaining_string, resp) = Response::parse_new(s)?;
    if remaining_string != "" {return Err("Left over string in Response parsing".to_string());}
    Ok(resp)
}

fn remove_start(start:&str,string:String) -> Result<String,String> {
    if !string.starts_with(start) {return Err(format!("String didn't start with {}", start));}
    let (_,rest) = string.split_at(start.len());
    Ok(rest.to_string())
}

trait DecodeIMAP {
    fn can_match(s:String) -> bool;
    
    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized;
}

impl DecodeIMAP for Base64 {
    fn can_match(s:String) -> bool {
        todo!()
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        todo!()
    }
}

impl DecodeIMAP for ContinueReq {
    fn can_match(s:String) -> bool {
        s.starts_with("+ ") && s.contains("\r\n")
    }

    fn parse_new(s:String) -> Result<(String,ContinueReq),String> {
        let mut rest = remove_start("+ ", s)?;
        let part = match rest.to_string() {
            s if RespText::can_match(s.to_string()) => {
                let (rs,part) = RespText::parse_new(s)?;
                rest = rs;
                ContinueReq::RespText(part)
            }
            s if Base64::can_match(s.to_string()) => {
                let (rs,part) = Base64::parse_new(s)?;
                rest = rs;
                ContinueReq::Base64(part)
            }
            _ => {return Err("ContinueReq parsing failure".to_string());}
        };
        let rest = remove_start("\r\n", rest)?;
        Ok((rest,part))
    }
}

impl DecodeIMAP for MessageData {
    fn can_match(s:String) -> bool {
        let Some((fs,ss)) = s.split_once(" ") else {return false;};
        NzNumber::can_match(fs.to_string()) && MessageDataComponent::can_match(ss.to_string())
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let Some((fs,ss)) = s.split_once(" ") else {return Err("String changed since match".to_string());};
        let (remaining_string,nz_number) = NzNumber::parse_new(fs.to_string())?;
        if remaining_string != "" {return Err("Left over string".to_string());}
        let (rest,message_data_component) = MessageDataComponent::parse_new(ss.to_string())?;
        Ok((rest,MessageData {nz_number,message_data_component}))
    }
}

impl DecodeIMAP for MessageDataComponent {
    fn can_match(s:String) -> bool {
        s.starts_with("EXPUNGE") || s.starts_with("FETCH ")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s {
            s if s.starts_with("EXPUNGE") => {
                let rest = remove_start("EXPUNGE", s)?;
                Ok((rest,MessageDataComponent::Expunge))
            }
            s if s.starts_with("FETCH ") => {
                let rest = remove_start("FETCH ", s)?;
                let (rest,msgAtt) = MsgAtt::parse_new(rest)?;
                Ok((rest.to_string(),MessageDataComponent::Fetch(msgAtt)))
            }
            _ => Err("MessageDataComponent parsing failure".to_string())
        }
    }
}

impl DecodeIMAP for MsgAtt {
    fn can_match(s:String) -> bool {
        s.starts_with("(") && s.contains(")")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let remaining_string : &mut String = &mut remove_start("(",s)?;
        let mut response_components : Vec<MsgAttComponent> = Vec::new();

        while !remaining_string.starts_with(")") {
            match remaining_string.to_string() {
                s if  MsgAttStatic::can_match(s.to_string()) => {
                    let (rs,next_part) =  MsgAttStatic::parse_new(s.to_string())?;
                    *remaining_string = rs;
                    response_components.push(MsgAttComponent::MsgAttStatic(next_part));
                }
                s if MsgAttDynamic::can_match(s.to_string()) => {
                    let (rs,next_part) =  MsgAttDynamic::parse_new(s.to_string())?;
                    *remaining_string = rs;
                    response_components.push(MsgAttComponent::MsgAttDynamic(next_part));
                }
                _ => {return Err("Unkown component in MsgAtt".to_string());}
            }

        }
        
        let rest = remove_start(")",remaining_string.to_string())?;
        Ok((rest,response_components))
        
    }
}

impl DecodeIMAP for MsgAttDynamic {
    fn can_match(s:String) -> bool {
        todo!()
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        todo!()
    }
}
impl DecodeIMAP for MsgAttStatic {
    fn can_match(s:String) -> bool {
        s.starts_with("ENVELOPE") || s.starts_with("RFC822") || 
        s.starts_with("BODY")|| s.starts_with("RFC822.SIZE")|| 
        s.starts_with("UID")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s {
            s if s.starts_with("RFC822") => {
                let rest = remove_start("RFC822 ", s)?;
                let (rest,part) = MsgAttStaticRFC822Component::parse_new(rest)?;
                Ok((rest,MsgAttStatic::RFC822(part)))
            }
            
            s if s.starts_with("RFC822.SIZE ") => {
                let rest = remove_start("RFC822.SIZE  ", s)?;
                let (rest,part) = u32::parse_new(rest)?;
                Ok((rest,MsgAttStatic::RFC822Size(part)))
            }
            
            s if MsgAttStaticBodyStructuredComponent::can_match(s.to_string()) => {
                let (rest,part) = MsgAttStaticBodyStructuredComponent::parse_new(s)?;
                Ok((rest,MsgAttStatic::StructuredBody(part)))
            }

            s if MsgAttStaticBodyNonStructuredComponent::can_match(s.to_string()) => {
                let (rest,part) = MsgAttStaticBodyNonStructuredComponent::parse_new(s)?;
                Ok((rest,MsgAttStatic::NonStructuredBody(part)))
            }
            _ => {todo!();Err("RespCondState didn't match".to_string())}
            
        }
    }
}

impl DecodeIMAP for MsgAttStaticBodyStructuredComponent {
    fn can_match(s:String) -> bool {
        s.starts_with("BODY ") || s.starts_with("BODYSTRUCTURE ")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        todo!();
        let mut structure = false;
        // match s {
        //     s if s.starts_with("BODY ") => {
        //         let rest = remove_start("RFC822 ", s)?;
        //         let (rest,part) = MsgAttStaticRFC822Component::parse_new(rest)?;
        //         // Ok((rest,MsgAttStatic::RFC822(part)))
        //     }
            
        //     s if s.starts_with("BODYSTRUCTURE ") => {
        //         let rest = remove_start("RFC822.SIZE  ", s)?;
        //         let (rest,part) = i64::parse_new(rest)?;
        //         // Ok((rest,MsgAttStatic::RFC822Size(part)))
        //     }
        //     _ => {Err("MsgAttStaticBodyComponent didn't match".to_string())}
            
        // };
    }
}

impl DecodeIMAP for MsgAttStaticBodyNonStructuredComponent {
    fn can_match(s:String) -> bool {
        s.starts_with("BODY")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let mut rest = remove_start("BODY", s)?;
        let (rs,section) = Section::parse_new(rest)?;
        rest = rs;
        let mut number = None;
        if rest.starts_with("<") {
            rest = remove_start("<", rest)?;
            let (rs,part) = Number::parse_new(rest)?;
            rest = remove_start(">", rs)?;
            number = Some(part);
        }
        let rest = remove_start(" ", rest)?;
        let (rest,nstring) = NString::parse_new(rest)?;
        Ok((rest,MsgAttStaticBodyNonStructuredComponent {section,number,nstring}))
    }
}

impl DecodeIMAP for NString {
    fn can_match(s:String) -> bool {
        true || s.starts_with("NIL")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s {
            s if s.starts_with("NIL") => {
                let rest = remove_start("NIL", s)?;
                Ok((rest,None))
            }
            s if (s.starts_with("{")) => {
                let rest = remove_start("{", s)?;
                let (rest,number) = Number::parse_new(rest)?;
                let rest = remove_start("}\r\n", rest)?;
                let (string,rest) = rest.split_at(number.try_into().unwrap());
                Ok((rest.to_string(),Some(string.to_string())))
            }
            _ => {
                Err("asd".to_string())
            }
        }
    }
}

impl DecodeIMAP for MsgAttStaticRFC822Component {
    fn can_match(s:String) -> bool {
        todo!()
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        todo!()
    }
}

impl DecodeIMAP for NzNumber {
    fn can_match(s:String) -> bool {
        s.split(" ").next().unwrap_or("a").parse::<i64>().is_ok()
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let (fs,ss) = s.split_at(s.chars().position(|c| !c.is_digit(10)).unwrap_or(s.len()));
        let number = fs.parse::<u32>().unwrap();
        Ok((ss.to_string(),number))
    }
}

impl DecodeIMAP for RespCondBye {
    fn can_match(s:String) -> bool {
        s.starts_with("BYE ")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let rest = remove_start("BYE ", s)?;
        let (rest,resp_text) = RespText::parse_new(rest)?;
        Ok((rest,RespCondBye { resp_text}))
        
    }
}

impl DecodeIMAP for RespCondState {
    fn can_match(s:String) -> bool {
        s.starts_with("OK ") || 
        s.starts_with("NO ") ||
        s.starts_with("BAD ")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s {
            s if s.starts_with("OK ") => {
                let rest = remove_start("OK ", s)?;
                let (rest,part) = RespText::parse_new(rest)?;
                Ok((rest,RespCondState::Ok(part)))
            }
            
            s if s.starts_with("NO ") => {
                let rest = remove_start("NO ", s)?;
                let (rest,part) = RespText::parse_new(rest)?;
                Ok((rest,RespCondState::No(part)))
            }
            
            s if s.starts_with("BAD ") => {
                let rest = remove_start("BAD ", s)?;
                let (rest,part) = RespText::parse_new(rest)?;
                Ok((rest,RespCondState::Bad(part)))
            }
            _ => {Err("RespCondState didn't match".to_string())}
            
        }

    }
} 

impl DecodeIMAP for Response {
    fn can_match(_s:String) -> bool {
        true
    }

    fn parse_new(s:String) -> Result<(String,Response),String> {
        let remaining_string : &mut String = &mut s.clone();
        let mut response_components : Vec<ResponseComponent> = Vec::new();
        while ContinueReq::can_match(remaining_string.to_string()) || ResponseData::can_match(remaining_string.to_string()) {
            if ContinueReq::can_match(remaining_string.to_string()) {
                let (rs,next_part) =  ContinueReq::parse_new(remaining_string.to_string())?;
                *remaining_string = rs;
                response_components.push(ResponseComponent::ContinueReq(next_part));
            } else if ResponseData::can_match(remaining_string.to_string()) {
                let (rs,next_part) =  ResponseData::parse_new(remaining_string.to_string())?;
                *remaining_string = rs;
                response_components.push(ResponseComponent::ResponseData(next_part));
            }
        }
        if !ResponseData::can_match(s) {return Err("ResponseDone not found".to_string());}
        let (rest,response_done) = ResponseDone::parse_new(remaining_string.to_string())?;
        Ok((rest,Response { response_components, response_done}))
    }
}

impl DecodeIMAP for ResponseData {
    fn can_match(s:String) -> bool {
        s.starts_with("* ") && s.contains("\r\n")
    }

    fn parse_new(s:String) -> Result<(String,ResponseData),String> {
        let mut remaining_string = remove_start("* ",s)?;
        let mut rd = Err("No response data component");
        match remaining_string.to_string() {
            s if RespCondBye::can_match(s.to_string()) => {
                let (rs,next_part) =  RespCondBye::parse_new(s.to_string())?;
                remaining_string = rs;
                rd = Ok(ResponseData::RespCondBye(next_part));
            }
            s if RespCondState::can_match(s.to_string()) => {
                let (rs,next_part) =  RespCondState::parse_new(s.to_string())?;
                remaining_string = rs;
                rd = Ok(ResponseData::RespCondState(next_part));
            }
            s if MessageData::can_match(s.to_string()) => {
                let (rs,next_part) =  MessageData::parse_new(s.to_string())?;
                remaining_string = rs;
                rd = Ok(ResponseData::MessageData(next_part));
            }
            _ => {return Err("ResponseData parsing failure".to_string());}
        }
        let rest = remove_start("\r\n",remaining_string.to_string())?;
        Ok((rest.to_string(),rd?))
    }
}

impl DecodeIMAP for ResponseDone {
    fn can_match(s:String) -> bool {
        ResponseFatal::can_match(s.to_string()) || ResponseTagged::can_match(s)
    }

    fn parse_new(s:String) -> Result<(String,ResponseDone),String> {
        match s.to_string() {
            s if ResponseFatal::can_match(s.to_string()) => {
                let (rest,part) = ResponseFatal::parse_new(s)?;
                Ok((rest,ResponseDone::ResponseFatal(part)))
            }
            s if ResponseTagged::can_match(s.to_string()) => {
                let (rest,part) = ResponseTagged::parse_new(s)?;
                Ok((rest,ResponseDone::ResponseTagged(part)))
            }
            _ => {return Err("ResponseDone parsing failure".to_string());}
        }
    }
}

impl DecodeIMAP for ResponseFatal {
    fn can_match(s:String) -> bool {
        s.starts_with("* ") && s.contains("\r\n")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let rest = remove_start("* ", s)?;
        let (rest,resp_cond_bye) = RespCondBye::parse_new(rest)?;
        let rest = remove_start("\r\n", rest)?;
        Ok((rest,ResponseFatal {resp_cond_bye}))
    }
}

impl DecodeIMAP for ResponseTagged {
    fn can_match(s:String) -> bool {
        Tag::can_match(s)
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let Some((fs,ss)) = s.split_once(" ") else {return Err("ResponseTagged parse error".to_string());};
        let (rest,tag) = Tag::parse_new(fs.to_string())?;
        if rest != "" {return Err("ResponseTagged parse error:Left over string".to_string());}
        let (rest,resp_cond_state) = RespCondState::parse_new(ss.to_string())?;
        let rest = remove_start("\r\n", rest)?;
        Ok((rest,ResponseTagged {resp_cond_state, tag}))
    }
}

impl DecodeIMAP for RespText {
    fn can_match(s:String) -> bool {
        true
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
       if RespTextCode::can_match(s.to_string()) {
            let (rest,code) = RespTextCode::parse_new(s)?;
            let (rest,text) = Text::parse_new(rest)?;
            Ok((rest,RespText {resp_text_code : Some(code), text}))
       }
       else  {
            let (rest,text) = Text::parse_new(s)?;
            Ok((rest,RespText {resp_text_code : None, text}))
       }


    }
}

impl DecodeIMAP for RespTextCode {
    fn can_match(s:String) -> bool {
        s.starts_with("[") && s.contains("] ") 
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let (_code,rest) = s.split_once("] ").unwrap();
        Ok((rest.to_string(),RespTextCode::Alert))
        // TODO:care about the right code
    }
}

impl DecodeIMAP for Section {
    fn can_match(s:String) -> bool {
        s.starts_with("[")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let rest = remove_start("[", s)?;
        if SectionSpec::can_match(rest.to_string()) {
            let (rest, part) = SectionSpec::parse_new(rest)?;
            let rest = remove_start("]", rest)?;
            Ok((rest,Some(part)))
        } else {
            let rest = remove_start("]", rest)?;
            Ok((rest,None))
        }
    }
}

impl DecodeIMAP for SectionSpec {
    fn can_match(s:String) -> bool {
        !s.starts_with("]")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        todo!()
    }
} 

impl DecodeIMAP for Tag {
    fn can_match(s:String) -> bool {
        true
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let invalid_chars = "(){\r%*\"\\";
        let (chars,ss) = s.split_at(s.chars().position(|c| invalid_chars.contains(c)).unwrap_or(s.len()));

        Ok((ss.to_string(),Tag {chars:chars.to_string()}))
    }
}

impl DecodeIMAP for Text {
    fn can_match(_s:String) -> bool {
        true
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let Some(index) = s.chars().position(|c| c == '\n' || c == '\r') else {return Err("Text parse did not find end of line".to_string())};
        let (text, rest) = s.split_at(index);
        Ok((rest.to_string(),Text {text : text.to_string()}))
    }
}
