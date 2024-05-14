pub(crate) struct Field {
    field_name : FieldName,
    field_body : Option<FieldBody>
}

struct FieldName {
    chars: String
} 

struct FieldBody {
    field_body_contents:FieldBodyContents ,
    next_field_body: Option<Box<FieldBody>>
}

struct  FieldBodyContents {
    ascii:String
}