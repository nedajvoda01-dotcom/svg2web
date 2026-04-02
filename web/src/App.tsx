import { useState } from "react";
import { useConverter } from "./hooks/useConverter";

function App() {
  const [svg, setSvg] = useState("");
  const { status, progress, result, error, convert, ready } = useConverter();

  return (
    <div style={{ maxWidth: 800, margin: "0 auto", padding: 24 }}>
      <h1>SVG2Web Converter</h1>
      <p style={{ color: "#666" }}>
        Status: <strong>{ready ? "WASM ready" : "Loading WASM..."}</strong>
      </p>

      <textarea
        value={svg}
        onChange={(e) => setSvg(e.target.value)}
        placeholder="Paste SVG here..."
        rows={10}
        style={{ width: "100%", fontFamily: "monospace", fontSize: 13 }}
      />

      <div style={{ margin: "12px 0" }}>
        <button
          onClick={() => convert(svg)}
          disabled={!ready || !svg.trim() || status === "optimizing"}
        >
          {status === "idle" || status === "done" || status === "error"
            ? "Convert"
            : `${status}... ${progress}%`}
        </button>
      </div>

      {error && (
        <div style={{ color: "red", marginBottom: 12 }}>Error: {error}</div>
      )}

      {result && (
        <div>
          <h3>Analysis</h3>
          <ul>
            <li>Complexity score: {result.analysis.complexity.score}</li>
            <li>Nodes: {result.analysis.complexity.node_count}</li>
            <li>Depth: {result.analysis.hierarchy.depth}</li>
            <li>Components: {result.analysis.components.length}</li>
          </ul>

          <h3>Optimized SVG</h3>
          <pre
            style={{
              maxHeight: 300,
              overflow: "auto",
              background: "#f5f5f5",
              padding: 12,
              fontSize: 12,
            }}
          >
            {result.optimizedSvg.substring(0, 2000)}
            {result.optimizedSvg.length > 2000 && "..."}
          </pre>
        </div>
      )}
    </div>
  );
}

export default App;
