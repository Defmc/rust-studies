use std::io::{stdin, stdout, Write};
use std::net::TcpStream;

fn main() {
    let mut username = String::from("");
    let mut sender = TcpStream::connect("0.0.0.0:9090")
        .expect("An error occurred while connecting to 0.0.0.0:9090...");

    let mut term_out = stdout();
    let term_in = stdin();

    print!("Username: ");
    term_out
        .flush()
        .expect("An error occurred while flushing stdout...");

    term_in
        .read_line(&mut username)
        .expect("Cannot read terminal's input...");
    username.pop();

    send(&username, String::from("Entered the chat!\n"), &mut sender)
        .expect("An error occurred while sending welcome message");

    loop {
        let mut input = String::from("");
        print!("Message: ");
        term_out
            .flush()
            .expect("An error occurred while flushing stdout...");

        term_in
            .read_line(&mut input)
            .expect("Cannot read terminal's input...");

        match input.as_str() {
            ":quit\n" | ":exit\n" => {
                send(&username, String::from("Left the chat!\n"), &mut sender)
                    .expect("An error occurred while sending exit message...");
                return;
            }
            _ => send(&username, input, &mut sender)
                .expect("An error occurred while sending message..."),
        };
    }
}

fn send(
    username: &String,
    message: String,
    tcpstream: &mut TcpStream,
) -> Result<usize, std::io::Error> {
    tcpstream.write(format!("{}: {}", username, message).as_bytes())
}
