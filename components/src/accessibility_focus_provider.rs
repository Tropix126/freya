use dioxus::prelude::*;
use freya_core::FocusReceiver;
use freya_hooks::{focus_node_id, FocusId};

/// Propagate changes from the Focus shared state to the AccessibilityState and viceversa
#[allow(non_snake_case)]
pub fn AccessibilityFocusProvider(cx: Scope) -> Element {
    let focused_id = use_shared_state::<Option<FocusId>>(cx).unwrap();
    let current_focused_id = *focused_id.read();

    use_effect(cx, &(current_focused_id,), move |(focused_id,)| {
        if let Some(focused_id) = focused_id {
            focus_node_id(cx, focused_id)
        }
        async move {}
    });

    use_effect(cx, (), {
        to_owned![focused_id];
        move |_| {
            let focus_id_listener = cx.consume_context::<FocusReceiver>();
            async move {
                let focus_id_listener = focus_id_listener.clone();
                if let Some(mut focus_id_listener) = focus_id_listener {
                    while focus_id_listener.changed().await.is_ok() {
                        *focused_id.write() = *focus_id_listener.borrow();
                    }
                }
            }
        }
    });

    None
}
