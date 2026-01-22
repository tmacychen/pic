# 下拉菜单实现方案 - 基于 Makepad Skills

## 问题
初始的下拉菜单实现无法正常显示，菜单项被压缩或隐藏。

## 解决方案
基于 `makepad-skills` 中的 `04-patterns/_base/02-modal-overlay.md` 模式，我们实现了一个自定义的 `LanguageMenu` 组件。

## 实现步骤

### 1. 创建自定义 LanguageMenu 组件 (`src/language_menu.rs`)
- 定义 `LanguageMenuAction` 枚举来处理菜单选择事件
- 实现 `LanguageMenu` Widget，继承自 View
- 使用 `WidgetMatchEvent` trait 来处理按钮点击事件
- 通过 `cx.widget_action()` 发送 action 到父组件

### 2. 在 widgets.rs 中定义 UI 布局
```rust
pub LanguageMenu = {{crate::language_menu::LanguageMenu}} {
    visible: false
    width: 120, height: Fit
    show_bg: true
    draw_bg: { color: (COLOR_BG_LIGHT) }
    flow: Down
    padding: (SPACING_SM)
    spacing: 0
    
    english_option = <Button> { ... }
    chinese_option = <Button> { ... }
}
```

### 3. 在 app.rs 中集成
- 导入 `LanguageMenuAction`
- 在 UI 定义中添加 `language_menu = <LanguageMenu> {}`
- 在事件处理中：
  - 点击"Language"按钮时切换菜单可见性
  - 处理 `LanguageMenuAction` 来响应菜单选择

### 4. 关键实现细节

**菜单可见性管理**：
```rust
self.language_menu_visible = !self.language_menu_visible;
self.ui.view(id!(language_menu)).set_visible(cx, self.language_menu_visible);
```

**事件处理**：
```rust
for action in actions {
    match action.as_widget_action().cast() {
        LanguageMenuAction::SelectEnglish => {
            self.set_language(cx, Language::English);
        }
        LanguageMenuAction::SelectChinese => {
            self.set_language(cx, Language::Chinese);
        }
        _ => {}
    }
}
```

## 文件结构
```
src/
├── app.rs                 # 主应用，集成菜单
├── language_menu.rs       # 自定义菜单组件（新增）
├── i18n.rs               # 国际化支持
├── shared/
│   └── widgets.rs        # UI 组件定义
└── ...
```

## 功能特性
✅ 点击"Language"按钮显示/隐藏菜单
✅ 菜单项有 hover 效果
✅ 选择菜单项后自动隐藏菜单
✅ 支持多语言（英文/中文）
✅ 菜单文本随语言切换而更新
✅ 输出 log 记录语言变更

## 参考资源
- Makepad Skills: `04-patterns/_base/02-modal-overlay.md`
- 使用 Widget trait 和 WidgetMatchEvent 来实现自定义组件
- 通过 action 系统实现组件间的通信

## 下一步改进
如果需要更高级的功能，可以参考：
- `04-patterns/_base/14-callout-tooltip.md` - 实现带箭头的浮动菜单
- `02-modal-overlay.md` - 使用 `DrawList2d::begin_overlay_reuse()` 实现真正的浮动层
