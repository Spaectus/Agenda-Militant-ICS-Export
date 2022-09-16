use std::str;
use std::error::Error;
use scraper::{Html, Selector};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use icalendar::{Calendar, CalendarDateTime, Component, Event};
use icalendar::CalendarDateTime::WithTimezone;

use url::{Url};


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Atcb {
    name: String,
    description: String,
    label: String,
    location: String,
    startDate: String,
    startTime: String,
    endDate: String,
    endTime: String,
    timeZone: String,
    options: Vec<String>,
    iCalFileName: String,
}

fn helper(date: &str, time: &str, timezone: &str) -> CalendarDateTime {
    let start_naive_datetime = chrono::NaiveDateTime::parse_from_str(&format!("{date} {time}"), "%Y-%m-%d %H:%M").unwrap();
    WithTimezone {
        date_time: start_naive_datetime,
        tzid: timezone.to_string(),
    }
}


fn get_html(url: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    Ok(resp)
}


fn event_from_url(url: &str, timezone: &str) -> Event {
    let html: String = get_html(url).unwrap();
    let document = Html::parse_document(&html);
    let selector = Selector::parse("div.atcb").unwrap();
    let div_atcb = document.select(&selector).next().unwrap();
    let json = div_atcb.inner_html();
    let hp: Atcb = serde_json::from_str(&json).unwrap();


    let final_timezone: &str = if timezone.is_empty() {
        &hp.timeZone
    } else {
        timezone
    };

    let start_date: CalendarDateTime = helper(&hp.startDate, &hp.startTime, final_timezone);
    let end_date: CalendarDateTime = helper(&hp.endDate, &hp.endTime, final_timezone);

    Event::new()
        .summary(&hp.name)
        .description(&hp.description)
        .url(url)
        .location(&hp.location)
        .starts(start_date)
        .ends(end_date)
        .done()

}

fn req(url_slashed: &str, start: &u64, end: &u64) -> Result<String, Box<dyn Error>> {
    let url = format!("{url_slashed}spip.php?page=mes_evenements.json&lang=fr&start={}&end={}", start, end);
    //println!("voici l'url : {}", url);

    let resp = reqwest::blocking::get(url)?
        .text()?;

    //println!("La rep :");
    //println!("{:#?}", resp);
    Ok(resp)
}

pub fn get_calendar(url: &str, start: u64, end: u64, tzid: &str) -> Calendar{
    let optional_slash: &str = if url.ends_with("/") {
        ""
    } else {
        "/"
    };
    let url_slashed = &format!("{url}{optional_slash}");
    let resp = req(url_slashed, &start, &end).unwrap();
    //println!("La rep :");
    //println!("{:#?}", resp);
    let vec: Vec<Value> = serde_json::from_str(&resp).unwrap();

    let mut calendar_elements_vector: Vec<Event> = Vec::new();

    for event_object in &vec {
        //println!("title : {}", &event_object["title"]);
        let url = &format!("{url_slashed}{}", event_object["url"].as_str().unwrap());
        let event = event_from_url(url, tzid);
        calendar_elements_vector.push(event)
    }


    calendar_elements_vector.into_iter()
        .collect::<Calendar>()
        .name( Url::parse(url).unwrap().host_str().unwrap())
        .timezone(&tzid).done()


}
