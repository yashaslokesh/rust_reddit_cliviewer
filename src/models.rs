
use serde_json::value::Value;

pub struct Link {
    pub author: String,
    pub gildings: Gildings,
    pub id: String,
    pub nsfw: bool,
    pub num_comments: u32,
    pub permalink: String,
    pub score: u32,
    pub subreddit: String,
    pub subreddit_id: String,
}

pub struct Redditor {

}

pub struct Gildings {
    silver: u8,
    gold: u8,
    platinum: u8,
}

impl Gildings {
    // Pass in the link's JSON, should have "data" and "kind": "t3"
    pub fn from_serde_map(link_data: &Value) -> Gildings {

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