use std::thread;

use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
    EXECUTION_STATE,
};

fn main() {
    println!(
        "{} version {} by {}.\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );
    println!("Try to prevent automatic hibernation and automatic screen lock...");

    let wake_flags: EXECUTION_STATE = ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED;
    let ret: EXECUTION_STATE = unsafe { SetThreadExecutionState(wake_flags) };

    if ret.0 == 0 {
        eprint!(
            "Failed to set execution state. Error code: {:?}. Exiting.\n",
            ret
        );
        std::process::exit(1);
    } else {
        println!("Set wake lock sucessfully.");
        //println!("Execution state: {:?}", ret);
        println!("Note: Wake lock only lasts as long as this program runs.");
    }

    // Add a signal handler to gracefully exit to reset wake look.
    ctrlc::set_handler(|| {
        println!("Received Ctrl+C. Resetting wake lock...");
        let _ret: EXECUTION_STATE = unsafe { SetThreadExecutionState(ES_CONTINUOUS) };
        std::process::exit(0);
    })
    .expect("Failed to set Ctrl+C handler");

    // Wake lock only lasts as long as this program runs.
    loop {
        thread::sleep(std::time::Duration::from_secs(60));
    }
}
