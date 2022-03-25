use std::error::Error;

use clap::ArgEnum;
use clap::Parser;
use windows::Win32::System::Power::{
    EXECUTION_STATE,
    SetThreadExecutionState
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// Keep a Windows machine awake
pub struct Args {
    /// Awake mode
    #[clap(long, short, arg_enum, default_value_t = AwakeMode::System)]
    pub awake_mode: AwakeMode,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum AwakeMode {
    /// Display ==> prevent the machine from going to sleep AND keep the display on
    Display,
    /// System ==> prevent the machine from going to sleep
    System
}

// Why wrap everything in a struct?
// Because I would like to ensure I always reset the thread execution state to ES_CONTINUOUS
//   and I ensure this via the Drop trait
struct StayAwake {
    es: EXECUTION_STATE,
}

impl StayAwake {
    fn new() -> Self {
        unimplemented!()
    }
}

impl Drop for StayAwake {
    fn drop(&mut self) {
        unimplemented!()
        /*
        let next_thread_exec_state = ExecutionState::ES_CONTINUOUS;
        let prev_thread_exec_state = (self.ste)(next_thread_exec_state);
        println!(
            "\nReset thread execution state:\n    {} ==> {}\n      {} ==> {}",
            String::from("From").red(),
            prev_thread_exec_state,
            String::from("To").blue(),
            next_thread_exec_state);
        */
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    println!("{:#?}", args);

    Ok(())
}