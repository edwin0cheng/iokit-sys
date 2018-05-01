// exports from <IOKit/hid/IOHIDValue.h>

use cf::{CFIndex};
use io_hid_base::{IOHIDValueRef};

extern "C" {
    pub fn IOHIDValueGetIntegerValue(value: IOHIDValueRef) -> CFIndex;
}
