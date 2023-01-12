#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use alloc::vec;
use log::info;
use uefi::prelude::*;
use uefi::table::runtime::RuntimeServices;
use uefi::guid;
use uefi::CString16;

unsafe fn init(boot_services: &BootServices) {
    uefi::alloc::init(boot_services);
}

#[entry]
unsafe fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();
    init(boot_services);

    printvar(system_table.runtime_services());
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}

fn printvar(runtime_services: &RuntimeServices) {
    let n = CString16::try_from("LoaderInfo").unwrap();
    let guid = uefi::table::runtime::VariableVendor( guid!("4a67b082-0a4c-41cf-b6c7-440b29bb8c4f") );

    let size = runtime_services.get_variable_size(n.as_ref(), &guid).expect("Error getting var size");
    // info!("variable size {}", size);

    let mut buf = vec![0u8; size];
    let _res = runtime_services.get_variable(n.as_ref(), &guid, buf.as_mut_slice()).expect("Error reading var");

    let u16v: alloc::vec::Vec<u16> = buf.chunks_exact(2).into_iter()
        .map(|a| match u16::from_ne_bytes([a[0],a[1]]) {
            x if x > 64 => x,
            _ => 0x002Eu16,
        })
        .chain([0u16]) // nul terminate
        .collect();
    // info!("u16v = {:?}", u16v);
    let s = CString16::try_from(u16v).expect("Errro converfting");
    info!("{} = \"{}\"", n, s);
}
