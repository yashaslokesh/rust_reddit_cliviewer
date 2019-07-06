extern crate cursive;
mod client;

use client::RedditClient;

use cursive::Cursive;
use cursive::views::{TextView, Dialog, LinearLayout, EditView, Button};
use cursive::menu::{MenuTree};
use cursive::traits::*;
use cursive::event::{Key};

fn setup_window(reddit_client: &'static RedditClient) -> Cursive {
    // Must unwrap the Result from Cursive::ncurses()
    let siv = Cursive::ncurses();
    let mut win = siv.unwrap();

    // let login_view = Dialog::new()
    //                     .title("Enter your login details")
    //                     .padding((1, 1, 1, 0))
    //                     .content(
    //                         EditView::new()
    //                             .with_id("username")
    //                             .fixed_width(20),
    //                     )
    //                     .button("Log In", |s| {s.pop_layer();});

    // Menubar setup
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
            .leaf("Quit", |s| s.quit()))
        .add_subtree(
            "Help",
            MenuTree::new()
                .subtree(
                    "Help",
                    MenuTree::new()
                        .leaf("General", |s| {
                            s.add_layer(Dialog::info("Help message!"))
                        })
                        .leaf("Online", |s| {
                            let text = "Google it yourself!\n\
                                        Kids, these days...";
                            s.add_layer(Dialog::info(text))
                        }),
                )
                .leaf("About", |s| {
                    s.add_layer(Dialog::info("Cursive v0.0.0"))
                }),
        )
        .add_subtree(
            "Accounts", 
            MenuTree::new()
                .leaf(
                    "Log In", |s| s.add_layer(
                        Dialog::new()
                            .title("Log In")
                            .padding((1, 1, 1, 0))
                            .content(
                                TextView::new("You will now be redirected to log in to Reddit.")
                                // LinearLayout::vertical()
                                // .child(
                                //     TextView::new("Username")
                                // )
                                // .child(
                                //     EditView::new()
                                //     .with_id("user")
                                //     .fixed_width(20)
                                // )
                                // .child(
                                //     TextView::new("Password")
                                // )
                                // .child(                                 
                                //     EditView::new()
                                //     .with_id("pass")
                                //     .fixed_width(20)
                                // )
                            )
                        .button("Cancel", |s| {
                            s.pop_layer();
                        })
                        .button("Continue", |s| {
                            s.pop_layer();
                            s.add_layer(create_auth_url_view());
                            reddit_client.redirect_user_for_auth();
                        })))
                .subtree(
                    "Recent",
                    MenuTree::new().with(|tree| {
                        for i in 1..4 {
                            tree.add_leaf(format!("Account {}", i), |_| ())
                        }
                    })
                )
        );

    win.add_layer(TextView::new("Hello World!\nPress q to quit.\nPress Esc to select menubar"));

    // Menu stays fixed at top of screen
    win.set_autohide_menu(false);
    // Focused on menu on startup
    win.select_menubar();

    // win.add_global_callback(event: E, cb: F)
    win.add_global_callback(Key::Esc, |s| s.select_menubar());
    win.add_global_callback('q', |s| s.quit());

    win
}

fn create_auth_url_view() -> Box<View> {
    let v = Dialog::new()
            .title("Please enter the url you were redirected to in your browser into the text field below")
            .padding((1, 1, 1, 0))
            .content(
                EditView::new()
                .with_id("auth_url")
                .fixed_width(30)
            )
            .button("Authenticate", |s| {
                let url = s.call_on_id("auth_url", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();

                client::process_redirect_url(&url);
                s.pop_layer();
            })
            .button("Cancel", |s| {s.pop_layer();});

    Box::new(v)
}


fn main() {
    let reddit_client = client::RedditClient::new();

    let mut win = setup_window(&reddit_client);
    win.run();

    // client::test_reqwest();
    // client::connect();
    // client::redirect_user_for_auth();
}