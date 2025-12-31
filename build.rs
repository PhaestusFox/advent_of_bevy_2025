fn main() {
    std::fs::create_dir_all("assets/days").unwrap();
    for day in 1..=25 {
        let day_path = format!("assets/days/day{:02}.input", day);
        match std::fs::exists(&day_path) {
            Ok(true) => continue,
            _ => {
                std::fs::write(day_path, "").unwrap();
            }
        }
    }
}
