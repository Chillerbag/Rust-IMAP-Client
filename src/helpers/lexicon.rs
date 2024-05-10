use std::vec::Vec;
use std::option::Option;

type AString = String;

type Atom = String;

type Base64 = String;

// enum Body {
//     BodyType1part(BodyType1part),
//     //WARN: type recursion stuff happens soon 
//     BodyTypeMpart(BodyTypeMpart),
// }

enum ContinueReq {
    RespText(RespText),
    Base64(Base64),
}

enum Flag {
    Answered,
    Flagged,
    Deleted,
    Seen,
    Draft,
    //WARN:replaces both flag-keyword / flag-extension
    Atom(Atom),
}

enum FlagFetch {
    Recent,
    Flag(Flag)
} 

type HeaderFldName = AString;

type HeaderList =  Vec<HeaderFldName>;

struct MessageData {
    nz_number: NzNumber,
    message_data_component : MessageDataComponent
}

enum MessageDataComponent {
    EXPUNGE,
    FETCH(MsgAtt),
}

type MsgAtt = Vec<MsgAttComponent>;

enum MsgAttComponent {
    MsgAttDynamic(MsgAttDynamic),
    MsgAttStatic(MsgAttStatic),
}

type MsgAttDynamic = Vec<FlagFetch>;

enum MsgAttStatic {
    // Envolope(Envolope),
    // Internaldate(DateTime),
    RFC822(MsgAttStaticRFC822Component),
    RFC822Size(Number),
    Body(MsgAttStaticBodyComponent),
    // UID(UID),
}

enum MsgAttStaticBodyComponent {
    // Structured((bool,Body)),
    NonStructured((Section,Number,NString)),
}

enum MsgAttStaticRFC822Component {
    Header,
    Text,
}

type NString = Option<String>; 

type Number = i64;

type NzNumber = i64;


enum RespCondBye {
    BYE,
    RespText(RespText)
}

enum RespCondState {
    OK(RespText),
    No(RespText),
    Bad(RespText),
}

struct Response {
    response_components :Vec<ResponseComponent>,
    response_done: ResponseDone,
}

enum ResponseComponent {
    ContinueReq(ContinueReq),
    ResponseData(ResponseData),
}

enum ResponseData {
    RespCondState(RespCondState),
    RespCondBye(RespCondBye),
    // MailboxData(MailboxData),
    MessageData(MessageData),
    // CapabilityData(CapabilityData),
}

enum ResponseDone {
    ResponseTagged(ResponseTagged),
    ResponseFatal(ResponseFatal),
}

type ResponseFatal = RespCondBye;

struct ResponseTagged {
    tag : Tag,
    resp_cond_state :RespCondState,
}
type Tag = String;

struct RespText {
    resp_text_code : Option<RespTextCode>,
    text : Text,
}

enum RespTextCode {
    //WARN:Many fields missing here
    Alert,
    // BADCHARSET(()),
    Parse,
    ReadOnly,
    ReadWrite,
    TryCreate,

}

type Section = SectionSpec;

enum SectionMsgtext {
    Header,
    HeaderFields((bool,HeaderList)),
    Text
}

type SectionPart = Vec<NzNumber>;

enum SectionSpec {
    SectionMsgtext(SectionMsgtext),
    SectionPart(SectionPart),
}

type Text = String;

