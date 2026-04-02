import { useState, useCallback } from "react";
import { useWorker } from "./useWorker";
import type { AnalysisResult, ConversionResult, ConversionStatus } from "../types";

export function useConverter() {
  const worker = useWorker();
  const [status, setStatus] = useState<ConversionStatus>("idle");
  const [progress, setProgress] = useState(0);
  const [result, setResult] = useState<ConversionResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const convert = useCallback(
    async (svg: string) => {
      try {
        setError(null);
        setResult(null);

        // Parse
        setStatus("parsing");
        setProgress(10);
        await worker.parse(svg);

        // Analyze
        setStatus("analyzing");
        setProgress(30);
        const analysisJson = await worker.analyze(svg);
        const analysis: AnalysisResult = JSON.parse(analysisJson);

        // Optimize
        setStatus("optimizing");
        setProgress(60);
        const optimizedSvg = await worker.optimize(svg);

        setProgress(100);
        setStatus("done");
        setResult({ analysis, optimizedSvg });
      } catch (err) {
        setStatus("error");
        setError(err instanceof Error ? err.message : String(err));
      }
    },
    [worker],
  );

  return { status, progress, result, error, convert, ready: worker.ready };
}
