#include <iostream>
#include <string>
#include <iomanip>
#include <bmplib.h>
#include <stdlib.h>

/*

    Struct for congig

    string input_path        - path to input file
    string output_path       - path to output file
    string watermark_path    - path to watermark

*/

struct Config {
    std::string input_path;
    std::string output_path;
    std::string watermark_path;
};

/*

    Struct for errors

    char* error_message     - error message
    int error_code          - error code

*/

struct Error {
    char* error_message;
    int error_code;

    Error(char* error_message_arg, int error_code_arg) {
        error_message = error_message_arg;
        error_code = error_code_arg;
    }

};

/*

   Config constructor

   Return:
    Config with default values

*/

Config* ConfigConstructor(){
    Config* res = new Config();

    // Config default values
    
    res->input_path = "";
    res->output_path = "";
    res->watermark_path = "";

    return res;
}

// errors
const Error* kUnknownFlagError          = new Error((char*)"unknown flag", -1);
const Error* kWrongUsageError           = new Error((char*)"wrong usage", -2);
const Error* kWrongArgumentError        = new Error((char*)"wrong argument", -3);

// long flags
const std::string kHelpLong             = "--help";
const std::string kInputLong            = "--input";
const std::string kOutputLong           = "--output";
const std::string kWatermarkLong        = "--watermark";

// short flags
const char kHelpShort                   = 'h';
const char kInputShort                  = 'i';
const char kOutputShort                 = 'o';
const char kWatermarkShort              = 'w';

/*

   Terminates the program execution immediately, prints the specified error message to the error stream, and exits with the provided status code.

    Arguments:
     const char* error  - The error message to be displayed
     int exit_code      - The exit status code.

*/

void Raise(const Error* error){
    std::cerr << "ERROR: " << error->error_message << "\n";
    std::cerr << " Try 'cftw-cli --help' for more information\n";

    exit(error->error_code);
}

/*

    Printing in console instruction for use cftw-cli

*/

void PrintHelp(){
    std::cout << 
        "Usage:\n"
        "  cftw-cli [OPTIONS]\n\n"
        
        "Description:\n"
        "  CLI tool for watermarking BMP images.\n";

    std::cout << "\nRequired flags:\n";
    
    std::cout << " " << std::left << std::setw(25) << "-i, --input <path>" << "Path to the source BMP image\n";
    std::cout << " " << std::left << std::setw(25) << "-w, --watermark <path>" << "Path to the BMP watermark\n";
    std::cout << " " << std::left << std::setw(25) << "-i, --output <path>" << "Path to the output BMP image\n";

    std::cout << "\nOptional flags:\n";

    std::cout << " " << std::left << std::setw(25) << "-h, --help" << "Display detailed description of configuration parameters and exit\n";

    exit(0);
}

/*

    Getting i + 1 argument from argv into configPtr

    Arguments:
     int argc           - number of arguments
     char** argv        - cl argument
     int* i             - pointer on parser iteration
     string* config_ptr  - pointer for result

    Return:
     int exit_code       - exit code

*/

std::string NextArg(int argc, char** argv, int i){
    if (i + 1 >= argc){
        Raise(kWrongUsageError);
    }else{
        if (argv[i + 1][0] != '-'){
            return std::string(argv[i + 1]);
        }
        else {
            Raise(kWrongUsageError);
        }
    }
    return "";
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

void ParseArguments(int argc, char** argv, Config* config){

    // Parsing Arguments

    for (int i = 1; i < argc; i++){

        // nullptr check

        if (argv[i] == nullptr){
            Raise(kWrongUsageError);
        }

        // \0 check

        else if (argv[i][0] == '\0'){
            Raise(kWrongUsageError);
        }

        else if (argv[i][0] == '-'){

            // \0 check
            if (argv[i][1] == '\0'){
                Raise(kWrongUsageError);
            }

            // Long Flags

            else if (argv[i][1] == '-'){

                std::string flag = std::string(argv[i]);

                // --help
                if (flag == kHelpLong){
                    PrintHelp();
                }
                
                // --input
                else if(flag == kInputLong){
                    config->input_path = NextArg(argc, argv, i);
                }
    
                // --output
                else if(flag == kOutputLong){
                    config->output_path = NextArg(argc, argv, i);       
                }

                // --watermark
                else if(flag == kWatermarkLong){
                    config->watermark_path = NextArg(argc, argv, i);    
                }

                // Unknown flag exeption
                else {
                    Raise(kUnknownFlagError); 
                }
            }

            // Sort Flags

            else{
                switch(argv[i][1]){
                    // -h
                    case(kHelpShort):
                        PrintHelp();
                        break;

                    // -i
                    case(kInputShort):
                        config->input_path = NextArg(argc, argv, i);
                        break;

                    // -o
                    case(kOutputShort):
                        config->output_path = NextArg(argc, argv, i);
                        break;

                    // -w
                    case(kWatermarkShort):
                        config->watermark_path = NextArg(argc, argv, i);
                        break;
        
                    // Unknown flag exeption
                    default:
                        Raise(kUnknownFlagError);
                        break;
                }
            }
        }
    }

    // Validity check
    
    if (config->input_path.empty()){
        Raise(kWrongArgumentError);
    }
    if (config->output_path.empty()){
        Raise(kWrongArgumentError);
    }
    if (config->watermark_path.empty()){
        Raise(kWrongArgumentError);
    }
}

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
