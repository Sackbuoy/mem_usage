use std::env;
use std::error::Error;
use std::process::{Child, Command};
use wait4::{ResUse, Wait4};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<String> = env::args().collect();

    match run_command(&mut args) {
        Ok(resources) => print_resources(&resources),
        Err(e) => return Err(e),
    };

    Ok(())
}

fn print_resources(resources: &ResUse) {
    println!("------------");
    println!("User time: {}ms", resources.rusage.utime.as_millis());
    println!("Kernel time: {}ms", resources.rusage.stime.as_millis());
    println!("Memory use: {}MB", resources.rusage.maxrss / (1024 * 1024));
}

fn run_command(args: &mut Vec<String>) -> Result<ResUse, Box<dyn Error>> {
    let mut cmd: Command = Command::new(&args[1]);

    for arg in &mut args[2..] {
        cmd.arg(arg);
    }

    let process_resources: ResUse;

    let _child_process: Child = match cmd.spawn() {
        Ok(mut child) => {
            process_resources = child.wait4()?;
            child
        }
        Err(e) => return Err(Box::new(e)),
    };

    Ok(process_resources)
}
