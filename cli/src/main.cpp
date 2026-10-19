#include <iostream>
#include <string>
#include <iomanip>
#include <bmplib.h>
#include <cstring>

using namespace std;

/*

    Struct for congig

    string inputPath        - path to input file
    string outputPath       - path to output file
    string watermarkPath    - path to watermark

*/

struct Config {
	string inputPath;
	string outputPath;
	string watermarkPath;
};

// long flags
const char* const kHelpLong = "--help";
const char* const kInputLong = "--input";
const char* const kOutputLong = "--output";
const char* const kWatermarkLong = "--watermark";

// short flags
const char kHelpShort = 'h';
const char kInputShort = 'i';
const char kOutputShort = 'o';
const char kWatermarkShort = 'w';

/*

    Printing in console instruction for use cftw-cli

*/

void printHelp(){
    cout << 
        "Usage:\n"
        "  cftw-cli [OPTIONS]\n\n"
        
        "Description:\n"
        "  CLI tool for watermarking BMP images.\n";

    cout << "\nRequired flags:\n";
    
    cout << " " << left << setw(25) << "-i, --input <path>" << "Path to the source BMP image\n";
    cout << " " << left << setw(25) << "-w, --watermark <path>" << "Path to the BMP watermark\n";
    cout << " " << left << setw(25) << "-i, --output <path>" << "Path to the output BMP image\n";

    cout << "\nOptional flags:\n";

    cout << " " << left << setw(25) << "-h, --help" << "Display detailed description of configuration parameters and exit\n";
}

/*

    Getting i + 1 argument from argv into configPtr

    Arguments:
     int argc           - number of arguments
     char** argv        - cl argument
     int* i             - pointer on parser iteration
     string* configPtr  - pointer for result

    Return:
     int exitCode       - exit code

*/

int nextArg(int argc, char** argv, int* i, string* configPtr){
    if (*i + 1 >= argc){
        return -2;
    }else{
        if (argv[*i + 1][0] != '-'){
            *configPtr = string(argv[*i + 1]);
            (*i) ++;
            return 0;
        }
        else {
            return -2;
        }
    }
}

/*
    
    Command line arguments parser

    Arguments:
     int argc           - number of arguments
     char** argv        - cl arguments
     Config* config     - config

    Return:
     int exitCode       - exit code

*/

int ParseArguments(int argc, char** argv, Config* config){

    // Parsing Arguments

    for (int i = 1; i < argc; i++){
        if (argv[i][0] == '-'){

            // Long Flags

            if (argv[i][1] == '-'){

                // --help
                if (strcmp(argv[i], kHelpLong) == 0){
                    printHelp();
                    return 1;
                }
                
                // --input
                else if(strcmp(argv[i], kInputLong) == 0){
                    if (nextArg(argc, argv, &i, &config->inputPath) != 0){
                        return -2;
                    }
                }
    
                // --output
                else if(strcmp(argv[i], kOutputLong) == 0){
                    if (nextArg(argc, argv, &i, &config->outputPath) != 0){
                        return -2;
                    }                    
                }

                // --watermark
                else if(strcmp(argv[i], kWatermarkLong) == 0){
                    if (nextArg(argc, argv, &i, &config->watermarkPath) != 0){
                        return -2;
                    }                    
                }

                // Unknown flag exeption
                else {
                    return -1; 
                }
            }

            // Sort Flags

            else{
                switch(argv[i][1]){
                    // -h
                    case(kHelpShort):
                        printHelp();
                        return 1;
                        break;

                    // -i
                    case(kInputShort):
                        if (nextArg(argc, argv, &i, &config->inputPath) != 0){
                            return -2;
                        }
                        break;

                    // -o
                    case(kOutputShort):
                        if (nextArg(argc, argv, &i, &config->outputPath) != 0){
                            return -2;
                        }

                        break;

                    // -w
                    case(kWatermarkShort):
                        if (nextArg(argc, argv, &i, &config->watermarkPath) != 0){
                            return -2;
                        }

                        break;
        
                    // Unknown flag exeption
                    default:
                        return -1;
                        break;
                }
            }
        }
    }

    // Validity check
    
    if (config->inputPath == ""){
        return -3;
    }
    if (config->outputPath == ""){
        return -3;
    }
    if (config->watermarkPath == ""){
        return -3;
    }

    return 0;
}

/*

    The main function of cftw-cli

*/

int main(int argc, char** argv){

    // Config
    Config config;

    // Config default values
    config.inputPath = "";
    config.outputPath = "";
    config.watermarkPath = "";

    // Parsing Arguments
	int exitCode = ParseArguments(argc, argv, &config);

    // handling of exitCode cases
    switch(exitCode){
        
        case(0):
            cout << config.inputPath << "\n";
    	    cout << config.outputPath << "\n";
    	    cout << config.watermarkPath << "\n";
            break;

        case(-1):       // Unknown flag
            cerr << "Error: unknown flag\n"
                "Try 'cftw-cli --help' for more information\n";
            break;

        case(-2):       // Wrong usage
            cerr << "Error: wrong usage\n"
                "Try 'cftw-cli --help' from more information\n";
            break;

        case(-3):       // Wrong argument
            cerr << "Error: wrong argument"
                "Try 'cftw-cli --help' from more information\n";
            break
    }

	return 0;	
}
