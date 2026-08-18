use std::{
    fs,
    io::{ErrorKind, Read, Result},
    os::unix::net::{Incoming, UnixListener, UnixStream},
    thread,
};

fn handle_client(mut stream: UnixStream) -> Result<()> {
    let mut msg_type_buf: [u8; 1] = [0u8];
    stream.read_exact(&mut msg_type_buf)?;
    let msg_type = msg_type_buf[0];
    match msg_type {
        0 => {
            println!("AI Prompt detected.")
        }
        1 => {
            println!("Function call detected.")
        }
        _ => {
            println!("something else detected.")
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let uid = users::get_current_uid();
    let socket_path = format!("/run/user/{uid}/angeld");
    if let Err(error) = fs::remove_file(&socket_path) {
        if error.kind() != ErrorKind::NotFound {
            return Err(error);
        }
    }

    let listener = UnixListener::bind(&socket_path)?;
    println!("Socket bound at {socket_path}");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(error) = handle_client(stream) {
                        eprintln!("{error}")
                    }
                });
            }
            Err(error) => eprintln!("{error}"),
        }
    }

    Ok(())
}
