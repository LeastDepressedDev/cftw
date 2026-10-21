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
    
    res->input_path = "";
    res->output_path = "";
    res->watermark_path = "";

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

