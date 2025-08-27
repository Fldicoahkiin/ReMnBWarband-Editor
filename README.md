# ReMnBWarband Editor (R球编辑器)

一个现代化的骑马与砍杀：战团 MOD 编辑工具，基于原版魔球编辑器重构，使用 Rust + Slint 原生技术栈构建。

**项目名称：** ReMnBWarband Editor  
**中文名称：** R球编辑器  
**版本：** 0.1.0  
**原版致敬：** 魔球编辑器 by SSgt_Edward & Ser_Charles

## 项目简介

R球编辑器是对经典的魔球编辑器(MnBWarband Editor)的现代化重构，采用Rust + Slint原生技术栈，为《骑马与砍杀：战团》MOD制作提供高性能、跨平台的编辑工具。

### 核心特性

- 🚀 **原生性能**: 基于Rust和Slint的原生GUI，零Web开销
- 🔒 **内存安全**: Rust所有权系统保证内存安全和并发安全
- 🌍 **跨平台**: 原生支持Windows、macOS、Linux
- 🐍 **Python集成**: 内置Python 2.7解释器，完整支持Module System
- 🎨 **现代UI**: 保留原版精神的现代化界面设计
- 📦 **完整功能**: 物品、兵种、触发器、Python脚本编辑
- 🔍 **智能检测**: 自动检测游戏安装路径和模组

## 技术栈

- **后端语言**: Rust 1.70+
- **UI框架**: Slint 1.3+ (原生GUI)
- **原生菜单**: muda 0.11+ (跨平台原生菜单栏)
- **Python集成**: PyO3 0.20+ (嵌入Python 2.7解释器)
- **数据序列化**: serde + chrono
- **构建工具**: Cargo

## 项目结构

```
ReMnBWarband-Editor/
├── src/                          # Rust 源码
│   ├── main.rs                   # 主入口点
│   ├── core/                     # 核心引擎模块
│   │   ├── app_manager.rs        # 应用管理器
│   │   ├── file_manager.rs       # 文件管理器
│   │   ├── menu_manager.rs       # 原生菜单管理器
│   │   ├── python_manager.rs     # Python集成管理
│   │   ├── game_detector.rs      # 游戏检测器
│   │   ├── validator.rs          # 数据验证器
│   │   └── parser/               # 数据解析器
│   ├── models/                   # 数据模型
│   │   ├── item.rs              # 物品模型
│   │   ├── troop.rs             # 兵种模型
│   │   ├── faction.rs           # 派系模型
│   │   └── trigger.rs           # 触发器模型
│   └── utils/                    # 工具函数
├── ui/                           # Slint UI 文件
│   ├── main.slint               # 主界面
│   └── components/              # UI组件
├── assets/                       # 静态资源
│   └── Images/                  # 图片资源(已迁移原版素材)
├── docs/                        # 文档
├── old_docs/                    # 原版技术资料
└── R球编辑器核心技术文档.md      # 核心技术文档
```

## 快速开始

### 环境要求
- Rust 1.70+
- Python 2.7+ (用于Module System兼容性)
- macOS 10.15+ / Windows 10+ / Linux (现代发行版)

### 编译运行
```bash
# 克隆项目
git clone <repository_url>
cd ReMnBWarband-Editor

# 设置Python兼容性环境变量
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1

# 编译项目
cargo build

# 运行项目
cargo run
```

详细的开发环境搭建指南请参考：[开发环境搭建](./docs/dev_setup.md)

## 支持的游戏路径

项目支持自动检测以下平台的骑砍安装：

### macOS
- Steam: `~/Library/Application Support/Steam/steamapps/common/MountBlade Warband`
- App Store: `/Applications/Mount&Blade Warband.app`

### Windows  
- Steam: `C:\Program Files (x86)\Steam\steamapps\common\MountBlade Warband`
- GOG: `C:\GOG Games\Mount & Blade Warband`

### Linux
- Steam: `~/.steam/steam/steamapps/common/MountBlade Warband`

### 模组支持
- 原版模组: `Modules/` 目录自动扫描
- Steam创意工坊: 自动解析 `steam_workshop_items.xml`

## 文档资源

- **[核心技术文档](./R球编辑器核心技术文档.md)** - 项目整体架构和技术选型
- **[开发指南](./docs/dev_setup.md)** - 开发环境和使用指南
- **[技术资料](./old_docs/)** - 原版VB6技术分析和格式规范

## 致敬原版

R球编辑器是对原版魔球编辑器的现代化重构，我们深深感谢原版开发团队：

- **主程序开发**: SSgt_Edward, Ser_Charles
- **美工设计**: 我不是个过客(hp_honey)
- **各模块负责人**: 详见应用内关于界面

新版本在保持原版精神的基础上，采用现代化技术栈重新实现，提供更好的性能和用户体验。

## 贡献指南

1. 阅读核心技术文档了解项目架构
2. 按照开发环境搭建指南配置环境
3. 参考技术资料了解文件格式和业务逻辑
4. 提交PR前请运行测试：`cargo test`

---

**R球编辑器** - 延续魔球传奇，开启MOD制作新纪元
