// exports from <IOKit/hid/IOHIDDevice.h>

use cf::{CFRunLoopRef, CFStringRef};
use io_hid_base::{IOHIDCallback, IOHIDDeviceRef, IOHIDElementRef, IOHIDValueRef};
use io_return::IOReturn;
use libc::c_void;
use types::io_service_t;
use types::IOOptionBits;

extern "C" {
    pub fn IOHIDDeviceClose(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;

    pub fn IOHIDDeviceRegisterRemovalCallback(
        device: IOHIDDeviceRef,
        callback: IOHIDCallback,
        context: *mut c_void,
    );
    pub fn IOHIDDeviceScheduleWithRunLoop(
        device: IOHIDDeviceRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
    pub fn IOHIDDeviceUnscheduleFromRunLoop(
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
