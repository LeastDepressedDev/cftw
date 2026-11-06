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

 
    std::cout << config->input_path << "\n";
    std::cout << config->output_path << "\n";
    std::cout << config->watermark_path << "\n";
        

	return 0;	
}
