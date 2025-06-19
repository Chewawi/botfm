use crate::core::structs::Command;

pub mod lastfm;
pub mod system;

macro_rules! register_commands {
    ($($module:ident),*) => {{
        let mut cmds: Vec<Command> = Vec::new();
        $(
            cmds.extend($module::register());
        )*
        cmds
    }};
}

pub fn register_all_commands() -> Vec<Command> {
    register_commands!(lastfm, system)
}