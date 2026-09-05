// Minimal read-only ZIP reader used to materialize KPAR (KerML Project Archive) files
// without adding an npm dependency to the repo-root scripts (which otherwise use only
// node: builtins). Supports the "stored" and "deflate" compression methods, which cover
// every KPAR artifact Spec42 publishes; no zip64 support since these archives are small.
import zlib from "node:zlib";

const END_OF_CENTRAL_DIRECTORY_SIGNATURE = 0x06054b50;
const CENTRAL_DIRECTORY_FILE_HEADER_SIGNATURE = 0x02014b50;
const LOCAL_FILE_HEADER_SIGNATURE = 0x04034b50;

function findEndOfCentralDirectory(buffer) {
  const minEocdSize = 22;
  const maxCommentLength = 65535;
  const searchStart = Math.max(0, buffer.length - minEocdSize - maxCommentLength);
  for (let offset = buffer.length - minEocdSize; offset >= searchStart; offset--) {
    if (buffer.readUInt32LE(offset) === END_OF_CENTRAL_DIRECTORY_SIGNATURE) {
      return offset;
    }
  }
  throw new Error("Not a valid ZIP file: end of central directory record not found");
}

function extractLocalEntry(buffer, localHeaderOffset, compressionMethod, compressedSize) {
  const signature = buffer.readUInt32LE(localHeaderOffset);
  if (signature !== LOCAL_FILE_HEADER_SIGNATURE) {
    throw new Error(`Unexpected local file header signature at offset ${localHeaderOffset}`);
  }
  const fileNameLength = buffer.readUInt16LE(localHeaderOffset + 26);
  const extraFieldLength = buffer.readUInt16LE(localHeaderOffset + 28);
  const dataStart = localHeaderOffset + 30 + fileNameLength + extraFieldLength;
  const compressedData = buffer.subarray(dataStart, dataStart + compressedSize);
  if (compressionMethod === 0) {
    return Buffer.from(compressedData);
  }
  if (compressionMethod === 8) {
    return zlib.inflateRawSync(compressedData);
  }
  throw new Error(`Unsupported ZIP compression method: ${compressionMethod}`);
}

/**
 * Reads a ZIP archive and returns a Map from entry name to its decompressed content.
 * Directory entries (names ending in "/") are omitted.
 * @param {Buffer} buffer
 * @returns {Map<string, Buffer>}
 */
export function readZipEntries(buffer) {
  const eocdOffset = findEndOfCentralDirectory(buffer);
  const entryCount = buffer.readUInt16LE(eocdOffset + 10);
  const centralDirectoryOffset = buffer.readUInt32LE(eocdOffset + 16);

  const entries = new Map();
  let offset = centralDirectoryOffset;
  for (let i = 0; i < entryCount; i++) {
    const signature = buffer.readUInt32LE(offset);
    if (signature !== CENTRAL_DIRECTORY_FILE_HEADER_SIGNATURE) {
      throw new Error(`Unexpected central directory signature at offset ${offset}`);
    }
    const compressionMethod = buffer.readUInt16LE(offset + 10);
    const compressedSize = buffer.readUInt32LE(offset + 20);
    const fileNameLength = buffer.readUInt16LE(offset + 28);
    const extraFieldLength = buffer.readUInt16LE(offset + 30);
    const fileCommentLength = buffer.readUInt16LE(offset + 32);
    const localHeaderOffset = buffer.readUInt32LE(offset + 42);
    const fileName = buffer
      .subarray(offset + 46, offset + 46 + fileNameLength)
      .toString("utf8");

    if (!fileName.endsWith("/")) {
      entries.set(
        fileName,
        extractLocalEntry(buffer, localHeaderOffset, compressionMethod, compressedSize)
      );
    }

    offset += 46 + fileNameLength + extraFieldLength + fileCommentLength;
  }
  return entries;
}
