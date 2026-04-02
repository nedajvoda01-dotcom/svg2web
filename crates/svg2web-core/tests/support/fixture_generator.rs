use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Deterministic SVG fixture generator.
///
/// Uses a seed-based RNG so the same (seed, params) always produces the same SVG.
pub struct FixtureGenerator {
    rng: StdRng,
    layers: usize,
    gradients: bool,
    filters: bool,
    text: bool,
}

impl FixtureGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            layers: 4,
            gradients: false,
            filters: false,
            text: false,
        }
    }

    pub fn with_layers(mut self, n: usize) -> Self {
        self.layers = n;
        self
    }

    pub fn with_gradients(mut self, enabled: bool) -> Self {
        self.gradients = enabled;
        self
    }

    pub fn with_filters(mut self, enabled: bool) -> Self {
        self.filters = enabled;
        self
    }

    pub fn with_text(mut self, enabled: bool) -> Self {
        self.text = enabled;
        self
    }

    pub fn generate(&mut self) -> String {
        let width = self.rng.gen_range(100..=800);
        let height = self.rng.gen_range(100..=800);

        let mut defs = String::new();
        let mut body = String::new();

        // Optional gradient definitions
        if self.gradients {
            let grad_count = self.rng.gen_range(1..=3);
            for g in 0..grad_count {
                let r1 = self.rng.gen_range(0u8..=255);
                let g1 = self.rng.gen_range(0u8..=255);
                let b1 = self.rng.gen_range(0u8..=255);
                let r2 = self.rng.gen_range(0u8..=255);
                let g2 = self.rng.gen_range(0u8..=255);
                let b2 = self.rng.gen_range(0u8..=255);
                defs.push_str(&format!(
                    r#"<linearGradient id="grad{g}"><stop offset="0%" stop-color="rgb({r1},{g1},{b1})"/><stop offset="100%" stop-color="rgb({r2},{g2},{b2})"/></linearGradient>"#,
                ));
            }
        }

        // Optional filter definitions
        if self.filters {
            defs.push_str(
                r#"<filter id="blur0"><feGaussianBlur stdDeviation="2"/></filter>"#,
            );
        }

        // Generate layers with shapes
        for _ in 0..self.layers {
            body.push_str("<g>");
            let shapes = self.rng.gen_range(1..=5);
            for _ in 0..shapes {
                let shape = self.random_shape(width, height);
                body.push_str(&shape);
            }
            body.push_str("</g>");
        }

        // Optional text elements
        if self.text {
            let texts = self.rng.gen_range(1..=3);
            for _ in 0..texts {
                let x = self.rng.gen_range(10..width);
                let y = self.rng.gen_range(10..height);
                let size = self.rng.gen_range(10..=32);
                body.push_str(&format!(
                    r#"<text x="{x}" y="{y}" font-size="{size}">Text</text>"#,
                ));
            }
        }

        let defs_section = if defs.is_empty() {
            String::new()
        } else {
            format!("<defs>{defs}</defs>")
        };

        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">{defs_section}{body}</svg>"#,
        )
    }

    fn random_shape(&mut self, w: u32, h: u32) -> String {
        let kind = self.rng.gen_range(0..4);
        let r = self.rng.gen_range(0u8..=255);
        let g = self.rng.gen_range(0u8..=255);
        let b = self.rng.gen_range(0u8..=255);
        let fill = format!("rgb({r},{g},{b})");

        match kind {
            0 => {
                // rect
                let x = self.rng.gen_range(0..w);
                let y = self.rng.gen_range(0..h);
                let rw = self.rng.gen_range(10..=w / 2);
                let rh = self.rng.gen_range(10..=h / 2);
                format!(r#"<rect x="{x}" y="{y}" width="{rw}" height="{rh}" fill="{fill}"/>"#)
            }
            1 => {
                // circle
                let cx = self.rng.gen_range(0..w);
                let cy = self.rng.gen_range(0..h);
                let cr = self.rng.gen_range(5..=w / 4);
                format!(r#"<circle cx="{cx}" cy="{cy}" r="{cr}" fill="{fill}"/>"#)
            }
            2 => {
                // ellipse
                let cx = self.rng.gen_range(0..w);
                let cy = self.rng.gen_range(0..h);
                let rx = self.rng.gen_range(5..=w / 4);
                let ry = self.rng.gen_range(5..=h / 4);
                format!(
                    r#"<ellipse cx="{cx}" cy="{cy}" rx="{rx}" ry="{ry}" fill="{fill}"/>"#
                )
            }
            _ => {
                // path (simple polygon)
                let points = self.rng.gen_range(3..=6);
                let mut d = String::new();
                for p in 0..points {
                    let px = self.rng.gen_range(0..w);
                    let py = self.rng.gen_range(0..h);
                    if p == 0 {
                        d.push_str(&format!("M{px} {py}"));
                    } else {
                        d.push_str(&format!(" L{px} {py}"));
                    }
                }
                d.push_str(" Z");
                format!(r#"<path d="{d}" fill="{fill}"/>"#)
            }
        }
    }
}
