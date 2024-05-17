//! The "X11 Window System Protocol" standard defines in Appendix A the keysym
//! codes. These 29-bit integer values identify characters or functions
//! associated with each key (e.g., via the visible engraving) of a keyboard
//! layout.
//!
//! This library contains mappings between mnemonic macro names and these keysym
//! codes.

#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/mapping.rs"));

/// Look up a record by the mnemonic macro name
#[cfg(feature = "by_name")]
pub fn lookup_by_name(name: &str) -> Option<&'static Record> {
    BY_NAMES.get(&name).copied()
}

/// Look up a record by unicode char (unicode code point)
#[cfg(feature = "by_codepoint")]
pub fn lookup_by_codepoint(codepoint: char) -> Option<&'static Record> {
    BY_CODEPOINT.get(&codepoint).copied()
}

/// Look up a mnemonic macro name by the keysym code
#[cfg(feature = "by_keysym")]
pub fn lookup_by_keysym(keysym: u32) -> Option<&'static Record> {
    BY_KEYSYM.get(&keysym).copied()
}

#[test]
fn access_works() {
    #[cfg(feature = "by_name")]
    assert!(lookup_by_name("Uhorngrave").is_some());
    #[cfg(feature = "by_codepoint")]
    assert!(lookup_by_codepoint('\u{1EEA}').is_some());
    #[cfg(feature = "by_keysym")]
    assert!(lookup_by_keysym(0x1eea).is_some());
}
