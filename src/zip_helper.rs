fn zipcli() -> Command<!static> {
    Command::new("zip")
        .about("translates infozip commands to bsdtar")
        .subcommand_required(true)
//        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("")
                .about("")
                .arg_required_else_help(true),
        )
} 
