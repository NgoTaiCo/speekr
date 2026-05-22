use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event_loop::{EventLoopProxy, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};
use thiserror::Error;
use wry::{WebView, WebViewBuilder};

use crate::AppEvent;

#[derive(Debug, Clone)]
pub struct PopupContent {
    pub source_language: String,
    pub target_language: String,
    pub original: String,
    pub translated: String,
    pub cursor_position: Option<(f64, f64)>,
}

#[derive(Default)]
pub struct TranslationPopup {
    current: Option<PopupWindow>,
}

struct PopupWindow {
    _window: Window,
    _webview: WebView,
}

#[derive(Debug, Error)]
pub enum PopupError {
    #[error("window error: {0}")]
    Window(#[from] tao::error::OsError),
    #[error("webview error: {0}")]
    WebView(#[from] wry::Error),
}

impl TranslationPopup {
    pub fn show<T: 'static>(
        &mut self,
        target: &EventLoopWindowTarget<T>,
        proxy: EventLoopProxy<AppEvent>,
        content: PopupContent,
    ) -> Result<(), PopupError> {
        self.current = None;

        let (x, y) = content.cursor_position.unwrap_or((80.0, 80.0));
        let window = WindowBuilder::new()
          .with_title("speekr Translate")
            .with_inner_size(LogicalSize::new(420.0, 280.0))
            .with_position(LogicalPosition::new(x + 14.0, y + 18.0))
            .with_decorations(false)
            .with_always_on_top(true)
            .with_resizable(false)
            .with_visible(true)
            .build(target)?;

        let html = render_html(&content);
        let builder = WebViewBuilder::new()
            .with_html(html)
            .with_ipc_handler(move |request| {
                if request.body() == "close" {
                    let _ = proxy.send_event(AppEvent::ClosePopup);
                }
            });

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let webview = builder.build(&window)?;

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let webview = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let vbox = window.default_vbox().expect("GTK vbox unavailable");
            builder.build_gtk(vbox)?
        };

        self.current = Some(PopupWindow {
            _window: window,
            _webview: webview,
        });

        Ok(())
    }

    pub fn close(&mut self) {
        self.current = None;
    }
}

fn render_html(content: &PopupContent) -> String {
    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<style>
html, body {{
  margin: 0;
  width: 100%;
  min-height: 100%;
  font-family: "Segoe UI", system-ui, -apple-system, BlinkMacSystemFont, sans-serif;
  background: #f8fafc;
  color: #1f2937;
}}
body {{
  box-sizing: border-box;
  padding: 14px;
  border: 1px solid #94a3b8;
}}
.top {{
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 10px;
}}
.question {{
  font-size: 13px;
  font-weight: 650;
  color: #0f766e;
}}
button {{
  border: 0;
  background: #e2e8f0;
  color: #334155;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  font-size: 18px;
  line-height: 1;
}}
.label {{
  margin: 12px 0 4px;
  font-size: 11px;
  color: #64748b;
  text-transform: uppercase;
  letter-spacing: .06em;
}}
.text {{
  font-size: 14px;
  line-height: 1.45;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
  max-height: 82px;
  overflow: auto;
}}
.translated {{
  font-size: 16px;
  color: #111827;
}}
</style>
</head>
<body>
  <div class="top">
    <div class="question">Translate? {source} -> {target}</div>
    <button onclick="window.ipc.postMessage('close')" title="Close">&times;</button>
  </div>
  <div class="label">Original</div>
  <div class="text">{original}</div>
  <div class="label">Translation</div>
  <div class="text translated">{translated}</div>
</body>
</html>"#,
        source = escape_html(&content.source_language),
        target = escape_html(&content.target_language),
        original = escape_html(&content.original),
        translated = escape_html(&content.translated),
    )
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
