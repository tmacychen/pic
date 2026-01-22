# Makepad 下拉菜单实现方案

## 问题分析
基于搜索结果，Makepad 框架中没有内置的 PopupMenu 或 Dropdown 组件。当前的实现方式（在 MenuBar 内部放置菜单）存在以下问题：

1. MenuBar 使用 `flow: Right` 布局，会压缩内部的竖向菜单
2. 菜单无法正确显示在按钮下方
3. 需要使用 Overlay 或其他方式来实现浮动菜单

## 推荐解决方案

### 方案 1：使用 Overlay（推荐）
在 Makepad 中，可以使用 Overlay 组件来实现浮动菜单。Overlay 允许在其他元素上方显示内容。

```rust
// 在 live_design 中定义
overlay_menu = <Overlay> {
    visible: false
    // 菜单内容
}
```

### 方案 2：使用 PageFlip 切换
使用 PageFlip 在不同的页面之间切换，其中一个页面是菜单。

### 方案 3：使用绝对定位
在 Makepad 中，可以使用 `abs_pos` 和 `abs_size` 来实现绝对定位。

## 实现步骤

1. 将菜单从 MenuBar 中分离出来
2. 使用 Overlay 或其他浮动容器
3. 通过状态变量控制菜单的显示/隐藏
4. 在事件处理中更新菜单的可见性

## 参考资源
- Makepad 官方文档：https://publish.obsidian.md/makepad-docs/
- Makepad GitHub：https://github.com/makepad/makepad
- Robrix 项目（使用 Makepad 的实际项目）：https://github.com/project-robius/robrix
