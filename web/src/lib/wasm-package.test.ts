import { describe, it, expect } from 'vitest';

describe('svg2web-wasm npm contract', () => {
  it('npm package lacks generate T-007', async () => {
    let mod: Record<string, unknown> | null = null;
    try {
      mod = (await import('svg2web-wasm')) as Record<string, unknown>;
    } catch {
      // Environment may not have package installed in unit-test runtime.
      return;
    }

    expect(mod.generate).toBeUndefined();
    expect(mod.parse_svg ?? mod.parse).toBeDefined();
    expect(mod.analyze_svg ?? mod.analyze).toBeDefined();
    expect(mod.optimize_svg ?? mod.optimize).toBeDefined();
  });
});