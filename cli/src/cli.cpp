#include <iostream>
#include <string>
#include <iomanip>
#include <bmplib.h>
#include <stdlib.h>
#include "globals.h"
#include "cli.h"
#include <vector>
#include <string.h>

/*

   Terminates the program execution immediately, prints the specified error message to the error stream, and exits with the provided status code.

    Arguments:
     const char* error  - The error message to be displayed
     int exit_code      - The exit status code.

*/

void Raise(const Error error){
    std::cerr << "ERROR: " << error.error_message << "\n";
    std::cerr << " Try 'cftw-cli --help' for more information\n"; // TODO: Make a Error type dependent

    exit(error.error_code);
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
    
    std::cout << " " << std::left << std::setw(50) << "-i, --input <path>" << "Path to the source BMP image\n";
    std::cout << " " << std::left << std::setw(50) << "-w, --watermark <path>" << "Path to the BMP watermark\n";
    std::cout << " " << std::left << std::setw(50) << "-o, --output <path>" << "Path to the output BMP image\n";

    std::cout << "\nOptional flags:\n";

    std::cout << " " << std::left << std::setw(50) << "-s, --size <width> [<height>]" << "Set dimensions. If height is omitted, defaults to width (square).\n";
    std::cout << " " << std::left << std::setw(50) << "-p, --padding <x-padding> [<y-padding>]" << "Set padding. If y-padding is omitted, defaults to x-padding.\n";
    std::cout << " " << std::left << std::setw(50) << "-c, --corner <position>" << "Set corner position.\n";
    std::cout << " " << std::left << std::setw(50) << "" << "Valid values: left-top, left-bottom, right-top, right-bottom.\n";
    std::cout << " " << std::left << std::setw(50) << "-t, --tile <x> [<y>]" << "Tile pattern <x> times horizontally and <y> times vertically.\n";
    std::cout << " " << std::left << std::setw(50) << "" << "If y in omitted, defaults to x.\n";
    std::cout << " " << std::left << std::setw(50) << "-h, --help" << "Display detailed description of configuration parameters and exit\n";

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

char* WrongUsageHandler(char* arg){
    if (arg == nullptr) {
        Raise(kWrongUsageError);
    }
    return arg;
}

char* NextArg(int argc, char** argv, int i){
    if (i + 1 >= argc){
        return nullptr;
    }else{
        if (argv[i + 1][0] != '-'){
            return argv[i + 1];
        }
        return nullptr;
    }
}

unsigned long ToInt(char* arg) {
    int i = 0;
    while (arg[i] != '\0') {
        if (arg[i] < '0' || arg[i] > '9') {
            Raise(kWrongArgumentError);
        }
        i++;
    }
    return std::stoi(arg);
}

/*

*/


int ParseCorner(char* arg){
    static const std::vector<std::pair<char*, int>> kMap = { 
        {(char*)"left-top",     0},
        {(char*)"left-bottom",  1},
        {(char*)"right-top",    2},
        {(char*)"right-bottom", 3}
    };

    for (const auto& [name, corner] : kMap) {
        if (strcmp(arg, name) == 0) {
            return corner;
        }
    }

    Raise(kWrongCornerError);
    return -1;
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
                Raise(kEmptyFlagError);
            }

            // Long Flags

            else if (argv[i][1] == '-'){
                if (argv[i][2] == '\0') {
                    Raise(kEmptyFlagError);
                }

                std::string flag = std::string(argv[i]);

                // --help
                if (flag == kHelpLong){
                    PrintHelp();
                }
                
                // --input
                else if(flag == kInputLong){
                    config->input_path = WrongUsageHandler(NextArg(argc, argv, i));
                }
    
                // --output
                else if(flag == kOutputLong){
                    config->output_path = WrongUsageHandler(NextArg(argc, argv, i));
                }

                // --watermark
                else if(flag == kWatermarkLong){
                    config->watermark_path = WrongUsageHandler(NextArg(argc, argv, i));    
                }

                // --size
                else if (flag == kSizeLong){
                    config->width = ToInt(WrongUsageHandler(NextArg(argc, argv, i)));
                    char* arg2 = NextArg(argc, argv, i + 1);
                    if (arg2 == nullptr) {
                        config->height = config->width;
                    }else {
                        config->height = ToInt(WrongUsageHandler(arg2));
                    }
                }

                // --padding
                else if (flag == kPaddingLong){
                    config->x_padding = ToInt(WrongUsageHandler(NextArg(argc, argv, i)));
                    char* arg2 = NextArg(argc, argv, i + 1);
                    if (arg2 == nullptr) {
                        config->y_padding = config->x_padding;
                    }else {
                        config->y_padding = ToInt(WrongUsageHandler(arg2));
                    }
                }

                // --corner
                else if(flag == kCornerLong){
                    config->corner = ParseCorner(WrongUsageHandler(NextArg(argc, argv, i)));
                }

                // --tile
                else if (flag == kTileLong){
                    config->x_tile = ToInt(WrongUsageHandler(NextArg(argc, argv, i)));
                    char* arg2 = NextArg(argc, argv, i + 1);
                    if (arg2 == nullptr) {
                        config->y_tile = config->x_tile;
                    }else {
                        config->y_tile = ToInt(WrongUsageHandler(arg2));
                    }
                }

                // Unknown flag exeption
                else {
                    Raise(kUnknownFlagError); 
                }
            }

            // Sort Flags

            else{
                int j = 1;

                while (argv[i][j] != '\0'){
                    switch(argv[i][j]){
                        // -h
                        case(kHelpShort):
                            PrintHelp();
                            break;
    
                        // -i
                        case(kInputShort):
                            config->input_path = WrongUsageHandler(NextArg(argc, argv, i)); 
                        break;

                        // -o
                        case(kOutputShort):
                            config->output_path = WrongUsageHandler(NextArg(argc, argv, i));
                            break;
    
                        // -w
                        case(kWatermarkShort):
                            config->watermark_path = WrongUsageHandler(NextArg(argc, argv, i));
                            break;
       
                        // -s
                        case(kSizeShort): {
                            config->width = ToInt(WrongUsageHandler(NextArg(argc, argv, i)));
                            char* arg2 = NextArg(argc, argv, i + 1);
                            if (arg2 == nullptr) {
                                config->height = config->width;
                            }else {
                                config->height = ToInt(WrongUsageHandler(arg2));
                            }
                            break;
                        }

                        // -p
                        case(kPaddingShort): {
                            config->x_padding = ToInt(WrongUsageHandler(NextArg(argc, argv, i)));
                            char* arg2 = NextArg(argc, argv, i + 1);
                            if (arg2 == nullptr) {
                                config->y_padding = config->x_padding;
                            }else {
                                config->y_padding = ToInt(WrongUsageHandler(arg2));
                            }
                            
                            break;
                        }

                        // -c
                        case(kCornerShort):
                            config->corner = ParseCorner(WrongUsageHandler(NextArg(argc, argv, i)));
                            break;

                        case(kTileShort): {
                            config->x_tile = ToInt(WrongUsageHandler(NextArg(argc, argv, i)));
                            char* arg2 = NextArg(argc, argv, i + 1);
                            if (arg2 == nullptr) {
                                config->y_tile = config->x_tile;
                            }else {
                                config->y_tile = ToInt(WrongUsageHandler(arg2));
                            }
                            break;
                        }

                        // Unknown flag exeption
                        default:
                            Raise(kUnknownFlagError);
                            break;
                    }

                    j ++;

                }
            }
        }
    }

    // Validity check
    
    if (config->input_path == NULL){
        Raise(kWrongArgumentError);
    }
    if (config->output_path == NULL){
        Raise(kWrongArgumentError);
    }
    if (config->watermark_path == NULL){
        Raise(kWrongArgumentError);
    }
    if (config->width == 0){
        Raise(kWrongSizeError);
    }
    if (config->height == 0){
        Raise(kWrongSizeError);
    }
    if (config->x_tile == 0){
        Raise(kWrongArgumentError);
    }
    if (config->y_tile == 0){
        Raise(kWrongArgumentError);
    }
}

