#pragma once

#include <iostream>
#include <string>
#include <iomanip>
#include <bmplib.h>
#include <stdlib.h>
#include "globals.h"

// errors
const Error kUnknownFlagError          = {(char*)"unknown flag", -1};
const Error kWrongUsageError           = {(char*)"wrong usage", -2};
const Error kWrongArgumentError        = {(char*)"wrong argument", -3};

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

extern std::string NextArg(int argc, char** argv, int i);

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
