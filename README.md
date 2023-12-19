# An RSync Wrapper
**built in Rust and configurable in TOML**

I decided to make this toy project because I just wanted a small command that
would do the rsync backups I wanted and log the result.

And that is exactly what Backer is. The TOML-based configuration for Backer
aims to be flexible and allows for intricately formatted logs.
Being built in Rust, it's robust and gives detailed Error messages.
Yeah, they aren't very pretty yet, but I'll work on that.

The project is at the moment Linux-only, as it relies on the `rsync`-command.


# Message to myself
**Still to be implemented**

- Preamptive checks for the source and target files of backups
- Formatted summary of backups at the end of a run call
- Check for recursive ctx-variables
- A better system for config errors and ctx-string-errors
- Prettier errors
- Subcommands
  - preview
  - manual
  - maybe more...?


# Configuration
Backer is configured via `$HOME/.config/backer.toml`.

An example config:
```toml
[variables]
date = "%Y-%m-%d"
log_dir = "~/backer-logs"
log_base = "${log_dir}/${date}"

[[run]]
source = "~"
target = "/media/backups/"
log.stderr = "${log_base}.err"
log.stdout = "${log_base}.out"
```

## Structure
Backer's config is structured into
- a `variables`-section
- a `template`-section
- an array of `run`-sections

```toml
[variables]
# Custom variables

[template]
# Default values for all backups

[[run]]
# Settings for each backup

[[run]]
# ...
```

### `[variables]`
You can define custom variables in this section!
These can be referenced from within [format strings](#FormatStrings).
The value of a variable is itself a format string and variables
can refer to each other.

At the moment, variables can even be recursive, which leads to a
very unceremonial stack overflow.

### `[template]`
This section allows you to override the default settings
for backups.

For reference, here's all the default values:
```toml
[template]
exclude = []
output = "default"

[template.method]
sudo = false
delete = false
dry_run = false

[template.log]
append = false
stderr = "errors.log"
stdout = "output.log"
format = "${log}"
```

For the specific functions of these values,
see [below](#ValuesAndSections).


### `[[run]]`
A `run` definition can include override any value from the
`template`-section - The structure is the exact same.

However, it must include a `source` and a `target` value,
to define what file or directory should be backed up to
where. These values are only found in a `run`-definition
and have no default value, as I'd find it a bit stupid to
define a default target and source location that is used
by multiple, consecutively executed backups.

<a name="ValuesAndSections" />
## Values and Sub-Sections

Here's an overview over all values found in the
`template`- and `run`-sections.

- `source`
  (format string)
  
  Path to the source file or source
  directory of the backup.
  
- `target`
  (format string)
  
  Path to the target file or target
  directory of the backup.
  
- `exclude`
  (Array of format strings)
  
  List of files or directories to exclude
  from the backup.
  
- `output`
  (number or string)
  
  Output level of the backup.
  
  Possible values:
  | number | string      | summary               |
  | ------ | ----------- | --------------------- |
  | `0`    | `"quiet"`   | Only print errors.    |
  | `1`    | `"default"` | Normal output.        |
  | `2`    | `"verbose"` | Print verbose output. |
  
- `method`
  (table)
  
  Details for how the backup
  should be executed.
  
  Values:
  - `sudo`
    (bool)
    
    Run backup with super user rights.
    
  - `delete`
    (bool)
    
    Delete files or directories that are in the
    target but not in the source directory.
    
 - `dry_run`
   (bool)
   
   Only generate output, don't copy anything.
   Will still generate logs.
   
- `log`
  (table)
  
  Definitions for logging the backup.
  
  Values: 
  - `append`
    (bool)
    
    Append to the log file instead of
    overwriting it
    
  - `stdout`
    (format string)
    
    Path to the file to log standard
    output to.
    
  - `stderr`
    (format string)
    
    Path to the file to log standard
    error to.
    
  - `format`
    (format string)
    
    Format of logs. Can reference
    the `${log}` variable, which
    refers to stdout / stderr.


<a name="FormatStrings" />
## Format Strings

Backer has it's own system for formatting strings.
Most configuration values rely on these format strings.

### Variables
A variable can be referenced from within a format string
like this: `"${<var>}"`.

Variable names can technically be any string.
They can even contain `}` by escaping: `"${example\{\}}"`.
However, it's adviced to stick to more conventional variable names.

Backer's format strings can reference any variable defined in
the config's `variables` section.
Additionaly, `source` and `target` are provided for every
format string related to a specific backup in the `run`-section.
The `log.format` string can also reference the special `log`-variable,
which refers to either the standard output or standard error of rsync,
depending on what is logged at the moment.

The `source`, `target` and `log` variables can be overwritten in the
`variables`-section! This might lead to behaviour you didn't intend
for, so keep that in mind!

### DateTime
Format strings can also compute date-time information like so: `"%<atom>"`.
Take a look at the custom `date`-variable defined in the example config:
```toml
date="%Y-%m-%d"
```
`%Y`, `%m` and `%d` refer to the current year, month and day, respectively.
The whole string would compute to something like `2023-12-19`.

To be honest, the functionality is straight up stolen from the `chrono`-crate.
See [their documentation](https://docs.rs/chrono/0.4.31/chrono/format/strftime/index.html)
for a full list of possible atoms.

### Literal
Anything part of a format string that isn't preceeded with a `$` or `%` will
be taken as a literal string. That includes `{` and `}`. Even though these
are part of referencing a variable, you can use them in literal strings freely.

In cas you do want to use a literal `$` or `%d` in a format string, you can
escape them with a backslash: `'There is a \$, but no variable'`.

If you use regular quotation marks in your configuration, you will have to use
_tow_ consecutive backslashes to escape a `$` or `%`: `"E\\$cape"`.
To avoid this, you can instead use toml's literal strings: `'E\$cape'`
