use cursive::view::View;
use cursive::views::{TextView, Button}

// I made this because I thought it was necessary
// Now I don't think it is...
// Can just configure a view dynamically without making
// a special new view.
pub struct SubmissionView {
    titleView: TextView,
    author_view: TextView,
    subreddit_view: TextView,
    link_info_view: TextView,
    upvote_button: Button,
    downvote_button: Button,
}

impl View for SubmissionView {

}
