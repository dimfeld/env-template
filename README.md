This utility renders a Handlebars-style template file from values in a `.env` file, optionally including values from the environment.

```
USAGE:
    env-template [FLAGS] [OPTIONS] <file>

FLAGS:
    -a, --all        Expose the entire environment to the template, not just the .env contents
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -v, --vars <vars>    Load the variables from this file instead of .env

ARGS:
    <file>    The template to render
```
