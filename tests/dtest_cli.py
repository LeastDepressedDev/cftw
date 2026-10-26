from decktest import fnc
from decktest.tests import *

ERROR_ARG_MISMATCH = """ERROR: wrong argument
 Try 'cftw-cli --help' for more information
"""

ERROR_WRONG_USAGE = """ERROR: wrong usage
 Try 'cftw-cli --help' for more information
"""

ERROR_UNKNOWN_FLAG = """ERROR: unknown flag
 Try 'cftw-cli --help' for more information
"""

ERROR_EMPTY_FLAG = """ERROR: empty flag
 Try 'cftw-cli --help' for more information
"""

# ERROR_INACCEPTABLE_FLAG = """ERROR: inaccaptable
#  Try 'cftw-cli --help' for more information
# """

app = fnc.Application("./cftw-cli")
ArgSection = TestSection("ArgSection", full_test_name=True, log_completed=True)

Test("NoArguments").expect_error(253, ERROR_ARG_MISMATCH).wrap_up(ArgSection)
Test("RandomArgument").cmd_line(["random_gibberish"]).expect_error(253, ERROR_ARG_MISMATCH).wrap_up(ArgSection)
Test("IncompleteCall").cmd_line(["-i", "something", "-o", "somethingelse"]).expect_error(253, ERROR_ARG_MISMATCH).wrap_up(ArgSection)

Test("WrongCall1").cmd_line(["-o", "-i"]).expect_error(254, ERROR_WRONG_USAGE).wrap_up(ArgSection)
Test("WrongCall2").cmd_line(["--output"]).expect_error(254, ERROR_WRONG_USAGE).wrap_up(ArgSection)
Test("WrongCallMixed").cmd_line(["--output", "-i"]).expect_error(254, ERROR_WRONG_USAGE).wrap_up(ArgSection)

Test("RandomShortFlag").cmd_line(["-a"]).expect_error(255, ERROR_UNKNOWN_FLAG).wrap_up(ArgSection)
Test("RandomLongFlag").cmd_line(["--oguzok"]).expect_error(255, ERROR_UNKNOWN_FLAG).wrap_up(ArgSection)
Test("RandomGibberishFlag1").cmd_line(["---\\a"]).expect_error(255, ERROR_UNKNOWN_FLAG).wrap_up(ArgSection)
Test("RandomGibberishFlag2").cmd_line(["-a-b-a"]).expect_error(255, ERROR_UNKNOWN_FLAG).wrap_up(ArgSection)

Test("EmptyShortFlag").cmd_line(["-", "something"]).expect_error(252, ERROR_EMPTY_FLAG).wrap_up(ArgSection)
Test("EmptyLongFlag").cmd_line(["--", "something"]).expect_error(252, ERROR_EMPTY_FLAG).wrap_up(ArgSection)

Test("MultisettingSingle").cmd_line(["-oiw", "path"]).expect("path\npath\npath\n").wrap_up(ArgSection)
Test("MultisettingSeparate").cmd_line(["-oi", "path", '-w', "wm_path"]).expect("path\npath\nwm_path\n").wrap_up(ArgSection)
Test("MultisettingSeparateMixed").cmd_line(["-io", "path", '--watermark', "wm_path"]).expect("path\npath\nwm_path\n").wrap_up(ArgSection)

Test("TemporaryCompleteCallShort").cmd_line(["-i", "source", "-w", "wm_source", "-o", "output_path"])\
    .expect("source\noutput_path\nwm_source\n").wrap_up(ArgSection)
Test("TemporaryCompleteCallLong").cmd_line(["--input", "source", "--watermark", "wm_source", "--output", "output_path"])\
    .expect("source\noutput_path\nwm_source\n").wrap_up(ArgSection)
Test("TemporaryCompleteCallMixed1").cmd_line(["--input", "source", "-w", "wm_source", "--output", "output_path"])\
    .expect("source\noutput_path\nwm_source\n").wrap_up(ArgSection)
Test("TemporaryCompleteCallMixed2").cmd_line(["--input", "source", "-w", "wm_source", "-o", "output_path"])\
    .expect("source\noutput_path\nwm_source\n").wrap_up(ArgSection)

exit(max([ArgSection.run(app)]))
