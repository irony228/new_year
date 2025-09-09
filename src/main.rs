use chrono::{Datelike, Local, TimeZone};

fn main() {
    let now = Local::now();
    let current_year = now.year();
    
    let next_new_year = match Local.with_ymd_and_hms(current_year + 1, 1, 1, 0, 0, 0) {
        chrono::LocalResult::Single(datetime) => datetime,
        _ => {
            eprintln!("Ошибка");
            return;
        }
    };
    
    let duration = next_new_year.signed_duration_since(now);
    
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    
    println!("Время до Нового года: Дней: {}, Часов: {}, Минут: {}, Секунд: {}.", days, hours, minutes, seconds);
}