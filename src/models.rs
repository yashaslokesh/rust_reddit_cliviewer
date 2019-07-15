use serde_json::value::Value;

pub trait RedditObject {
    fn from_serde_map(object_data: &Value) -> Self;
}

pub struct Link {
    author: String,
    gildings: Gildings,
    id: String,
    nsfw: bool,
    num_comments: u32,
    permalink: String,
    score: u32,
    subreddit: String,
    subreddit_id: String,
}

impl RedditObject for Link {
    fn from_serde_map(link_data: &Value) -> Link {
        // Make sure the passed in JSON is for a link
        assert!(link_data["kind"] == "t3");

        let data = &link_data["data"];

        // Kinda redundant for now
        // TODO: Find a better way to do the gildings creation
        let gildings = Gildings::from_serde_map(link_data);

        let new_link = Link {
            author: get_string_from_string_value(&data["author"]),
            gildings: gildings,
            id: get_string_from_string_value(&data["id"]),
            nsfw: data["over_18"].as_bool().unwrap(),
            num_comments: get_u32_from_num_value(&data["num_comments"]),
            permalink: get_string_from_string_value(&data["permalink"]),
            score: get_u32_from_num_value(&data["score"]),
            subreddit: get_string_from_string_value(&data["subreddit"]),
            subreddit_id: get_string_from_string_value(&data["subreddit_id"]),
        };

        new_link
    }
}



pub struct Redditor {}



pub struct Gildings {
    silver: u8,
    gold: u8,
    platinum: u8,
}

impl RedditObject for Gildings {
    // Pass in the link's JSON, should have "data" and "kind": "t3"
    fn from_serde_map(link_data: &Value) -> Gildings {
        // Makes sure the passed in JSON is for a Reddit link
        assert!(link_data["kind"] == "t3");

        let gildings = &link_data["data"]["gildings"];

        let silver: u8 = match gildings.get("gid_1") {
            Some(s) => s.as_u64().unwrap() as u8,
            None => 0,
        };

        let gold: u8 = match gildings.get("gid_2") {
            Some(s) => s.as_u64().unwrap() as u8,
            None => 0,
        };

        let platinum: u8 = match gildings.get("gid_3") {
            Some(s) => s.as_u64().unwrap() as u8,
            None => 0,
        };

        Gildings {
            silver: silver,
            gold: gold,
            platinum: platinum,
        }
    }
}

fn get_string_from_string_value(v: &Value) -> String {
    String::from(v.as_str().unwrap())
}

fn get_u32_from_num_value(v: &Value) -> u32 {
    v.as_u64().unwrap() as u32
}