//! 屏幕捕获模块
//! 基于 xcap 库，抽象跨平台屏幕捕获

use anyhow::{Context, Result};
use image::ImageBuffer;

/// 显示器信息
#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub frequency: f32,
    pub is_primary: bool,
}

/// 列出所有显示器
pub fn list_monitors() -> Result<Vec<MonitorInfo>> {
    let monitors = xcap::Monitor::all()
        .context("无法枚举显示器，请检查屏幕捕获权限")?;

    let infos: Vec<MonitorInfo> = monitors
        .iter()
        .map(|m| MonitorInfo {
            name: m.name().to_string(),
            width: m.width(),
            height: m.height(),
            frequency: m.frequency() as f32,
            is_primary: m.is_primary(),
        })
        .collect();

    Ok(infos)
}

/// 捕获指定区域的截图
/// - monitor_index: 显示器索引 (从0开始)
/// - x, y: 相对于该显示器的起始坐标
/// - width, height: 捕获区域的宽高（为0则捕获整个显示器）
pub fn capture_region(
    monitor_index: usize,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
    let monitors = xcap::Monitor::all()?;
    let monitor = monitors
        .get(monitor_index)
        .context(format!("显示器 {} 不存在", monitor_index))?;

    let img = monitor.capture_image()
        .context("截图失败，请确认屏幕捕获权限已授予")?;

    // xcap 返回的像素格式是 BGRA，转为 RGBA ImageBuffer
    let (w, h) = (img.width(), img.height());
    let rgba_data: Vec<u8> = img
        .as_raw()
        .chunks(4)
        .flat_map(|pixel| [pixel[2], pixel[1], pixel[0], pixel[3]])
        .collect();

    let full_img: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(w, h, rgba_data)
            .context("无法构造图像缓冲")?;

    // 如果指定了区域，进行裁剪
    let actual_w = if width == 0 { w } else { width };
    let actual_h = if height == 0 { h } else { height };
    let actual_x = x.min(w.saturating_sub(1));
    let actual_y = y.min(h.saturating_sub(1));

    if actual_w != w || actual_h != h || x != 0 || y != 0 {
        let cropped = image::imageops::crop_imm(&full_img, actual_x, actual_y, actual_w, actual_h);
        Ok(cropped.to_image())
    } else {
        Ok(full_img)
    }
}

/// 保存截图为 PNG 文件
pub fn save_png(
    img: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    path: &str,
) -> Result<()> {
    img.save(path)
        .context(format!("无法保存截图到 {}", path))?;
    tracing::info!("截图已保存: {}", path);
    Ok(())
}

/// 生成默认截图文件名
pub fn default_filename() -> String {
    let now = chrono::Local::now();
    format!("chana_{}.png", now.format("%Y%m%d_%H%M%S"))
}
