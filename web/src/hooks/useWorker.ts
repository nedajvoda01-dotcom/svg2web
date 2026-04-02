import { useEffect, useRef, useCallback, useState } from "react";

interface WorkerState {
  ready: boolean;
  error: string | null;
}

let nextId = 1;

export function useWorker() {
  const workerRef = useRef<Worker | null>(null);
  const [state, setState] = useState<WorkerState>({ ready: false, error: null });

  useEffect(() => {
    const worker = new Worker(
      new URL("../workers/convert.worker.ts", import.meta.url),
      { type: "module" },
    );
    workerRef.current = worker;

    // Send a no-op parse to trigger WASM init
    const initId = nextId++;
    const onReady = (e: MessageEvent) => {
      if (e.data.id === initId) {
        setState({ ready: true, error: null });
        worker.removeEventListener("message", onReady);
      }
    };
    worker.addEventListener("message", onReady);
    worker.postMessage({
      type: "PARSE",
      payload: { svg: '<svg xmlns="http://www.w3.org/2000/svg"/>' },
      id: initId,
    });

    return () => worker.terminate();
  }, []);

  const send = useCallback(
    (type: string, payload: Record<string, unknown>): Promise<string> => {
      return new Promise((resolve, reject) => {
        const worker = workerRef.current;
        if (!worker) {
          reject(new Error("Worker not initialized"));
          return;
        }
        const id = nextId++;
        const handler = (e: MessageEvent) => {
          if (e.data.id !== id) return;
          worker.removeEventListener("message", handler);
          if (e.data.type === "ERROR") {
            reject(new Error(e.data.payload));
          } else {
            resolve(e.data.payload);
          }
        };
        worker.addEventListener("message", handler);
        worker.postMessage({ type, payload, id });
      });
    },
    [],
  );

  const parse = useCallback((svg: string) => send("PARSE", { svg }), [send]);
  const analyze = useCallback((svg: string) => send("ANALYZE", { svg }), [send]);
  const optimize = useCallback((svg: string) => send("OPTIMIZE", { svg }), [send]);

  return { ...state, parse, analyze, optimize };
}
