

const API_BASE: &str = "http://127.0.0.1:3000";

pub fn api_url(path: &str) -> String {
    format!("{API_BASE}{path}")
}