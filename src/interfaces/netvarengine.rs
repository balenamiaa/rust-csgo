use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::interfaces::{BaseClient, ClientClassIterator};
use crate::structs::{CRecvProp, ClientClass};
use crate::structs::{CRecvTable, EPropType};

use once_cell::sync::OnceCell;

use crate::memory::{get_module_handle, pattern_scan};
use crate::structs::EntityIndex;

pub static INSTANCE: OnceCell<NetvarEngine> = OnceCell::new();

#[derive(Debug)]
pub struct NetvarEngine {
    pub netvars: HashMap<String, usize>,
    pub tables: Vec<&'static mut CRecvTable>,
}

impl Default for NetvarEngine {
    fn default() -> Self {
        let mut ret = Self {
            netvars: HashMap::new(),
            tables: Vec::new(),
        };

        ret.initialize();

        ret
    }
}

impl NetvarEngine {
    unsafe fn store_props(
        &mut self,
        group_name: String,
        recv_table: &'static mut CRecvTable,
        child_offset: usize,
    ) {
        for idx in 0..recv_table.n_props {
            let prop = recv_table.p_props.add(idx);

            if !prop.is_null() {
                let prop: &'static mut CRecvProp = std::mem::transmute(prop);
                let child = prop.data_table;

                if !child.is_null() {
                    let child: &'static mut CRecvTable = std::mem::transmute(child);

                    if child.n_props > 0 {
                        self.store_props(group_name.clone(), child, prop.offset as usize);
                    }
                }

                let var_name = CStr::from_ptr(prop.prop_name).to_str().unwrap().to_owned();

                let formatted = format!("{}->{}", group_name, var_name);

                if !self.netvars.contains_key(&var_name)
                    && (prop.prop_type == EPropType::Int
                        || prop.prop_type == EPropType::Vec
                        || prop.prop_type == EPropType::VecXY
                        || prop.prop_type == EPropType::String
                        || prop.prop_type == EPropType::Float)
                {
                    self.netvars.insert(
                        formatted.to_owned(),
                        prop.offset as usize + child_offset as usize,
                    );
                }
            }
        }
    }

    fn initialize(&mut self) {
        self.tables.clear();
        for client_class in ClientClassIterator::default() {
            let recv_table = client_class.recv_table;

            if !recv_table.is_null() {
                unsafe {
                    let recv_table_0: &mut CRecvTable = std::mem::transmute(recv_table); // TODO: Yuck. Implement this in a less uglier way, please.
                    let recv_table_1: &mut CRecvTable = std::mem::transmute(recv_table); // TODO: Yuck. Implement this in a less uglier way, please.
                                                                                         // Sorry, rustc. ^,^

                    self.tables.push(recv_table_0);

                    let table_name = CStr::from_ptr(recv_table_1.table_name)
                        .to_str()
                        .unwrap()
                        .to_owned();

                    self.store_props(table_name, recv_table_1, 0);
                }
            }
        }

        self.netvars
            .insert("CustomTable->Dormancy".to_owned(), 0xED);
        unsafe {
            self.netvars.insert(
                "CustomTable->InReload".to_owned(),
                pattern_scan(
                    get_module_handle("client.dll") as *mut _,
                    "C6 87 ? ? ? ? ? 8B 06 8B CE FF 90",
                ) as usize,
            );
        }
    }

    //I'd rather provide a static public wrapper than expose this because of the netvars_derive proc macro's limitation;
    fn get_offset(&self, table: &str, netvar: &str) -> usize {
        let pair = format!("{}->{}", table, netvar);

        if self.netvars.contains_key(&pair) {
            *self.netvars.get(&pair).unwrap()
        } else {
            usize::default()
        }
    }

    pub fn get() -> &'static NetvarEngine {
        INSTANCE
            .get()
            .expect("Tried to get NetvarEngine without initializing it first")
    }
}

pub fn get_offset(table: &str, netvar: &str) -> usize {
    NetvarEngine::get().get_offset(table, netvar)
}
