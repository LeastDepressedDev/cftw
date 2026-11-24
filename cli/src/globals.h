#ifndef GLOBALS_H
#define GLOBALS_H

/*

    Struct for congig

    string input_path        - path to input file
    string output_path       - path to output file
    string watermark_path    - path to watermark

*/

struct Config {
    char* input_path;
    char* output_path;
    char* watermark_path;
    unsigned long width;
    unsigned long height;
    int corner;
    unsigned long x_padding;
    unsigned long y_padding;
    unsigned long x_tile;
    unsigned long y_tile;
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

#endif
