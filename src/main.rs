use cursive::views::{TextView};

fn main() {

    let mut cursive = cursive::default();

    cursive.add_global_callback('q', |s| s.quit());
    cursive.add_layer(TextView::new("Welcome to envoy! Press <q> to quit."));
    cursive.run();
}
