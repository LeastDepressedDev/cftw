#include <iostream>
#include "globals.h"

/*

   Config constructor

   Return:
    Config with default values

*/

Config* ConfigConstructor(){
    Config* res = new Config();

    // Config default values
    
    res->input_path = nullptr;
    res->output_path = nullptr;
    res->watermark_path = nullptr;

    res->x_padding = 0;
    res->y_padding = 0;
    res->width = -1;
    res->height = -1;
    res->corner = 0;
    res->x_tile = 1;
    res->y_tile = 1;

    return res;
}

/*

   Config constructor

   Arguments:
    error_message_arg       - error message
    error_code_arg          - error code

   Return:
    Error with error message and error code

*/

Error* ErrorConstructor(char* error_message_arg, int error_code_arg){
    Error* res = new Error();

    res->error_message = error_message_arg;
    res->error_code = error_code_arg;

    return res;
}

