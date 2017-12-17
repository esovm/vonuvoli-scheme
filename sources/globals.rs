

use super::counters::PermutationCounter;
use super::runtime::Handle;




pub mod exports {
	pub use super::context_handles_next;
	pub use super::bindings_handles_next;
	pub use super::lambdas_handles_next;
	pub use super::ports_handles_next;
}




static mut CONTEXT_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x514765cd,
		offset : 0x4d564bb6,
		initialized : false,
	};

#[ inline (always) ]
pub fn context_handles_next () -> (Handle) {
	unsafe {
		Handle::new (CONTEXT_HANDLES.next ())
	}
}




static mut BINDINGS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0xc8b5516d,
		offset : 0x767a8d5c,
		initialized : false,
	};

#[ inline (always) ]
pub fn bindings_handles_next () -> (Handle) {
	unsafe {
		Handle::new (BINDINGS_HANDLES.next ())
	}
}




static mut LAMBDAS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x33faad68,
		offset : 0xe28c918f,
		initialized : false,
	};

#[ inline (always) ]
pub fn lambdas_handles_next () -> (Handle) {
	unsafe {
		Handle::new (LAMBDAS_HANDLES.next ())
	}
}




static mut PORTS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0xa7e6ecf7,
		offset : 0x1e5c47b7,
		initialized : false,
	};

#[ inline (always) ]
pub fn ports_handles_next () -> (Handle) {
	unsafe {
		Handle::new (PORTS_HANDLES.next ())
	}
}

