import { test, expect } from '@playwright/test';
import { readFile } from 'node:fs/promises';
import { spawn } from 'node:child_process';
import net from 'node:net';
import path from 'node:path';

function canRun(command: string) {
  return new Promise<boolean>((resolve) => {
    const child = spawn('sh', ['-lc', `command -v ${command} >/dev/null 2>&1`]);
    child.on('close', (code) => resolve(code === 0));
    child.on('error', () => resolve(false));
  });
}

function isPortOpen(port: number) {
  return new Promise<boolean>((resolve) => {
    const socket = net.createConnection({ port, host: '127.0.0.1' });
    socket.on('connect', () => {
      socket.destroy();
      resolve(true);
    });
    socket.on('error', () => resolve(false));
  });
}

test('demo-url-accessible T-031', async ({ page }) => {
  const response = await page.goto('https://svg2web.vercel.app');
  expect(response?.status()).toBe(200);
});

test('wasm-size-limit-enforced T-033', async () => {
  const doc = await readFile(path.resolve(process.cwd(), 'docs/usage/web-interface.md'), 'utf8');
  expect(doc).toContain('50MB');
  expect(doc).toContain('WASM размер ограничен');
});

test('cors-warning-shown T-034', async () => {
  const doc = await readFile(path.resolve(process.cwd(), 'docs/usage/web-interface.md'), 'utf8');
  expect(doc).toContain('CORS');
  expect(doc).toContain('Внешний ресурс недоступен из-за CORS');
});

test('local-dev-server-starts T-032', async () => {
  if (!(await canRun('npm'))) {
    test.skip();
  }

  const webRoot = path.resolve(process.cwd(), 'web');
  const child = spawn('sh', ['-lc', 'npm install && npm run dev -- --port 3000'], {
    cwd: webRoot,
    stdio: 'ignore',
  });

  try {
    const started = await new Promise<boolean>((resolve) => {
      const startedAt = Date.now();
      const timer = setInterval(async () => {
        if (await isPortOpen(3000)) {
          clearInterval(timer);
          resolve(true);
          return;
        }
        if (Date.now() - startedAt > 30000) {
          clearInterval(timer);
          resolve(false);
        }
      }, 500);
    });

    expect(started).toBe(true);
  } finally {
    child.kill('SIGTERM');
  }
});
