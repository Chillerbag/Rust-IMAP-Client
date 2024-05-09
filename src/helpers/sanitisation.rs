

pub(crate) fn sanitise_string_to_literal(string :&str)-> String{
    format!("{{{}}}\r\n{}",string.len(), string )
}