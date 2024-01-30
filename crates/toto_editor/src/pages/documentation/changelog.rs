use leptonic::prelude::*;
use leptos::*;

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageChangelog() -> impl IntoView {
    view! {
        <H1>"Changelog"</H1>

        <H2>"0.3.0"</H2>

        <H3>"Added:"</H3>
        <ul>
            <li>"The `Consumer` type. Use `Consumer<In>` when you would otherwise write `Callback<In, ()>`."</li>
            <li>"The `Producer` type. Use `Producer<Out>` when you would otherwise write `Callback<(), Out>`."</li>
            <li>"The `ViewProducer` type. Use `ViewProducer` when you would otherwise write `Callback<(), leptos::View>`."</li>
            <li>"The `ViewCallback` type. Use `ViewCallback<In>` when you would otherwise write `Callback<In, leptos::View>`."</li>
        </ul>

        <H3>"Changed:"</H3>
        <ul>
            <li>"Updated to leptos 0.5.1. No more `cx`!"</li>
            <li>"The `render_option` prop for select inputs no longer requires you to call `.into_view()` on whatever your closure returns."</li>
            <li>"Collapsibles now use the slot approach."</li>
        </ul>

        <H3>"Fixed:"</H3>
        <ul>
            <li>"Fixed a bug which prevented progress bars in their indeterminate state to animate."</li>
        </ul>

        <H3>"Removed:"</H3>
        <ul>
            <li>"The `Callback` and `Callable` types moved into leptos itself! They should still be accessible on most use-sites as they are now imported with `use leptos::*`, which should already be present in most places the leptonic Callback was used before."</li>
        </ul>

        <H2>"0.2.0"</H2>

        <H3>"Added:"</H3>
        <ul>
            <li>
                "Added the `Out` type. An enum abstracting over `Callback`s and `WriteSignal`s."
                " Components can use this type when it is equally likely that a user will provide either of the previously mentioned types."
                " In case of the WriteSignal, the user just wants a new value to be stored."
                " In case of a callback, the user wants fine control over how a new value is handled."
                " The input component is the first one using it, as mentioned in the `Changed` section."
            </li>
            <li>"The Select, OptionalSelect and Multiselect components now accept a `class` prop with which custom classes can be attached to a rendered <leptonic-select> element."</li>
            <li>"The `Kbd` component together with `KbdShortcut`, displaying keyboard keys and shortcuts."</li>
            <li>"The `Chip` component now accepts custom `id`, `class` and `style` props."</li>
            <li>"You can now use the new `--slider-bar-background-image` CSS variable to style the bar of a `Slider` with more control. Defaults to `none`. `--slider-bar-background-color` is still the preferred way to style the bar if no image is needed. The image property will overwrite the color."</li>
            <li>"Also added `--slider-range-background-color`, `--slider-range-background-image`, `--slider-knob-border-width`, `--slider-knob-border-color`, `--slider-knob-border-style`, `--slider-knob-background-color` and `--slider-knob-halo-background-color`."</li>
            <li>"The background color/image of the selection of a `Slider` can now be styled using `--slider-range-background-color`, defaulting to `var(--brand-color)`, and `--slider-range-background-image`, defaulting to `none`."</li>
            <li>"Initial version of a `ColorPicker` component."</li>
        </ul>

        <H3>"Changed:"</H3>
        <ul>
            <li>"The DateSelector components on_change prop now takes a Callback instead of a generic function."</li>
            <li>"Buttons of type `outlined` now use --button-outlined-[color]-... variables for their styling."</li>
            <li>"Buttons of type `filled` now use --button-filled-[color]-... variables for their styling."</li>
            <li>"Buttons of type `outlined` and color `primary` now use a dark text color, contrasting the default bright background."</li>
            <li>"When using an input of type `Number`, you now have to supply optional `min`, `max` and `step` values which are propagated to the underlying input element."</li>
            <li>"The Input `set` prop is now optional."</li>
            <li>"The Input `set` prop is no longer generic. It now expects an `Out<String>`, which can either be a `WriteSignal` or a custom `Callback`."</li>
            <li>"The Slider and RangerSlider `set_value` props are no longer generic. They now expect an `Out<f64>`, which can either be a `WriteSignal` or a custom `Callback`."</li>
            <li>"The Toggle `on_toggle` prop is now called `set_value` and is no longer generic. It now expect an `Out<bool>`, which can either be a `WriteSignal` or a custom `Callback`."</li>
            <li>"The TiptapEditor `set_value` prop is no longer generic. It now expect an `Option<Out<TiptapContent>>`, which can either be a `WriteSignal` or a custom `Callback`."</li>
            <li>
                "All components using custom attributes now render them with a \"data-\" prefix, allowing them to be standard-compliant and distinguishable from well-known / standard attributes."
                " `leptonic-theme` styling changed appropriately."
            </li>
            <li>"Prop `max` of the ProgressBar is now a MaybeSignal."</li>
            <li>"Prop `progress` of the ProgressBar is now a MaybeSignal."</li>
            <li>
                "Prop `title` of the Alert is now a Callback instead of a generic Fn closure."
                " Expect the now necessary `create_callback(move |()| {})` when instantiating a component with a callback prop to become a simple `move || {}` after a migration to leptos 0.5!"
            </li>
            <li>"The Slider `step` prop is now optional, making continuous sliders with maximum precision easier to set up."</li>
            <li>"The `Input` component was split into `TextInput`, `PasswordInput` and `NumberInput`. Their `label` prop was renamed to `placeholder`. The `InputType` enum was removed."</li>
            <li>"All `Select` components now require a `search_text_provider` prop. The `SelectOption` trait no longer enforces `Display` to be implemented."</li>
        </ul>

        <H3>"Fixed:"</H3>
        <ul>
            <li>"A button with variants now properly respects its disabled state."</li>
            <li>"A button with variants now only triggers one of its actions (either main or variant) per interaction."</li>
            <li>"Buttons of type `flat` and color `info` now receive correct styling."</li>
            <li>"The installation instructions now include a section describing how to enable the required web_sys_unstable_apis opt-in."</li>
        </ul>

        <H2>"0.1.0"</H2>

        <P>"Initial release."</P>

        <H3>"Added utilities:"</H3>
        <ul>
            <li>"Callback types"</li>
            <li>"OptionalMaybeSignal type"</li>
            <li>"Global event listener contexts"</li>
        </ul>

        <H3>"Added components:"</H3>
        <ul>
            <li>"Root component"</li>
            <li>"Skeleton component and styles"</li>
            <li>"Stack component and styles"</li>
            <li>"Grid component and styles"</li>
            <li>"Separator component and styles"</li>
            <li>"Tab components and styles"</li>
            <li>"Collapsible components and styles"</li>
            <li>"AppBar components and styles"</li>
            <li>"Drawer components and styles"</li>
            <li>"Button component and styles"</li>
            <li>"Input component and styles"</li>
            <li>"Date selector component and styles"</li>
            <li>"Slider component and styles"</li>
            <li>"Select component and styles"</li>
            <li>"Toggle component and styles"</li>
            <li>"Alert component and styles"</li>
            <li>"Toast component and styles"</li>
            <li>"Modal components and styles"</li>
            <li>"Progress component and styles"</li>
            <li>"Popover component and styles"</li>
            <li>"Chip component and styles"</li>
            <li>"Icon component and styles"</li>
            <li>"Link component and styles"</li>
            <li>"Anchor component and styles"</li>
            <li>"Typography components and styles"</li>
            <li>"Transition components and styles"</li>
        </ul>
    }
}
