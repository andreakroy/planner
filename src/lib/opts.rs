use super::event::Event;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub struct ListOpts {
    /// The name of the calendar to display.
    pub calendar_name: String,
    /// Flag to display every event in the calendar.
    #[structopt(short, long)]
    pub all: bool,
    /// Flag to display every event in the calendar starting today.
    #[structopt(short, long)]
    pub today: bool,
    /// Flag to display every event in the calendar starting this week.
    #[structopt(short, long)]
    pub week: bool,
    /// Flag to display every event in the calendar starting this week.
    #[structopt(short, long)]
    pub month: bool,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct CalOpts {
    /// Name of the calendar.
    pub calendar_name: String,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(about = "A lightweight command line calendar tool.")]
pub enum SubCommands {
    /// Command to create a new calendar in the database.
    #[structopt(name = "new")]
    New(CalOpts),
    /// Command to list events in a calendar with configurable options.
    #[structopt(name = "ls")]
    List(ListOpts),
    /// Command to add an event to a given calendar.
    #[structopt(name = "add")]
    Add(Event),
    /// Command to remove an existing calendar from the database.
    #[structopt(name = "rm")]
    Remove(CalOpts),
}
