use std::fs;
use chrono::{DateTime, TimeZone, Utc, Date};


fn main() {
    let file_path = "storage/109.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let string = contents.split("\n");
    let mut vec: Vec<&str> = string.collect();

    // let date: Date<Utc> = Utc.ymd(2020, 1, 1);
    // let dt: DateTime<Utc> = date.and_hms(23, 0, 12);
    //
    // println!("{:?}", dt);

    let dt = DateTime::parse_from_str(vec[1], "%Y-%m-%dT%H:%M:%S %z").unwrap();
    let dn: DateTime<Utc> = Utc::now();
    let dnt = dn.format("%Y-%m-%dT%H:%M:%S %z").to_string();

    println!("{:?}", dt.to_string());
    println!("{:?}", dnt);

    vec[1] = &dnt;

    // println!("With text:\n{contents}");
    println!("{:?}", vec);

    let new_str = vec
        .into_iter()
        .filter(|el| el != &"")
        .map(|el| format!("{}\n", el))
        .collect::<String>();

    println!("{:?}", new_str);

    fs::write(file_path, new_str).unwrap();
}
