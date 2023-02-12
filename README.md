# fix-iso-dates

This is a toy project for improving my Rust skills.

It reads data from stdin and writes it to stdout, where all ISO dates (like, e.g., 2023-02-10) are adjusted such that
they are not less than the date which is passed with the option `-m/--min-date`. The value defaults to today, which is
replaced by the current date.

Note that everything that matches the regular expression `\d{4}-\d{2}-\d{2}` is considered an ISO date here for
simplicity.