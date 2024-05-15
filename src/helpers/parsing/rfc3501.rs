use crate::helpers::lexicon::rfc3501::*;
use super::general::*;



pub(crate) fn parse_response(s:String) -> Result<Response,String> {
    let (remaining_string, resp) = Response::parse_new(s)?;
    if remaining_string != "" {return Err("Left over string in Response parsing".to_string());}
    Ok(resp)
}

impl DecodeProtocol for Base64 {
    fn can_parse(_s:String) -> bool {
        todo!()
    }

    fn parse_new(_s:String) -> Result<(String,Self),String> where Self: Sized {
        todo!()
    }
}

impl DecodeProtocol for ContinueReq {
    fn can_parse(s:String) -> bool {
        s.starts_with("+ ") && s.contains("\r\n")
    }

    fn parse_new(s:String) -> Result<(String,ContinueReq),String> {
        let mut rest = remove_start("+ ", s)?;
        let part = match rest.to_string() {
            s if RespText::can_parse(s.to_string()) => {
                let (rs,part) = RespText::parse_new(s)?;
                rest = rs;
                ContinueReq::RespText(part)
            }
            s if Base64::can_parse(s.to_string()) => {
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

impl DecodeProtocol for EnvNAddress {
    fn can_parse(s:String) -> bool {
        s.starts_with("(") ||  s.starts_with("NIL")
    }
    fn parse_new(s:String) -> Result<(String, Self), String> where Self: Sized {
        if s.starts_with("NIL") {
            let rest = remove_start("NIL", s)?;
            let addresses:Vec<Address> = Vec::new();
            return Ok((rest, EnvNAddress { address: addresses}))
        }
        match s {
            s if s.starts_with("(") => {
                let mut rest = remove_start("(", s)?;
                let mut addresses:Vec<Address> = Vec::new();
                
                while !rest.starts_with(")") {
                    match rest {
                        s if s.starts_with("NIL") => {
                            rest = remove_start("NIL", s)?;
                        }
                        _ => {
                            let (new_rest, address) = Address::parse_new(rest)?;
                            rest = new_rest;
                            addresses.push(address);
                            
                            if rest.starts_with(" ") {
                                rest = remove_start(" ", rest)?;
                            } else {
                                break; 
                            }
                        }
                    }
                }
                
                if rest.starts_with(")") {
                    rest = remove_start(")", rest)?;
                } else {
                    return Err("Invalid format: ')' not found".to_string());
                }
                
                Ok((rest, EnvNAddress { address: addresses}))
            }
            _ => Err("Envelope parsing failure".to_string())
        }
    }
}


impl DecodeProtocol for Envelope {
    fn can_parse(s:String) -> bool {
        s.starts_with("(") && s.contains("\r\n")
    }
    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let rest = remove_start("(", s)?;
        let (rest,env_date) = NString::parse_new(rest)?;
        let rest = remove_start(" ", rest)?;
        let (rest,env_subject) = NString::parse_new(rest)?;
        let rest = remove_start(" ", rest)?;
        let (rest,env_from) = EnvNAddress::parse_new(rest)?;
        let rest = remove_start(" ", rest)?;
        let (rest,env_sender) = EnvNAddress::parse_new(rest)?;
        let rest = remove_start(" ", rest)?;
        let (rest,env_reply_to) = EnvNAddress::parse_new(rest)?;
        let rest = remove_start(" ", rest)?;
        let (rest,env_to) = EnvNAddress::parse_new(rest)?;
        let rest = remove_start(" ", rest)?;
        let (rest,env_cc) = EnvNAddress::parse_new(rest)?;
        let rest = remove_start(" ", rest)?;
        let (rest,env_bcc) = EnvNAddress::parse_new(rest)?;
        let rest = remove_start(" ", rest)?;
        let (rest,env_in_reply_to) = NString::parse_new(rest)?;
        let rest = remove_start(" ", rest)?;
        let (rest,env_message_id) = NString::parse_new(rest)?;
        let rest = remove_start(")", rest)?;
        Ok((rest,Envelope{env_date, env_subject, env_from, env_sender, env_in_reply_to, env_to, env_bcc, env_reply_to, env_cc, env_message_id}))
        
    }
    
}

impl DecodeProtocol for Address {
    fn can_parse(s:String) -> bool {
        s.starts_with("(")
    }
    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s {
            s if s.starts_with("(") => {
                let rest = remove_start("(", s)?;
                let (rest,addr_name) = NString::parse_new(rest)?;
                let rest = remove_start(" ", rest)?;
                let (rest,addr_adl) = NString::parse_new(rest)?;
                let rest = remove_start(" ", rest)?;
                let (rest,addr_mailbox) = NString::parse_new(rest)?;
                let rest = remove_start(" ", rest)?;
                let (rest,addr_host) = NString::parse_new(rest)?;
                let rest = remove_start(")", rest)?;        
                Ok((rest,Address{addr_name, addr_adl, addr_mailbox, addr_host}))
                
            }
            _ => Err("Address parsing failure".to_string())
        }
        
    }
    
}
impl DecodeProtocol for MessageData {
    fn can_parse(s:String) -> bool {
        let Some((fs,ss)) = s.split_once(" ") else {return false;};
        NzNumber::can_parse(fs.to_string()) && MessageDataComponent::can_parse(ss.to_string())
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let Some((fs,ss)) = s.split_once(" ") else {return Err("String changed since match".to_string());};
        let (remaining_string,nz_number) = NzNumber::parse_new(fs.to_string())?;
        if remaining_string != "" {return Err("Left over string".to_string());}
        let (rest,message_data_component) = MessageDataComponent::parse_new(ss.to_string())?;
        Ok((rest,MessageData {nz_number,message_data_component}))
    }
}

impl DecodeProtocol for MessageDataComponent {
    fn can_parse(s:String) -> bool {
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
                let (rest,msg_att) = MsgAtt::parse_new(rest)?;
                Ok((rest.to_string(),MessageDataComponent::Fetch(msg_att)))
            }
            _ => Err("MessageDataComponent parsing failure".to_string())
        }
    }
}

impl DecodeProtocol for MsgAtt {
    fn can_parse(s:String) -> bool {
        s.starts_with("(") && s.contains(")")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let remaining_string : &mut String = &mut remove_start("(",s)?;
        let mut response_components : Vec<MsgAttComponent> = Vec::new();

        while !remaining_string.starts_with(")") {
            match remaining_string.to_string() {
                s if  MsgAttStatic::can_parse(s.to_string()) => {
                    let (rs,next_part) =  MsgAttStatic::parse_new(s.to_string())?;
                    *remaining_string = rs;
                    response_components.push(MsgAttComponent::MsgAttStatic(next_part));
                }
                s if MsgAttDynamic::can_parse(s.to_string()) => {
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

impl DecodeProtocol for MsgAttDynamic {
    fn can_parse(_s:String) -> bool {
        todo!()
    }

    fn parse_new(_s:String) -> Result<(String,Self),String> where Self: Sized {
        todo!()
    }
}
impl DecodeProtocol for MsgAttStatic {
    fn can_parse(s:String) -> bool {
        s.starts_with("ENVELOPE") || s.starts_with("RFC822") || 
        s.starts_with("BODY")|| s.starts_with("RFC822.SIZE")|| 
        s.starts_with("UID")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s {
            s if s.starts_with("ENVELOPE ") => {
                let rest = remove_start("ENVELOPE ", s)?;
                let (rest,part) = Envelope::parse_new(rest)?;
                Ok((rest,MsgAttStatic::Envelope(part)))
            }
            s if s.starts_with("RFC822 ") => {
                let rest = remove_start("RFC822 ", s)?;
                let (rest,part) = MsgAttStaticRFC822Component::parse_new(rest)?;
                Ok((rest,MsgAttStatic::RFC822(part)))
            }
            
            s if s.starts_with("RFC822.SIZE ") => {
                let rest = remove_start("RFC822.SIZE  ", s)?;
                let (rest,part) = u32::parse_new(rest)?;
                Ok((rest,MsgAttStatic::RFC822Size(part)))
            }
            
            s if MsgAttStaticBodyStructuredComponent::can_parse(s.to_string()) => {
                let (rest,part) = MsgAttStaticBodyStructuredComponent::parse_new(s)?;
                Ok((rest,MsgAttStatic::StructuredBody(part)))
            }

            s if MsgAttStaticBodyNonStructuredComponent::can_parse(s.to_string()) => {
                let (rest,part) = MsgAttStaticBodyNonStructuredComponent::parse_new(s)?;
                Ok((rest,MsgAttStatic::NonStructuredBody(part)))
            }
            _ => {todo!();}
            
        }
    }
}

impl DecodeProtocol for MsgAttStaticBodyStructuredComponent {
    fn can_parse(s:String) -> bool {
        s.starts_with("BODY ") || s.starts_with("BODYSTRUCTURE ")
    }

    fn parse_new(_s:String) -> Result<(String,Self),String> where Self: Sized {
        todo!();
        //let mut structure = false;
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

impl DecodeProtocol for MsgAttStaticBodyNonStructuredComponent {
    fn can_parse(s:String) -> bool {
        s.starts_with("BODY")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let mut rest = remove_start("BODY", s)?;
        let (rs,section) = Section::parse_new(rest)?;
        rest =rs;
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

impl DecodeProtocol for String {
    fn can_parse(s:String) -> bool {
        s.starts_with("{") || s.starts_with("\"") 
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s {
            //literal
            s if (s.starts_with("{")) => {
                let rest = remove_start("{", s)?;
                let (rest,number) = Number::parse_new(rest)?;
                let rest = remove_start("}\r\n", rest)?;
                let (string,rest) = rest.split_at(number.try_into().unwrap());
                Ok((rest.to_string(), string.to_string()))
            }
            //quoted
            s if (s.starts_with("\""))  => {
                //TODO: do we need to convert backslash special chars in the quoted strings
                let rest = remove_start("\"", s)?;
                let invalid_chars = "\r\n\"";
                let (chars,rest) = rest.split_at(rest.chars().position(|c| invalid_chars.contains(c)).unwrap_or(rest.len()));
                let rest = remove_start("\"", rest.to_string())?;
                Ok((rest.to_string(),chars.to_string()))
            }
            _ => Err("String parse error".to_string())
        }
    }
}

impl DecodeProtocol for NString {
    fn can_parse(s:String) -> bool {
        String::can_parse(s.to_string()) || s.starts_with("NIL")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s {
            s if s.starts_with("NIL") => {
                let rest = remove_start("NIL", s)?;
                Ok((rest,None))
            }
            s if String::can_parse(s.to_string()) => {
                let (rest,part) = String::parse_new(s)?;
                Ok((rest,Some(part)))   
            }
            _ => {
                Err("Nstring parse error".to_string())
            }
        }
    }
}

impl DecodeProtocol for MsgAttStaticRFC822Component {
    fn can_parse(_s:String) -> bool {
        todo!()
    }

    fn parse_new(_s:String) -> Result<(String,Self),String> where Self: Sized {
        todo!()
    }
}

impl DecodeProtocol for NzNumber {
    fn can_parse(s:String) -> bool {
        s.split(" ").next().unwrap_or("a").parse::<i64>().is_ok()
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let (fs,ss) = s.split_at(s.chars().position(|c| !c.is_digit(10)).unwrap_or(s.len()));
        let number = fs.parse::<u32>().unwrap();
        Ok((ss.to_string(),number))
    }
}

impl DecodeProtocol for RespCondBye {
    fn can_parse(s:String) -> bool {
        s.starts_with("BYE ")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let rest = remove_start("BYE ", s)?;
        let (rest,resp_text) = RespText::parse_new(rest)?;
        Ok((rest,RespCondBye { resp_text}))
        
    }
}

impl DecodeProtocol for RespCondState {
    fn can_parse(s:String) -> bool {
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

impl DecodeProtocol for Response {
    fn can_parse(_s:String) -> bool {
        true
    }

    fn parse_new(s:String) -> Result<(String,Response),String> {
        let remaining_string : &mut String = &mut s.clone();
        let mut response_components : Vec<ResponseComponent> = Vec::new();
        while ContinueReq::can_parse(remaining_string.to_string()) || ResponseData::can_parse(remaining_string.to_string()) {
            if ContinueReq::can_parse(remaining_string.to_string()) {
                let (rs,next_part) =  ContinueReq::parse_new(remaining_string.to_string())?;
                *remaining_string = rs;
                response_components.push(ResponseComponent::ContinueReq(next_part));
            } else if ResponseData::can_parse(remaining_string.to_string()) {
                let (rs,next_part) =  ResponseData::parse_new(remaining_string.to_string())?;
                *remaining_string = rs;
                response_components.push(ResponseComponent::ResponseData(next_part));
            }
        }
        if !ResponseDone::can_parse(s) {return Err("ResponseDone not found".to_string());}
        let (rest,response_done) = ResponseDone::parse_new(remaining_string.to_string())?;
        Ok((rest,Response { response_components, response_done}))
    }
}

impl DecodeProtocol for ResponseData {
    fn can_parse(s:String) -> bool {
        s.starts_with("* ") && s.contains("\r\n")
    }

    fn parse_new(s:String) -> Result<(String,ResponseData),String> {
        let mut remaining_string = remove_start("* ",s)?;
        let mut rd = Err("ResponseData parsing failure:No response data component");
        match remaining_string.to_string() {
            s if RespCondBye::can_parse(s.to_string()) => {
                let (rs,next_part) =  RespCondBye::parse_new(s.to_string())?;
                remaining_string = rs;
                rd = Ok(ResponseData::RespCondBye(next_part));
            }
            s if RespCondState::can_parse(s.to_string()) => {
                let (rs,next_part) =  RespCondState::parse_new(s.to_string())?;
                remaining_string = rs;
                rd = Ok(ResponseData::RespCondState(next_part));
            }
            s if MessageData::can_parse(s.to_string()) => {
                let (rs,next_part) =  MessageData::parse_new(s.to_string())?;
                remaining_string = rs;
                rd = Ok(ResponseData::MessageData(next_part));
            }
            _ => {}
        }
        let rest = remove_start("\r\n",remaining_string.to_string())?;
        Ok((rest.to_string(),rd?))
    }
}

impl DecodeProtocol for ResponseDone {
    fn can_parse(s:String) -> bool {
        ResponseFatal::can_parse(s.to_string()) || ResponseTagged::can_parse(s)
    }

    fn parse_new(s:String) -> Result<(String,ResponseDone),String> {
        match s.to_string() {
            s if ResponseFatal::can_parse(s.to_string()) => {
                let (rest,part) = ResponseFatal::parse_new(s)?;
                Ok((rest,ResponseDone::ResponseFatal(part)))
            }
            s if ResponseTagged::can_parse(s.to_string()) => {
                let (rest,part) = ResponseTagged::parse_new(s)?;
                Ok((rest,ResponseDone::ResponseTagged(part)))
            }
            _ => {return Err("ResponseDone parsing failure".to_string());}
        }
    }
}

impl DecodeProtocol for ResponseFatal {
    fn can_parse(s:String) -> bool {
        s.starts_with("* ") && s.contains("\r\n")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let rest = remove_start("* ", s)?;
        let (rest,resp_cond_bye) = RespCondBye::parse_new(rest)?;
        let rest = remove_start("\r\n", rest)?;
        Ok((rest,ResponseFatal {resp_cond_bye}))
    }
}

impl DecodeProtocol for ResponseTagged {
    fn can_parse(s:String) -> bool {
        Tag::can_parse(s)
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

impl DecodeProtocol for RespText {
    fn can_parse(_s:String) -> bool {
        true
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
       if RespTextCode::can_parse(s.to_string()) {
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

impl DecodeProtocol for RespTextCode {
    fn can_parse(s:String) -> bool {
        s.starts_with("[") && s.contains("] ") 
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let (_code,rest) = s.split_once("] ").unwrap();
        Ok((rest.to_string(),RespTextCode::Alert))
        // TODO:care about the right code
    }
}

impl DecodeProtocol for Section {
    fn can_parse(s:String) -> bool {
        s.starts_with("[")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let rest = remove_start("[", s)?;
        if SectionSpec::can_parse(rest.to_string()) {
            let (rest, part) = SectionSpec::parse_new(rest)?;
            let rest = remove_start("]", rest)?;
            Ok((rest,Some(part)))
        } else {
            let rest = remove_start("]", rest)?;
            Ok((rest,None))
        }
    }
}

impl DecodeProtocol for SectionSpec {
    fn can_parse(s:String) -> bool {
        !s.starts_with("]")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s.to_string() {
            s if SectionMsgtext::can_parse(s.to_string()) => {
                let (rest,part) = SectionMsgtext::parse_new(s)?;
                Ok((rest,SectionSpec::SectionMsgtext(part)))
            }
            s if SectionSpecComponent::can_parse(s.to_string()) => {
                let (rest,part) = SectionSpecComponent::parse_new(s)?;
                Ok((rest,SectionSpec::SectionSpecComponent(part)))
                
            }
            _ => {Err("SectionSpec didnt match".to_string())}
        }
    }
}

impl DecodeProtocol for SectionSpecComponent {
    fn can_parse(s:String) -> bool {
        SectionPart::can_parse(s)
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let (mut rest,section_part) = SectionPart::parse_new(s)?;
        let mut section_text =None;
        if rest.starts_with(".") {
            rest = remove_start(".", rest)?;
            let (rs,part) = SectionText::parse_new(rest)?;
            rest = rs;
            section_text = Some(part);
        }
        Ok((rest,SectionSpecComponent {section_part, section_text}))
    }
}

impl DecodeProtocol for SectionText {
    fn can_parse(s:String) -> bool {
        s.starts_with("MIME") || SectionMsgtext::can_parse(s)
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s {
            s if s.starts_with("MIME") => {
                let rest = remove_start("MIME", s)?;
                Ok((rest,SectionText::MIME))
            }
            s if SectionMsgtext::can_parse(s.to_string()) => {
                let (rest,part) = SectionMsgtext::parse_new(s)?;
                Ok((rest,SectionText::SectionMsgtext(part)))
            }
            _ => Err("SectionText parsing failure".to_string())
        }
    }
}



impl DecodeProtocol for SectionMsgtext {
    fn can_parse(s:String) -> bool {
        s.starts_with("HEADER") ||
        s.starts_with("TEXT")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        match s {
            s if s.starts_with("HEADER.FIELDS ") => {
                let mut rest = remove_start("HEADER.FIELDS ", s)?;
                let mut not = false;
                if rest.starts_with(".NOT") {
                    rest = remove_start(".NOT", rest)?;
                    not = true;
                }
                let (rs,header_list) = HeaderList::parse_new(rest)?;
                Ok((rs,SectionMsgtext::HeaderFields((not,header_list))))

            }
            s if s.starts_with("HEADER") => {
                let rest = remove_start("HEADER", s)?;
                Ok((rest,SectionMsgtext::Header))
            }
            s if s.starts_with("TEXT") => {
                let rest = remove_start("TEXT", s)?;
                Ok((rest,SectionMsgtext::Text))
            }
            _ => Err("SectionMsgtext parsing failure".to_string())
        }
    }
}
impl DecodeProtocol for SectionPart {
    fn can_parse(s:String) -> bool {
        NzNumber::can_parse(s)
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let mut rest = s;
        let mut numbers = Vec::new();
        while NzNumber::can_parse(rest.to_string()) {
            let (rs,part) = NzNumber::parse_new(rest)?; 
            numbers.push(part);
            rest = remove_start(".", rs)?;
        }
        Ok((rest,SectionPart {numbers}))
    }
}

impl DecodeProtocol for HeaderList {
    fn can_parse(s:String) -> bool {
        s.starts_with("(")
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let mut header_fld_names = Vec::new();
        let rest = remove_start("(", s)?;
        let (mut rest,part) = HeaderFldName::parse_new(rest)?;
        header_fld_names.push(part);
        while !rest.starts_with(")") {
            rest = remove_start(" ", rest)?;
            let (rs,part) = HeaderFldName::parse_new(rest)?;
            header_fld_names.push(part);
            rest =rs;
        }
        let rest = remove_start(")", rest)?;
        Ok((rest,HeaderList {header_fld_names}))
    }
}
impl DecodeProtocol for AString {
    fn can_parse(s:String) -> bool {
        //TODO:Add CTL here and the other 3 occurances
        let invalid_astring_chars = "(){ %*\"\\";
        !invalid_astring_chars.contains(s.chars().next().unwrap_or('\n')) || String::can_parse(s)
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let invalid_astring_chars = "(){ \r%*\"\\";
        match s {
            s if String::can_parse(s.to_string()) => {
                let (rest,part) = String::parse_new(s)?;
                Ok((rest,AString::String(part)))
            }
            s if !invalid_astring_chars.contains(s.chars().next().unwrap_or('\r')) =>{                
                let (chars,ss) = s.split_at(s.chars().position(|c| invalid_astring_chars.contains(c)).unwrap_or(s.len()));

                Ok((ss.to_string(),AString::Achars(chars.to_string())))
            }

            _ => Err("Astring parsing failure".to_string())
        }
    }
}

impl DecodeProtocol for Tag {
    fn can_parse(_s:String) -> bool {
        true
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let invalid_chars = "(){ %*\"\\+";
        let (chars,ss) = s.split_at(s.chars().position(|c| invalid_chars.contains(c)).unwrap_or(s.len()));

        Ok((ss.to_string(),Tag {chars:chars.to_string()}))
    }
}

impl DecodeProtocol for Text {
    fn can_parse(_s:String) -> bool {
        true
    }

    fn parse_new(s:String) -> Result<(String,Self),String> where Self: Sized {
        let Some(index) = s.chars().position(|c| c == '\n' || c == '\r') else {return Err("Text parse did not find end of line".to_string())};
        let (text, rest) = s.split_at(index);
        Ok((rest.to_string(),Text {text : text.to_string()}))
    }
}
