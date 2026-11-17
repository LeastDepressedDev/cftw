#include <iostream>
#include "../include/bmplib.h"

#include "globals.h"
#include "processor.h"

int BMPprocessor(Config *config) {
    ResultBmpImage input_img = open_bmp((const char*)config->input_path);
    ResultBmpImage wm_img = open_bmp((const char*)config->watermark_path);

    if (input_img.opFlag != FLAG_OBMP_OK) {
        std::cout << input_img.ptr << std::endl;
        free_string(input_img.ptr);
        return input_img.opFlag;
    }
    if (wm_img.opFlag != FLAG_OBMP_OK) {
        std::cout << wm_img.ptr << std::endl;
        free_string(wm_img.ptr);
        return wm_img.opFlag;
    }

    ResultBmpImage wm2_img = shrink(wm_img.resultPtr, input_img.resultPtr->ulWidth, input_img.resultPtr->ulHeight);

    if (wm2_img.opFlag != FLAG_OBMP_OK) {
        std::cout << wm2_img.ptr << std::endl;
        free_string(wm2_img.ptr);
        return wm2_img.opFlag;
    }

    ResultBmpImage result_img = merge(input_img.resultPtr, wm2_img.resultPtr, 0.5);
    if (result_img.opFlag != FLAG_OBMP_OK) {
        std::cout << result_img.ptr << std::endl;
        free_string(result_img.ptr);
        return result_img.opFlag;
    }

    write_bmp(config->output_path, result_img.resultPtr);

    free_bmp(input_img.resultPtr);
    free_bmp(wm_img.resultPtr);
    free_bmp(wm2_img.resultPtr);
    free_bmp(result_img.resultPtr);

    return 0;
}
