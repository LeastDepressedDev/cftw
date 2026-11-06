#pragma once

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

   Config constructor

   Return:
    Config with default values

*/

extern Config* ConfigConstructor();

/*

    Struct for errors

    char* error_message     - error message
    int error_code          - error code

*/

struct Error {
    char* error_message;
    int error_code;
};

/*

   Config constructor

   Arguments:
    error_message_arg       - error message
    error_code_arg          - error code

   Return:
    Error with error message and error code

*/

extern Error* ErrorConstructor(char* error_message_arg, int error_code_arg);
