use bitflags::bitflags;
use libfido2_sys::*;
use std::ptr::NonNull;

#[derive(PartialEq, Eq)]
pub struct Device {
    pub(crate) raw: NonNull<fido_dev>,
}

impl Device {
    pub fn is_fido2(&self) -> bool {
        unsafe { fido_dev_is_fido2(self.raw.as_ptr()) }
    }

    pub fn ctap_hid_info(&self) -> CTAPHIDInfo {
        unsafe {
            let device = self.raw.as_ptr();
            let protocol = fido_dev_protocol(device);
            let major = fido_dev_major(device);
            let minor = fido_dev_minor(device);
            let build = fido_dev_build(device);
            let flags = fido_dev_flags(device);
            let flags = CTAPHIDCapabilities::from_bits(flags).expect("Invalid capability flags");

            CTAPHIDInfo {
                protocol,
                major,
                minor,
                build,
                capabilities: flags,
            }
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Drop for Device {
    fn drop(&mut self) {
        let mut device = self.raw.as_ptr();
        unsafe {
            // This can return an error
            let _ = fido_dev_close(device);
            fido_dev_free(&mut device as *mut _);
        }
        assert!(device.is_null(), "Device was not freed");
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CTAPHIDInfo {
    pub protocol: u8,
    pub major: u8,
    pub minor: u8,
    pub build: u8,
    pub capabilities: CTAPHIDCapabilities,
}

bitflags! {
    pub struct CTAPHIDCapabilities: u8 {
        const WINK = FIDO_CAP_WINK as u8;
        const CBOR = FIDO_CAP_CBOR as u8;
        const NMSG = FIDO_CAP_NMSG as u8;
    }
}
