declare module "*.toml" {
  export function decompress(src: Uint8Array, max_uncompressed_size: number): Uint8Array;
}
