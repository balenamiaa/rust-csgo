use std::os::raw::c_char;

use super::recvprop;
use crate::structs::ClientClassId;

type CreateClientClassFn = unsafe extern "system" fn(ent: i32, serial: i32);
type CreateEventFn = unsafe extern "system" fn();

#[repr(C)]
pub struct ClientClass {
    create_client_class: CreateClientClassFn,
    create_event: CreateEventFn,
    network_name: *mut c_char,
    pub recv_table: *mut recvprop::CRecvTable,
    pub next: *mut ClientClass,
    pub class_id: ClientClassId,
}
