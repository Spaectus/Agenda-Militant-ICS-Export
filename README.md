# Agenda Militant ICS Export

This program allows you to create a web service distributing the french activist agenda ([Agenda Militant](https://www.agendamilitant.org/)) in `.ics` format. You will then have a URL of the agenda allowing you to synchronize your agenda with the activist agenda.

# Installation

## Software requirement

1. Cargo/Rust installed (If not download it [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)).

## Manual installation

```
git clone git clone https://github.com/Spaectus/Agenda-Militant-ICS-Export.git
cargo build --release
```

# Usage

```
cargo run --release
```

Then go to `http://localhost:8080/` to get the activist agenda as an ical file.

If you wish to specify a particular port (8000 for example):
```
cargo run --release -- --port 8000
```

# Docker

```shell
docker build -t agenda_militant_ics_export:distroless .
docker run -d --rm --name agenda_militant_ics_export -p 80:8080 agenda_militant_ics_export:distroless
```