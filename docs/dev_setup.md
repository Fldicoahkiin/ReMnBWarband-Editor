# R球编辑器开发文档

## 项目概述

R球编辑器是基于原版MnBWarband Editor的现代化重构版本，采用Rust + Slint技术栈开发。本编辑器用于《骑马与砍杀：战团》游戏MOD内容的编辑，支持物品、兵种、触发器、Python脚本等各种游戏元素的编辑。

## 技术栈

### 核心技术
- **后端语言**: Rust 1.70+
- **UI框架**: Slint 1.3+ (原生GUI)
- **Python集成**: PyO3 0.20+ (嵌入Python解释器)
- **构建工具**: Cargo
- **跨平台支持**: Windows、macOS、Linux

### 开发工具
- **IDE**: Visual Studio Code + Rust Analyzer
- **版本控制**: Git
- **UI设计**: Slint Language Server

## 当前项目状态

### 已完成功能
- ✅ **核心架构**: 完整的Rust + Slint项目结构
- ✅ **数据模型**: Item、Troop、Faction、Trigger等核心数据结构
- ✅ **解析器系统**: 支持游戏数据文件解析和验证
- ✅ **Python集成**: PyO3集成Python解释器，支持脚本执行
- ✅ **UI组件**: 模块化UI设计，包含欢迎界面、编辑器、关于界面
- ✅ **图片素材**: 完整迁移原项目图片资源并分类整理
- ✅ **原作者致敬**: 保留原版开发团队信息和项目历史

### 核心模块
1. **AppManager**: 统一应用管理器，集成所有核心功能
2. **DataManager**: 数据管理，包含ItemManager、TroopManager、TriggerManager
3. **PythonManager**: Python脚本执行和模板管理
4. **FileManager**: 文件读写和路径管理
5. **DataValidator**: 数据完整性验证

## 快速开始

### 环境要求
- Rust 1.70+
- Python 3.8+ (用于PyO3集成)

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

## 项目结构

### 当前目录结构
```
ReMnBWarband-Editor/
├── src/                          # Rust 源码
│   ├── main.rs                   # 主入口点
│   ├── core/                     # 核心引擎模块
│   │   ├── mod.rs               # 模块声明
│   │   ├── app_manager.rs       # 应用管理器
│   │   ├── file_manager.rs      # 文件管理器
│   │   ├── python_manager.rs    # Python集成管理
│   │   └── parser/              # 数据解析器
│   ├── models/                   # 数据模型
│   │   ├── mod.rs               # 模块声明
│   │   ├── item.rs              # 物品数据模型
│   │   ├── faction.rs           # 派系数据模型
│   │   └── ...                  # 其他数据模型
│   └── utils/                    # 工具函数
│       ├── mod.rs               # 模块声明
│       ├── constants.rs         # 常量定义
│       └── encoding.rs          # 编码工具
├── ui/                          # Slint UI 文件
│   ├── main.slint              # 主界面
│   └── components/             # UI组件
│       ├── welcome.slint       # 欢迎界面
│       ├── about.slint         # 关于界面
│       ├── item_editor.slint   # 物品编辑器
│       ├── python_editor.slint # Python编辑器
│       └── ...                 # 其他组件
├── assets/                      # 静态资源
│   └── Images/                 # 图片资源
│       ├── backgrounds/        # 背景图片
│       ├── icons/             # 图标文件
│       ├── ui_elements/       # UI元素
│       ├── game_themes/       # 游戏主题图片
│       └── logos/             # Logo文件
├── docs/                       # 文档
├── build.rs                    # 构建脚本
├── Cargo.toml                  # 依赖配置
└── README.md                   # 项目说明
```

### 核心依赖 (Cargo.toml)
```toml
[dependencies]
slint = "1.3"
pyo3 = { version = "0.20", features = ["auto-initialize"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"

[build-dependencies]
slint-build = "1.3"
```

## 功能特性

### 编辑器功能
- **物品编辑器**: 支持武器、防具、道具等物品属性编辑
- **兵种编辑器**: 兵种属性、技能、装备配置
- **触发器编辑器**: 游戏事件和脚本逻辑编辑
- **Python脚本**: 内置Python解释器，支持批量处理和自定义脚本

### UI特性
- **现代化界面**: 基于Slint的原生GUI，性能优异
- **模块化设计**: 组件化UI架构，易于维护和扩展
- **原版致敬**: 保留原作者信息和项目历史
- **图片素材**: 完整迁移原项目的图标和背景资源

### 技术特性
- **内存安全**: Rust语言保证内存安全和线程安全
- **高性能**: 零成本抽象，原生编译
- **跨平台**: 支持Windows、macOS、Linux
- **Python集成**: PyO3无缝集成Python生态

## 开发指南

### 编译和运行
```bash
# 设置环境变量（必需）
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1

# 编译项目
cargo build

# 运行项目
cargo run

# 发布版本编译
cargo build --release
```

### 常见问题

#### Python集成问题
如果遇到PyO3编译错误，确保设置了正确的环境变量：
```bash
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
```

#### Slint UI编译错误
检查UI文件语法，特别注意：
- `wrap`属性应使用`word-wrap`而不是`true`
- 图片路径使用相对路径格式
- 组件导入路径正确

## 图片资源

### 资源分类
项目已完成原版图片素材的迁移和整理：

- **backgrounds/**: 背景图片和横幅
- **icons/**: 各种功能图标
- **ui_elements/**: UI控件元素
- **game_themes/**: 游戏主题相关图片
- **logos/**: Logo和标识文件

### 在UI中使用图片
```slint
// 使用背景图片
background: @image-url("../../assets/Images/backgrounds/Banner.bmp");

// 使用图标
image: @image-url("../../assets/Images/icons/Knight.ico");
```

## 项目历史

R球编辑器是对原版MnBWarband Editor的现代化重构，致敬原版开发团队：
- **主程序开发**: SSgt_Edward, Ser_Charles
- **美工设计**: 我不是个过客(hp_honey)
- **各模块负责人**: 详见关于界面

新版本在保持原版精神的基础上，采用现代化技术栈重新实现，提供更好的性能和用户体验。

---

*本文档记录了R球编辑器的当前开发状态和使用指南。*
