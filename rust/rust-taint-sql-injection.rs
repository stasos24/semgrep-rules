use rocket::get;

#[get("/divide?<denom>")]
fn divide_by(denom: i32) -> String {
    let result = 100 / denom; // Tainted 'denom' flows into denominator
    format!("Result: {}", result)
}
