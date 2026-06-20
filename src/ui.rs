//! GUI 模块
//! 基于 egui 构建截图工具的主界面

use crate::capture;
use anyhow::Result;

/// 截图模式
#[derive(Debug, Clone, Copy, PartialEq)]
enum CaptureMode {
    FullScreen,
    Region,
}

/// 刹那主应用状态
pub struct ChanaApp {
    monitors: Vec<capture::MonitorInfo>,
    selected_monitor: usize,
    capture_mode: CaptureMode,
    // 区域截图参数
    region_x: u32,
    region_y: u32,
    region_w: u32,
    region_h: u32,
    // 状态
    last_screenshot_path: Option<String>,
    status_message: String,
    is_capturing: bool,
    // 历史截图预览 (RGBA bytes, width, height)
    preview_data: Option<(Vec<u8>, u32, u32)>,
}

impl Default for ChanaApp {
    fn default() -> Self {
        let monitors = capture::list_monitors().unwrap_or_default();
        Self {
            monitors,
            selected_monitor: 0,
            capture_mode: CaptureMode::FullScreen,
            region_x: 0,
            region_y: 0,
            region_w: 400,
            region_h: 300,
            last_screenshot_path: None,
            status_message: String::from("就绪 — 选择截取方式后点击「刹那捕获」"),
            is_capturing: false,
            preview_data: None,
        }
    }
}

impl ChanaApp {
    /// 执行截图
    fn do_capture(&mut self) {
        self.is_capturing = true;
        self.status_message = "正在截图...".to_string();

        let (x, y, w, h) = match self.capture_mode {
            CaptureMode::FullScreen => (0, 0, 0, 0),
            CaptureMode::Region => (self.region_x, self.region_y, self.region_w, self.region_h),
        };

        match capture::capture_region(self.selected_monitor, x, y, w, h) {
            Ok(img) => {
                // 保存预览数据
                let (pw, ph) = (img.width(), img.height());
                let preview = img.to_vec();
                self.preview_data = Some((preview, pw, ph));

                // 保存文件
                let filename = capture::default_filename();
                let path = format!("./{}", filename);
                match capture::save_png(&img, &path) {
                    Ok(_) => {
                        self.last_screenshot_path = Some(path.clone());
                        self.status_message = format!(
                            "✓ 截图成功! {}x{} → {} (已复制到剪贴板)",
                            pw, ph, filename
                        );

                        // 尝试复制到剪贴板
                        if let Err(e) = copy_to_clipboard(&img) {
                            tracing::warn!("复制到剪贴板失败: {}", e);
                        }
                    }
                    Err(e) => {
                        self.status_message = format!("✗ 保存失败: {}", e);
                    }
                }
            }
            Err(e) => {
                self.status_message = format!("✗ 截图失败: {}", e);
            }
        }
        self.is_capturing = false;
    }
}

/// 将截图复制到系统剪贴板
fn copy_to_clipboard(img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Result<()> {
    // 转为 RGBA → BGRA 格式（macOS 剪贴板偏好 BGRA）
    let (w, h) = (img.width() as usize, img.height() as usize);
    let bgra: Vec<u8> = img
        .pixels()
        .flat_map(|p| [p.0[2], p.0[1], p.0[0], p.0[3]])
        .collect();

    let img_data = arboard::ImageData {
        width: w,
        height: h,
        bytes: std::borrow::Cow::Borrowed(&bgra),
    };

    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| anyhow::anyhow!("无法打开剪贴板: {}", e))?;

    clipboard.set_image(img_data)
        .map_err(|e| anyhow::anyhow!("无法设置剪贴板图像: {}", e))?;

    Ok(())
}

impl eframe::App for ChanaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部横幅
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚡ 刹那 Chana");
                ui.separator();
                ui.label("跨平台截图工具 · Demo v0.1.0");
            });
        });

        // 底部状态栏
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.is_capturing {
                    ui.spinner();
                }
                ui.label(&self.status_message);
            });
        });

        // 主内容区
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // ── 显示器信息 ──
                ui.collapsing("🖥 显示器信息", |ui| {
                    if self.monitors.is_empty() {
                        ui.colored_label(egui::Color32::RED, "⚠ 未检测到显示器");
                        if ui.button("⟳ 刷新").clicked() {
                            self.monitors = capture::list_monitors().unwrap_or_default();
                        }
                    } else {
                        egui::Grid::new("monitor_grid").striped(true).show(ui, |ui| {
                            ui.label("索引"); ui.label("名称"); ui.label("分辨率"); ui.label("刷新率"); ui.label("主屏");
                            ui.end_row();
                            for (i, m) in self.monitors.iter().enumerate() {
                                ui.label(i.to_string());
                                ui.label(&m.name);
                                ui.label(format!("{} × {}", m.width, m.height));
                                ui.label(format!("{:.0} Hz", m.frequency));
                                ui.label(if m.is_primary { "★" } else { "" });
                                ui.end_row();
                            }
                        });
                    }
                });

                ui.separator();

                // ── 截图模式选择 ──
                ui.heading("📸 截图模式");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.capture_mode, CaptureMode::FullScreen, "全屏截图");
                    ui.selectable_value(&mut self.capture_mode, CaptureMode::Region, "区域截图");
                });

                // 显示器选择
                if !self.monitors.is_empty() {
                    ui.horizontal(|ui| {
                        ui.label("目标显示器:");
                        egui::ComboBox::from_id_salt("monitor_select")
                            .selected_text(format!("{} — {}", self.selected_monitor, self.monitors.get(self.selected_monitor).map(|m| m.name.as_str()).unwrap_or("?")))
                            .show_ui(ui, |ui| {
                                for (i, m) in self.monitors.iter().enumerate() {
                                    ui.selectable_value(&mut self.selected_monitor, i, format!("{}: {} ({}x{})", i, m.name, m.width, m.height));
                                }
                            });
                    });
                }

                // 区域截图参数
                if self.capture_mode == CaptureMode::Region {
                    ui.separator();
                    ui.label("区域参数:");
                    egui::Grid::new("region_params").show(ui, |ui| {
                        ui.label("X:"); ui.add(egui::DragValue::new(&mut self.region_x).speed(1).clamp_range(0..=10000)); ui.end_row();
                        ui.label("Y:"); ui.add(egui::DragValue::new(&mut self.region_y).speed(1).clamp_range(0..=10000)); ui.end_row();
                        ui.label("宽:"); ui.add(egui::DragValue::new(&mut self.region_w).speed(1).clamp_range(1..=10000)); ui.end_row();
                        ui.label("高:"); ui.add(egui::DragValue::new(&mut self.region_h).speed(1).clamp_range(1..=10000)); ui.end_row();
                    });
                }

                ui.separator();

                // ── 捕获按钮 ──
                ui.add_sized(
                    [200.0, 40.0],
                    egui::Button::new("⚡ 刹那捕获")
                        .fill(egui::Color32::from_rgb(233, 69, 96)),
                ).clicked().then(|| {
                    self.do_capture();
                });

                if !self.is_capturing {
                    ui.label("快捷键: Ctrl+Shift+A (待实现)");
                }

                ui.separator();

                // ── 预览区 ──
                ui.heading("🖼 截图预览");
                if let Some((ref data, w, h)) = self.preview_data {
                    // 计算显示尺寸（按比例缩放以适应UI）
                    let max_w = ui.available_width().min(800.0);
                    let scale = (max_w / w as f32).min(500.0 / h as f32).min(1.0);
                    let disp_w = (w as f32 * scale).max(1.0);
                    let disp_h = (h as f32 * scale).max(1.0);

                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        [w as usize, h as usize],
                        data,
                    );

                    let texture = ctx.load_texture(
                        "screenshot_preview",
                        color_image,
                        egui::TextureOptions::LINEAR,
                    );

                    ui.image(egui::ImageSource::Texture(
                        egui::load::SizedTexture::new(&texture, [disp_w, disp_h]),
                    ));

                    ui.label(format!("原始尺寸: {} × {} | 显示缩放: {:.0}%", w, h, scale * 100.0));

                    if let Some(ref path) = self.last_screenshot_path {
                        ui.label(format!("已保存: {}", path));
                    }
                } else {
                    ui.add_sized(
                        [ui.available_width(), 200.0],
                        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(80.0);
                                ui.label("还没有截图，点击「⚡ 刹那捕获」开始");
                            });
                        }),
                    );
                }

                ui.separator();

                // ── 说明 ──
                ui.collapsing("📖 使用说明", |ui| {
                    ui.label("1. 在上方选择截图模式（全屏/区域）");
                    ui.label("2. 如需区域截图，设置 X/Y/宽/高 参数");
                    ui.label("3. 点击「刹那捕获」按钮");
                    ui.label("4. 截图自动保存到当前目录，并复制到剪贴板");
                    ui.label("5. 这个 Demo 仅演示核心管道，完整交互将在后续版本实现");
                    ui.add_space(8.0);
                    ui.label("ℹ️ macOS 用户若遇到权限问题，请到「系统设置 → 隐私与安全性 → 屏幕录制」授予终端权限");
                });
            });
        });
    }
}
