## `lzw-tiff-decoder`

[![npm version](https://badge.fury.io/js/@developmentseed%2Flzw-tiff-decoder.svg)](https://badge.fury.io/js/@developmentseed%2Flzw-tiff-decoder)

A WASM-based LZW decoder for tiff images. Uses [`weezl`](https://github.com/image-rs/lzw), a
purely safe and dependency-less Rust crate providing LZW decoding.

The full bundle size is **15kb**.

## Installation

```bash
npm install @developmentseed/lzw-tiff-decoder
```

## Usage

```javascript
import { decompress } from '@developmentseed/lzw-tiff-decoder';

const compressedBytes = new Uint8Array(/* tile or strip from tiff */);
const maxUncompressedSize = tileWidth * tileHeight * bitsPerSample / 8;

const decoded = await decompress(compressedBytes, maxUncompressedSize);
```

## Development

```bash
npm install
npm run build
```

## Publish

```bash
npm publish
```
