mod lib;

use lib::db::Writeable;
use lib::event::Event;
use lib::opts::SubCommands;
use structopt::StructOpt;

fn main() {
    let cmd = SubCommands::from_args();
    match cmd {
        SubCommands::Add(opts) => {
            match Event::write(&opts) {
                Ok(()) => println!("Event written to [{}].", &opts.calendar_name),
                Err(e) => println!("{}", e)
            };
        }
        SubCommands::List(opts) => {
            match Event::read(&opts) {
                Ok(events) => {
                    if events.is_empty() {
                        println!("No events to list...");
                    }
                    for event in events {
                        println!("{}", event);
                    }
                }
                Err(e) => println!("{}", e),
            };
        }
        SubCommands::New(opts) => {
            match Event::create(&opts) {
                Ok(()) => println!("New calendar [{}] created.", &opts.calendar_name),
                Err(e) => println!("{}", e),
            };
        }
        SubCommands::Remove(opts) => {
            match Event::remove(&opts) {
                Ok(()) => println!("Calendar [{}] deleted.", &opts.calendar_name),
                Err(e) => println!("{}", e),
            };
        }
    };
}
