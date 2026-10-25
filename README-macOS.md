# NekoSportsWorldTool macOS 使用说明（Apple Silicon）

本文面向 Apple Silicon Mac 用户，说明如何安装、首次启动、登录、配置、使用和更新 NekoSportsWorldTool。

当前推荐的安装包是：

```text
NekoSportsWorldTool-macos-arm64-app.zip
```

它适用于 Apple Silicon 芯片的 Mac，包括 M1、M2、M3、M4 及后续 Apple Silicon 系列。这个版本是原生 arm64 程序，不需要 Rosetta 2，也不需要在 Mac 上安装 Rust、Xcode 或 Homebrew。

> 注意：本项目仅供学习和研究。请遵守学校、平台和当地规则，不要用于违反校规或服务条款的场景。

## 1. 使用前确认

### 1.1 确认 Mac 是 Apple Silicon

在 macOS 左上角打开：

```text
苹果菜单  -> 关于本机
```

查看“芯片”一栏：

- 显示 `Apple M1`、`Apple M2`、`Apple M3`、`Apple M4` 等：使用 `arm64` 包。
- 显示 `Intel`：不要使用本文的 arm64 包，应下载 `x64` 包。

也可以在终端执行：

```bash
uname -m
```

输出 `arm64` 即可使用本文安装包。

### 1.2 系统版本

本次远程构建的最低目标系统为 macOS 11.0。建议使用较新的 macOS 版本。项目目前主要完成了远程构建、单元测试、Mach-O 架构检查、应用包检查和临时签名验证；不同 macOS 版本的 Gatekeeper 提示文字可能略有差异。

### 1.3 网络和账号

首次登录需要网络连接。应用会访问项目配置的运动服务接口、学校路网接口以及更新检查地址。请确保 Mac 可以正常访问这些服务。

## 2. 获取 Apple Silicon 安装包

### 2.1 使用已经下载的安装包

当前已生成的安装包文件名为：

```text
NekoSportsWorldTool-macos-arm64-app.zip
```

如果你从 GitHub Actions 下载，通常会先得到一个外层构建产物压缩包，文件名类似：

```text
NekoSportsWorldTool-macos-arm64.zip
```

解压外层压缩包后，再找到并解压其中的：

```text
NekoSportsWorldTool-macos-arm64-app.zip
```

最终应当看到：

```text
NekoSportsWorldTool.app
```

不要直接运行外层构建产物，也不要把 `.app` 文件当作普通目录复制到聊天工具后再运行。优先使用 ZIP 文件解压，以便保留应用内部的可执行权限和签名资源。

### 2.2 从 GitHub Actions 下载

1. 打开仓库的 GitHub 页面并登录 GitHub。
2. 进入 `Actions`。
3. 选择 `macOS` 工作流。
4. 打开一个结论为 `Success` 的运行记录。
5. 滚动到页面底部的 `Artifacts`。
6. 下载 `NekoSportsWorldTool-macos-arm64`。
7. 先解压外层构建产物，再解压其中的 `NekoSportsWorldTool-macos-arm64-app.zip`。

GitHub Actions 构建产物默认只保留 30 天。若页面中已经没有对应构建产物，需要重新运行工作流或使用仓库中新的成功构建。

## 3. 安装到“应用程序”

1. 在“下载”文件夹中双击 `NekoSportsWorldTool-macos-arm64-app.zip`。
2. 得到 `NekoSportsWorldTool.app` 后，将它拖到“应用程序”文件夹。
3. 从“应用程序”文件夹启动应用。

推荐的最终路径是：

```text
/Applications/NekoSportsWorldTool.app
```

不要把应用长期放在 ZIP 压缩包内运行，也不要通过 `sudo` 启动。应用使用的是当前登录用户的配置目录，使用管理员权限运行可能造成配置文件归属不一致。

## 4. 首次启动和 Gatekeeper

当前安装包使用 Apple 的临时签名（ad-hoc signing），没有使用 Apple Developer 证书进行公证。因此，第一次打开时 macOS 可能提示：

- 无法验证开发者；
- 应用来自身份不明的开发者；
- macOS 已阻止打开此应用。

如果你确认安装包来自可信的仓库构建记录，可以按下面的方式允许本次应用运行：

1. 先在“应用程序”中双击一次应用，让 macOS 生成拦截记录。
2. 打开：`系统设置 -> 隐私与安全性`。
3. 在安全性区域找到关于 NekoSportsWorldTool 被阻止的提示。
4. 点击“仍要打开”或同等含义的按钮。
5. 回到应用，再次启动并在确认窗口中选择打开。

不同 macOS 版本的菜单名称可能显示为“隐私与安全性”或“安全性与隐私”。

不要为了运行程序而关闭系统完整性保护、全局关闭 Gatekeeper，或对来源不明的文件盲目执行去隔离命令。如果系统提示文件已损坏或怀疑文件被篡改，建议重新下载并先进行校验；如果出现明确的恶意软件警告，应停止运行并检查构建来源。

## 5. 校验下载文件

构建产物中包含 SHA-256 校验文件。终端进入下载目录后，可以先计算 ZIP 的摘要：

```bash
cd ~/Downloads
shasum -a 256 NekoSportsWorldTool-macos-arm64-app.zip
```

然后将输出的摘要与构建产物中的 `.sha256` 文件进行比较。当前这次构建的 arm64 应用包摘要是：

```text
497e86afa718269df2b80839bdc5ba2247ddda7f6193a8e43d4558f4fc3036a8
```

该摘要只对应当前构建的 `NekoSportsWorldTool-macos-arm64-app.zip`。以后重新构建或重新下载其他版本时，应以对应构建产物中的 `.sha256` 文件为准。

如果希望检查解压后的应用结构：

```bash
file /Applications/NekoSportsWorldTool.app/Contents/MacOS/nekosportsworldtool
codesign --verify --deep --strict --verbose=2 /Applications/NekoSportsWorldTool.app
```

Apple Silicon 包的 `file` 输出应包含 `arm64`。`codesign` 校验成功时会输出验证通过的信息。

## 6. 第一次打开后的设置

### 6.1 关闭自动更新检查

当前 macOS 安装包来自 `Two-Brush/NekoSportsWorldTool` fork，而桌面端自动更新逻辑默认面向上游仓库。为了避免应用后续检查到不适合当前 fork 的 Release，建议首次打开后立即关闭启动检查：

1. 打开应用顶部的“关于”页。
2. 找到“启动时检查更新”。
3. 选择“关闭”。
4. 确认页面出现设置已保存的提示。

“关于”页中的“检查更新”是手动检查入口。除非你确认更新来源和目标仓库正确，否则不要直接使用“立即更新”。手动替换版本时，应替换完整的 `.app`，不要只替换 `Contents/MacOS/nekosportsworldtool`，否则可能破坏应用包结构或签名。

### 6.2 登录

应用启动后，在顶部的“账号与登录”区域填写：

- 账号；
- 密码；
- 是否记住密码。

点击“登录”。登录成功后，页面会显示当前用户信息，并可以使用“登出”。如果会话过期，先重新登录，再重试需要联网的操作。

勾选“记住密码”会把账号和密码保存到本机应用数据目录的 `config.json` 中。该文件不是 macOS 钥匙串格式，属于应用自己的本地配置，因此不要把整个配置目录上传、提交到 Git 或发送给其他人。

### 6.3 学校和定位信息

首次使用时，根据页面提示选择学校、校区和运动区域。设备信息页包含网络身份和定位锚点等设置：

- 设备身份属于持久化配置，不要频繁随机生成或反复更换；
- 定位锚点请按页面提示填写，先确认纬度，再确认经度；
- 修改城市、校区或定位后，必须点击“保存”；
- 不要因为当前电脑是 Apple Silicon，就把设备信息切换成 Android，Apple Silicon 只是 Mac 的 CPU 架构，不是应用内的手机身份设置。

如果学校路网无法加载，先检查登录状态、学校选择、定位是否正确以及网络连接，再进入“路网”页重试。

## 7. 图形界面主要功能

应用窗口默认大小约为 880 x 720，顶部有以下页面：

### 跑步

用于准备和提交跑步流程。常见参数包括：

- 距离范围（公里）；
- 配速范围（秒/公里）；
- 手动海拔；
- 开始时间；
- 追溯天数；
- 路线模式。

留空手动海拔时使用自动海拔。填写单个数值时使用固定海拔，填写类似 `11.6-22.8` 的范围时使用范围内的海拔曲线。

“计划预览”和路线预览用于检查参数。“开始跑步”会进入实际提交流程，执行前应确认账号、学校、距离、时间和路线都正确。预览或本地 GPX 分析不等于服务器成绩提交，也不代表学校系统一定接受结果。

### AI 运动

用于查看和执行 AI 运动相关任务。涉及批量操作时，应先检查任务项目、次数和分数参数，再确认提交。

### 运动记录

用于查看已经获取到的运动记录。网络请求失败时，先检查会话是否过期。

### 数据

用于查看学期完成度、个人数据、违规自查和排行榜等信息。页面为空不一定表示程序故障，也可能是服务端当前没有返回有效数据。

### 我的

用于查看当前用户相关信息。

### 设备信息

用于查看和保存设备身份、设备型号、系统版本、城市和定位锚点。除非确实需要迁移设备或修复配置，不要频繁修改设备身份。

### 运行日志

用于查看当前运行期间的应用日志。日志主要保存在内存中，应用退出后不保证保留完整内容；需要反馈问题时，应在问题发生后立即复制相关日志，并遮盖账号、手机号、令牌、坐标和其他个人信息。

### 路网

用于获取学校路网、加载本地 OSM 文件和预览道路路线。支持常见的 OSM XML 文件，例如：

```text
.osm
.xml
```

当前应用不直接把 `.pbf` 或任意 JSON 文件当作 OSM XML 读取。若使用其他格式，需要先转换成应用支持的格式。Mac 上选择文件时使用 Mac 路径，例如：

```text
/Users/你的用户名/Downloads/school.osm
```

不要继续使用 Windows 路径，例如 `C:\Users\...`。

### 关于

用于查看版本、手动检查更新和设置启动更新检查模式。Mac 用户建议在这里选择“关闭”启动时更新检查。

## 8. 真实 GPX 的本地分析

“导入真实 GPX 并分析”只用于本地查看采样点、海拔范围、累计爬升和累计下降等信息，不会自动把 GPX 上传为成绩，也不会把它直接接入跑步提交流程。

GPX 坐标按 WGS84 读取。导入前确认文件来源可靠，并注意 GPX 可能包含精确位置、时间和个人活动信息。

## 9. macOS 数据目录

macOS 版本的配置和运行数据保存在：

```text
~/Library/Application Support/NekoSportsWorldTool/
```

在 Finder 中打开：

1. 按 `Command + Shift + G`；
2. 输入上面的路径；
3. 按回车。

也可以在终端执行：

```bash
open "$HOME/Library/Application Support/NekoSportsWorldTool"
```

常见文件和目录包括：

| 文件或目录 | 作用 |
| --- | --- |
| `config.json` | 账号记忆、登录偏好、距离、配速、海拔、路网路径和更新设置 |
| `identity.json` | 持久化设备身份和设备相关信息 |
| `session.json` | 当前登录会话，包含登录状态数据 |
| `points_cache.json` | 定位点等短期缓存 |
| `ai_sports.json` | AI 运动项目缓存 |
| `school_roads/` | 学校路网缓存 |
| `submission_receipts/` | 提交去重记录，避免重复提交 |
| `diagnostic-*.json` | 本地诊断导出文件 |

不要直接修改这些 JSON 文件，除非你清楚字段含义并且已经备份。尤其不要删除 `identity.json` 或 `submission_receipts/` 来“解决”普通网络问题；这可能导致设备身份变化或重复提交保护失效。

## 10. 备份和迁移

### 10.1 备份

退出应用后，可以把整个数据目录备份到桌面：

```bash
ditto "$HOME/Library/Application Support/NekoSportsWorldTool" \
  "$HOME/Desktop/NekoSportsWorldTool-backup-$(date +%Y%m%d-%H%M%S)"
```

备份目录包含隐私信息和会话信息，不要上传到公共仓库或发送给他人。

### 10.2 从旧版本迁移

1. 退出旧版本和新版本。
2. 先按上一节的方法备份当前 Mac 数据目录。
3. 将旧版本的数据文件复制到 macOS 目录中。
4. 启动新版本并检查账号、设备信息、学校和定位。
5. 如果会话已过期，重新登录。

迁移时只复制配置和运行数据，不要把 Windows 的 `exe`、整个旧程序目录或不相关的缓存目录复制成 Mac 应用。旧版本中的 OSM 路径如果是 Windows 路径，需要在“路网”页重新选择 Mac 上的文件。

如果无法确认旧数据是否兼容，先保留完整备份，不要直接覆盖当前数据。继续使用同一账号时，至少应一起保留以下配置和去重记录；缓存可以由程序重新获取：

```text
config.json
identity.json
session.json
submission_receipts/
```

迁移 `session.json` 后仍可能需要重新登录，这是正常现象。

## 11. 手动更新

推荐的更新流程是：

1. 退出正在运行的 NekoSportsWorldTool。
2. 下载新的 `NekoSportsWorldTool-macos-arm64-app.zip`。
3. 校验 SHA-256。
4. 解压得到新的 `NekoSportsWorldTool.app`。
5. 将旧的应用移到废纸篓，或先改名留作回滚。
6. 把新的应用拖入“应用程序”文件夹。
7. 启动新版本并检查数据是否仍然存在。

应用数据在 `~/Library/Application Support/NekoSportsWorldTool/`，正常替换 `.app` 不会删除这些数据。不要只替换 `.app/Contents/MacOS/nekosportsworldtool`，也不要把一个版本的可执行文件覆盖到另一个版本的应用包中。

如果更新后 macOS 再次提示安全确认，按“首次启动和 Gatekeeper”一节操作。若更新后的应用打不开，先恢复旧的完整 `.app`，不要删除数据目录。

## 12. 命令行使用

普通用户只需要双击 `.app`。如果需要使用 CLI，应用内的可执行文件位于：

```text
/Applications/NekoSportsWorldTool.app/Contents/MacOS/nekosportsworldtool
```

建议始终给路径加双引号：

```bash
APP_BIN="/Applications/NekoSportsWorldTool.app/Contents/MacOS/nekosportsworldtool"
"$APP_BIN" help
```

常用命令示例：

```bash
APP_BIN="/Applications/NekoSportsWorldTool.app/Contents/MacOS/nekosportsworldtool"

# 查看完整命令帮助
"$APP_BIN" help

# 读取已有运动记录
"$APP_BIN" records

# 查看学期信息
"$APP_BIN" semester

# 使用本地 GPX 或 JSON 做模板分析，不上传文件
"$APP_BIN" template --file "$HOME/Downloads/run.gpx"
```

不带参数运行可启动 GUI：

```bash
"$APP_BIN"
```

CLI 和 GUI 使用同一个 macOS 数据目录，因此不要在多个版本之间交叉使用不同来源的可执行文件。`login --pass <密码>` 可能会把密码留在终端历史记录中，日常使用建议通过 GUI 登录；如果必须使用 CLI，使用后及时清理终端历史记录，并注意本地配置仍可能保存登录信息。

## 13. 常见问题

### 双击没有反应

先从终端启动一次，以便看到错误信息：

```bash
"/Applications/NekoSportsWorldTool.app/Contents/MacOS/nekosportsworldtool"
```

如果终端中出现 GUI 启动错误，保存错误文本。也可以检查：

```bash
sw_vers
file /Applications/NekoSportsWorldTool.app/Contents/MacOS/nekosportsworldtool
codesign --verify --deep --strict --verbose=2 /Applications/NekoSportsWorldTool.app
```

### 提示不是 Apple Silicon 程序

确认使用的是 `arm64` 包，并检查 `file` 输出是否包含 `arm64`。如果输出是 `x86_64`，说明拿到了 Intel 包；如果无法识别，说明解压或下载过程可能损坏。

### 提示“应用已损坏”

先重新下载并比较 SHA-256。确认完整解压了内层 `*-app.zip`，不要从外层 Actions 压缩包中直接拖出不完整的 `.app`。如果仍然提示损坏，同时检查 Gatekeeper 和签名输出；不要对不确定来源的文件强行绕过系统保护。

### 登录失败或提示 401

检查网络，确认账号密码正确，并尝试在 GUI 中重新登录。若很久没有使用，旧会话可能已经过期。不要先删除 `identity.json`；设备身份问题和登录会话问题是两类不同问题。

### 路网为空或无法下载

依次检查：

1. 是否已经登录；
2. 学校和校区是否选择正确；
3. 设备信息页的城市、纬度、经度是否已经保存；
4. 网络是否能访问服务；
5. “路网”页是否需要点击“重试获取路网”。

### 更新后配置不见了

确认打开的是 `/Applications/NekoSportsWorldTool.app`，并检查：

```text
~/Library/Application Support/NekoSportsWorldTool/
```

如果你曾经用终端直接运行过另一个目录中的旧二进制文件，两个程序可能共享数据目录但版本不同。关闭所有实例后，再从应用程序文件夹启动，并保留更新前的备份。

### 需要提交问题反馈

请提供以下信息：

- macOS 版本：`sw_vers` 输出；
- Mac 架构：`uname -m` 输出；
- 应用版本；
- 出错页面和操作步骤；
- 已脱敏的运行日志或终端错误。

请先遮盖账号、密码、手机号、登录令牌、精确坐标、设备 ID、MAC 地址和 GPX 内容。不要直接打包上传整个 `~/Library/Application Support/NekoSportsWorldTool/` 目录。

## 14. 卸载

只卸载程序：

1. 退出应用；
2. 在“应用程序”中将 `NekoSportsWorldTool.app` 移到废纸篓；
3. 保留 `~/Library/Application Support/NekoSportsWorldTool/`，以后重新安装仍可继续使用配置。

同时删除本地数据：

1. 先按备份章节备份整个数据目录；
2. 在 Finder 中打开 `~/Library/Application Support/`；
3. 将 `NekoSportsWorldTool` 文件夹移到废纸篓；
4. 清空废纸篓前再次确认其中没有需要保留的配置、会话、路网或提交记录。

删除数据会清除登录会话、记住的账号密码、设备身份、缓存和提交去重记录。重新安装后，应用会生成新的本地数据目录。

## 15. 当前构建信息

本说明对应的 Apple Silicon 构建信息：

```text
仓库：Two-Brush/NekoSportsWorldTool
源码提交：fd79b1ddcf78d4b2d75e544c8157cc308fcf7f86
GitHub Actions 运行编号：40171412954（Success）
架构：aarch64-apple-darwin / arm64
最低目标系统：macOS 11.0
打包格式：NekoSportsWorldTool.app + ZIP
签名：ad-hoc，未进行 Apple 公证
```

使用时以对应 GitHub Actions 成功构建中的文件和校验值为准。README 只说明使用方式，不会替代构建产物本身。
