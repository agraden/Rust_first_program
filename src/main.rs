fn main() {
    let mut answer = "blue".to_string();
    answer = my_fav_color();
    println! ("My current favorite color is {}", answer)
}

fn my_fav_color() -> String {
    "red".to_string()
}