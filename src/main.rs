extern crate cursive;
mod client;

use cursive::Cursive;
use cursive::views::{TextView, Dialog};
use cursive::menu::{MenuTree};
use cursive::traits::*;
use cursive::event::{Key};

fn setup_window() -> Cursive {
    // Must unwrap the Result from Cursive::ncurses()
    let siv = Cursive::ncurses();
    let mut win = siv.unwrap();

    win.menubar()
        .add_subtree("File",
        MenuTree::new()
            .leaf("New", |s| s.add_layer(Dialog::info("New file!")))
            .subtree("Recent", MenuTree::new().with(|tree| {
                for i in 1..100 {
                    tree.add_leaf(format!("Item {}", i), |_| ())
                }
            }))
            .delimiter()
            .with(|tree| {
                for i in 1..10 {
                    tree.add_leaf(format!("Option {}", i), |_| ());
                }
            })
            .delimiter()
            .leaf("Quit", |s| s.quit()));

    win.add_layer(TextView::new("Hello World!\nPress q to quit.\nPress Esc to select menubar"));

    // win.select_menubar();
    win.set_autohide_menu(false);

    // win.add_global_callback(event: E, cb: F)
    win.add_global_callback(Key::Esc, |s| s.select_menubar());
    win.add_global_callback('q', |s| s.quit());

    win
}


fn main() {
    // let mut win = setup_window();
    // win.run();

    client::test_reqwest();
}