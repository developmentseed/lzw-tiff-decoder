import { decompress as wasmDecompress } from '../Cargo.toml';

export function decompress(src: Uint8Array, max_uncompressed_size: number): Uint8Array {
  const decoded = wasmDecompress(src, max_uncompressed_size);

  if (decoded.length === 0) {
    throw Error("Failed to decode with LZW decoder.");
  }

  return decoded;
}
