```markdown
# Извлечение ресурсов

## Обзор

Модуль `extractor` отвечает за обнаружение, извлечение и оптимизацию ресурсов из SVG:
- Встроенные изображения (base64)
- Внешние изображения (xlink:href)
- Шрифты и семейства
- Внешние CSS/JS

---

## Извлечение изображений

### Обработка base64

```rust
use data_url::DataUrl;
use image::{ImageFormat, ImageReader};

pub struct Base64Image {
    pub data: Vec<u8>,
    pub mime: String,
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
}

impl Base64Image {
    pub fn from_data_url(data_url: &str) -> Result<Self> {
        // Декодирование data URL
        let url = DataUrl::parse(data_url)?;
        let (data, _) = url.decode_to_vec()?;
        
        // Определение формата по MIME
        let mime = url.mime_type().to_string();
        let format = match mime.as_str() {
            "image/png" => ImageFormat::Png,
            "image/jpeg" => ImageFormat::Jpeg,
            "image/svg+xml" => ImageFormat::Svg,
            _ => return Err(Error::UnsupportedFormat(mime)),
        };
        
        // Получение размеров
        let reader = ImageReader::new(std::io::Cursor::new(&data))
            .with_guessed_format()?;
        let dimensions = reader.into_dimensions()?;
        
        Ok(Self {
            data,
            mime,
            format,
            width: dimensions.0,
            height: dimensions.1,
        })
    }
}
```

### Конвертация в WebP

```rust
#[cfg(feature = "webp")]
pub fn convert_to_webp(image: &Base64Image, quality: u8) -> Result<Vec<u8>> {
    use image::codecs::webp::WebPEncoder;
    
    // Декодирование исходного изображения
    let img = image::load_from_memory(&image.data)?;
    
    // Кодирование в WebP
    let mut webp_data = Vec::new();
    let encoder = WebPEncoder::new(&mut webp_data);
    encoder.encode(&img.to_rgba8(), quality)?;
    
    Ok(webp_data)
}
```

### Генерация вариантов

```rust
pub struct ImageVariants {
    pub default: Vec<u8>,      // WebP, качество 85%
    pub high_dpi: Option<Vec<u8>>, // 2x версия
    pub fallback: Vec<u8>,      // Оригинальный формат
}

impl ImageVariants {
    pub fn from_base64(image: Base64Image, quality: u8) -> Result<Self> {
        let webp = convert_to_webp(&image, quality)?;
        
        // Генерация 2x версии (масштабирование)
        let high_dpi = if image.width > 100 {
            Some(convert_to_webp_with_scale(&image, quality, 2.0)?)
        } else {
            None
        };
        
        Ok(Self {
            default: webp,
            high_dpi,
            fallback: image.data,
        })
    }
}
```

---

## Детекция шрифтов

### Парсинг font-family

```rust
use cssparser::{Parser, SourcePosition, Token};

pub fn parse_font_families(style: &str) -> Vec<String> {
    let mut families = Vec::new();
    let mut parser = Parser::new(style);
    
    while let Ok(token) = parser.next() {
        match token {
            Token::Ident(ident) => {
                families.push(ident.to_string());
            }
            Token::QuotedString(s) => {
                families.push(s.to_string());
            }
            _ => {}
        }
        
        // Пропуск запятых
        if parser.try_parse(|p| p.expect_comma()).is_ok() {
            continue;
        }
    }
    
    families
}
```

### Определение веса и начертания

```rust
pub struct FontInfo {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub fallback: Vec<String>,
}

impl FontInfo {
    pub fn from_style(style: &Style) -> Self {
        let family = style.font_family.clone().unwrap_or_default();
        let weight = match style.font_weight {
            Some(100..=300) => FontWeight::Light,
            Some(400..=500) => FontWeight::Normal,
            Some(600..=700) => FontWeight::Bold,
            Some(800..=900) => FontWeight::ExtraBold,
            _ => FontWeight::Normal,
        };
        
        let style = match style.font_style {
            Some("italic") => FontStyle::Italic,
            Some("oblique") => FontStyle::Oblique,
            _ => FontStyle::Normal,
        };
        
        Self {
            family,
            weight,
            style,
            fallback: vec![
                "system-ui".to_string(),
                "sans-serif".to_string(),
            ],
        }
    }
}
```

### Поиск в Google Fonts

```rust
pub async fn lookup_google_font(family: &str) -> Result<FontAsset> {
    let url = format!(
        "https://fonts.googleapis.com/css2?family={}:wght@{}&display=swap",
        family.replace(' ', "+"),
        "400;500;600;700"
    );
    
    let response = reqwest::get(&url).await?;
    let css = response.text().await?;
    
    // Парсинг CSS для получения URL шрифтов
    let font_urls = extract_font_urls(&css)?;
    
    // Скачивание файлов шрифтов
    let mut files = HashMap::new();
    for (format, url) in font_urls {
        let font_data = reqwest::get(&url).await?.bytes().await?;
        files.insert(format, font_data.to_vec());
    }
    
    Ok(FontAsset {
        family: family.to_string(),
        files,
        weights: vec![400, 500, 600, 700],
        styles: vec!["normal".to_string()],
    })
}
```

---

## Fetch внешних ресурсов

### Асинхронная загрузка с retry

```rust
use tokio::time::{sleep, Duration};

pub async fn fetch_with_retry(
    url: &str,
    max_retries: u32,
    timeout: Duration,
) -> Result<Vec<u8>> {
    let mut attempt = 0;
    let mut backoff = Duration::from_millis(100);
    
    loop {
        match fetch_with_timeout(url, timeout).await {
            Ok(data) => return Ok(data),
            Err(e) => {
                attempt += 1;
                if attempt >= max_retries {
                    return Err(e);
                }
                
                tracing::warn!(
                    "Fetch failed (attempt {}/{}): {}",
                    attempt, max_retries, e
                );
                
                sleep(backoff).await;
                backoff = backoff * 2;
            }
        }
    }
}

async fn fetch_with_timeout(url: &str, timeout: Duration) -> Result<Vec<u8>> {
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .user_agent("svg2web/0.2.0")
        .build()?;
    
    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        return Err(Error::HttpError(response.status()));
    }
    
    Ok(response.bytes().await?.to_vec())
}
```

### Кэширование загрузок

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct FetchCache {
    cache: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl FetchCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn get_or_fetch(&self, url: &str) -> Result<Vec<u8>> {
        {
            let cache = self.cache.lock().await;
            if let Some(data) = cache.get(url) {
                tracing::debug!("Cache hit: {}", url);
                return Ok(data.clone());
            }
        }
        
        let data = fetch_with_retry(url, 3, Duration::from_secs(30)).await?;
        
        {
            let mut cache = self.cache.lock().await;
            cache.insert(url.to_string(), data.clone());
        }
        
        Ok(data)
    }
}
```

---

## Обработка ошибок

### Типы ошибок

```rust
#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("unsupported image format: {0}")]
    UnsupportedFormat(String),
    
    #[error("failed to decode image: {0}")]
    DecodeError(#[from] image::ImageError),
    
    #[error("failed to fetch resource: {0}")]
    FetchError(String),
    
    #[error("HTTP error: {0}")]
    HttpError(StatusCode),
    
    #[error("timeout while fetching {0}")]
    TimeoutError(String),
    
    #[error("CORS blocked: {0}")]
    CorsError(String),
    
    #[error("invalid font data: {0}")]
    FontError(String),
}
```

### Graceful fallback

```rust
pub async fn extract_image_safe(element: &SVGElement) -> Option<ImageAsset> {
    match extract_image(element).await {
        Ok(asset) => Some(asset),
        Err(ExtractError::UnsupportedFormat(_)) => {
            tracing::warn!("Unsupported format, skipping");
            None
        }
        Err(ExtractError::TimeoutError(url)) => {
            tracing::warn!("Timeout loading {}, using placeholder", url);
            Some(ImageAsset::placeholder())
        }
        Err(e) => {
            tracing::error!("Failed to extract image: {}", e);
            None
        }
    }
}
```

---

## Полный pipeline извлечения

```rust
pub async fn extract_assets(
    element: &SVGElement,
    options: &ExtractOptions,
) -> Result<ExtractedAssets> {
    let cache = FetchCache::new();
    let mut images = Vec::new();
    let mut fonts = Vec::new();
    
    // Извлечение изображений
    if options.extract_images {
        for img in find_image_elements(element) {
            if let Some(asset) = extract_image_with_options(&img, &cache, options).await? {
                images.push(asset);
            }
        }
    }
    
    // Извлечение шрифтов
    if options.extract_fonts {
        let families = collect_font_families(element);
        for family in families {
            if let Some(asset) = lookup_font(&family, options).await? {
                fonts.push(asset);
            }
        }
    }
    
    Ok(ExtractedAssets {
        images,
        fonts,
        external: ExternalAssets {
            stylesheets: extract_stylesheets(element),
            scripts: extract_scripts(element),
        },
    })
}
```

---

## Пример использования

```rust
let options = ExtractOptions {
    extract_images: true,
    convert_to_webp: true,
    webp_quality: 85,
    extract_fonts: true,
    resolve_external: true,
    timeout_ms: 30000,
};

let assets = extract_assets(&svg_element, &options).await?;

for img in assets.images {
    println!("Image: {}x{} ({} bytes)", img.width, img.height, img.data.len());
}

for font in assets.fonts {
    println!("Font: {} ({} variants)", font.family, font.files.len());
}
```
```