/*
------------SANITISE_STRING_TO_LITERAL-----------
convert the string to a literal so we know size
-------------------------------------------------
*/
pub(crate) fn sanitise_string_to_literal(string :&str)-> String{
    format!("{{{}}}\r\n{}",string.len(), string )
}