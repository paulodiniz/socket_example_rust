use std::io::net::tcp::TcpStream;

fn main() {
  let mut socket = TcpStream::connect("127.0.0.1:2000").unwrap();
  let response = match socket.read_to_end() {
    Err(_) => panic!("at the disco"),
    Ok(value) => String::from_utf8(value).unwrap(),
  };

  println!("{}", response);

}
