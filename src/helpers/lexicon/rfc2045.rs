
#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) field_name : FieldName,
    pub(crate) field_body : Option<FieldBody>
}

#[derive(Debug)]
pub(crate) struct FieldName {
    pub(crate) chars: String
} 

#[derive(Debug)]
pub(crate) struct FieldBody {
    pub(crate) field_body_contents:FieldBodyContents ,
    pub(crate) next_field_body: Option<Box<FieldBody>>
}

#[derive(Debug)]
pub(crate) struct FieldBodyContents {
    pub(crate) ascii:String
}