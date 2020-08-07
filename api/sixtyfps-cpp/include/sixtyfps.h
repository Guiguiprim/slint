#pragma once

// In C++17, it is conditionally supported, but still valid for all compiler we care
#pragma GCC diagnostic ignored "-Winvalid-offsetof"

#include <vector>
#include <memory>

namespace sixtyfps::internal {
// Workaround https://github.com/eqrion/cbindgen/issues/43
struct ComponentVTable;
struct ItemVTable;
}
#include "sixtyfps_internal.h"
#include "sixtyfps_gl_internal.h"

namespace sixtyfps {

extern "C" {
extern const internal::ItemVTable RectangleVTable;
extern const internal::ItemVTable BorderRectangleVTable;
extern const internal::ItemVTable TextVTable;
extern const internal::ItemVTable TouchAreaVTable;
extern const internal::ItemVTable ImageVTable;
extern const internal::ItemVTable PathVTable;
extern const internal::ItemVTable FlickableVTable;
}

// Bring opaque structure in scope
using internal::ComponentVTable;
using ItemTreeNode = internal::ItemTreeNode<uint8_t>;
using ComponentRef = VRef<ComponentVTable>;
using ItemVisitorRefMut = VRefMut<internal::ItemVisitorVTable>;
using internal::EasingCurve;
using internal::TextHorizontalAlignment;
using internal::TextVerticalAlignment;
using internal::WindowProperties;
using internal::Slice;

struct ComponentWindow
{
    ComponentWindow() { internal::sixtyfps_component_window_gl_renderer_init(&inner); }
    ~ComponentWindow() { internal::sixtyfps_component_window_drop(&inner); }
    ComponentWindow(const ComponentWindow &) = delete;
    ComponentWindow(ComponentWindow &&) = delete;
    ComponentWindow &operator=(const ComponentWindow &) = delete;

    template<typename Component>
    void run(Component *c)
    {
        auto props = c->window_properties();
        sixtyfps_component_window_run(
                &inner, VRefMut<ComponentVTable> { &Component::component_type, c }, &props);
    }

private:
    internal::ComponentWindowOpaque inner;
};

using internal::BorderRectangle;
using internal::Flickable;
using internal::Image;
using internal::Path;
using internal::Rectangle;
using internal::Text;
using internal::TouchArea;

constexpr inline ItemTreeNode make_item_node(std::uintptr_t offset,
                                                      const internal::ItemVTable *vtable,
                                                      uint32_t child_count, uint32_t child_index)
{
    return ItemTreeNode { ItemTreeNode::Item_Body {
            ItemTreeNode::Tag::Item, { vtable, offset }, child_count, child_index } };
}

constexpr inline ItemTreeNode make_dyn_node(std::uintptr_t offset)
{
    return ItemTreeNode { ItemTreeNode::DynamicTree_Body {
            ItemTreeNode::Tag::DynamicTree, offset } };
}

using internal::sixtyfps_visit_item_tree;
using internal::MouseEvent;
using internal::InputEventResult;
template<typename HandleDynamic>
inline InputEventResult process_input_event(
    ComponentRef component, int64_t &mouse_grabber, MouseEvent mouse_event,
    Slice<ItemTreeNode> tree, HandleDynamic handle_dynamic)
{
     if (mouse_grabber != -1) {
        auto item_index = mouse_grabber & 0xffffffff;
        auto rep_index = mouse_grabber >> 32;
        auto offset = internal::sixtyfps_item_offset(component, tree, item_index);
        mouse_event.pos = { mouse_event.pos.x - offset.x , mouse_event.pos.y - offset.y };
        const auto &item_node = tree.ptr[item_index];
        InputEventResult result = InputEventResult::EventIgnored;
        switch (item_node.tag) {
            case ItemTreeNode::Tag::Item:
                result = item_node.item.item.vtable->input_event( {
                    item_node.item.item.vtable,
                    reinterpret_cast<char*>(component.instance) + item_node.item.item.offset,
                } , mouse_event);
                break;
            case ItemTreeNode::Tag::DynamicTree:
                result = handle_dynamic(item_node.dynamic_tree.index, rep_index, &mouse_event);
                break;
        }
        if (result != InputEventResult::GrabMouse) {
            mouse_grabber = -1;
        }
        return result;
    } else {
        return internal::sixtyfps_process_ungrabbed_mouse_event(component, mouse_event, &mouse_grabber);
    }
}

// layouts:
using internal::grid_layout_info;
using internal::GridLayoutCellData;
using internal::GridLayoutData;
using internal::PathLayoutData;
using internal::PathLayoutItemData;
using internal::solve_grid_layout;
using internal::solve_path_layout;

// models

struct Model
{
    virtual ~Model() = default;
    Model() = default;
    Model(const Model &) = delete;
    Model &operator=(const Model &) = delete;
    virtual int count() const = 0;
    virtual const void *get(int i) const = 0;
};

template<int Count, typename ModelData>
struct ArrayModel : Model
{
    std::array<ModelData, Count> data;
    template<typename... A>
    ArrayModel(A &&... a) : data { std::forward<A>(a)... }
    {
    }
    ArrayModel(int x) { }
    int count() const override { return Count; }
    const void *get(int i) const override { return &data[i]; }
};

struct IntModel : Model
{
    IntModel(int d) : data(d) { }
    int data;
    int count() const override { return data; }
    const void *get(int) const override { return &data; }
};

template<typename C>
struct Repeater
{
    std::vector<std::unique_ptr<C>> data;

    template<typename Parent>
    void update_model(Model *model, const Parent *parent) const
    {
        auto &data = const_cast<Repeater *>(this)->data;
        data.clear();
        auto count = model->count();
        for (auto i = 0; i < count; ++i) {
            auto x = std::make_unique<C>();
            x->parent = parent;
            x->update_data(i, model->get(i));
            data.push_back(std::move(x));
        }
    }

    intptr_t visit(ItemVisitorRefMut visitor) const
    {
        for (auto i = 0; i < data.size(); ++i) {
            const auto &x = data.at(i);
            VRef<ComponentVTable> ref { &C::component_type, x.get() };
            if (ref.vtable->visit_children_item(ref, -1, visitor) != -1) {
                return i;
            }
        }
        return -1;
    }
};

Flickable::Flickable()
{
    sixtyfps_flickable_data_init(&data);
}
Flickable::~Flickable()
{
    sixtyfps_flickable_data_free(&data);
}

template<int Major, int Minor, int Patch>
struct VersionCheckHelper
{
};

} // namespace sixtyfps
