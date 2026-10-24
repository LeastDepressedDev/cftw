#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Margin {
    int left, top;
    int right, bottom;
} Margin;

typedef struct Pixel {
    unsigned char r;
    unsigned char g;
    unsigned char b;
} Pixel;

typedef struct BmpImage {
    Pixel* ptrRawData; // array of RGBA with length = uiWidth*uiHeight
    unsigned long ulRawDataSize; // amount of bytes stored in ptrRawData

    unsigned int uiWidth; // Width in pixels
    unsigned int uiHeight; // Height in pixels
} BmpImage;


//
// Results
//

/**
* Abstract results

    typedef struct ResultT* {
        unsigned char opFlag; // Operating result flag
        char* ptr; // Additional info ptr. If nullptr - no additional info. If not nullptr - then it is a \0 terminated string.
    } ResultBmpImage;

*/

/**
* Result, involving returning a nothing.
*/
typedef struct ResultVoid {
    unsigned char opFlag;
    char* ptr;
} ResultVoid;

/**
* Result, involving returning a BmpImage.
*/
typedef struct ResultBmpImage {
    unsigned char opFlag;
    const char* ptr;
    BmpImage* resultPtr; // Null on error
} ResultBmpImage;

/**
* Result, involving returning a RGBA.
*/
typedef struct ResultPixel {
    Pixel result; // If opFlag indicates failure - reading and using result is Undefined Behaviour
    unsigned char opFlag;
} ResultPixel;



#define FLAG_OBMP_OK 0 // Everything is fine - resultPtr is guaranteed to be non nullptr.
#define FLAG_OBMP_WMN 1 // Wrong magic numbers - resultPtr is guaranteed to be nullptr.
#define FLAG_OBMP_UNSUPPORTED 2 // Unsupported - resultPtr is guaranteed to be nullptr. Additional info contains details about what exactly is unsupported.
#define FLAG_OBMP_IO 3 // IO error - resultPtr is guaranteed to be nullptr. Additional info contains details about what exactly happend.

/**
*
* Reads file from fs and constructs BmpImage instance in heap.
*
* # Arguments
*   path - '\0' terminated absolute or relative path.
* # Returns
*   ResultBmpImage - op flags defined with FLAG_OBMP_*
*
*
* # Note
*   Path is not not guaranteed library side check required.
*/
extern ResultBmpImage open_bmp(const char* path);

/**
*
* Writes BmpImage instance as bmp file according to `path`.
*
* # Arguments
*   path - '\0' terminated absolute or relative path.
*   img - pointer to instance of BmpImage.
*
*
*
* # Note
*   Path is not not guaranteed library side check required.
*   img is guaranteed to be a correct BmpImage instance.
*/
extern const char* write_bmp(const char* path, BmpImage* img);

/**
*
* Converts 2D coordinate into 1D coordinate using width and height of given BmpImage.
*
* # Argmuents
*   img - pointer to the BmpImage instance
*   x, y - coordinates of the pixel
* # Returns
*   1D coordinate of corresponding pixel in ptrRawData
*
*
* # Note
*   img is guaranteed to be valid BmpImages and non nullptr.
*   x and y not guaranteed to be valid, library side check required
*
*   returns (unsigned int)::max if outside image
*/
extern unsigned int index(BmpImage* img, unsigned int x, unsigned int y);

/**
*
* Safely reads pixel from given BmpImage instance with given x and y coordinate.
*
* # Argumnets
*   img - pointer to the BmpImage instance
*   x, y - coordinates of the pixel
*
*
* # Note
*   x and y not guaranteed to be valid, library side check required.
*   img is guaranteed to be valid BmpImages and non nullptr.
*   Implementation of this function must relay on `unsigned long index(BmpImage* img, unsigned int x, unsigned int y)`
*/
extern ResultPixel read_pixel(BmpImage* img, unsigned int x, unsigned int y);

/**
*
* Safely sets pixel in given BmpImage instance with given x and y coordinate.
*
* # Argumnets
*   img - pointer to the BmpImage instance
*   x, y - coordinates of the pixel
*
*
* # Note
*   x and y not guaranteed to be valid, library side check required.
*   img is guaranteed to be valid BmpImages and non nullptr.
*   Implementation of this function must relay on `unsigned long index(BmpImage* img, unsigned int x, unsigned int y)`
*/
extern unsigned char set_pixel(BmpImage* img, unsigned int x, unsigned int y, Pixel color);

/**
*
* Creates BmpImage instance in heap and merges img1 and img2 pixel by pixel with blend coefficient `blend`.
*
* # Arguments
*   img1, img2 - pointers to the BmpImage instances
*   blend - blending coefficient(decides the impact of each img1 and img2 on resulting image)
*
* # Returns
*   pointer to BmpImage, result of merge
*
*
* # Note
*   img1 and img2 are guaranteed to be valid BmpImages and non nullptr.
*   img1 and img2 must not be changed by the merge function behaviour.
*/
extern ResultBmpImage merge(const BmpImage* img1, const BmpImage* img2, float blend);

/**
*
* Creates BmpImage instance in heap and apply margin.
* Margin consists of left, top, right, bottom.
* For example, if we have margin { 0, -50, 20, 0} that means we need a resulting image to be expanded 50 pixel up, -20 pixel from right to left.
*
* # Arguments
*   img - pointer to the BmpImage intance
*   marg - margin structure
*   fill - color that will be placed, if new image area > old image area
*
* # Returns
*   pointer to BmpImage, result of margin
*
*
* # Note
*   img is guaranteed to be valid BmpImages and non nullptr.
*   img must not be changed by the merge function behaviour.
*/
extern ResultBmpImage margin(const BmpImage* img, Margin marg, Pixel fill);

/**
*
* Creates BmpImage instance in heap and apply shrink.
* Shrink - mean make it lower in width and height, but keep the shape. Like, shittify the quality.
*
* # Arguments
*   img - pointer to the BmpImage intance
*   newWidth, newHeight - new dimensions of image
*
* # Returns
*   pointer to BmpImage, result of margin
*
*
* # Note
*   img is guaranteed to be valid BmpImages and non nullptr.
*   img must not be changed by the merge function behaviour.
*/
extern ResultBmpImage shrink(const BmpImage* img, unsigned int newWidth, unsigned int newHeight);

#ifdef __cplusplus
}
#endif
