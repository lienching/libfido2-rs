use libfido2::*;
use std::ffi::CString;

// Source: https://github.com/Yubico/libfido2/blob/master/examples/cred.c
const CLIENT_DATA_HASH: [u8; 32] = [
    0xf9, 0x64, 0x57, 0xe7, 0x2d, 0x97, 0xf6, 0xbb, 0xdd, 0xd7, 0xfb, 0x06, 0x37, 0x62, 0xea, 0x26,
    0x20, 0x44, 0x8e, 0x69, 0x7c, 0x03, 0xf2, 0x31, 0x2f, 0x99, 0xdc, 0xaf, 0x3e, 0x8a, 0x91, 0x6b,
];
const USER_ID: [u8; 32] = [
    0x78, 0x1c, 0x78, 0x60, 0xad, 0x88, 0xd2, 0x63, 0x32, 0x62, 0x2a, 0xf1, 0x74, 0x5d, 0xed, 0xb2,
    0xe7, 0xa4, 0x2b, 0x44, 0x89, 0x29, 0x39, 0xc5, 0x56, 0x64, 0x01, 0x27, 0x0d, 0xbb, 0xc4, 0x49,
];

const USER_NAME: &'static str = "John Doe";
const RELYING_PARTY_ID: &'static str = "localhost";
const RELYING_PARTY_NAME: &'static str = "Oost West, Thuis Best";

pub fn main() {
    match _main() {
        Ok(()) => {}
        Err(e) => eprintln!("{}", e),
    }
}

pub fn _main() -> Result<(), FidoError> {
    let fido = Fido::new();
    let detected_devices = fido.detect_devices(1);
    let info = detected_devices.iter().next().expect("No device found");
    println!("Found device: {:#?}", info);
    let mut device = fido.new_device(info.path).expect("Unable to open device");
    println!("Mode: {:?}", device.mode());
    println!("CTAPHID info: {:#?}", device.ctap_hid_info());
    println!(
        "CBOR info: {:#?}",
        device
            .request_cbor_data()
            .expect("Unable to request CBOR info")
            .as_ref()
    );

    let mut creator = fido.new_credential_creator();
    creator.set_type(CredentialType::ES256)?;
    creator.set_client_data_hash(&CLIENT_DATA_HASH)?;
    creator.set_relying_party(
        &CString::new(RELYING_PARTY_ID).unwrap(),
        &CString::new(RELYING_PARTY_NAME).unwrap(),
    )?;
    creator.set_user(&USER_ID, &CString::new(USER_NAME).unwrap(), None, None)?;

    let credential = device.request_credential_creation(creator, None)?;
    println!("Created credential: {:?}", credential.as_ref());
    Ok(())
}