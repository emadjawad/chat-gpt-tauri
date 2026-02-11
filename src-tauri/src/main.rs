#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tokio::time::{sleep, Duration};
use url::Url;

// حقن خفيف: CSS + محاولة إخفاء الأزرار مرة واحدة (بدون loops)
const HIDE_MIC_JS: &str = r#"
(() => {
  const STYLE_ID = "cgpt-hide-mic-style-once";
  if (!document.getElementById(STYLE_ID)) {
    const style = document.createElement("style");
    style.id = STYLE_ID;
    style.textContent = `
      /* Voice / mic buttons (best-effort, safe if missing) */
      button[aria-label*="Dictate" i],
      button[aria-label*="Voice" i],
      button[aria-label*="Microphone" i],
      button[title*="Dictate" i],
      button[title*="Voice" i],
      button[title*="Microphone" i],
      [data-testid*="dictat" i],
      [data-testid*="voice" i],
      [data-testid*="microphone" i]
      { display: none !important; visibility: hidden !important; }
    `;
    document.head.appendChild(style);
  }

  // pass واحد إضافي (بدون مراقبة DOM)
  const needles = ["dictate", "voice", "microphone", "mic"];
  const btns = Array.from(document.querySelectorAll("button, [role='button']"));
  for (const el of btns) {
    const label =
      (el.getAttribute("aria-label") || "") + " " +
      (el.getAttribute("data-testid") || "") + " " +
      (el.getAttribute("title") || "");
    const hay = label.toLowerCase();
    if (needles.some(n => hay.includes(n))) {
      el.style.setProperty("display", "none", "important");
      el.style.setProperty("visibility", "hidden", "important");
    }
  }
  return true;
})();
"#;

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      let app_handle = app.handle().clone();

      // Splash
      let _splash = WebviewWindowBuilder::new(app, "splash", WebviewUrl::App("splash.html".into()))
        .title("ChatGPT")
        .inner_size(420.0, 360.0)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .build()?;

      // Main (hidden first)
      let _main = WebviewWindowBuilder::new(
        app,
        "main",
        WebviewUrl::External(Url::parse("https://chatgpt.com").unwrap()),
      )
      .title("ChatGPT")
      .inner_size(1280.0, 850.0)
      .resizable(true)
      .visible(false)
      // ✅ هذا السطر هو المهم لتسريع الفتح (كاش ثابت)
      .data_directory(app.path().app_data_dir().unwrap())
      .build()?;

      // Runtime: splash -> main
      tauri::async_runtime::spawn(async move {
        // اترك splash يظهر لحظة
        sleep(Duration::from_millis(900)).await;

        // Fade-out splash (إذا موجود بالـ splash.html)
        if let Some(s) = app_handle.get_webview_window("splash") {
          let _ = s.eval("window.__fadeOut && window.__fadeOut()");
        }

        // انتظر نهاية الفيد
        sleep(Duration::from_millis(260)).await;

        // أظهر main
        if let Some(m) = app_handle.get_webview_window("main") {
          let _ = m.show();
          let _ = m.set_focus();

          // حقن مرة واحدة بعد الظهور
          // (وأعدها مرة ثانية بعد ثانيتين فقط لالتقاط إعادة رسم مبكرة بدون loop)
          let _ = m.eval(HIDE_MIC_JS);
          sleep(Duration::from_millis(2000)).await;
          let _ = m.eval(HIDE_MIC_JS);
        }

        // أغلق splash
        if let Some(s) = app_handle.get_webview_window("splash") {
          let _ = s.close();
        }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
