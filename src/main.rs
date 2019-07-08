// extern crate cursive;
mod client;

use client::RedditClient;

// Align will only align the content within each view, not the view itself
// So it aligns the children, not self
// use cursive::align::Align;
use cursive::event::Key;
use cursive::menu::MenuTree;
use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{BoxView, Dialog, EditView, TextView};
use cursive::Cursive;

use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

fn setup_window() -> Cursive {
    // Must unwrap the Result from Cursive::ncurses()
    let siv = Cursive::ncurses();
    let mut win = siv.unwrap();

    // Menubar setup
    win.menubar()
        .add_subtree(
            "File",
            MenuTree::new()
                .leaf("New", |s| s.add_layer(Dialog::info("New file!")))
                .subtree(
                    "Recent",
                    MenuTree::new().with(|tree| {
                        for i in 1..100 {
                            tree.add_leaf(format!("Item {}", i), |_| ())
                        }
                    }),
                )
                .delimiter()
                .with(|tree| {
                    for i in 1..10 {
                        tree.add_leaf(format!("Option {}", i), |_| ());
                    }
                })
                .delimiter()
                .leaf("Quit", |s| s.quit()),
        )
        .add_subtree(
            "Help",
            MenuTree::new()
                .subtree(
                    "Help",
                    MenuTree::new()
                        .leaf("General", |s| s.add_layer(Dialog::info("Help message!")))
                        .leaf("Online", |s| {
                            let text = "Google it yourself!\n\
                                        Kids, these days...";
                            s.add_layer(Dialog::info(text))
                        }),
                )
                .leaf("About", |s| s.add_layer(Dialog::info("Cursive v0.0.0"))),
        )
        .add_subtree(
            "Accounts",
            MenuTree::new()
                .leaf("Log In", |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("Log In")
                            .padding((1, 1, 1, 0))
                            .content(TextView::new(
                                "You will now be redirected to log in to Reddit.",
                            ))
                            .button("Cancel", |s| {
                                s.pop_layer();
                            })
                            .button("Continue", |s| {
                                s.pop_layer();
                                s.add_layer(create_auth_url_view());
                                
                                R_CLIENT
                                    .lock()
                                    .unwrap()
                                    .redirect_user_for_auth();
                                
                            }),
                    )
                })
                .subtree(
                    "Recent",
                    MenuTree::new().with(|tree| {
                        for i in 1..4 {
                            tree.add_leaf(format!("Account {}", i), |_| ())
                        }
                    }),
                ),
        );

    // win.add_layer(TextView::new(
    //     "Hello World!\nPress q to quit.\nPress Esc to select menubar",
    //     ).align(Align::top_right())
    // );

    win.add_layer(BoxView::with_full_screen(TextView::new(
        "Hello World!\nPress q to quit.\nPress Esc to select menubar",
    )));

    let theme = configure_custom_theme(&win);
    win.set_theme(theme);

    // Menu stays fixed at top of screen
    win.set_autohide_menu(false);
    // Focused on menu on startup
    // win.select_menubar();

    // win.add_global_callback(event: E, cb: F)
    win.add_global_callback(Key::Esc, |s| s.select_menubar());
    win.add_global_callback('q', |s| s.quit());

    win
}

fn create_auth_url_view() -> Box<View> {
    let v = Dialog::new()
        .title(
            "Please enter the url you were redirected to in your browser into the text field below",
        )
        .padding((1, 1, 1, 0))
        .content(EditView::new().with_id("auth_url").fixed_width(30))
        .button("Authenticate", |s| {
            let url = s
                .call_on_id("auth_url", |view: &mut EditView| view.get_content())
                .unwrap();

            let result = R_CLIENT.lock().unwrap().parse_redirect_url(&url);

            s.add_layer(
                Dialog::new()
                    .title("Notice")
                    .padding((1, 1, 1, 0))
                    .content(TextView::new(format!(
                        "Result of attempted Log in: {}",
                        result
                    )))
                    .button("OK", |s| {
                        s.pop_layer();
                        s.pop_layer();
                    }),
            );

            print!("Result: {}", result);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        });

    Box::new(v)
}

fn configure_custom_theme(win: &Cursive) -> Theme {
    // For now, get the current theme and configure it
    // TODO: Configure theme via a configuration file

    let mut theme: Theme = win.current_theme().clone();

    // Refer docs:
    // https://docs.rs/cursive/0.12.0/x86_64-apple-darwin/cursive/theme/enum.PaletteColor.html
    theme.palette[PaletteColor::Background] = Color::Light(BaseColor::Black);
    theme.shadow = false;

    theme
}

lazy_static! {
    static ref R_CLIENT: Mutex<RedditClient> = Mutex::new(RedditClient::new());
}

fn main() {
    let mut win = setup_window();
    win.run();
}
