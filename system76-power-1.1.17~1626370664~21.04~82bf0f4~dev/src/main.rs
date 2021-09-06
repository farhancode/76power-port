#![deny(clippy::all)]

use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};
use log::LevelFilter;
use std::process;
use system76_power::{charge_thresholds::get_charge_profiles, client, daemon, logging};

fn main() {
    let matches = App::new("system76-power")
        .about("Utility for managing graphics and power profiles")
        .version(env!("CARGO_PKG_VERSION"))
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("daemon")
                .about("Runs the program in daemon mode")
                .long_about(
                    "Registers a new DBUS service and starts an event loop to listen for, and \
                     respond to, DBUS events from clients",
                )
                .arg(
                    Arg::with_name("quiet")
                        .short("q")
                        .long("quiet")
                        .help("Set the verbosity of daemon logs to 'off' [default is 'info']")
                        .global(true)
                        .group("verbosity"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Set the verbosity of daemon logs to 'debug' [default is 'info']")
                        .global(true)
                        .group("verbosity"),
                ),
        )
        .subcommand(
            SubCommand::with_name("profile")
                .about("Query or set the power profile")
                .long_about(
                    "Queries or sets the power profile.\n\n - If an argument is not provided, the \
                     power profile will be queried\n - Otherwise, that profile will be set, if it \
                     is a valid profile",
                )
                .arg(
                    Arg::with_name("profile")
                        .help("set the power profile")
                        .possible_values(&["battery", "balanced", "performance"])
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("graphics")
                .about("Query or set the graphics mode")
                .long_about(
                    "Query or set the graphics mode.\n\n - If an argument is not provided, the \
                     graphics profile will be queried\n - Otherwise, that profile will be set, if \
                     it is a valid profile\n\nA reboot is required after switching modes.",
                )
                .subcommand(
                    SubCommand::with_name("compute")
                        .about("Like integrated, but the dGPU is available for compute"),
                )
                .subcommand(
                    SubCommand::with_name("hybrid")
                        .about("Set the graphics mode to Hybrid (PRIME)"),
                )
                .subcommand(
                    SubCommand::with_name("integrated")
                        .about("Set the graphics mode to integrated"),
                )
                .subcommand(
                    SubCommand::with_name("nvidia").about("Set the graphics mode to NVIDIA"),
                )
                .subcommand(
                    SubCommand::with_name("switchable")
                        .about("Determines if the system has switchable graphics"),
                )
                .subcommand(
                    SubCommand::with_name("power")
                        .about("Query or set the discrete graphics power state")
                        .arg(
                            Arg::with_name("state")
                                .help("Set whether discrete graphics should be on or off")
                                .possible_values(&["auto", "off", "on"]),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("charge-thresholds")
                .about("Set thresholds for battery charging")
                // Autogenerated usage seemed to have issues
                .usage("system76-power charge-thresholds [<start> <end> | --profile <profile>]")
                .group(
                    ArgGroup::with_name("profile-or-thresholds")
                        .arg("thresholds")
                        .arg("profile")
                        .arg("list-profiles"),
                )
                .arg(
                    Arg::with_name("profile")
                        .long("profile")
                        .help("Profile name")
                        .required(false)
                        .takes_value(true)
                        .possible_values(
                            &get_charge_profiles()
                                .iter()
                                .map(|p| p.id.as_str())
                                .collect::<Vec<_>>(),
                        ),
                )
                .arg(
                    Arg::with_name("list-profiles")
                        .long("list-profiles")
                        .help("List profiles")
                        .required(false),
                )
                .arg(
                    Arg::with_name("thresholds")
                        .help("Charge thresholds")
                        .validator(|s| {
                            if let Ok(v) = u8::from_str_radix(&s, 10) {
                                if v <= 100 {
                                    return Ok(());
                                }
                            }
                            Err("Not an integer between 0 and 100".to_string())
                        })
                        .number_of_values(2)
                        // `number_of_values` seems insufficient:
                        // https://github.com/clap-rs/clap/issues/2229
                        .max_values(2)
                        .value_names(&["start", "end"])
                        .required(false),
                ),
        )
        .get_matches();

    let res = match matches.subcommand() {
        ("daemon", Some(matches)) => {
            if let Err(why) = logging::setup(if matches.is_present("verbose") {
                LevelFilter::Debug
            } else if matches.is_present("quiet") {
                LevelFilter::Off
            } else {
                LevelFilter::Info
            }) {
                eprintln!("failed to set up logging: {}", why);
                process::exit(1);
            }

            if unsafe { libc::geteuid() } == 0 {
                daemon::daemon()
            } else {
                Err("must be run as root".to_string())
            }
        }
        (subcommand, Some(matches)) => client::client(subcommand, matches),
        _ => unreachable!(),
    };

    match res {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}