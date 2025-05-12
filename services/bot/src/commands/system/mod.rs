use crate::core::structs::Command;

pub mod system_info;

pub fn register(
) -> Vec<Command> {
    vec![system_info::system_info()]
}
