
use std::fs;

use crate::spip_calendar::get_calendar;

pub mod spip_calendar;

fn main() {
    let tzid = "Europe/Paris";
    let start = 1658707200;
    let end = 1662595200;
    let url = "https://www.agendamilitant.org/";
    let calendar = get_calendar(url, start, end, tzid);

    let str_calendar = calendar.to_string();
    println!("{}", str_calendar);
    fs::write("export.ical", str_calendar).expect("Impossible de créer le fichier ical");

}
