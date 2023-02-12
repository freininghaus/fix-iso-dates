# fix-iso-dates

This is a toy project for improving my Rust skills.

It reads data from stdin and writes it to stdout, where all ISO dates (like, e.g., 2023-02-10) are adjusted such that
they are not less than the date which is passed with the option `-m/--min-date`. The value defaults to today, which is
replaced by the current date.

Note that everything that matches the regular expression `\d{4}-\d{2}-\d{2}` is considered an ISO date here for
simplicity.

## Example
Running 
```bash
echo '{"date": "2022-01-01"}' | fix-iso-dates --min-date 2023-02-12
```
or
```bash
echo '{"date": "2022-01-01"}' | fix-iso-dates -m 2023-02-12
```
will print this to standard output:
```json
{"date": "2023-02-12"}
```
The command
```bash
echo '{"date": "2022-01-01"}' | fix-iso-dates
```
will output the same if 2023-02-12 happens to be the current date.
