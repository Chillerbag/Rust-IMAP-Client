use std::vec::Vec;
use std::option::Option;

pub(crate) type AString = String;

pub(crate) type Atom = String;

#[derive(Debug)]
pub(crate) struct Base64{ base64:String}

// enum Body {
//     BodyType1part(BodyType1part),
//     //WARN: type recursion stuff happens soon 
//     BodyTypeMpart(BodyTypeMpart),
// }

#[derive(Debug)]
pub(crate) enum ContinueReq {
    RespText(RespText),
    Base64(Base64),
}

#[derive(Debug)]
pub(crate) enum Flag {
    Answered,
    Flagged,
    Deleted,
    Seen,
    Draft,
    //WARN:replaces both flag-keyword / flag-extension
    Atom(Atom),
}

#[derive(Debug)]
pub(crate) enum FlagFetch {
    Recent,
    Flag(Flag)
} 

pub(crate) type HeaderFldName = AString;

pub(crate) type HeaderList =  Vec<HeaderFldName>;

#[derive(Debug)]
pub(crate) struct MessageData {
    pub(crate) nz_number: NzNumber,
    pub(crate) message_data_component : MessageDataComponent
}

#[derive(Debug)]
pub(crate) enum MessageDataComponent {
    Expunge,
    Fetch(MsgAtt),
}

pub(crate) type MsgAtt = Vec<MsgAttComponent>;

#[derive(Debug)]
pub(crate) enum MsgAttComponent {
    MsgAttDynamic(MsgAttDynamic),
    MsgAttStatic(MsgAttStatic),
}

pub(crate) type MsgAttDynamic = Vec<FlagFetch>;

#[derive(Debug)]
pub(crate) enum MsgAttStatic {
    // Envolope(Envolope),
    // Internaldate(DateTime),
    RFC822(MsgAttStaticRFC822Component),
    RFC822Size(Number),
    NonStructuredBody(MsgAttStaticBodyNonStructuredComponent),
    StructuredBody(MsgAttStaticBodyStructuredComponent),
    // UID(UID),
}

#[derive(Debug)]
pub(crate) struct MsgAttStaticBodyStructuredComponent {
    structure:bool,
    // body:Body,
    //TODO:Body
}

#[derive(Debug)]
pub(crate) struct MsgAttStaticBodyNonStructuredComponent {
    pub(crate) section:Section,
    pub(crate) number:Option<Number>,
    pub(crate) nstring:NString,
}

#[derive(Debug)]
pub(crate) enum MsgAttStaticRFC822Component {
    Header,
    Text,
}

pub(crate) type NString = Option<String>; 

//TODO:Differentiate these Numbers into structs that hold only u32, need different impl DecodeIMAP
pub(crate) type Number = u32;

pub(crate) type NzNumber = u32;

#[derive(Debug)]
pub(crate) struct RespCondBye {
    pub(crate) resp_text:RespText
}

#[derive(Debug)]
pub(crate) enum RespCondState {
    Ok(RespText),
    No(RespText),
    Bad(RespText),
}

#[derive(Debug)]
pub(crate) struct Response {
    pub(crate) response_components :Vec<ResponseComponent>,
    pub(crate) response_done: ResponseDone,
}

#[derive(Debug)]
pub(crate) enum ResponseComponent {
    ContinueReq(ContinueReq),
    ResponseData(ResponseData),
}

#[derive(Debug)]
pub(crate) enum ResponseData {
    RespCondBye(RespCondBye),
    RespCondState(RespCondState),
    // MailboxData(MailboxData),
    MessageData(MessageData),
    // CapabilityData(CapabilityData),
}

#[derive(Debug)]
pub(crate) enum ResponseDone {
    ResponseTagged(ResponseTagged),
    ResponseFatal(ResponseFatal),
}

#[derive(Debug)]
pub(crate) struct ResponseFatal {
    pub(crate) resp_cond_bye:RespCondBye
}

#[derive(Debug)]
pub(crate) struct ResponseTagged {
    pub(crate) tag : Tag,
    pub(crate) resp_cond_state :RespCondState,
}

#[derive(Debug)]
pub(crate) struct RespText {
    pub(crate) resp_text_code : Option<RespTextCode>,
    pub(crate) text : Text,
}

#[derive(Debug)]
pub(crate) enum RespTextCode {
    //WARN:Many fields missing here
    Alert,
    // BADCHARSET(()),
    Parse,
    ReadOnly,
    ReadWrite,
    TryCreate,
}

pub(crate) type Section = Option<SectionSpec>;


#[derive(Debug)]
pub(crate) enum SectionMsgtext {
    Header,
    HeaderFields((bool,HeaderList)),
    Text
}

pub(crate) type SectionPart = Vec<NzNumber>;

#[derive(Debug)]
pub(crate) enum SectionSpec {
    SectionMsgtext(SectionMsgtext),
    SectionPart(SectionPart),
}

#[derive(Debug)]
pub(crate) struct Tag {
    pub(crate) chars:String
}

#[derive(Debug)]
pub(crate) struct Text {
    pub(crate) text:String
}

