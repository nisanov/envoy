use cursive::views::{Dialog, TextView};

fn main() {
    // Creates the cursive root - required for every application.
    let mut cursive = cursive::default();

    // Creates a dialog with a single "Quit" button
    cursive.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
        .title("Cursive")
        .button("Quit", |s| s.quit()));

    // Starts the event loop.
    cursive.run();
}
