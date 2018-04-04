
from k2.component.decorator import requires

@requires(rlog='RLogAnalyzer')
def test_config_long(rlog):
    output = rlog('--input IN --output OUT --summary SUMMARY --verbose --verbose CFG_FILE')
    expected = """\
def      : CFG_FILE
command  : Scan
in       : IN
out      : OUT
summary  : SUMMARY
verbosity: 2"""
    assert expected in output, output


@requires(rlog='RLogAnalyzer')
def test_config_short(rlog):
    output = rlog('-i IN -o OUT -s SUMMARY -v -v CFG_FILE')
    expected = """\
def      : CFG_FILE
command  : Scan
in       : IN
out      : OUT
summary  : SUMMARY
verbosity: 2"""
    assert expected in output, output

@requires(rlog='RLogAnalyzer')
def test_config_default(rlog):
    output = rlog('-v CFG_FILE')
    expected = """\
def      : CFG_FILE
command  : Scan
in       : -
out      : -
summary  : -
verbosity: 1"""
    assert expected in output, output

@requires(rlog='RLogAnalyzer')
def test_config_not_printed_with_verbosity_zero(rlog):
    output = rlog('CFG_FILE')
    expected = "def      : CFG_FILE"
    assert expected not in output, output
