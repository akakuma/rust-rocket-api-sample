#[get("/")]
pub fn ping() -> &'static str {
    "ok"
}
