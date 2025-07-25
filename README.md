# Rust Team Calendars
This repository defines the calendars for the teams of the Rust project. Each team or working group
can define a TOML file listing their events and a iCalendar file will be generated and hosted on
GitHub Pages for contributors and interested parties to add to their calendar application of choice.

## Why this?
Some teams had previously used Google Calendar for their team calendar, but this wasn't ideal as
permissions to update and add to the calendar had to be managed manually by team leads using
a platform that the project otherwise didn't use. In contrast, everyone in the project already
has a GitHub account and we already have processes and tooling in place to manage access to GitHub
repositories.

## How do I subscribe to these calendars?
Each of the toml files in this repository generates an `ics` file of the same name at
`https://rust-lang.github.io/calendar/$name.ics`. Below are links to the `ics` files for all the
current calendars in this repository:

- [All teams and working groups](https://rust-lang.github.io/calendar/all.ics)
  - [Leadership Council](https://rust-lang.github.io/calendar/council.ics)
  - [Compiler Team + Working Groups](https://rust-lang.github.io/calendar/compiler.ics)
    - [Compiler Team](https://rust-lang.github.io/calendar/compiler.events-only.ics)
    - [Stable MIR Project Group](https://rust-lang.github.io/calendar/project-stable-mir.ics)
    - [Rust Analyzer](https://rust-lang.github.io/calendar/rust-analyzer.ics)
    - [Async Working Group](https://rust-lang.github.io/calendar/wg-async.ics)
    - [Types Team](https://rust-lang.github.io/calendar/types.ics)
    - [Debugging Working Group](https://rust-lang.github.io/calendar/wg-debugging.ics)
    - [Macros Working Group](https://rust-lang.github.io/calendar/wg-macros.ics)
    - [Performance Working Group](https://rust-lang.github.io/calendar/wg-performance.ics)
  - [Dev Tools Team](https://rust-lang.github.io/calendar/dev-tools.ics)
    - [Cargo Team](https://rust-lang.github.io/calendar/cargo.ics)
      - [Office Hours](https://rust-lang.github.io/calendar/cargo/office-hours.ics)
      - [Resolver and PubGrub Office Hours](https://rust-lang.github.io/calendar/cargo/resolver-pubgrub-office-hours.ics)
      - [Team Meeting](https://rust-lang.github.io/calendar/cargo/team-meeting.ics)
    - [Clippy Team](https://rust-lang.github.io/calendar/clippy.ics)
    - [Rustdoc Team](https://rust-lang.github.io/calendar/rustdoc.ics)
    - [Testing DevEx Team](https://rust-lang.github.io/calendar/testing-devex.ics)
  - [Infrastructure Team](https://rust-lang.github.io/calendar/infra.ics)
  - [Bootstrap Team](https://rust-lang.github.io/calendar/bootstrap.ics)
  - [Language Team](https://rust-lang.github.io/calendar/lang.ics)
    - [Spec Team](https://rust-lang.github.io/calendar/spec.ics)
    - [Style Team](https://rust-lang.github.io/calendar/style.ics)
    - [Vision Team](https://rust-lang.github.io/calendar/vision.ics)
  - [Library Team](https://rust-lang.github.io/calendar/libs.ics)
  - Working Groups
    - [Embedded Devices Working Group](https://rust-lang.github.io/calendar/wg-embedded.ics)
    - [Binary Size Working Group](https://rust-lang.github.io/calendar/wg-binary-size.ics)
  - [Rust Releases](https://rust-lang.github.io/calendar/release.ics)

You can copy these links and import them into your calendar application of choice.

## How do I add a calendar?
Add a new file in the repository with an appropriate name. Add the path to the `meta.includes` list
of any other calendars where that makes sense. For example, top-level team calendars are included
by `all.toml`, and working group calendars are included by their team's calendar.

In the new file, copy the following snippet to get started:

```toml
name = "Name your calendar"
description = "Describe your calendar"

[meta]
includes = [ ]
```

Add any new calendars to the list above in [*How do I subscribe to these calendars?*][subscribe].

## How do I remove a calendar?
We shouldn't remove calendars, we can just stop using them - we can rename the files in this
repository to have an `archived-` prefix if we want.

## How do I add an event?
First, select a TOML file that is relevant for your event - normally a team or working group's
calendar will be appropriate.

Next, add a `events` table with the correct details for your event. You can copy the following
snippet to get started:

```toml
[[events]]
uid = "UID - see below!"
title = "Name your event"
description = "Describe your event"
location = "Where does the event take place - Zoom, Zulip?"
last_modified_on = "2024-01-05T15:46:00.00Z"
start = "2024-01-11T15:00:00.00Z"
end = "2024-01-11T16:00:00.00Z"
status = "confirmed"
organizer = { name = "Who is running the event", email = "What is their email (or team's email)" }
```

All dates must be in RFC 3339 format (the same as in the examples above), and must be in UTC (ending
in `Z`).

Each event have a globally UID (because calendars can be included in other calendars,
they must be globally unique for this whole repository). It should never be changed after the
calendar has been published, see [How to generate a UID for an event?](#how-do-i-generate-a-uid-for-a-new-event).

See [*What is the schema for the calendars?*][schema] for a list of options you can set in an event.

## How do I update an event?
Modify whatever details you like - except `uid`, which **should never be changed** - and make sure
to update `last_modified_on`. Some clients use `last_modified_on` to decide whether to publish an
update to an event or recurrence or not. To ensure the client will publish the event, set
`last_modified_on` to a value in the future (a few days past the current date should be enough).

## How do I generate a UID for a new event?

To generate a UID, run:

```sh
cargo xtask generate-uid
```

Use the printed value as the `uid` for the event.

## How do I remove an event?
If the event isn't recurring, then you don't need to remove it, it'll just stay in the calendar
in the past. If the event is incorrect, you can always update it, see
[*How do I update an event?*][update].

If the event is recurring, add `until` to the recurrence rules to stop the event atthe current date,
preventing future recurrences. For example, this rule..

```toml
recurrence_rules = [ { frequency = "weekly" } ]
```

..would become..

```toml
# replace `until` with today's date
recurrence_rules = [ { frequency = "weekly", until = "2024-01-08T13:50:00.00Z" } ]
```

## What is the schema for the calendars?
You can see the [`example.toml`][example] from the [`calendar-generation`][calendar_generation]
repository for a complete list of all supported configuration options.

[calendar_generation]: https://github.com/rust-lang/calendar-generation/
[example]: https://github.com/rust-lang/calendar-generation/blob/main/example.toml
[schema]: https://github.com/rust-lang/calendar/tree/main#what-is-the-schema-for-the-calendars
[subscribe]: https://github.com/rust-lang/calendar/tree/main#how-do-i-subscribe-to-these-calendars
[update]: https://github.com/rust-lang/calendar/tree/main#how-do-i-update-an-event

## Testing and validating calendar files

To test that your calendar changes work, you can install the calendar generation tool and generate the `all.ics` calendar (which should include every other calendar, but you can run it on a specific file too).
```shell-session
$ cargo install --git https://github.com/rust-lang/calendar-generation.git
$ toml-to-ical -i all.toml -o /dev/null
```

#### License

Licensed under either of <a href="https://www.apache.org/licenses/LICENSE-2.0">Apache License,
Version 2.0</a> or <a href="https://opensource.org/licenses/MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
