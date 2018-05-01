// exports from <IOKit/hid/IOHIDDevice.h>

use cf::{CFRunLoopRef, CFStringRef};
use io_hid_base::{IOHIDDeviceCallback, IOHIDDeviceRef, IOHIDElementRef, IOHIDValueRef};
use io_return::IOReturn;
use types::io_service_t;
use libc::c_void;

extern "C" {
    pub fn IOHIDDeviceRegisterRemovalCallback(
        device: IOHIDDeviceRef,
        callback: IOHIDDeviceCallback,
        context: *mut c_void,
    );
    pub fn IOHIDDeviceScheduleWithRunLoop(
        device: IOHIDDeviceRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
    pub fn IOHIDDeviceGetService(device: IOHIDDeviceRef) -> io_service_t;
    pub fn IOHIDDeviceGetValue(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        pvalue: *mut IOHIDValueRef,
    ) -> IOReturn;
}
