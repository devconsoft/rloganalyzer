
from k2.component.decorator import requires

@requires(rlog='RLogAnalyzer')
def test_version(rlog):
    output = rlog('--version')
    assert "rloganalyzer" in output, output
