#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
extern crate futures;
extern crate gloo_net;
extern crate sycamore;
extern crate web_sys;

mod structs;
mod add_users;
mod app;
mod libs;
use app::App;

pub fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
