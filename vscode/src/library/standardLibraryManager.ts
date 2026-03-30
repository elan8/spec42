import * as fs from "fs";
import * as path from "path";
import * as https from "https";
import * as vscode from "vscode";
import AdmZip from "adm-zip";

const GLOBAL_STATE_KEY = "spec42.standardLibrary.metadata";

export type StandardLibraryConfig = {
  enabled: boolean;
  version: string;
  repo: string;
  contentPath: string;
};

export type StandardLibraryMetadata = {
  installedVersion: string;
  installPath: string;
  installedAt: string;
  repo: string;
  contentPath: string;
};

export type StandardLibraryStatus = {
  enabled: boolean;
  pinnedVersion: string;
  installedVersion?: string;
  installPath?: string;
  isInstalled: boolean;
};

export class StandardLibraryManager {
  constructor(private readonly context: vscode.ExtensionContext) {}

  getStatus(config: StandardLibraryConfig): StandardLibraryStatus {
    const metadata = this.getMetadata();
    const isInstalled =
      !!metadata &&
      metadata.installedVersion === config.version &&
      fs.existsSync(metadata.installPath);
    return {
      enabled: config.enabled,
      pinnedVersion: config.version,
      installedVersion: metadata?.installedVersion,
      installPath: metadata?.installPath,
      isInstalled,
    };
  }

  getInstalledPath(config: StandardLibraryConfig): string | undefined {
    const status = this.getStatus(config);
    if (!status.enabled || !status.isInstalled || !status.installPath) {
      return undefined;
    }
    return status.installPath;
  }

  async installPinnedStandardLibrary(
    config: StandardLibraryConfig,
    progress?: vscode.Progress<{ message?: string; increment?: number }>
  ): Promise<StandardLibraryMetadata> {
    if (!config.version.trim()) {
      throw new Error("spec42.standardLibrary.version must not be empty.");
    }
    if (!config.repo.trim()) {
      throw new Error("spec42.standardLibrary.repo must not be empty.");
    }
    const normalizedContentPath = config.contentPath.replace(/^\/+|\/+$/g, "");
    if (!normalizedContentPath) {
      throw new Error("spec42.standardLibrary.contentPath must not be empty.");
    }

    const installBase = this.getInstallBasePath();
    await fs.promises.mkdir(installBase, { recursive: true });

    progress?.report({ message: "Downloading SysML v2 release archive..." });
    const zipBuffer = await downloadBuffer(
      `https://codeload.github.com/${config.repo}/zip/refs/tags/${encodeURIComponent(config.version)}`
    );

    progress?.report({ message: "Extracting standard library..." });
    const versionRoot = path.join(installBase, config.version);
    const contentTarget = path.join(versionRoot, normalizedContentPath);
    await fs.promises.rm(versionRoot, { recursive: true, force: true });
    await fs.promises.mkdir(contentTarget, { recursive: true });

    const zip = new AdmZip(zipBuffer);
    const entries = zip.getEntries();
    if (!entries.length) {
      throw new Error("Downloaded archive is empty.");
    }
    const rootPrefix = entries[0].entryName.split("/")[0];
    const contentPrefix = `${rootPrefix}/${normalizedContentPath}/`;
    const matching = entries.filter((entry) =>
      entry.entryName.startsWith(contentPrefix)
    );
    if (matching.length === 0) {
      throw new Error(
        `Path '${normalizedContentPath}' not found in archive for version '${config.version}'.`
      );
    }

    for (const entry of matching) {
      if (entry.isDirectory) {
        continue;
      }
      const relative = entry.entryName.slice(contentPrefix.length);
      const destination = path.join(contentTarget, relative);
      await fs.promises.mkdir(path.dirname(destination), { recursive: true });
      await fs.promises.writeFile(destination, entry.getData());
    }

    const metadata: StandardLibraryMetadata = {
      installedVersion: config.version,
      installPath: contentTarget,
      installedAt: new Date().toISOString(),
      repo: config.repo,
      contentPath: normalizedContentPath,
    };
    await this.context.globalState.update(GLOBAL_STATE_KEY, metadata);
    return metadata;
  }

  async removeInstalledStandardLibrary(): Promise<{
    removedVersion?: string;
    removedPath?: string;
    removed: boolean;
  }> {
    const metadata = this.getMetadata();
    if (!metadata) {
      return { removed: false };
    }
    const versionRoot = path.dirname(metadata.installPath);
    await fs.promises.rm(versionRoot, { recursive: true, force: true });
    await this.context.globalState.update(GLOBAL_STATE_KEY, undefined);
    return {
      removed: true,
      removedVersion: metadata.installedVersion,
      removedPath: metadata.installPath,
    };
  }

  private getMetadata(): StandardLibraryMetadata | undefined {
    return this.context.globalState.get<StandardLibraryMetadata>(GLOBAL_STATE_KEY);
  }

  private getInstallBasePath(): string {
    return path.join(this.context.globalStorageUri.fsPath, "standard-library");
  }
}

async function downloadBuffer(url: string, redirects = 0): Promise<Buffer> {
  if (redirects > 5) {
    throw new Error("Too many HTTP redirects while downloading standard library.");
  }
  return new Promise((resolve, reject) => {
    const req = https.get(
      url,
      {
        headers: {
          "User-Agent": "spec42-vscode-extension",
          Accept: "application/octet-stream",
        },
      },
      (res) => {
        const status = res.statusCode ?? 0;
        if (
          [301, 302, 303, 307, 308].includes(status) &&
          typeof res.headers.location === "string"
        ) {
          res.resume();
          const next = new URL(res.headers.location, url).toString();
          downloadBuffer(next, redirects + 1).then(resolve).catch(reject);
          return;
        }
        if (status < 200 || status >= 300) {
          const chunks: Buffer[] = [];
          res.on("data", (chunk) => chunks.push(Buffer.from(chunk)));
          res.on("end", () => {
            reject(
              new Error(
                `Failed to download standard library (HTTP ${status}). ${Buffer.concat(chunks).toString("utf8").slice(0, 200)}`
              )
            );
          });
          return;
        }
        const chunks: Buffer[] = [];
        res.on("data", (chunk) => chunks.push(Buffer.from(chunk)));
        res.on("end", () => resolve(Buffer.concat(chunks)));
      }
    );
    req.on("error", reject);
  });
}
