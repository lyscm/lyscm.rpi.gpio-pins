#[get("/logs")]
pub async fn get_logs() -> String {
    "This is the logging".to_string()
}
