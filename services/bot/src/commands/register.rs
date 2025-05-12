use lumi::serenity_prelude as serenity;

pub fn create_application_commands<U, E>(
    commands: &[lumi::Command<U, E>],
) -> (
    Vec<serenity::CreateCommand<'static>>,
    Vec<serenity::CreateCommand<'static>>,
) {
    fn recursively_add_context_menu_commands<U, E>(
        builder: &mut Vec<serenity::CreateCommand<'static>>,
        command: &lumi::Command<U, E>,
    ) {
        if let Some(context_menu_command) = command.create_as_context_menu_command() {
            builder.push(context_menu_command);
        }
        for subcommand in &command.subcommands {
            recursively_add_context_menu_commands(builder, subcommand);
        }
    }

    let mut commands_builder = Vec::with_capacity(commands.len());
    let mut owner_commands = Vec::new();

    for command in commands {
        if let Some(slash_command) = command.create_as_slash_command() {
            if command
                .category
                .as_deref()
                .is_some_and(|desc| desc.to_lowercase().starts_with("owner"))
            {
                owner_commands.push(slash_command);
            } else {
                commands_builder.push(slash_command);
            }
        }
        recursively_add_context_menu_commands(&mut commands_builder, command);
    }
    (commands_builder, owner_commands)
}