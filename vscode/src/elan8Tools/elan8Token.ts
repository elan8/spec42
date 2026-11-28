import { mkdir, mkdtemp, readFile, rename, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

export const elan8TokensPage = "https://www.elan8.com/account?section=tokens";
export const enterElan8TokenLabel = "Enter Elan8 API token…";

export type Elan8TokenStatus = "valid" | "noAccess" | "missing" | "invalid" | "unverified";

function tokenDirectory(environment: NodeJS.ProcessEnv): string {
  // Match the shared Elan8 CLI configuration lookup, including environment overrides.
  const { ELAN8_CONFIG_DIR, XDG_CONFIG_HOME, APPDATA, HOME } = environment;
  return ELAN8_CONFIG_DIR
    || (XDG_CONFIG_HOME && join(XDG_CONFIG_HOME, "elan8"))
    || (process.platform === "win32" && APPDATA && join(APPDATA, "Elan8"))
    || (HOME && join(HOME, ".config", "elan8"))
    || join(tmpdir(), "elan8");
}

export async function checkElan8Token(
  signal: AbortSignal, environment = process.env,
): Promise<Elan8TokenStatus> {
  try {
    signal.throwIfAborted();
    let token = environment.ELAN8_TOKEN?.trim();
    if (!token) {
      try {
        token = (await readFile(join(tokenDirectory(environment), "token"), { encoding: "utf8", signal })).trim();
      } catch (error) {
        if ((error as NodeJS.ErrnoException).code === "ENOENT") return "missing";
        throw error;
      }
    }
    return await verifyElan8Token(token, signal);
  } catch {
    signal.throwIfAborted();
    return "unverified";
  }
}

async function verifyElan8Token(token: string, signal: AbortSignal): Promise<Elan8TokenStatus> {
  signal.throwIfAborted();
  if (!/^elan8_[A-Za-z0-9_-]{43}$/.test(token)) return "invalid";
  try {
    const response = await fetch("https://api.elan8.com/api/v1/token/license", {
      method: "POST",
      headers: { Authorization: `Bearer ${token}` },
      redirect: "error",
      signal: AbortSignal.any([signal, AbortSignal.timeout(8000)]),
    });
    if (response.status === 403) {
      const result = await response.json() as { error?: { code?: string } };
      if (result.error?.code === "no_license" || result.error?.code === "access_ending") return "noAccess";
    } else {
      await response.body?.cancel();
      if (response.status === 200) return "valid";
      if (response.status === 401) return "invalid";
    }
    return "unverified";
  } catch {
    signal.throwIfAborted();
    return "unverified";
  }
}

export async function saveElan8Token(
  value: string, signal: AbortSignal, environment = process.env,
): Promise<Elan8TokenStatus> {
  const token = value.trim();
  const status = await verifyElan8Token(token, signal);
  if (status !== "valid" && status !== "noAccess") return status;
  signal.throwIfAborted();
  const directory = tokenDirectory(environment);
  await mkdir(directory, { recursive: true, mode: 0o700 });
  const staging = await mkdtemp(join(directory, ".token-"));
  try {
    const candidate = join(staging, "token");
    await writeFile(candidate, token, { flag: "wx", mode: 0o600, signal });
    signal.throwIfAborted();
    await rename(candidate, join(directory, "token"));
  } finally {
    await rm(staging, { recursive: true, force: true }).catch(() => {});
  }
  return status;
}
