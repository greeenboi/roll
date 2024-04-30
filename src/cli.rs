use clap::{Arg, ArgMatches, Command, ArgAction};

pub fn get_arguments()-> ArgMatches{
    Command::new("roll - better curl")
        .author("greenboi <suvan.gowrishanker.204@gmail.com>")
        .version("1.0")
        .about("It helps to make HTTP methods")
        .arg(Arg::new("url").index(1).required(true))
        .arg(
            Arg::new("x-method")
                .help("Which http method do you want to use")
                .long("x-method")
                .short('X'),
        )
        .arg(
            Arg::new("data")
                .help("Which Payload you want to send with the request")
                .long("data")
                .short('d'),
        )
        .arg(
            Arg::new("headers")
                .help("Request headers to send with the request")
                .long("header")
                .short('H')
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("verbose")
                .help("verbose mode to see the request and response")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}