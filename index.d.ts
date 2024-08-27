/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/**
 * Will find thumbnail for provided file
 * @example
 * ```ts
 * const thumb = getThumb("C:\\Users\\User\\Downloads\\picture.jpg", 96);
 *
 * if (thumb) {
 *   console.log(thumb) // Uint8Array
 * }
 * ```
 */
export declare function getThumb(path: string, size: 16 | 32 | 48 | 96 | 256 | 768 | 1280 | 1920 | 2560 | (number & {})): Uint8Array | null
