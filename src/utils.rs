use std::env;

pub fn get_whitelist() -> Vec<String> {
    env::var("WHITELIST")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn check_access(user_id: String) -> bool {
    let whitelist = get_whitelist();
    whitelist.contains(&user_id)
}