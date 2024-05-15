/*
-------------------MOD.rs for lexicon------------
defines the command modules a part of this crate
also uses send_and_receive to deal with sending the command
and receiving the command from the server
------------------------------------------------
*/


pub mod login;
pub mod list;
pub mod mime;
pub mod parse;
pub mod retrieve;
use crate::helpers;
use helpers::send_and_receive;
