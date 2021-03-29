mod session;

use session::Session;

fn main() {
    let mut session = Session::default();
    session.start_session();
}
