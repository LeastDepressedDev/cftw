#include <iostream>
#include "cli.h"
#include "processor.h"

/*

    The main function of cftw-cli

*/

int main(int argc, char** argv){

    // Config
    Config* config = ConfigConstructor();

    // Parsing Arguments
	ParseArguments(argc, argv, config);

    // Calling MBP processor
    BMPprocessor(config);

    return 0;
}
