use crate::core::structs::Data;

pub mod system_info;

pub fn register(
) -> Vec<lumi::Command<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>> {
    vec![system_info::system_info()]
}
