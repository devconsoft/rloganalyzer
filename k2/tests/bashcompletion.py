
import os.path


from k2.component.decorator import requires

def test_autocomplete():
    fname = '../target/debug/rloganalyzer.bash'
    assert os.path.isfile(fname), 'file does not exist: {}'.format(fname)
