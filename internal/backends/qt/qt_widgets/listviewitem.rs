// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-1.1 OR LicenseRef-Slint-commercial

use i_slint_core::input::FocusEventResult;

use super::*;

#[repr(C)]
#[derive(FieldOffsets, Default, SlintElement)]
#[pin]
pub struct NativeStandardListViewItem {
    pub x: Property<LogicalLength>,
    pub y: Property<LogicalLength>,
    pub width: Property<LogicalLength>,
    pub height: Property<LogicalLength>,
    pub item: Property<i_slint_core::model::StandardListViewItem>,
    pub index: Property<i32>,
    pub is_selected: Property<bool>,
    pub cached_rendering_data: CachedRenderingData,
    pub has_hover: Property<bool>,
    /// Specify that this item is in fact used in a ComboBox
    pub combobox: Property<bool>,
    widget_ptr: std::cell::Cell<SlintTypeErasedWidgetPtr>,
    animation_tracker: Property<i32>,
}

impl Item for NativeStandardListViewItem {
    fn init(self: Pin<&Self>) {
        let animation_tracker_property_ptr = Self::FIELD_OFFSETS.animation_tracker.apply_pin(self);
        self.widget_ptr.set(cpp! { unsafe [animation_tracker_property_ptr as "void*"] -> SlintTypeErasedWidgetPtr as "std::unique_ptr<SlintTypeErasedWidget>"  {
            return make_unique_animated_widget<QWidget>(animation_tracker_property_ptr);
        }})
    }

    fn geometry(self: Pin<&Self>) -> LogicalRect {
        LogicalRect::new(
            LogicalPoint::from_lengths(self.x(), self.y()),
            LogicalSize::from_lengths(self.width(), self.height()),
        )
    }

    fn layout_info(
        self: Pin<&Self>,
        orientation: Orientation,
        _window_adapter: &Rc<dyn WindowAdapter>,
    ) -> LayoutInfo {
        let index: i32 = self.index();
        let item = self.item();
        let text: qttypes::QString = item.text.as_str().into();
        let combobox: bool = self.combobox();

        let s = cpp!(unsafe [
            index as "int",
            text as "QString",
            combobox as "bool"
        ] -> qttypes::QSize as "QSize" {
            ensure_initialized();

            QStyleOptionComboBox cb_opt;
            if (combobox && qApp->style()->styleHint(QStyle::SH_ComboBox_Popup, &cb_opt, nullptr)) {
                QStyleOptionMenuItem option;
                option.text = text;
                option.text.replace(QChar('&'), QLatin1String("&&"));
                return qApp->style()->sizeFromContents(QStyle::CT_MenuItem, &option, QSize{}, nullptr);
            } else {
                QStyleOptionViewItem option;
                option.decorationPosition = QStyleOptionViewItem::Left;
                option.decorationAlignment = Qt::AlignCenter;
                option.displayAlignment = Qt::AlignLeft|Qt::AlignVCenter;
                option.showDecorationSelected = qApp->style()->styleHint(QStyle::SH_ItemView_ShowDecorationSelected, nullptr, nullptr);
                if (index % 2) {
                    option.features |= QStyleOptionViewItem::Alternate;
                }
                option.features |= QStyleOptionViewItem::HasDisplay;
                option.text = text;
                return qApp->style()->sizeFromContents(QStyle::CT_ItemViewItem, &option, QSize{}, nullptr);
                }
        });
        let min = match orientation {
            Orientation::Horizontal => s.width,
            Orientation::Vertical => s.height,
        } as f32;
        LayoutInfo { min, preferred: min, ..LayoutInfo::default() }
    }

    fn input_event_filter_before_children(
        self: Pin<&Self>,
        _: MouseEvent,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
    ) -> InputEventFilterResult {
        InputEventFilterResult::ForwardAndIgnore
    }

    fn input_event(
        self: Pin<&Self>,
        _event: MouseEvent,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &i_slint_core::items::ItemRc,
    ) -> InputEventResult {
        InputEventResult::EventIgnored
    }

    fn key_event(
        self: Pin<&Self>,
        _: &KeyEvent,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
    ) -> KeyEventResult {
        KeyEventResult::EventIgnored
    }

    fn focus_event(
        self: Pin<&Self>,
        _: &FocusEvent,
        _window_adapter: &Rc<dyn WindowAdapter>,
        _self_rc: &ItemRc,
    ) -> FocusEventResult {
        FocusEventResult::FocusIgnored
    }

    fn_render! { this dpr size painter widget initial_state =>
        let index: i32 = this.index();
        let is_selected: bool = this.is_selected();
        let combobox: bool = this.combobox();
        let has_hover: bool = this.has_hover();
        let item = this.item();
        let text: qttypes::QString = item.text.as_str().into();
        cpp!(unsafe [
            painter as "QPainterPtr*",
            widget as "QWidget*",
            size as "QSize",
            dpr as "float",
            index as "int",
            is_selected as "bool",
            has_hover as "bool",
            text as "QString",
            initial_state as "int",
            combobox as "bool"
        ] {
            QStyleOptionComboBox cb_opt;
            if (combobox && qApp->style()->styleHint(QStyle::SH_ComboBox_Popup, &cb_opt, widget)) {
                widget->setProperty("_q_isComboBoxPopupItem", true);
                QStyleOptionMenuItem option;
                option.initFrom(widget);
                option.state |= QStyle::State(initial_state);
                option.rect = QRect(QPoint(), size / dpr);
                option.menuRect = QRect(QPoint(), size / dpr);
                option.state = QStyle::State_Enabled;
                if (has_hover) {
                    option.state |= QStyle::State_MouseOver;
                    option.state |= QStyle::State_Selected;
                }
                option.text = text;
                option.text.replace(QChar('&'), QLatin1String("&&"));
                option.checked = is_selected;
                option.menuItemType = QStyleOptionMenuItem::Normal;
                //option.reservedShortcutWidth = 0;
                //option.maxIconWidth = 4;

                qApp->style()->drawControl(QStyle::CE_MenuItem, &option, painter->get(), widget);
                widget->setProperty("_q_isComboBoxPopupItem", {});
            } else {
                QStyleOptionViewItem option;
                option.initFrom(widget);
                option.state |= QStyle::State(initial_state);
                option.rect = QRect(QPoint(), size / dpr);
                option.state = QStyle::State_Enabled;
                if (is_selected) {
                    option.state |= QStyle::State_Selected;
                }
                if (has_hover) {
                    option.state |= QStyle::State_MouseOver;
                }
                option.decorationPosition = QStyleOptionViewItem::Left;
                option.decorationAlignment = Qt::AlignCenter;
                option.displayAlignment = Qt::AlignLeft|Qt::AlignVCenter;
                option.showDecorationSelected = qApp->style()->styleHint(QStyle::SH_ItemView_ShowDecorationSelected, nullptr, nullptr);

                if (index % 2) {
                    option.features |= QStyleOptionViewItem::Alternate;
                }
                option.features |= QStyleOptionViewItem::HasDisplay;

                option.text = text;
                // CE_ItemViewItem in QCommonStyle calls setClipRect on the painter and replace the clips. So we need to cheat.
                auto engine = (*painter)->paintEngine();
                auto old_clip = engine->systemClip();
                auto new_clip = ((*painter)->clipRegion() * (*painter)->transform());
                if (!old_clip.isNull())
                    new_clip &= old_clip;
                engine->setSystemClip(new_clip);

                qApp->style()->drawPrimitive(QStyle::PE_PanelItemViewRow, &option, painter->get(), widget);
                qApp->style()->drawControl(QStyle::CE_ItemViewItem, &option, painter->get(), widget);
                engine->setSystemClip(old_clip);
                // Qt is seriously bugged, setSystemClip will be scaled by the scale factor
                auto actual_clip = engine->systemClip();
                if (actual_clip != old_clip) {
                    QSizeF s2 = actual_clip.boundingRect().size();
                    QSizeF s1 = old_clip.boundingRect().size();
                    engine->setSystemClip(old_clip * QTransform::fromScale(s1.width() / s2.width(), s1.height() / s2.height()));
                }
            }
        });
    }
}

impl ItemConsts for NativeStandardListViewItem {
    const cached_rendering_data_offset: const_field_offset::FieldOffset<Self, CachedRenderingData> =
        Self::FIELD_OFFSETS.cached_rendering_data.as_unpinned_projection();
}

declare_item_vtable! {
fn slint_get_NativeStandardListViewItemVTable() -> NativeStandardListViewItemVTable for NativeStandardListViewItem
}
