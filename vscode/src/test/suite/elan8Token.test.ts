import { strict as assert } from "node:assert";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { checkElan8Token, saveElan8Token } from "../../elan8Tools/elan8Token";

test("a standalone check uses the environment token before the shared saved token", async test => {
  const directory = await mkdtemp(join(tmpdir(), "spec42-elan8-token-"));
  try {
    const saved = `elan8_${"a".repeat(43)}`;
    const supplied = `elan8_${"b".repeat(43)}`;
    const environment = { ELAN8_CONFIG_DIR: directory };
    const headers: string[] = [];
    const request = test.mock.method(globalThis, "fetch", async (url: string, options: RequestInit) => {
      assert.equal(url, "https://api.elan8.com/api/v1/token/license");
      assert.equal(options.method, "POST");
      assert.equal(options.redirect, "error");
      headers.push(new Headers(options.headers).get("authorization") ?? "");
      return new Response(null, { status: 200 });
    });
    const signal = new AbortController().signal;
    assert.equal(await checkElan8Token(signal, environment), "missing");
    assert.equal(request.mock.callCount(), 0);
    await writeFile(join(directory, "token"), `${saved}\n`);
    assert.equal(await checkElan8Token(signal, environment), "valid");
    assert.equal(await checkElan8Token(signal, { ...environment, ELAN8_TOKEN: supplied }), "valid");
    assert.equal(await checkElan8Token(signal, { ...environment, ELAN8_TOKEN: "malformed" }), "invalid");
    assert.deepEqual(headers, [`Bearer ${saved}`, `Bearer ${supplied}`]);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test("a rejected token, missing entitlement, and an unreachable server remain distinguishable", async test => {
  const environment = { ELAN8_TOKEN: `elan8_${"c".repeat(43)}` };
  const signal = new AbortController().signal;
  const request = test.mock.method(globalThis, "fetch", async () => new Response(null, { status: 401 }));
  assert.equal(await checkElan8Token(signal, environment), "invalid");
  request.mock.mockImplementation(async () => Response.json({ error: { code: "no_license" } }, { status: 403 }));
  assert.equal(await checkElan8Token(signal, environment), "noAccess");
  request.mock.mockImplementation(async () => { throw new Error("Network unavailable"); });
  assert.equal(await checkElan8Token(signal, environment), "unverified");
});

test("only a verified token replaces the shared token file, even without a license", async test => {
  const directory = await mkdtemp(join(tmpdir(), "spec42-elan8-token-"));
  try {
    const file = join(directory, "token");
    const original = `elan8_${"d".repeat(43)}`;
    const replacement = `elan8_${"e".repeat(43)}`;
    const environment = { ELAN8_CONFIG_DIR: directory };
    await writeFile(file, original);
    const signal = new AbortController().signal;
    const request = test.mock.method(globalThis, "fetch", async () => new Response(null, { status: 401 }));
    assert.equal(await saveElan8Token(replacement, signal, environment), "invalid");
    assert.equal(await readFile(file, "utf8"), original);
    request.mock.mockImplementation(async () => { throw new Error("Network unavailable"); });
    assert.equal(await saveElan8Token(replacement, signal, environment), "unverified");
    assert.equal(await readFile(file, "utf8"), original);
    request.mock.mockImplementation(async () => Response.json({ error: { code: "no_license" } }, { status: 403 }));
    assert.equal(await saveElan8Token(replacement, signal, environment), "noAccess");
    assert.equal(await readFile(file, "utf8"), replacement);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});
