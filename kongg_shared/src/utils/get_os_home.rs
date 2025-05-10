use dirs::home_dir;

pub fn get_home() -> Option<String> {
    // let home = env::var("Home").or_else(|_| env::var("USERPROFILE"));
    // if let Ok(res) = home {
    //     return Some(res);
    // };
    // None

    if let Some(home_dir) = home_dir() {
        return Some(home_dir.display().to_string());
    }
    None
}
