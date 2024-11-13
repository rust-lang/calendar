use std::process::ExitCode;

/// Generate a UID for an RFC 5545 iCalendar event.
///
/// The iCalender (IETF) RFC 5545 requires, in section 3.8.4.7, that
/// the `UID` be a:
///
/// > ...persistent, globally unique identifier for the calendar
/// > component.
///
/// In this context, "globally unique" means that the identifier must
/// not collide with any other identifier for any other calendar event
/// across all calendars, ever.
///
/// To achieve this, we'll generate a hexidecimal encoding of 160
/// randomly-generated bits.
fn print_uid() {
    const UID_LEN: usize = 20;
    let mut xs = [0u8; UID_LEN];
    getrandom::getrandom(&mut xs).unwrap();
    xs.iter().for_each(|x| print!("{x:02x}"));
    println!();
}

fn print_usage() {
    const USAGE: &str = "\
Usage: cargo xtask [OPTIONS] COMMAND

Commands:
  generate-uid, uid    Generate an event UID

Options:
  -h                   Print help
";
    println!("{USAGE}");
}

fn err_usage() -> ExitCode {
    print_usage();
    ExitCode::FAILURE
}

fn main() -> ExitCode {
    let mut args = std::env::args();
    let _arg0 = args.next().unwrap();
    let Some(arg1) = args.next() else { return err_usage() };
    let None = args.next() else { return err_usage() };
    match &*arg1 {
        "-h" => print_usage(),
        "generate-uid" | "uid" => print_uid(),
        _ => return err_usage(),
    }
    ExitCode::SUCCESS
}
