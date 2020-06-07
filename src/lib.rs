use plist::Value;
use std::error::Error;

#[derive(Debug)]
#[non_exhaustive]
pub struct Prefers {
    pub color_scheme: Option<String>,
    pub reduced_motion: Option<bool>,
}

pub fn prefers() -> Result<Prefers, Box<dyn Error>> {
    let root = dirs::home_dir().unwrap_or_else(|| "/".into());
    fn string(value: &Value, name: &str) -> Option<String> {
        Some(value
        .as_dictionary()?
        .get(name)?
        .as_string()?
        .into())
    }
    fn boolean(value: &Value, name: &str) -> Option<bool> {
        value
        .as_dictionary()?
        .get(name)?
        .as_boolean()
    }
    // defaults read -g
    let globals = Value::from_file(root.join("Library/Preferences/.GlobalPreferences.plist"))?;
    let color_scheme: Option<String> = string(&globals, "AppleInterfaceStyle");
    // defaults read  com.apple.universalaccess.plist
    let accessibility =
        Value::from_file(root.join("Library/Preferences/com.apple.universalaccess.plist"))?;
    let reduced_motion: Option<bool> = boolean(&accessibility, "reduceMotion");
    Ok(Prefers {
        color_scheme,
        reduced_motion,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
