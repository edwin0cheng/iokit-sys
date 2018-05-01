// exports from <IOKit/hid/IOHIDKeys.h>

use libc::{c_void, c_int};

pub type IOHIDReportType = c_int;
pub const kIOHIDReportTypeInput: IOHIDReportType   = 0;
pub const kIOHIDReportTypeOutput: IOHIDReportType  = 1;
pub const kIOHIDReportTypeFeature: IOHIDReportType = 2;
pub const kIOHIDReportTypeCount: IOHIDReportType   = 3;

pub type IOHIDElementCookie = *mut c_void;

pub type IOHIDElementType = c_int;
pub const kIOHIDElementTypeInput_Misc: IOHIDElementType      = 1;
pub const kIOHIDElementTypeInput_Button: IOHIDElementType    = 2;
pub const kIOHIDElementTypeInput_Axis: IOHIDElementType      = 3;
pub const kIOHIDElementTypeInput_ScanCodes: IOHIDElementType = 4;
pub const kIOHIDElementTypeOutput: IOHIDElementType          = 129;
pub const kIOHIDElementTypeFeature: IOHIDElementType         = 257;
pub const kIOHIDElementTypeCollection: IOHIDElementType      = 513;
