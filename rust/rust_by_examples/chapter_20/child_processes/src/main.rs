// we have some helpful structs for child processes
// process::Output for finished ones
// process:Command for building processes
// process::Child repressents child processes
use std::io::prelude::*;
use std::process::Command;
use std::process::Stdio;
fn main() {
    // we pass the process, in this case rustc
    // we could change this to another process, or a meaningless one if you want to experiment
    let output = Command::new("rustc")
        // we pass --version to rustc, which will make it print the version
        // we can pass a bad argument to rustc aswell
        .arg("--version")
        // we get the output of the arg
        .output()
        // output returns result, get the value out of it, or if it's an error, panic
        .unwrap_or_else(|e| {
            panic!("failed to execute process: {e}");
        });
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
    // more examples
    // Spawn the `wc` command
    // First check if windows
    let mut cmd = if cfg!(target_family = "windows") {
        // if it is, its more complicated because windows
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command")
            .arg("$input | Measure-Object -Line -Word -Character");
        cmd
        // if it isnt, we are in linux or mac most likely and thus we can just start the wc command
    } else {
        Command::new("wc")
    };
    // we can use stdin and stdout or even stderr to talk to the command and see response
    let process = match cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    const PANGRAM: &'static str = "the quick brown fox jumps over the lazy dog\n";

    // we give the process a string as input using stdin
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise `wc` wouldn't start processing the
    // input we just sent.

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    // we write to s using stdout from the process
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }

    // certain process might take a while, so we have to use wait function to wait for them to
    // finish
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
