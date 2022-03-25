use std::error::Error;
use std::io;
use std::io::Write;

use clap::Parser;
use colored::Colorize;
use windows::Win32::System::Power::{
    // ES_AWAYMODE_REQUIRED,
    ES_CONTINUOUS,
    ES_DISPLAY_REQUIRED,
    ES_SYSTEM_REQUIRED,
    // ES_USER_PRESENT,
    EXECUTION_STATE,
    SetThreadExecutionState
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// Keep a Windows machine awake
pub struct Args {
    /// Keep display on
    #[clap(long)]
    pub display: bool,
}

// Storing the execution state within a struct ensures that the thread execution state
//   is reset to ES_CONTINUOUS (via the implementation of the Drop trait) when the struct
//   goes out of scope
struct StayAwake(EXECUTION_STATE) ;

impl StayAwake {
    fn new() -> Self {
        Self(ES_CONTINUOUS)
    }

    fn update_execution_state(&self, next_es: EXECUTION_STATE) -> EXECUTION_STATE {
        unsafe { SetThreadExecutionState(ES_CONTINUOUS | next_es) }
    }
}

impl Drop for StayAwake {
    fn drop(&mut self) {
        let next_es = ES_CONTINUOUS;
        let next_es_label = execution_state_as_string(next_es);
        let prev_es = self.update_execution_state(next_es);
        let prev_es_label = execution_state_as_string(prev_es);
        println!(
            "\nReset thread execution state:\n    {} ==> {} ({:#X})\n      {} ==> {} ({:#X})",
            String::from("From").red(),
            prev_es_label,
            prev_es.0,
            String::from("To").blue(),
            next_es_label,
            next_es.0
        );
    }
}

const ES_CONTINUOUS_BITOR_ES_DISPLAY_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(ES_CONTINUOUS.0 | ES_DISPLAY_REQUIRED.0);
const ES_CONTINUOUS_BITOR_ES_SYSTEM_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(ES_CONTINUOUS.0 | ES_SYSTEM_REQUIRED.0);

fn execution_state_as_string(es: EXECUTION_STATE) -> String {
    match es {
        ES_CONTINUOUS => String::from("ES_CONTINUOUS"),
        ES_DISPLAY_REQUIRED => String::from("ES_DISPLAY_REQUIRED"),
        ES_SYSTEM_REQUIRED => String::from("ES_SYSTEM_REQUIRED"),
        ES_CONTINUOUS_BITOR_ES_DISPLAY_REQUIRED => String::from("ES_CONTINUOUS | ES_DISPLAY_REQUIRED"),
        ES_CONTINUOUS_BITOR_ES_SYSTEM_REQUIRED => String::from("ES_CONTINUOUS | ES_SYSTEM_REQUIRED"),
        _ => String::from("???")
    }
}

/***
From Microsoft documentation
https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate#parameters


##########################
## ES_AWAYMODE_REQUIRED ##
##########################
Value ==> 0x00000040
Description ==> Enables away mode. This value must be specified with ES_CONTINUOUS.

Away mode should be used only by media-recording and media-distribution applications that must perform critical background processing on desktop computers while the computer appears to be sleeping. See Remarks.


###################
## ES_CONTINUOUS ##
###################
Value ==> 0x80000000
Description ==> Informs the system that the state being set should remain in effect until the next call that uses ES_CONTINUOUS and one of the other state flags is cleared.


#########################
## ES_DISPLAY_REQUIRED ##
#########################
Value ==> 0x00000002
Description ==> Forces the display to be on by resetting the display idle timer.


########################
## ES_SYSTEM_REQUIRED ##
########################
Value ==> 0x00000001
Description ==> Forces the system to be in the working state by resetting the system idle timer.


#####################
## ES_USER_PRESENT ##
#####################
Value ==> 0x00000004
Description ==> This value is not supported. If ES_USER_PRESENT is combined with other esFlags values, the call will fail and none of the specified states will be set.

*/

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // requested execution state
    let req_es = if args.display {
        println!("Running in ``{}`` mode ==> the machine will not go to sleep and the display will remain on", String::from("Display").green());

        ES_DISPLAY_REQUIRED
    } else {
        println!("Running in ``{}`` mode ==> the machine will not go to sleep", String::from("System").green());

        ES_SYSTEM_REQUIRED
    };

    // state to set - must combine ES_CONTINUOUS with another state
    let next_es = ES_CONTINUOUS | req_es;
    let next_es_label = execution_state_as_string(next_es);

    // initialize struct
    let sa = StayAwake::new();

    // set thread execution state
    let prev_es = sa.update_execution_state(next_es);
    let prev_es_label = execution_state_as_string(prev_es);

    // print
    println!(
        "\nSet thread execution state:\n    {} ==> {} ({:#X})\n      {} ==> {} ({:#X})",
        String::from("From").purple(),
        prev_es_label,
        prev_es.0,
        String::from("To").cyan(),
        next_es_label,
        next_es.0
    );

    print!("\nPress ``{}`` key to reset ", String::from("Enter").yellow());
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer)?;

    // After exiting main, StayAwake instance is dropped and the thread execution
    //   state is reset to ES_CONTINUOUS
    Ok(())
}
