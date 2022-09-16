use std::collections::HashMap;
use tiny_http::{Server, Response, Header};
use url::{Url};
use std::time::SystemTime;
use clap::{Arg, App};

pub mod lib;
use crate::lib::get_calendar;


const ICS_FILENAME: &str = "agendamilitant.ics";
const AGENDA_URL: &str = "https://www.agendamilitant.org";
const DEFAULT_PORT: &str = "8080";
const TIMEZONE: &str = "Europe/Paris";

fn run(port: u16, url: &str) {
    let target_url = &format!("0.0.0.0:{port}");
    let server = Server::http(target_url).unwrap();
    println!("The server is listening : {target_url}");
    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                 request.method(),
                 request.url(),
                 request.headers()
        );
        let end_url = request.url();
        let parsed_url = Url::parse(&format!("http://example.com{end_url}")).unwrap();
        let _hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
        //println!("la hash map {:?}", hash_query);
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let delta = 3600 * 24 * 60;
        let start = now - delta;
        let end = now + delta;
        let calendar = get_calendar(url, start, end, TIMEZONE);

        let mut response = Response::from_string(calendar.to_string());

        for head in ["Content-Type: text/calendar; charset=utf-8", &format!("Content-Disposition: attachment; filename=\"{}\"", ICS_FILENAME)] {
            let header: Header = head.parse().unwrap();
            response.add_header(header);
        }
        request.respond(response).unwrap();
    }
}

fn main() {
    let matches = App::new("Agenda Militant ICS Export")
        .about("Create a web service to download the activist agenda (agendamilitant.org) as an ical file.")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_parser(clap::value_parser!(u16).range(0..65536))
                //.value_parser(clap::builder::RangedU64ValueParser::<u16>::new().range())
                .takes_value(true)
                .default_value(DEFAULT_PORT)
                .help("Port on which you want to have your agenda."))
        .get_matches();

    let port:u16 = *matches.get_one("port").unwrap();
    run(port, AGENDA_URL);
}
