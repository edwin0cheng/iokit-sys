// exports from <IOKit/hid/IOHIDElement.h>

use cf::{CFTypeID, CFIndex, CFArrayRef};
use io_hid_base::{IOHIDElementRef};
use io_hid_keys::{IOHIDElementCookie, IOHIDElementType};

extern "C" {
    pub fn IOHIDElementGetTypeID() -> CFTypeID;
    pub fn IOHIDElementGetCookie(element: IOHIDElementRef) -> IOHIDElementCookie;
    pub fn IOHIDElementGetUsagePage(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetUsage(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetType(element: IOHIDElementRef) -> IOHIDElementType;
    pub fn IOHIDElementGetChildren(element: IOHIDElementRef) -> CFArrayRef;

    pub fn IOHIDElementGetLogicalMin(element: IOHIDElementRef) -> CFIndex;
    pub fn IOHIDElementGetLogicalMax(element: IOHIDElementRef) -> CFIndex;
}

