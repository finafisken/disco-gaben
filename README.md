# Gaben [![Build Status](https://travis-ci.org/finafisken/disco-gaben.svg?branch=master)](https://travis-ci.org/finafisken/disco-gaben)

Gaben is a Discord bot written in Rust

## Commands
To trigger a command use the following syntax `!{command group} {command} {...arguments}` eg. `!event add 2019-01-25T00:00 "Anthem Demo" https://www.ea.com/games/anthem`

### `event` commands
- `list` will list the current events that are available
- `join` join a specified event. Arguments provided are:
  - *id* the id of the event eg. `!event join 5678`
- `add` add an event to the event list. Arguments provided are:
  - *date* provide the date in this format `YYYY-MM-ddThh:mm`
  - *title* provide title in qoutes to support spaces
  - *link* url to page relevant for event
