#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Margin {
    int32_t left, top;
    int32_t right, bottom;
} Margin;

typedef struct Pixel {
    unsigned char r;
    unsigned char g;
    unsigned char b;
} Pixel;

/*
 * Layout mirrors the Rust `BmpImage` struct exactly.
 * `size` and `capacity` are measured in Pixel elements (not bytes).
 */
typedef struct BmpImage {
    Pixel*        ptrRawData;
    unsigned long ulRawDataSize;        /* number of Pixels in ptrRawData */
    unsigned long ulRawDataCap;    /* allocated capacity, in Pixels */
    unsigned long ulWidth;       /* width in pixels */
    unsigned long wlHeight;      /* height in pixels */
    unsigned char header[12];  /* BITMAPFILEHEADER bytes */
    unsigned char dib_header[40]; /* BITMAPINFOHEADER bytes */
} BmpImage;

/* ---------------------------------------------------------------- *
 * Results
 * ---------------------------------------------------------------- */

typedef struct ResultBmpImage {
    const char*   ptr;      /* \0-terminated string on error, else NULL */
    BmpImage*     resultPtr; /* NULL on error */
    unsigned char opFlag;
} ResultBmpImage;

typedef struct ResultPixel {
    Pixel         result; /* valid only if opFlag == FLAG_OBMP_OK */
    unsigned char opFlag;
} ResultPixel;

#define FLAG_OBMP_OK          0
#define FLAG_OBMP_WMN         1 /* wrong magic numbers */
#define FLAG_OBMP_UNSUPPORTED 2
#define FLAG_OBMP_IO          3

/* ---------------------------------------------------------------- *
 * API
 * ---------------------------------------------------------------- */

/*
 * Reads a BMP file and constructs a BmpImage on the heap.
 * Returns a ResultBmpImage. On success, `resultPtr` is non-null and
 * must be released with `free_bmp`.
 * On error `ptr` (if non-null) must be released with `free_string`.
 * # Throws (C side)
 *   Does not throw. Never returns NULL `resultPtr` on success.
 */
extern ResultBmpImage open_bmp(const char* path);

/*
 * Writes `img` to `path`.
 * Returns NULL on success, or a heap-allocated \0-terminated error
 * message on failure. Caller must free the returned string with
 * `free_string`.
 */
extern const char* write_bmp(const char* path, BmpImage* img);

/*
 * 2D → 1D index. Returns UINT32_MAX when (x, y) is out of bounds.
 */
extern unsigned int index(BmpImage* img, unsigned int x, unsigned int y);

/*
 * Safely reads a pixel. On failure returns tag=1 (result is meaningless).
 */
extern ResultPixel read_pixel(BmpImage* img, unsigned int x, unsigned int y);

/*
 * Safely sets a pixel. Returns non-zero on success.
 */
extern unsigned char set_pixel(BmpImage* img, unsigned int x, unsigned int y, Pixel color);

/*
 * Blend img1 and img2: result = img1 * (1 - blend) + img2 * blend.
 * Dimensions must match. Result must be freed with `free_bmp`.
 * # Throws (C side)
 *   Does not throw.
 */
extern ResultBmpImage merge(const BmpImage* img1, const BmpImage* img2, float blend);

/*
 * Applies a margin.
 *   positive value = crop from that side,
 *   negative value = expand from that side.
 * e.g. {0, -50, 20, 0} => expand 50 rows at top, crop 20 cols at right.
 * Fill color is used for newly-exposed pixels. Result must be freed with
 * `free_bmp`.
 */
extern ResultBmpImage margin(const BmpImage* img, Margin marg, Pixel fill);

/*
 * Rescales the image to (newWidth, newHeight) with a box filter.
 * Result must be freed with `free_bmp`.
 */
extern ResultBmpImage shrink(const BmpImage* img, unsigned int newWidth, unsigned int newHeight);

/* ---------------------------------------------------------------- *
 * Memory management
 * ---------------------------------------------------------------- */

/*
 * Frees a BmpImage previously returned by open_bmp / merge / margin /
 * shrink. NULL is a no-op.
 */
extern void free_bmp(BmpImage* img);

/*
 * Frees a string previously returned by write_bmp or by the `ptr`
 * field of a failing ResultBmpImage. NULL is a no-op.
 */
extern void free_string(const char* s);

#ifdef __cplusplus
}
#endif
