#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use log::info;
use uefi::prelude::*;
use uefi::proto::device_path::text::{
    AllowShortcuts, DevicePathToText, DisplayOnly,
};
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::SearchType;
use uefi::table::runtime::RuntimeServices;
use uefi::{Identify, Result};
use uefi::guid;
use uefi::CStr16;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();

    print_image_path(boot_services).unwrap();

    info!("Hello world!");
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}

fn printvar(runtime_services: &RuntimeServices) {
    let varname: CStr16 = "Boot0000";
    let guid = uefi::table::runtime::VariableVendor( guid!("8be4df61-93ca-11d2-aa0d-00e098032b8c") );

    let size = runtime_services.get_variable_size(&varname, &guid);
    info!("varialbe size {}", size);
    let mut buf: [u8; 64] = ();
    runtime_services.get_variable(&varname, &guid, &mut buf)?;
    info!("var = {}", &buf);

}

fn print_image_path(boot_services: &BootServices) -> Result {
    let loaded_image = boot_services
        .open_protocol_exclusive::<LoadedImage>(boot_services.image_handle())?;

    let device_path_to_text_handle = *boot_services
        .locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))?
        .handles()
        .first()
        .expect("DevicePathToText is missing");

    let device_path_to_text = boot_services
        .open_protocol_exclusive::<DevicePathToText>(
            device_path_to_text_handle,
        )?;

    let image_device_path =
        loaded_image.file_path().expect("File path is not set");
    let (ptr, size) = loaded_image.info();
    info!("size: {}", size);
    let image_device_path_text = device_path_to_text
        .convert_device_path_to_text(
            boot_services,
            image_device_path,
            DisplayOnly(true),
            AllowShortcuts(false),
        )
        .expect("convert_device_path_to_text failed");

    info!("Image path: {}", &*image_device_path_text);
    Ok(())
}
