mod session;
mod processing;

use session::Session;
fn main() {
    let mut session = Session::new();
    session.start_session();
}