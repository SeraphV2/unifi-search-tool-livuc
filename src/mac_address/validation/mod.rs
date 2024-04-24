// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY THE BUILD SCRIPT

use once_cell::sync::Lazy;
use regex_automata::{
	dfa::{dense, Automaton},
	util::wire::AlignAs,
	Anchored, Input,
};

pub static MAC_ADDR_REGEX: Lazy<dense::DFA<&'static [u32]>> = Lazy::new(|| {
	static ALIGNED: &AlignAs<[u8], u32> = &AlignAs {
		_align: [],
		#[cfg(target_endian = "big")]
		bytes: *include_bytes!("mac_addr_regex.bigendian.dfa"),
		#[cfg(target_endian = "little")]
		bytes: *include_bytes!("mac_addr_regex.littleendian.dfa"),
	};
	let (dense_dfa, _) = dense::DFA::from_bytes(&ALIGNED.bytes).expect("serialized dense::DFA should be valid");
	dense_dfa
});

pub(crate) fn text_is_valid_mac<S: AsRef<[u8]>>(text: S) -> bool {
	MAC_ADDR_REGEX
		.try_search_fwd(&Input::new(&text).anchored(Anchored::Yes).earliest(true))
		.map_or(false, |x| x.is_some())
}