import { execFile } from "node:child_process";
import { createHash } from "node:crypto";
import { createReadStream, createWriteStream } from "node:fs";
import { mkdir, mkdtemp, realpath, rename, rm, stat, writeFile } from "node:fs/promises";
import { delimiter, dirname, extname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { Readable, Transform } from "node:stream";
import { pipeline } from "node:stream/promises";
import { ReadableStream } from "node:stream/web";

export const DOC42_VERSION = "0.8.1";
export const DOC42_INSTALL_GUIDE = "https://elan8.github.io/doc42-release/installation";

const releaseUrl = `https://github.com/elan8/doc42-release/releases/download/v${DOC42_VERSION}`;
const packageLimit = 160 * 1024 * 1024;
const executableName = process.platform === "win32" ? "doc42.exe" : "doc42";

export const doc42Package = process.arch !== "x64" ? undefined : {
  win32: `doc42-${DOC42_VERSION}-windows-x64.zip`,
  linux: `doc42-${DOC42_VERSION}-linux-x64.tar.gz`,
}[process.platform as "win32" | "linux"];

export interface Doc42Installation {
  path: string;
  version: string;
}

export interface Doc42Document {
  qualified_name: string;
  title?: string | null;
  subtitle?: string | null;
  number?: string | null;
  revision?: string | null;
}

export function doc42DocumentLabel(document: Doc42Document): string {
  return document.subtitle?.trim() || document.title?.trim() || document.qualified_name;
}

export interface Doc42Diagnostic {
  message: string;
  code: string;
  severity: "error" | "warning" | "information";
  location?: { uri: string; line: number; column: number };
}

interface BuildManifest {
  documents: (Doc42Document & { file: string })[];
  errors: number;
  warnings: number;
}

export interface ProcessResult {
  stdout: string;
  stderr: string;
  exitCode: number;
}

export function containsPath(root: string, file: string): boolean {
  const name = relative(root, file);
  return !isAbsolute(name) && name !== ".." && !name.startsWith(`..${sep}`);
}

export function resolveDoc42LibraryPaths(root: string, editorPaths: string[], doc42Paths: string[] | null): string[] {
  return (doc42Paths ?? editorPaths).filter(path => path.trim().length > 0).map(path => resolve(root, path));
}

export function runDoc42(
  executable: string,
  args: string[],
  options: { cwd?: string; signal?: AbortSignal; timeout?: number } = {},
): Promise<ProcessResult> {
  const { signal, ...processOptions } = options;
  return new Promise((resolve, reject) => {
    signal?.throwIfAborted();
    const child = execFile(executable, args, {
      ...processOptions,
      windowsHide: true,
      maxBuffer: 16 * 1024 * 1024,
    }, (error, stdout, stderr) => {
      signal?.removeEventListener("abort", cancel);
      if (signal?.aborted) {
        reject(signal.reason);
      } else if (error && typeof error.code !== "number") {
        reject(error);
      } else {
        resolve({ stdout, stderr, exitCode: typeof error?.code === "number" ? error.code : 0 });
      }
    });
    const cancel = () => { child.kill(); };
    signal?.addEventListener("abort", cancel, { once: true });
    child.stdin?.on("error", () => {
      // The exit callback reports failures if the process closes stdin early.
    });
    child.stdin?.end();
  });
}

export function parseDoc42Json<T>(result: ProcessResult, acceptedCodes = [0]): T {
  if (!acceptedCodes.includes(result.exitCode)) {
    throw new Error(result.stderr.trim() || `Doc42 exited with code ${result.exitCode}.`);
  }
  return JSON.parse(result.stdout) as T;
}

export async function readBuiltPdfs(output: ProcessResult, root: string, run: string, names: string[]) {
  const manifest = parseDoc42Json<BuildManifest>(output);
  const remaining = new Set(names);
  if (manifest.errors !== 0 || !names.length || manifest.documents.length !== names.length
    || !manifest.documents.every(document => remaining.delete(document.qualified_name)) || remaining.size) {
    throw new Error("Doc42 did not return a successful manifest for the selected documents.");
  }
  const seen = new Set<string>();
  const outputDirectory = await realpath(run);
  const documents = [];
  for (const document of manifest.documents) {
    const file = resolve(root, document.file);
    if (dirname(file) !== run || extname(file).toLowerCase() !== ".pdf") {
      throw new Error("Doc42 returned a PDF path outside the build output or with an invalid extension.");
    }
    const actualFile = await realpath(file);
    if (!containsPath(outputDirectory, actualFile) || seen.has(actualFile) || !(await stat(actualFile)).isFile()) {
      throw new Error("Doc42 returned an invalid or duplicate PDF.");
    }
    seen.add(actualFile);
    documents.push({ ...document, file });
  }
  return { documents, warnings: manifest.warnings };
}

async function readVersion(executable: string, signal: AbortSignal): Promise<string | undefined> {
  try {
    const file = await stat(executable);
    if (!file.isFile()) return undefined;
    const result = await runDoc42(executable, ["--version"], { signal, timeout: 5000 });
    return result.exitCode === 0 ? /^doc42 (\d+\.\d+\.\d+)\s*$/.exec(result.stdout)?.[1] : undefined;
  } catch {
    signal.throwIfAborted();
    return undefined;
  }
}

export async function findDoc42(storage: string, workspaceRoots: string[], signal: AbortSignal) {
  const managed = join(storage, DOC42_VERSION, executableName);
  const external = (process.env.PATH ?? "").split(delimiter)
    .filter(isAbsolute)
    .map(directory => join(directory, executableName))
    .filter(executable => !workspaceRoots.some(root => containsPath(root, executable)));
  let incompatible: Doc42Installation | undefined;
  for (const executable of new Set([managed, ...external])) {
    signal.throwIfAborted();
    const version = await readVersion(executable, signal);
    if (version === DOC42_VERSION) return { compatible: { path: executable, version }, incompatible };
    if (version && !incompatible) incompatible = { path: executable, version };
  }
  return { compatible: undefined, incompatible };
}

export function releaseChecksum(checksums: string, archive: string): string {
  const entries = checksums.split(/\r?\n/)
    .map(line => /^([a-f\d]{64}) [ *](.+)$/i.exec(line))
    .filter(entry => entry?.[2] === archive);
  if (entries.length !== 1) throw new Error(`Expected one published checksum for ${archive}.`);
  return entries[0]![1].toLowerCase();
}

async function download(asset: string, signal: AbortSignal): Promise<Response> {
  const response = await fetch(`${releaseUrl}/${asset}`, {
    signal: AbortSignal.any([signal, AbortSignal.timeout(180_000)]),
  });
  if (!response.ok || !response.body) throw new Error(`Could not download ${asset} (HTTP ${response.status}).`);
  return response;
}

async function extractExecutable(archive: string, signal: AbortSignal): Promise<Buffer> {
  // Stream a single fixed archive member to stdout. The archive never chooses an output path.
  const tar = process.platform === "win32" && process.env.SystemRoot
    ? join(process.env.SystemRoot, "System32", "tar.exe") : "tar";
  return new Promise((resolve, reject) => {
    execFile(tar, ["-xOf", archive, executableName], {
      cwd: dirname(archive),
      encoding: "buffer",
      windowsHide: true,
      maxBuffer: packageLimit,
      signal,
    }, (error, stdout) => {
      if (error) reject(error);
      else resolve(stdout);
    });
  });
}

export async function installDoc42(
  storage: string, signal: AbortSignal, report: (message: string) => void,
): Promise<void> {
  if (!doc42Package) throw new Error("This platform has no managed Doc42 package.");
  await mkdir(storage, { recursive: true });
  const staging = await mkdtemp(join(storage, ".install-"));
  try {
    const archive = join(staging, doc42Package);
    const checksums = await download("SHA256SUMS.txt", signal);
    const expectedHash = releaseChecksum(await checksums.text(), doc42Package);
    const response = await download(doc42Package, signal);
    let received = 0;
    const limitSize = new Transform({
      transform(chunk: Buffer, _encoding, callback) {
        received += chunk.length;
        if (received > packageLimit) callback(new Error("The Doc42 download exceeded the package size limit."));
        else callback(null, chunk);
      },
    });
    await pipeline(
      Readable.fromWeb(response.body as ReadableStream<Uint8Array>),
      limitSize,
      createWriteStream(archive, { flags: "wx" }),
      { signal },
    );
    report("Verifying the release…");
    const hash = createHash("sha256");
    for await (const chunk of createReadStream(archive, { signal })) hash.update(chunk);
    if (hash.digest("hex") !== expectedHash) throw new Error("The Doc42 archive did not match its published checksum.");

    const candidate = join(staging, executableName);
    await writeFile(candidate, await extractExecutable(archive, signal), { flag: "wx", mode: 0o755 });
    if (await readVersion(candidate, signal) !== DOC42_VERSION) {
      throw new Error("The downloaded Doc42 executable could not report the expected version.");
    }
    const destination = join(storage, DOC42_VERSION);
    await mkdir(destination, { recursive: true });
    signal.throwIfAborted();
    const installed = join(destination, executableName);
    try {
      await rename(candidate, installed);
    } catch (error) {
      // Another VS Code window may have finished installing the same release.
      if (await readVersion(installed, signal) !== DOC42_VERSION) throw error;
    }
  } finally {
    await rm(staging, { recursive: true, force: true });
  }
}
