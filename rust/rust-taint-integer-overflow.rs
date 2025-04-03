use rocket::get;

#[get("/calc?<num>")]
fn calculate(num: i32) -> String {
    let result = num * 1000; // Tainted 'num' flows into multiplication
    format!("Result: {}", result)
}
