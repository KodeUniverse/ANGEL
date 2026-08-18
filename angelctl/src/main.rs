use clap::{Args, Parser, Subcommand};
use std::{
    io::{self, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Ask(AskArgs),
    Call,
}
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MsgType {
    AiPrompt = 0,
    FunctionCall = 1,
    Info = 2,
}
struct AngelCtlMsg<'a> {
    header: MsgType,
    body: &'a [u8],
}

impl<'a> AngelCtlMsg<'a> {
    fn new(header: MsgType, body: &'a [u8]) -> Self {
        Self { header, body }
    }
}
#[derive(Args, Debug)]
struct AskArgs {
    prompt: String,
}

fn socket_path() -> PathBuf {
    let uid = users::get_current_uid();
    PathBuf::from(format!("/run/user/{uid}/angeld"))
}

fn connect_to_socket(socket_path: &PathBuf) -> io::Result<UnixStream> {
    UnixStream::connect(socket_path)
}

fn send_msg(msg: &AngelCtlMsg) -> io::Result<()> {
    let mut stream = connect_to_socket(&socket_path())?;
    let body_buf_len = msg.body.len() as u64;

    let mut buf: Vec<u8> = Vec::with_capacity(1 + 8 + msg.body.len());
    buf.push(msg.header as u8);
    let body_len_bytes: [u8; 8] = body_buf_len.to_be_bytes();
    buf.extend_from_slice(&body_len_bytes);
    buf.extend_from_slice(msg.body);

    stream.write_all(buf.as_slice())?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ask(args) => {
            let msg = AngelCtlMsg::new(MsgType::AiPrompt, args.prompt.as_bytes());
            send_msg(&msg).unwrap();
        }
        Commands::Call => {}
    }
}
