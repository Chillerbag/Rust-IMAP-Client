use std::vec::Vec;
use std::option::Option;

pub(crate) type AString = String;

pub(crate) type Atom = String;

pub(crate) struct Base64{ base64:String}

// enum Body {
//     BodyType1part(BodyType1part),
//     //WARN: type recursion stuff happens soon 
//     BodyTypeMpart(BodyTypeMpart),
// }

pub(crate) enum ContinueReq {
    RespText(RespText),
    Base64(Base64),
}

pub(crate) enum Flag {
    Answered,
    Flagged,
    Deleted,
    Seen,
    Draft,
    //WARN:replaces both flag-keyword / flag-extension
    Atom(Atom),
}

pub(crate) enum FlagFetch {
    Recent,
    Flag(Flag)
} 

pub(crate) type HeaderFldName = AString;

pub(crate) type HeaderList =  Vec<HeaderFldName>;

pub(crate) struct MessageData {
    pub(crate) nz_number: NzNumber,
    pub(crate) message_data_component : MessageDataComponent
}

pub(crate) enum MessageDataComponent {
    Expunge,
    Fetch(MsgAtt),
}

pub(crate) type MsgAtt = Vec<MsgAttComponent>;

pub(crate) enum MsgAttComponent {
    MsgAttDynamic(MsgAttDynamic),
    MsgAttStatic(MsgAttStatic),
}

pub(crate) type MsgAttDynamic = Vec<FlagFetch>;

pub(crate) enum MsgAttStatic {
    // Envolope(Envolope),
    // Internaldate(DateTime),
    RFC822(MsgAttStaticRFC822Component),
    RFC822Size(Number),
    Body(MsgAttStaticBodyComponent),
    // UID(UID),
}

pub(crate) enum MsgAttStaticBodyComponent {
    // Structured((bool,Body)),
    NonStructured((Section,Number,NString)),
}

pub(crate) enum MsgAttStaticRFC822Component {
    Header,
    Text,
}

pub(crate) type NString = Option<String>; 

pub(crate) type Number = i64;

pub(crate) type NzNumber = i64;


pub(crate) struct RespCondBye {
    pub(crate) resp_text:RespText
}

pub(crate) enum RespCondState {
    Ok(RespText),
    No(RespText),
    Bad(RespText),
}

pub(crate) struct Response {
    pub(crate) response_components :Vec<ResponseComponent>,
    pub(crate) response_done: ResponseDone,
}

pub(crate) enum ResponseComponent {
    ContinueReq(ContinueReq),
    ResponseData(ResponseData),
}

pub(crate) enum ResponseData {
    RespCondBye(RespCondBye),
    RespCondState(RespCondState),
    // MailboxData(MailboxData),
    MessageData(MessageData),
    // CapabilityData(CapabilityData),
}

pub(crate) enum ResponseDone {
    ResponseTagged(ResponseTagged),
    ResponseFatal(ResponseFatal),
}

pub(crate) struct ResponseFatal {
    pub(crate) resp_cond_bye:RespCondBye
}

pub(crate) struct ResponseTagged {
    pub(crate) tag : Tag,
    pub(crate) resp_cond_state :RespCondState,
}
pub(crate) struct Tag {
    pub(crate) chars:String
}

pub(crate) struct RespText {
    pub(crate) resp_text_code : Option<RespTextCode>,
    pub(crate) text : Text,
}

pub(crate) enum RespTextCode {
    //WARN:Many fields missing here
    Alert,
    // BADCHARSET(()),
    Parse,
    ReadOnly,
    ReadWrite,
    TryCreate,
}

pub(crate) type Section = SectionSpec;

pub(crate) enum SectionMsgtext {
    Header,
    HeaderFields((bool,HeaderList)),
    Text
}

pub(crate) type SectionPart = Vec<NzNumber>;

pub(crate) enum SectionSpec {
    SectionMsgtext(SectionMsgtext),
    SectionPart(SectionPart),
}

pub(crate) struct Text {
    pub(crate) text:String
}

