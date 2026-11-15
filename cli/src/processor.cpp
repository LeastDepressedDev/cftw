#include <iostream>
#include <bmplib.h>

#include "globals.h"
#include "processor.h"

int BMPprocessor(Config *config) {
    ResultBmpImage input_img = open_bmp((const char*)config->input_path);
    ResultBmpImage wm_img = open_bmp((const char*)config->watermark_path);

    if (input_img.opFlag != FLAG_OBMP_OK) {
        return input_img.opFlag;
    }
    if (wm_img.opFlag != FLAG_OBMP_OK) {
        return wm_img.opFlag;
    }

    ResultBmpImage wm2_img = shrink(wm_img.resultPtr, 200, 200);
    
    if (wm2_img.opFlag != FLAG_OBMP_OK) {
        return wm2_img.opFlag;
    }

    //ResultBmpImage result_img = merge(input_img.resultPtr, output_img.resultPtr, 0.5);
    //if (result_img.opFlag != FLAG_OBMP_OK) {
    //    return result_img.opFlag;
    //}

    //BmpImage* img = (BmpImage*)malloc(sizeof(BmpImage));
   
    std::cout << (int)((char*)&wm_img.resultPtr->ulWidth)[4] << " " << wm_img.resultPtr->wlHeight << "\n";
    std::cout << wm_img.resultPtr->ulRawDataSize << " " << wm_img.resultPtr->ulRawDataCap << "\n";

    std::cout << write_bmp(config->output_path, wm2_img.resultPtr);
    
    free_bmp(input_img.resultPtr);
    free_bmp(wm_img.resultPtr);
    free_bmp(wm2_img.resultPtr);

    return 0;
}
