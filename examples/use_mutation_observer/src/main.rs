use leptos::html::Div;
use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_mutation_observer_with_options;
use std::time::Duration;

#[component]
fn Demo() -> impl IntoView {
    let el = create_node_ref::<Div>();
    let (messages, set_messages) = create_signal(vec![]);
    let (class_name, set_class_name) = create_signal(String::new());
    let (style, set_style) = create_signal(String::new());

    let mut init = web_sys::MutationObserverInit::new();
    init.attributes(true);

    use_mutation_observer_with_options(
        el,
        move |mutations, _| {
            if let Some(mutation) = mutations.first() {
                set_messages.update(move |messages| {
                    messages.push(format!("{:?}", mutation.attribute_name()));
                });
            }
        },
        init,
    );

    let _ = set_timeout_with_handle(
        move || {
            set_class_name.set("test test2".to_string());
        },
        Duration::from_millis(1000),
    );

    let _ = set_timeout_with_handle(
        move || {
            set_style.set("color: red;".to_string());
        },
        Duration::from_millis(1550),
    );

    let enum_msgs =
        Signal::derive(move || messages.get().into_iter().enumerate().collect::<Vec<_>>());

    view! {
        <div node_ref=el class=move || class_name.get() style=move || style.get()>
            <For
                each=move || enum_msgs.get()
                // list only grows so this is fine here
                key=|message| message.0
                view=|message| view! { <div>"Mutation Attribute: " <code>{message.1}</code></div> }
            />
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
