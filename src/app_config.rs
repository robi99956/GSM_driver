use clap::{Arg, command, ArgAction, ArgGroup};

const SOCKET_OPT_LONG: &str = "socket";
const SOCKET_OPT_SHORT: char = 's';

const CLI_OPT_LONG: &str = "cli";
const CLI_OPT_SHORT: char = 'c';

const DEVICE_OPT_LONG: &str = "device";
const DEVICE_OPT_SHORT: char = 'd';

#[derive(Debug)]
pub struct AppConfig {
    use_cli: bool,
    socket_path: Option<String>,
    device_type: Option<String>,
}

impl AppConfig {
    pub fn new_from_cli_args() -> AppConfig {
        let matches = command!() 
            .arg(Arg::new("cli")
                .id(CLI_OPT_LONG)
                .short(CLI_OPT_SHORT)
                .long(CLI_OPT_LONG)
                .action(ArgAction::SetTrue)
                .help("Enable cli-like communication")
            )
            .arg(Arg::new("socket")
                .name("socket file")
                .id(SOCKET_OPT_LONG)
                .short(SOCKET_OPT_SHORT)
                .long(SOCKET_OPT_LONG)
                .action(ArgAction::Set)
                .help("Enable unix socket communication")
            )
            .group(ArgGroup::new("com options")
                .args(&[CLI_OPT_LONG, SOCKET_OPT_LONG])
                .multiple(true)
                .required(true)
            )
            .arg(Arg::new("device")
                .id(DEVICE_OPT_LONG)
                .short(DEVICE_OPT_SHORT)
                .long(DEVICE_OPT_LONG)
                .action(ArgAction::Set)
                .help("GSM device to use, leave empty to auto detect")
            )
            .get_matches();

        return AppConfig { 
            use_cli: *matches.get_one::<bool>(CLI_OPT_LONG)
                .expect("Parsing cli arguments failed"),
            device_type: matches.get_one::<String>(DEVICE_OPT_LONG).cloned(),
            socket_path: matches.get_one::<String>(SOCKET_OPT_LONG).cloned(),
        };
    }
}