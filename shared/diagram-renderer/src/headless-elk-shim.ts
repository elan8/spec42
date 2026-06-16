type ElkOptions = Record<string, unknown>;

export default class HeadlessElk {
  constructor(options: ElkOptions = {}) {
    const global = globalThis as unknown as {
      __spec42ElkCtor?: new (options: ElkOptions) => unknown;
      __spec42ElkWorkerCtor?: new () => unknown;
    };
    const ElkCtor = global.__spec42ElkCtor;
    const WorkerCtor = global.__spec42ElkWorkerCtor;
    if (typeof ElkCtor !== "function" || typeof WorkerCtor !== "function") {
      throw new Error("Spec42 headless ELK constructors were not installed");
    }
    return new ElkCtor({
      ...options,
      workerFactory: () => new WorkerCtor(),
    }) as HeadlessElk;
  }
}
