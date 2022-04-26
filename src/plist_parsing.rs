#[derive(serde::Deserialize)]
struct Spaces {
    SpacesDisplayConfiguration: DisplayConfiguration,

    #[serde(rename = "spans-displays")]
    spans_displays: bool,
}

#[derive(serde::Deserialize, Debug)]
struct DisplayConfiguration {
    #[serde(rename = "Management Data")]
    management: plist::Dictionary,

    #[serde(rename = "Space Properties")]
    properties: Vec<DisplayData>,
}

#[derive(serde::Deserialize, Debug)]
struct DisplayData {
    name: String,
    windows: Vec<i32>,
}

#[test]
fn go() {
    use plist::Value;

    let book: Spaces =
        plist::from_file("/Users/jonkelley/Library/Preferences/com.apple.spaces.plist")
            .expect("failed to read book.plist");

    for (key, value) in &book.SpacesDisplayConfiguration.management {
        println!("{:?}, {:#?}", key, value);
    }

    for prop in &book.SpacesDisplayConfiguration.properties {
        println!("{:?}\n", prop);
    }

    println!(
        "there are {} spaces",
        book.SpacesDisplayConfiguration.properties.len()
    );
}
