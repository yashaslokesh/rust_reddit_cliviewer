// Declare all modules used here,
// so that if other files want to use any of these modules,
// they are available. Add here even if main.rs doesn't use them,
// because adding them here puts them into the crate root.
// refer:
// https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
mod client;
mod models;

use client::RedditClient;

// Align will only align the content within each view, not the view itself
// So it aligns the children, not self
// use cursive::align::Align;
// use cursive::event::Key;
// use cursive::menu::MenuTree;
// use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
// use cursive::traits::*;
use cursive::{
    align::VAlign,
    event::Key,
    menu::MenuTree,
    theme::{BaseColor, Color, ColorStyle, PaletteColor, Theme},
    traits::*,
    view::SizeConstraint,
    views::{
        BoxView, Button, Canvas, Dialog, EditView, LinearLayout, ListView, OnEventView, PaddedView,
        ScrollView, SelectView, TextView,
    },
    Cursive, Printer,
};

use reqwest;

use std::sync::Mutex;

use lazy_static::lazy_static;

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

                                R_CLIENT.lock().unwrap().redirect_user_for_auth();
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

    win.add_layer(get_front_page());

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
    theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::View] = Color::Light(BaseColor::Black);
    theme.palette[PaletteColor::Primary] = Color::Light(BaseColor::White);
    theme.shadow = false;

    theme
}

fn get_front_page() -> ScrollView<OnEventView<LinearLayout>> {
    let links = R_CLIENT.lock().unwrap().get_hot();

    // let mut list = ListView::new();
    let mut list = OnEventView::new(LinearLayout::vertical()).on_event_inner(
        Key::Enter,
        |l: &mut LinearLayout, _| {
            println!("idx: {}", l.get_focus_index());

            // l.get_child_mut(l.get_focus_index()).unwrap().None
            None
        },
    );

    for link in links {
        // let link_wrapper = Boxable

        let mut listing = OnEventView::new(
            Canvas::wrap(BoxView::with_full_width(PaddedView::new(
                ((1, 1), (1, 1)),
                LinearLayout::vertical()
                    .child(TextView::new(link.title()))
                    .child(
                        LinearLayout::horizontal()
                            .child(TextView::new(format!("r/{}", link.subreddit())))
                            // .child(TextView::new(link.author()))
                            .child(PaddedView::new(
                                ((2, 2), (0, 0)),
                                TextView::new(link.author()),
                            )),
                    ),
            )))
            .with_draw(draw),
        );

        list.get_inner_mut().add_child(listing);
    }

    // list.

    // BoxView::with_full_screen(list)
    ScrollView::new(list)
}

fn draw(t: &BoxView<PaddedView<LinearLayout>>, p: &Printer) {
    let style = ColorStyle::new(Color::Dark(BaseColor::Blue), Color::Light(BaseColor::Blue));

    p.with_color(style, |printer: &Printer| {
        // printer.print_box((0, 0), printer.output_size, false);

        let offset_printer = printer.offset((0, 0));

        offset_printer.print_box((0, 0), offset_printer.output_size, false);

        t.draw(&offset_printer);
        // printer.print((0, 0), "+");
    })
}

lazy_static! {
    static ref R_CLIENT: Mutex<RedditClient> = Mutex::new(RedditClient::new());
}

fn main() {
    let mut win = setup_window();
    win.run();

    // let c = reqwest::Client::new();

    // let mut r = c.get("https://api.reddit.com/hot").send();

    // let m = r.unwrap().text().unwrap();

    // println!("{}", m);

    // test();
}

fn test() {
    let t = R_CLIENT.lock().unwrap().get_hot();
    println!("OK",);
}

// impl
