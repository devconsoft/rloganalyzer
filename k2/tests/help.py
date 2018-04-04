
from k2.component.decorator import requires

@requires(rlog='RLogAnalyzer')
def test_help(rlog):
    output = rlog('--help')
    help_text = """\
USAGE:
    rloganalyzer [FLAGS] [OPTIONS] <CONFIG_FILE> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Sets the level of verbosity

OPTIONS:
    -i, --in <INPUT_FILE>           Sets the input file to use. Dash (-) for stdin.
    -o, --output <OUTPUT_FILE>      Sets the output file for the report to use. Dash (-) for stdout.
    -s, --summary <SUMMARY_FILE>    Sets the summary output file. Dash (-) for stdout.

ARGS:
    <CONFIG_FILE>    Configuration definition file to use.

SUBCOMMANDS:
    config-check    Runs configuration definition check - then exits.
    help            Prints this message or the help of the given subcommand(s)"""
    assert help_text in output, output
