//! 中文字体安装。
//!
//! 桌面端按平台在系统字体目录查找中文字体注入 egui：
//! - Windows: 微软雅黑 / 黑体 / 宋体
//! - macOS:   苹方 / 华文黑体 / 冬青黑体
//! - Linux:   Noto Sans CJK / 文泉驿
//! 全部失败时兜底加载内置的 NotoSansSC 字体，保证任何平台中文都能正常显示。
//! 仍失败返回 None（不崩溃，界面中文会显示为方块但不影响功能）。

#[cfg(not(target_os = "android"))]
use std::path::PathBuf;

/// 内置兜底中文字体（全平台可用）。
#[cfg(not(target_os = "android"))]
const FALLBACK_FONT: &[u8] = include_bytes!("../../assets/fonts/NotoSansSC-VF.ttf");

#[cfg(target_os = "android")]
const ANDROID_FONT: &[u8] = include_bytes!("../../assets/fonts/NotoSansSC-VF.ttf");

/// 字体候选：(所在目录, 文件名)。
/// macOS 中文按优先级：苹方 > 冬青黑体 > 华文黑体。
#[cfg(not(target_os = "android"))]
const CANDIDATES: &[(&str, &str)] = &[
    // Windows
    ("msyh.ttc", ""),
    ("msyh.ttf", ""),
    ("simhei.ttf", ""),
    ("simsun.ttc", ""),
    // macOS 系统字体
    ("PingFang.ttc", ""),
    ("Hiragino Sans GB.ttc", ""),
    ("STHeiti Light.ttc", ""),
    ("STHeiti Medium.ttc", ""),
    ("Songti.ttc", ""),
    // Linux 常见 CJK
    ("NotoSansCJK-Regular.ttc", ""),
    ("wqy-microhei.ttc", ""),
];

/// 返回当前平台的系统字体目录列表（按查找顺序）。
#[cfg(not(target_os = "android"))]
fn font_dirs() -> Vec<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let dir = std::env::var("WINDIR")
            .map(|w| PathBuf::from(w).join("Fonts"))
            .unwrap_or_else(|_| PathBuf::from(r"C:\Windows\Fonts"));
        return vec![dir];
    }
    #[cfg(target_os = "macos")]
    {
        let mut v = vec![
            PathBuf::from("/System/Library/Fonts"),
            PathBuf::from("/Library/Fonts"),
        ];
        if let Ok(home) = std::env::var("HOME") {
            v.push(PathBuf::from(home).join("Library/Fonts"));
        }
        // 部分系统把 CJK 字体放在 Supplemental 子目录
        v.push(PathBuf::from("/System/Library/Fonts/Supplemental"));
        return v;
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        return vec![
            PathBuf::from("/usr/share/fonts"),
            PathBuf::from("/usr/local/share/fonts"),
        ];
    }
}

/// 把字体数据注入 egui（proportional 置前，monospace 追加）。
#[cfg(not(target_os = "android"))]
fn inject(ctx: &egui::Context, name: &str, data: egui::FontData) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(name.to_string(), data);
    if let Some(list) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        list.insert(0, name.to_string());
    }
    if let Some(list) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        list.push(name.to_string());
    }
    ctx.set_fonts(fonts);
}

/// 安装中文字体到 egui。返回实际加载的字体名（None = 全部失败）。
pub fn install(ctx: &egui::Context) -> Option<String> {
    #[cfg(target_os = "android")]
    {
        let name = "Noto Sans SC".to_owned();
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            .insert(name.clone(), egui::FontData::from_static(ANDROID_FONT));
        if let Some(list) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            list.insert(0, name.clone());
        }
        if let Some(list) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
            list.push(name.clone());
        }
        ctx.set_fonts(fonts);
        return Some(name);
    }

    #[cfg(not(target_os = "android"))]
    {
        // ① 先按平台查找系统中文字体
        for dir in font_dirs() {
            for (file, _) in CANDIDATES {
                let path = dir.join(file);
                let Ok(data) = std::fs::read(&path) else {
                    continue;
                };
                inject(ctx, file, egui::FontData::from_owned(data));
                return Some(file.to_string());
            }
        }
        // ② 系统字体全部缺失时，兜底内置字体（保证中文可读）
        let name = "Noto Sans SC (内置)";
        inject(ctx, name, egui::FontData::from_static(FALLBACK_FONT));
        Some(name.to_string())
    }
}
