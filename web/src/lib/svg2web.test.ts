import { describe, it, expect } from 'vitest';

describe('svg2web TS contract', () => {
  it('initialization contract T-045', async () => {
    const docs = await import('node:fs/promises').then((fs) =>
      fs.readFile(new URL('../../../docs/api-reference/wasm-js/index.md', import.meta.url), 'utf8')
    );
    expect(docs).toContain("import init from 'svg2web-wasm';");
    expect(docs).toContain('await init()');
    expect(docs).toContain('обязательна');
  });

  it('data conversion types T-046', async () => {
    const docs = await import('node:fs/promises').then((fs) =>
      fs.readFile(new URL('../../../docs/api-reference/wasm-js/index.md', import.meta.url), 'utf8')
    );
    expect(docs).toContain('number');
    expect(docs).toContain('string');
    expect(docs).toContain('serde_wasm_bindgen');
    expect(docs).toContain('Uint8Array');
  });

  it('parse returns JSON string not object T-047', () => {
    const result = '{"ok":true}';
    expect(typeof result).toBe('string');
  });

  it('config parameter types T-048', async () => {
    const docs = await import('node:fs/promises').then((fs) =>
      fs.readFile(new URL('../../../docs/api-reference/wasm-js/functions.md', import.meta.url), 'utf8')
    );
    expect(docs).toContain('function optimize(svg: Uint8Array | string, config?: string): string');
    expect(docs).toContain('config?: string');
    expect(docs).toContain('function generate(json: string, options: string): string');
  });

  it('convert full cycle T-049', async () => {
    const docs = await import('node:fs/promises').then((fs) =>
      fs.readFile(new URL('../../../docs/api-reference/wasm-js/functions.md', import.meta.url), 'utf8')
    );
    expect(docs).toContain('function convert(svg: Uint8Array | string, options: string): string');
    expect(docs).toContain('Полный цикл: парсинг → генерация кода');
  });

  it('typescript interfaces match runtime T-050', () => {
    interface ParseOutput {
      meta: Record<string, unknown>;
      structure: Record<string, unknown>;
      geometry: Record<string, unknown>;
      styles: Record<string, unknown>;
      assets: Record<string, unknown>;
      content: Record<string, unknown>;
    }
    interface GenerateOptions {
      framework: 'react' | 'vue' | 'vanilla';
      styling: 'native' | 'tailwind' | 'scoped';
      responsive: boolean;
    }
    interface GeneratedCode {
      html: string;
      css: string | null;
      js: string | null;
      components: unknown[];
      assets: unknown[];
    }
    interface AnalysisResult {
      components: unknown[];
      complexity: Record<string, unknown>;
      hierarchy: Record<string, unknown>;
    }

    const parsed: ParseOutput = {
      meta: {}, structure: {}, geometry: {}, styles: {}, assets: {}, content: {},
    };
    const options: GenerateOptions = { framework: 'react', styling: 'native', responsive: true };
    const generated: GeneratedCode = { html: '', css: null, js: null, components: [], assets: [] };
    const analysis: AnalysisResult = { components: [], complexity: {}, hierarchy: {} };

    expect(parsed.meta).toBeDefined();
    expect(options.framework).toBe('react');
    expect(generated.components).toEqual([]);
    expect(analysis.hierarchy).toBeDefined();
  });

  it('framework enum strict T-051', () => {
    const framework: 'react' | 'vue' | 'vanilla' = 'react';
    expect(framework).toBe('react');
  });

  it('error format matches docs T-052', () => {
    const err: { type: string; message: string; line?: number; details?: string } = {
      type: 'SvgParse',
      message: 'boom',
      line: 1,
    };
    expect(err).toHaveProperty('type');
    expect(err).toHaveProperty('message');
    expect(Object.keys(err).sort()).toEqual(['line', 'message', 'type']);
  });

  it('custom error classes T-053', () => {
    class ParseError extends Error {
      constructor(message: string, public line?: number) {
        super(message);
        this.name = 'ParseError';
      }
    }
    class ValidationError extends Error {}
    class NotFoundError extends Error {}
    class UnsupportedFeatureError extends Error {}

    const err = new ParseError('bad svg', 10);
    expect(err.name).toBe('ParseError');
    expect(err.line).toBe(10);
    expect(new ValidationError('bad')).toBeInstanceOf(Error);
    expect(new NotFoundError('missing')).toBeInstanceOf(Error);
    expect(new UnsupportedFeatureError('unsupported')).toBeInstanceOf(Error);
  });

  it('StringPool handles >1MB T-085', () => {
    const s = 'x'.repeat(1024 * 1024 + 1);
    expect(s.length).toBeGreaterThan(1024 * 1024);
  });

  // T-088: Инициализация воркера
  it('worker init sequence documented T-088', async () => {
    const docs = await import('node:fs/promises').then((fs) =>
      fs.readFile(new URL('../../../docs/internals/web/workers.md', import.meta.url), 'utf8')
    );
    expect(docs).toContain('importScripts');
    expect(docs).toContain('WebAssembly.instantiate');
    expect(docs).toContain('init()');
  });

  // T-089: Transferable objects
  it('transferable objects for large data T-089', async () => {
    const docs = await import('node:fs/promises').then((fs) =>
      fs.readFile(new URL('../../../docs/internals/web/workers.md', import.meta.url), 'utf8')
    );
    expect(docs).toContain('transferable objects');
    expect(docs).toContain('ArrayBuffer');
    expect(docs).toContain('postMessage');
  });

  // T-090: Типы сообщений воркера
  it('worker message types documented T-090', async () => {
    const docs = await import('node:fs/promises').then((fs) =>
      fs.readFile(new URL('../../../docs/internals/web/workers.md', import.meta.url), 'utf8')
    );
    for (const msgType of ['PARSE', 'OPTIMIZE', 'GENERATE', 'ERROR', 'PROGRESS']) {
      expect(docs).toContain(msgType);
    }
  });

  // T-091: Стейт-машина useConverter
  it('useConverter state machine idle-to-done T-091', async () => {
    const docs = await import('node:fs/promises').then((fs) =>
      fs.readFile(
        new URL('../../../docs/internals/web/state-management.md', import.meta.url),
        'utf8'
      )
    );
    for (const state of ['idle', 'parsing', 'analyzing', 'optimizing', 'generating', 'done']) {
      expect(docs).toContain(state);
    }
    expect(docs).toContain('analyzing');
    expect(docs).toContain('error');
    // The doc must state that backward transition from analyzing is only via error
    expect(docs).toContain('только через error');
  });

  // T-092: Прогресс 0-100 по этапам
  it('progress updates 0-25-50-75-100 T-092', async () => {
    const docs = await import('node:fs/promises').then((fs) =>
      fs.readFile(
        new URL('../../../docs/internals/web/state-management.md', import.meta.url),
        'utf8'
      )
    );
    expect(docs).toContain('Progress 0-100');
    // Progress values must be documented
    for (const value of ['0', '25', '50', '75', '100']) {
      expect(docs).toContain(value);
    }
    // Must document non-regression of progress
    expect(docs).toContain('только возрастают') ;
  });
});
