/** Coalesces lifecycle wakes and user selections. It tracks request ownership, not model freshness. */
export class DiagramRefreshQueue {
  private revision = 0;
  private pending = false;
  private active: number | undefined;

  request(): void { this.revision++; this.pending = true; }
  begin(): number | undefined {
    if (!this.pending || this.active !== undefined) return undefined;
    this.pending = false;
    this.active = this.revision;
    return this.active;
  }
  isCurrent(ticket: number): boolean { return this.active === ticket && this.revision === ticket; }
  finish(ticket: number): void { if (this.active === ticket) this.active = undefined; }
  cancel(): void { this.revision++; this.pending = false; this.active = undefined; }
}
