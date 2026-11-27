import { strict as assert } from "node:assert";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import {
  doc42DocumentLabel, parseDoc42Json, readBuiltPdfs, releaseChecksum, resolveDoc42LibraryPaths, runDoc42,
} from "../../elan8Tools/doc42Cli";

test("Doc42 uses the individual document subtitle when documents share a title", () => {
  const document = { qualified_name: "RobotDocuments::crs", title: "Autonomous Floor Cleaning Robot" };
  assert.equal(doc42DocumentLabel({ ...document, subtitle: "Customer Requirements Specification" }),
    "Customer Requirements Specification");
  assert.equal(doc42DocumentLabel({ ...document, subtitle: "System Requirements Specification" }),
    "System Requirements Specification");
  assert.equal(doc42DocumentLabel({ ...document, subtitle: null }), document.title);
  assert.equal(doc42DocumentLabel({ qualified_name: document.qualified_name }), document.qualified_name);
});

test("Doc42 can omit the editor's library roots while retaining an explicit override", () => {
  const root = resolve("project");
  const editorPaths = ["../domain/model"];
  assert.deepEqual(resolveDoc42LibraryPaths(root, editorPaths, null), [resolve(root, editorPaths[0])]);
  assert.deepEqual(resolveDoc42LibraryPaths(root, editorPaths, []), []);
  assert.deepEqual(resolveDoc42LibraryPaths(root, editorPaths, ["extra/model"]), [join(root, "extra", "model")]);
});

test("selects only the checksum for the exact release asset", () => {
  const hash = "a".repeat(64);
  const name = "doc42-0.8.1-windows-x64.zip";
  const checksums = `${"b".repeat(64)}  ${name}.backup\r\n${hash.toUpperCase()} *${name}\r\n`;
  assert.equal(releaseChecksum(checksums, name), hash);
  assert.throws(() => releaseChecksum(checksums, "doc42-0.8.1-linux-x64.tar.gz"));
  assert.throws(() => releaseChecksum(checksums.repeat(2), name));
});

test("accepts failed validation JSON when requested and keeps process failures distinct", async () => {
  const result = await runDoc42(process.execPath, ["-e", "console.log(JSON.stringify({ errors: 1 })); process.exitCode = 1"]);
  assert.deepEqual(parseDoc42Json(result, [0, 1]), { errors: 1 });
  assert.throws(() => parseDoc42Json(result), /code 1/);
  const failed = await runDoc42(process.execPath, ["-e", "console.error('render failed'); process.exitCode = 2"]);
  assert.throws(() => parseDoc42Json(failed, [0, 1]), /render failed/);
});

test("cancels an active CLI process", async () => {
  const controller = new AbortController();
  const result = runDoc42(process.execPath, ["-e", "setTimeout(() => {}, 10_000)"], { signal: controller.signal });
  controller.abort();
  await assert.rejects(result, { name: "AbortError" });
});

test("accepts a batch only when it produced distinct PDFs for every selected document", async () => {
  const run = await mkdtemp(join(tmpdir(), "spec42-doc42-"));
  try {
    const root = dirname(run);
    const file = join(run, "FIRST-001.pdf");
    const secondFile = join(run, "SECOND-002.pdf");
    const first = { file, qualified_name: "ProductDocs::first", title: "First document" };
    const second = { file: secondFile, qualified_name: "ProductDocs::second", title: "Second document" };
    const names = [first.qualified_name, second.qualified_name];
    await writeFile(file, "%PDF-1.7\n");
    await writeFile(secondFile, "%PDF-1.7\n");
    const output = (documents: (typeof first)[], errors = 0) => ({
      stdout: JSON.stringify({ documents, errors, warnings: 1 }),
      exitCode: 0,
      stderr: "",
    });
    assert.deepEqual(await readBuiltPdfs(output([first]), root, run, [first.qualified_name]), {
      documents: [first], warnings: 1,
    });
    assert.deepEqual(await readBuiltPdfs(output([second, first]), root, run, names), {
      documents: [second, first], warnings: 1,
    });
    await assert.rejects(readBuiltPdfs(output([first]), root, run, names), /selected documents/);
    await assert.rejects(readBuiltPdfs(output([first, first]), root, run, names), /selected documents/);
    await assert.rejects(readBuiltPdfs(output([first, second], 1), root, run, names), /successful manifest/);
    await assert.rejects(readBuiltPdfs(output([first, { ...second, file }]), root, run, names), /duplicate PDF/);
    await assert.rejects(readBuiltPdfs(output([first, { ...second, file: join(root, "outside.pdf") }]), root, run, names),
      /outside the build output/);
  } finally {
    await rm(run, { recursive: true, force: true });
  }
});
