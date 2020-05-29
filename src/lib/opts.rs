use super::event::Event;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub struct ListOpts {
    pub calendar_name: String,
    #[structopt(short, long)]
    pub all: bool,
    #[structopt(short, long)]
    pub today: bool,
    #[structopt(short, long)]
    pub week: bool,
    #[structopt(short, long)]
    pub month: bool,
    #[structopt(short, long)]
    pub verbose: bool
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct CalOpts {
    pub calendar_name: String,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(about="command line planner/calendar tool")]
pub enum SubCommands {
    #[structopt(name="new")]
    New(CalOpts),
    #[structopt(name="ls")]
    List(ListOpts),
    #[structopt(name="add")]
    Add(Event),
    #[structopt(name="rm")]
    Remove(CalOpts)
}

