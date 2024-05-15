/*
-------------------MOD.rs for lexicon------------
defines the two modules that are a part of this crate
we allow dead code here. This is justifiable
because we want to parse the full response from the IMAP
server for completeness and extensibility. 
This is entirely sensible to do, as we should implement the
full response as defined in the section 9 of the IMAP rfc3501
rather than operate selectively. this is bad practice.
------------------------------------------------
*/

#[allow(dead_code)]
pub mod rfc2045;

#[allow(dead_code)]
pub mod rfc3501;
