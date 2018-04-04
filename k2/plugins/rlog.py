from k2.component.decorator import component, requires

@requires(exec='Exec')
@component(name='RLogAnalyzer')
def rlog(exec):
    def _exec(*args, **kwargs):
        cmd = '../target/debug/rloganalyzer '
        cmd += ' '.join(args)
        exit_code = kwargs.get('exit_code', 0)
        return exec.send_line(cmd, expected_exit_code=exit_code)
    return _exec
