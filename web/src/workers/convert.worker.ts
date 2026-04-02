import init, {
  parse_svg,
  analyze_svg,
  optimize_svg,
  init_panic_hook,
} from "svg2web-wasm";

let wasmReady = false;

async function ensureInit() {
  if (!wasmReady) {
    await init();
    init_panic_hook();
    wasmReady = true;
  }
}

self.onmessage = async (e: MessageEvent) => {
  const { type, payload, id } = e.data;

  try {
    await ensureInit();

    let result: string;

    switch (type) {
      case "PARSE":
        result = parse_svg(payload.svg);
        self.postMessage({ type: "PARSED", payload: result, id });
        break;

      case "ANALYZE":
        result = analyze_svg(payload.svg);
        self.postMessage({ type: "ANALYZED", payload: result, id });
        break;

      case "OPTIMIZE":
        result = optimize_svg(payload.svg);
        self.postMessage({ type: "OPTIMIZED", payload: result, id });
        break;

      default:
        self.postMessage({
          type: "ERROR",
          payload: `Unknown message type: ${type}`,
          id,
        });
    }
  } catch (err) {
    self.postMessage({
      type: "ERROR",
      payload: err instanceof Error ? err.message : String(err),
      id,
    });
  }
};

export {};
