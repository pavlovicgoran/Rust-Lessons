// Module definition

pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
}

// Module and its content is private by default
mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}
