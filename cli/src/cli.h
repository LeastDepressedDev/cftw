#ifndef CLI_H
#define CLI_H

#include <iostream>
#include <string>
#include <iomanip>
#include "../include/bmplib.h"
#include <stdlib.h>
#include "globals.h"

// errors
const Error kUnknownFlagError           = {(char*)"unknown flag", -1};
const Error kWrongUsageError            = {(char*)"wrong usage", -2};
const Error kWrongArgumentError         = {(char*)"wrong argument", -3};
const Error kEmptyFlagError             = {(char*)"empty flag", -4};
const Error kWrongCornerError           = {(char*)"wrong argument\nAvailable options: left-top, left-bottom, right-top, right-bottom.", -3};
const Error kWrongSizeError             = {(char*)"wrong argument\nSize must be greater than 0", -3};

// long flags
const std::string kHelpLong             = "--help";
const std::string kInputLong            = "--input";
const std::string kOutputLong           = "--output";
const std::string kWatermarkLong        = "--watermark";
const std::string kSizeLong             = "--size";
const std::string kPaddingLong          = "--padding";
const std::string kCornerLong           = "--corner";
const std::string kTileLong             = "--tile";

// short flags
const char kHelpShort                   = 'h';
const char kInputShort                  = 'i';
const char kOutputShort                 = 'o';
const char kWatermarkShort              = 'w';
const char kSizeShort                   = 's';
const char kPaddingShort                = 'p';
const char kCornerShort                 = 'c';
const char kTileShort                   = 't';

extern int ParseCorner(char* arg);

/*

   Terminates the program execution immediately, prints the specified error message to the error stream, and exits with the provided status code.

    Arguments:
     const char* error  - The error message to be displayed
     int exit_code      - The exit status code.

*/

extern void Raise(const Error error);

/*

    Printing in console instruction for use cftw-cli

*/

extern void PrintHelp();

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

extern char* NextArg(int argc, char** argv, int i);

/*
    
    Command line arguments parser

    Arguments:
     int argc           - number of arguments
     char** argv        - cl arguments
     Config* config     - config

    Return:
     int exitCode       - exit code

*/

extern void ParseArguments(int argc, char** argv, Config* config);

#endif
