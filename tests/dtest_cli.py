from decktest import fnc
from decktest.tests import *

ERROR_MISMATCH = """ERROR: wrong argument
 Try 'cftw-cli --help' for more information
"""

app = fnc.Application("./cftw-cli")
ArgSection = TestSection("ArgSection", full_test_name=True, log_completed=True)

Test("NoArguments").expect_error(253, ERROR_MISMATCH).wrap_up(ArgSection)

exit(ArgSection.run(app))