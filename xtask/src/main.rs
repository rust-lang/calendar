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
}

fn print_usage() -> ExitCode {
    const USAGE: &str = "\
Usage: cargo xtask [OPTIONS] COMMAND [COMMAND_OPTIONS]

Commands:
  generate-uid, uid    Generate an event UID
    -n                 Suppress trailing newline

Options:
  -h                   Print help
";
    println!("{USAGE}");
    ExitCode::SUCCESS
}

fn err_usage() -> ExitCode {
    print_usage();
    ExitCode::FAILURE
}

fn main() -> ExitCode {
    let mut args = std::env::args();
    let _arg0 = args.next().unwrap();
    let Some(arg1) = args.next() else { return err_usage() };
    match &*arg1 {
        "-h" => return print_usage(),
        "generate-uid" | "uid" => {
            let mut trailing_nl = true;
            match args.next().as_deref() {
                Some("-h") => return print_usage(),
                Some("-n") => trailing_nl = false,
                None => {}
                _ => return err_usage(),
            };
            print_uid();
            if trailing_nl {
                println!();
            }
        }
        _ => return err_usage(),
    }
    ExitCode::SUCCESS
}
