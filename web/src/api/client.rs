

const API_BASE: &str = "http://localhost:3002";
// const API_BASE: &str = "https://atz19.tail18537f.ts.net";


pub fn api_url(path: &str) -> String {
    format!("{API_BASE}{path}")
}