export interface ConversionOptions {
  format: "react" | "vue" | "vanilla";
  optimize: boolean;
}

export interface AnalysisResult {
  components: Component[];
  complexity: ComplexityMetrics;
  hierarchy: HierarchyInfo;
}

export interface Component {
  id: string;
  template: string;
  occurrences: number;
}

export interface ComplexityMetrics {
  score: number;
  node_count: number;
  path_count: number;
  path_complexity: number;
  gradient_count: number;
  text_count: number;
  image_count: number;
}

export interface HierarchyInfo {
  depth: number;
  max_width: number;
}

export interface ConversionResult {
  analysis: AnalysisResult;
  optimizedSvg: string;
}

export type ConversionStatus =
  | "idle"
  | "parsing"
  | "analyzing"
  | "optimizing"
  | "done"
  | "error";
