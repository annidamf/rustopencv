extern crate opencv;
extern crate enigo;

mod utils;
// mod mouse;
mod timm_barth;
mod eye_tracker;

fn main() {

    eye_tracker::run().expect("Runtime error")

}
