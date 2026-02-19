## `lzw-tiff-decoder`
[![npm version](https://badge.fury.io/js/lzw-tiff-decoder.svg)](https://badge.fury.io/js/lzw-tiff-decoder)

A WASM-based LZW decoder for tiff images. Uses [`weezl`](https://github.com/image-rs/lzw), a
purely safe and dependency-less Rust crate providing LZW decoding.

## Installation

```bash
$ npm install lzw-tiff-decoder
```

## Usage

```javascript
import { decompress } from 'lzw-tiff-decoder';

const compressedBytes = new Uint8Array(/* tile or strip from tiff */);
const maxUncompressedSize = tileWidth * tileHeight * bitsPerSample / 8;

const decoded = await decompress(compressedBytes, maxUncompressedSize);
```

## Development

```bash
$ cd js && npm install
$ npm run build
```

## Publish

```
cd js
npm publish
```
