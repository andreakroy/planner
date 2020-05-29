extern crate termion;

use chrono::{ DateTime, Local, ParseResult };
use chrono::offset::TimeZone;
use std::fmt;
use structopt::StructOpt;
use termion::{ color, style };

static FORMAT_STRING : &str = "%m-%d-%Y %R";

fn parse_datetime(src: &str) -> ParseResult<DateTime<Local>> {
    Local.datetime_from_str(src, FORMAT_STRING)
}
#[derive(Debug, PartialEq, StructOpt)]
pub struct Event {
    pub calendar_name:String,
    pub title : String,
    #[structopt(parse(try_from_str="parse_datetime"))]
    pub start : DateTime<Local>,
    #[structopt(parse(try_from_str="parse_datetime"))]
    pub end : Option<DateTime<Local>>,
    pub description : Option<String>
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}{}{}{}{}", style::Bold, style::Underline, color::Fg(color::Cyan), 
            self.title, style::Reset)?;
        writeln!(f, "{}{}start: {}{}{}", color::Fg(color::Green),
            style::Bold, style::Reset, self.start.format(FORMAT_STRING),
            style::Reset)?;
        match self.end {
            Some(e) => writeln!(f, "{}{}end:   {}{}{}", color::Fg(color::Magenta),
                style::Bold, style::Reset, e.format(FORMAT_STRING), style::Reset)?,
            None => (),
        };
        match &self.description {
            Some(d) => writeln!(f, "{}{}", d, style::Reset)?,
            None => (),
        };
        write!(f, "{}", style::Reset)


    }
}

