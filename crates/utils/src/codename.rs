use std::collections::HashMap;

use crate::VERSION;

/// https://github.com/importantimport/hatsu/milestones
pub fn codename() -> &'static str {
    let version = String::from(VERSION);
    let hashmap: HashMap<&str, &str> = HashMap::from([
        ("0.1", "01_ballade"),
        ("0.2", "celluloid"),
        ("0.3", "Strobe Nights"),
        ("0.4", "Clover Club"),
        ("0.5", "Sakura No Kisetsu"),
        ("0.6", "World Is Mine"),
        ("0.7", "Nisoku Hokou"),
        ("0.8", "Sweet Devil"),
        ("0.9", "Unfragment"),
        ("1.0", "Hoshi No Kakera"),
    ]);

    hashmap
        .iter()
        .find(|(major_minor, _)| version.starts_with(*major_minor))
        .map_or("Cat Food", |(_, codename)| codename)
}
