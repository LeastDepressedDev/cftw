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
        free_bmp(input_img.resultPtr);
        return input_img.opFlag;
    }
    if (wm_img.opFlag != FLAG_OBMP_OK) {
        std::cout << wm_img.ptr << std::endl;
        free_string(wm_img.ptr);
        free_bmp(input_img.resultPtr);
        free_bmp(wm_img.resultPtr);
        return wm_img.opFlag;
    }

    unsigned long result_width = input_img.resultPtr->ulWidth;
    unsigned long result_height = input_img.resultPtr->ulHeight;

    if (config->width != 0 && config->height != 0) {
        result_width = config->width;
        result_height = config->height;
    }

    ResultBmpImage wm_sized_img = shrink(wm_img.resultPtr, result_width, result_height);

    if (wm_sized_img.opFlag != FLAG_OBMP_OK) {
        std::cout << wm_sized_img.ptr << std::endl;
        free_string(wm_sized_img.ptr);
        free_bmp(input_img.resultPtr);
        free_bmp(wm_img.resultPtr);
        free_bmp(wm_sized_img.resultPtr);
        return wm_sized_img.opFlag;
    }

    int32_t top_padding = config->y_padding;
    int32_t left_padding = config->x_padding;
    int32_t bottom_padding = input_img.resultPtr->ulHeight - result_height - top_padding;
    int32_t right_padding = input_img.resultPtr->ulWidth - result_width - left_padding;

    if (bottom_padding < 0 || right_padding < 0) {
        return -1;
    }

    Margin padding {0, 0, 0, 0};
    Pixel pixel = {0, 0, 0};

    if (config->corner == 0) {
        padding = {-left_padding, -bottom_padding, -right_padding, -top_padding};
    } else if (config->corner == 1) {
        padding = {-left_padding, -top_padding, -right_padding, -bottom_padding};
    } else if (config->corner == 2) {
        padding = {-right_padding, -bottom_padding, -left_padding, -top_padding};
    } else if (config->corner == 3) {
        padding = {-right_padding, -top_padding, -left_padding, -bottom_padding};
    }

    ResultBmpImage wm_margin_img = margin(wm_sized_img.resultPtr, padding, pixel);

    if (wm_margin_img.opFlag != FLAG_OBMP_OK) {
        std::cout << wm_margin_img.ptr << std::endl;
        free_string(wm_margin_img.ptr);
        free_bmp(input_img.resultPtr);
        free_bmp(wm_img.resultPtr);
        free_bmp(wm_sized_img.resultPtr);
        free_bmp(wm_margin_img.resultPtr);
        return wm_margin_img.opFlag;
    }

    ResultBmpImage result_img = merge(input_img.resultPtr, wm_margin_img.resultPtr, 0.5);
    if (result_img.opFlag != FLAG_OBMP_OK) {
        std::cout << result_img.ptr << std::endl;
        free_string(result_img.ptr);
        free_bmp(input_img.resultPtr);
        free_bmp(wm_img.resultPtr);
        free_bmp(wm_sized_img.resultPtr);
        free_bmp(wm_margin_img.resultPtr);
        free_bmp(result_img.resultPtr);
        return result_img.opFlag;
    }

    write_bmp(config->output_path, result_img.resultPtr);

    free_bmp(input_img.resultPtr);
    free_bmp(wm_img.resultPtr);
    free_bmp(wm_sized_img.resultPtr);
    free_bmp(wm_margin_img.resultPtr);
    free_bmp(result_img.resultPtr);

    return 0;
}