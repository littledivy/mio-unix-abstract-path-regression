use mio::net::UnixStream;

fn main() {
  UnixStream::connect("\0aaa").unwrap();
}
