use crate::core::{ElementMaybeSignal, Size};
use crate::{use_resize_observer_with_options, UseResizeObserverOptions};
use crate::{watch_with_options, WatchOptions};
use default_struct_builder::DefaultBuilder;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

/// Reactive size of an HTML element.
///
/// > This function requires `--cfg=web_sys_unstable_apis` to be activated as
/// [described in the wasm-bindgen guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html).
///
/// Please refer to [ResizeObserver on MDN](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver)
/// for more details.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_element_size)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{use_element_size, UseElementSizeReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = create_node_ref();
///
/// let UseElementSizeReturn { width, height } = use_element_size(el);
///
/// view! {
///     <div node_ref=el>
///         "Width: " {width}
///         "Height: " {height}
///     </div>
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// Please refer to ["Functions with Target Elements"](https://leptos-use.rs/server_side_rendering.html#functions-with-target-elements)
///
/// ## See also
///
/// - [`use_resize_observer`]
pub fn use_element_size<El, T>(target: El) -> UseElementSizeReturn
where
    El: Clone,
    El: Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    use_element_size_with_options(target, UseElementSizeOptions::default())
}

/// Version of [`use_element_size`] that takes a `UseElementSizeOptions`. See [`use_element_size`] for how to use.
pub fn use_element_size_with_options<El, T>(
    target: El,
    options: UseElementSizeOptions,
) -> UseElementSizeReturn
where
    El: Clone,
    El: Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    let window = window();
    let box_ = options.box_;
    let initial_size = options.initial_size;

    let target = (target).into();

    let is_svg = {
        let target = target.clone();

        move || {
            if let Some(target) = target.get_untracked() {
                target
                    .into()
                    .namespace_uri()
                    .map(|ns| ns.contains("svg"))
                    .unwrap_or(false)
            } else {
                false
            }
        }
    };

    let (width, set_width) = create_signal(options.initial_size.width);
    let (height, set_height) = create_signal(options.initial_size.height);

    {
        let target = target.clone();

        let _ = use_resize_observer_with_options::<ElementMaybeSignal<T, web_sys::Element>, _, _>(
            target.clone(),
            move |entries, _| {
                let entry = &entries[0];

                let box_size = match box_ {
                    web_sys::ResizeObserverBoxOptions::ContentBox => entry.content_box_size(),
                    web_sys::ResizeObserverBoxOptions::BorderBox => entry.border_box_size(),
                    web_sys::ResizeObserverBoxOptions::DevicePixelContentBox => {
                        entry.device_pixel_content_box_size()
                    }
                    _ => unreachable!(),
                };

                if is_svg() {
                    if let Some(target) = target.get() {
                        if let Ok(Some(styles)) = window.get_computed_style(&target.into()) {
                            set_height.set(
                                styles
                                    .get_property_value("height")
                                    .map(|v| v.parse().unwrap_or_default())
                                    .unwrap_or_default(),
                            );
                            set_width.set(
                                styles
                                    .get_property_value("width")
                                    .map(|v| v.parse().unwrap_or_default())
                                    .unwrap_or_default(),
                            );
                        }
                    }
                } else if !box_size.is_null() && !box_size.is_undefined() && box_size.length() > 0 {
                    let format_box_size = if box_size.is_array() {
                        box_size.to_vec()
                    } else {
                        vec![box_size.into()]
                    };

                    set_width.set(format_box_size.iter().fold(0.0, |acc, v| {
                        acc + v.as_ref().clone().unchecked_into::<BoxSize>().inline_size()
                    }));
                    set_height.set(format_box_size.iter().fold(0.0, |acc, v| {
                        acc + v.as_ref().clone().unchecked_into::<BoxSize>().block_size()
                    }))
                } else {
                    // fallback
                    set_width.set(entry.content_rect().width());
                    set_height.set(entry.content_rect().height())
                }
            },
            options.into(),
        );
    }

    let _ = watch_with_options(
        move || target.get(),
        move |ele, _, _| {
            if ele.is_some() {
                set_width.set(initial_size.width);
                set_height.set(initial_size.height);
            } else {
                set_width.set(0.0);
                set_height.set(0.0);
            }
        },
        WatchOptions::default().immediate(false),
    );

    UseElementSizeReturn {
        width: width.into(),
        height: height.into(),
    }
}

#[derive(DefaultBuilder)]
/// Options for [`use_element_size_with_options`].
pub struct UseElementSizeOptions {
    /// Initial size returned before any measurements on the `target` are done. Also the value reported
    /// at first when the `target` is a signal and changes.
    initial_size: Size,

    /// The box that is used to determine the dimensions of the target. Defaults to `ContentBox`.
    pub box_: web_sys::ResizeObserverBoxOptions,
}

impl Default for UseElementSizeOptions {
    fn default() -> Self {
        Self {
            initial_size: Size::default(),
            box_: web_sys::ResizeObserverBoxOptions::ContentBox,
        }
    }
}

impl From<UseElementSizeOptions> for UseResizeObserverOptions {
    fn from(options: UseElementSizeOptions) -> Self {
        Self::default().box_(options.box_)
    }
}

/// The return value of [`use_element_size`].
pub struct UseElementSizeReturn {
    /// The width of the element.
    pub width: Signal<f64>,
    /// The height of the element.
    pub height: Signal<f64>,
}

#[wasm_bindgen]
extern "C" {
    type BoxSize;

    #[wasm_bindgen(method, getter = blockSize)]
    fn block_size(this: &BoxSize) -> f64;

    #[wasm_bindgen(method, getter = inlineSize)]
    fn inline_size(this: &BoxSize) -> f64;
}
