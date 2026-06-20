//! 刹那 (Chana) — 跨平台截图工具 Demo
//!
//! 当前版本: v0.1.0 — 屏幕捕获 + GUI 基础骨架
//! 运行: cargo run

mod capture;
mod ui;

use anyhow::Result;
use tracing_subscriber;

fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("chana=debug".parse().unwrap())
        )
        .init();

    tracing::info!("刹那 (Chana) v{} 启动中...", env!("CARGO_PKG_VERSION"));

    // 枚举显示器
    let monitors = capture::list_monitors()?;
    tracing::info!("检测到 {} 个显示器", monitors.len());
    for (i, m) in monitors.iter().enumerate() {
        tracing::info!(
            "  显示器 {}: {} ({}x{} @ {:.0}Hz)",
            i, m.name, m.width, m.height, m.frequency
        );
    }

    // 启动 GUI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 650.0])
            .with_title("刹那 Chana — 截图工具 Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "刹那 Chana",
        options,
        Box::new(|_cc| Ok(Box::new(ui::ChanaApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("GUI 启动失败: {}", e))?;

    Ok(())
}
