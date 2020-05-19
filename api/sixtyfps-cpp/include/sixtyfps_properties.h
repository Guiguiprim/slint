#pragma once
#include <string_view>
#include "sixtyfps_properties_internal.h"

namespace sixtyfps {

template<typename T>
struct Property
{
    Property() {
        internal::sixtyfps_property_init(&inner);
    }
    ~Property() {
        internal::sixtyfps_property_drop(&inner);
    }
    Property(const Property&) = delete;
    Property(Property&&) = delete;
    Property &operator=(const Property&) = delete;

    void operator=(const T &value) {
        set(value);
    }

    void set(const T &value) const {
        this->value = value;
    }

    const T &get() const {
        internal::sixtyfps_property_update(&inner, &value);
        return value;
    }

    template <typename F>
    void set_binding(F binding) const {
        internal::sixtyfps_property_set_binding(
            &inner,
            [](const void *user_data, const void *value) {
                *reinterpret_cast<T*>(value) = (*reinterpret_cast<F*>(user_data))();
            },
            new F(binding),
            [](const void *user_data) {
                delete reinterpret_cast<F*>(user_data);
            }
        );
    }

private:
    internal::PropertyHandleOpaque inner;
    mutable T value;
};
}
