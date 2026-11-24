from decktest import fnc
from decktest.tests import *
import os

os.makedirs("out/arg_checks")
os.makedirs("out/generative_checks")
os.makedirs("out/edge_cases")

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


ErrorHandling = TestSection("ErrorHandling", full_test_name=True, log_completed=True)

Test("NoArguments").expect_error(253, ERROR_ARG_MISMATCH).wrap_up(ErrorHandling)
Test("RandomArgument").cmd_line(["random_gibberish"]).expect_error(253, ERROR_ARG_MISMATCH).wrap_up(ErrorHandling)
Test("IncompleteCall").cmd_line(["-i", "something", "-o", "somethingelse"]).expect_error(253, ERROR_ARG_MISMATCH).wrap_up(ErrorHandling)

Test("WrongCall1").cmd_line(["-o", "-i"]).expect_error(254, ERROR_WRONG_USAGE).wrap_up(ErrorHandling)
Test("WrongCall2").cmd_line(["--output"]).expect_error(254, ERROR_WRONG_USAGE).wrap_up(ErrorHandling)
Test("WrongCallMixed").cmd_line(["--output", "-i"]).expect_error(254, ERROR_WRONG_USAGE).wrap_up(ErrorHandling)

Test("RandomShortFlag").cmd_line(["-a"]).expect_error(255, ERROR_UNKNOWN_FLAG).wrap_up(ErrorHandling)
Test("RandomLongFlag").cmd_line(["--oguzok"]).expect_error(255, ERROR_UNKNOWN_FLAG).wrap_up(ErrorHandling)
Test("RandomGibberishFlag1").cmd_line(["---\\a"]).expect_error(255, ERROR_UNKNOWN_FLAG).wrap_up(ErrorHandling)
Test("RandomGibberishFlag2").cmd_line(["-a-b-a"]).expect_error(255, ERROR_UNKNOWN_FLAG).wrap_up(ErrorHandling)

Test("EmptyShortFlag").cmd_line(["-", "something"]).expect_error(252, ERROR_EMPTY_FLAG).wrap_up(ErrorHandling)
Test("EmptyLongFlag").cmd_line(["--", "something"]).expect_error(252, ERROR_EMPTY_FLAG).wrap_up(ErrorHandling)



CorrectArgumentsSection = TestSection("CorrectArgsSection", full_test_name=True, log_completed=True)

Test("DefaultCall").cmd_line(["--input", "img1.bmp", "--output", "out/arg_checks/DefaultCall.bmp", "--watermark", "wm1.bmp"]).expect("").wrap_up(CorrectArgumentsSection)
Test("DefaultCallReordered").cmd_line(["--watermark", "wm1.bmp", "--input", "img1.bmp", "--output", "out/arg_checks/DefaultCallReordered.bmp"]).expect("").wrap_up(CorrectArgumentsSection)
Test("ShortCall").cmd_line(["-i", "img1.bmp", "-o", "out/arg_checks/ShortCall.bmp", "-w", "wm1.bmp"]).expect("").wrap_up(CorrectArgumentsSection)

Test("MixedCall1").cmd_line(["-i", "img1.bmp", "--output", "out/arg_checks/MixedCall_1.bmp", "-w", "wm1.bmp"]).expect("").wrap_up(CorrectArgumentsSection)
Test("MixedCall2").cmd_line(["-o", "out/arg_checks/MixedCall_2.bmp", "--watermark", "wm1.bmp", "--input", "img1.bmp"]).expect("").wrap_up(CorrectArgumentsSection)

Test("ShortlessCall1").cmd_line(["-io", "img1.bmp", "out/arg_checks/ShortlessCall_1.bmp", "--watermark", "wm1.bmp"]).expect("").wrap_up(CorrectArgumentsSection)
Test("ShortlessCall1").cmd_line(["-iwo", "img1.bmp", "wm1.bmp", "out/arg_checks/ShortlessCall_1.bmp"]).expect("").wrap_up(CorrectArgumentsSection)



GenerativeSection = TestSection("GenerativeSection", full_test_name=True, log_completed=True)

Test("BasicCase").cmd_line(["-iow", "img1.bmp", "out/generative_checks/BasicCase.bmp", "wm1.bmp", "-s", "100"]).expect("").wrap_up(GenerativeSection)
Test("AdvancedCase").cmd_line(["-iow", "img1.bmp", "out/generative_checks/AdvancedCase.bmp", "wm2.bmp", "-s", "100", "50", "-p", "250", "15"]).expect("").wrap_up(GenerativeSection)
Test("AdvancedCaseShort").cmd_line(["-iow", "img1.bmp", "out/generative_checks/AdvancedCaseShort.bmp", "wm3.bmp", "-c", "right-bottom", "--padding", "400", "50"])\
    .expect("").wrap_up(GenerativeSection)

Test("TilingTest1").cmd_line(["-iow", "img1.bmp", "out/generative_checks/TilingTest1.bmp", "wm1.bmp", "-t", "10"]).expect("").wrap_up(GenerativeSection)
Test("TilingTest2").cmd_line(["-iow", "img1.bmp", "out/generative_checks/TilingTest2.bmp", "wm2.bmp", "--tile", "10", "30"]).expect("").wrap_up(GenerativeSection)
Test("TilingTest3").cmd_line(["-iow", "img1.bmp", "out/generative_checks/TilingTest3.bmp", "wm3.bmp", "--corner", "left-bottom", "-t", "10", "-p", "50", "--size", "400"]).expect("").wrap_up(GenerativeSection)



EdgeCases = TestSection("EdgeCases", full_test_name=True, log_completed=True)

Test("SelfImage").cmd_line(["-iow", "img1.bmp", "out/edge_cases/SelfImage.bmp", "img1.bmp"]).expect("").wrap_up(EdgeCases)
Test("TwoBigImages").cmd_line(["-iow", "img1.bmp", "out/edge_cases/TwoBigImages.bmp", "img2.bmp"]).expect("").wrap_up(EdgeCases)
Test("TiledSelfImage").cmd_line(["-iow", "img1.bmp", "out/edge_cases/TiledSelfImage.bmp", "img1.bmp", "-t", "20", "--size", "100"]).expect("").wrap_up(EdgeCases)
Test("BiiiiiiiiiiigImage").cmd_line(["-iow", "img1.bmp", "out/edge_cases/BiiiiiiiiiiigImage.bmp", "img2.bmp", "--size", "10000000"]).expect("").wrap_up(EdgeCases)

Test("ToooBig").cmd_line(["-iow", "img1.bmp", "out/edge_cases/__TooBig.bmp", "wm2.bmp", "-t", "10", "-s", "20000000000000000000000000000000000000000000"]).expect_error(253, ERROR_ARG_MISMATCH).wrap_up(EdgeCases)
Test("NegativeNumbers").cmd_line(["-iow", "img1.bmp", "out/edge_cases/__NegativeNumbers.bmp", "wm2.bmp", "-t", "-10", "-s", "20"]).expect_error(253, ERROR_ARG_MISMATCH).wrap_up(EdgeCases)



exit(max([
    [ErrorHandling.run(app), print("\n\n\n\n")][0], 
    [CorrectArgumentsSection.run(app), print("\n\n\n\n")][0], 
    [GenerativeSection.run(app), print("\n\n\n\n")][0], 
    [EdgeCases.run(app), print("\n\n\n\n")][0]
]))
