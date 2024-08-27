# thumbcache

Napi version of [thumbcache](https://github.com/yhdgms1/thumbcache). By this library you can get windows file thumbnail of the file.

```ts
// Path to file and size of thumbnail
const thumb = getThumb("C:\\Users\\User\\Downloads\\picture.jpg", 96);

// Uint8Array of .bmp image file
console.log(thumb)
```