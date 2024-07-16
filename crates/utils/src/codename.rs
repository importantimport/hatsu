use crate::VERSION;

/// https://github.com/importantimport/hatsu/milestones
pub fn codename() -> &'static str {
    match String::from(VERSION) {
        v if v.starts_with("0.1") => "01_ballade",
        v if v.starts_with("0.2") => "celluloid",
        v if v.starts_with("0.3") => "Strobe Nights",
        v if v.starts_with("0.4") => "Clover Club",
        v if v.starts_with("0.5") => "Sakura No Kisetsu",
        v if v.starts_with("0.6") => "World Is Mine",
        v if v.starts_with("0.7") => "Nisoku Hokou",
        v if v.starts_with("0.8") => "Sweet Devil",
        v if v.starts_with("0.9") => "Unfragment",
        v if v.starts_with("1.0") => "Hoshi No Kakera",
        // easter egg
        _ => "Cat Food",
    }
}
