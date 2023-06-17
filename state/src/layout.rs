use std::sync::{Arc, Mutex};

use dioxus_native_core::{
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    NodeId, SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;
use freya_common::NodeReferenceLayout;
use tokio::sync::mpsc::UnboundedSender;
use torin::prelude::*;

use crate::{CustomAttributeValues, Parse};

#[derive(Default, Clone, Debug, Component)]
pub struct LayoutState {
    pub width: Size,
    pub height: Size,
    pub minimum_width: Size,
    pub minimum_height: Size,
    pub maximum_height: Size,
    pub maximum_width: Size,
    pub padding: Paddings,
    pub direction: DirectionMode,
    pub node_id: NodeId,
    pub scroll_y: f32,
    pub scroll_x: f32,
    pub display: DisplayMode,
    pub node_ref: Option<UnboundedSender<NodeReferenceLayout>>,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for LayoutState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[
            "width",
            "height",
            "min_height",
            "min_width",
            "max_height",
            "max_width",
            "padding",
            "direction",
            "scroll_y",
            "scroll_x",
            "display",
            "reference",
        ]))
        .with_tag()
        .with_text();

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let torin_layout = context.get::<Arc<Mutex<Torin<NodeId>>>>().unwrap();
        let scale_factor = context.get::<f32>().unwrap();

        let mut layout = LayoutState::default();

        layout.direction = if let Some("label") = node_view.tag() {
            DirectionMode::Horizontal
        } else if let Some("paragraph") = node_view.tag() {
            DirectionMode::Horizontal
        } else if let Some("text") = node_view.tag() {
            DirectionMode::Horizontal
        } else if node_view.text().is_some() {
            DirectionMode::Horizontal
        } else {
            DirectionMode::Vertical
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "width" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(width) = Size::parse(value, Some(*scale_factor)) {
                                layout.width = width;
                            }
                        }
                    }
                    "height" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(height) = Size::parse(value, Some(*scale_factor)) {
                                layout.height = height;
                            }
                        }
                    }
                    "min_height" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(min_height) = Size::parse(value, Some(*scale_factor)) {
                                layout.minimum_height = min_height;
                            }
                        }
                    }
                    "min_width" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(min_width) = Size::parse(value, Some(*scale_factor)) {
                                layout.minimum_width = min_width;
                            }
                        }
                    }
                    "max_height" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(max_height) = Size::parse(value, Some(*scale_factor)) {
                                layout.maximum_height = max_height;
                            }
                        }
                    }
                    "max_width" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(max_width) = Size::parse(value, Some(*scale_factor)) {
                                layout.maximum_width = max_width;
                            }
                        }
                    }
                    "padding" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(paddings) = Paddings::parse(value, Some(*scale_factor)) {
                                layout.padding = paddings;
                            }
                        }
                    }
                    "direction" => {
                        if let Some(value) = attr.value.as_text() {
                            layout.direction = match value {
                                "horizontal" => DirectionMode::Horizontal,
                                "both" => DirectionMode::Both,
                                _ => DirectionMode::Vertical,
                            }
                        }
                    }
                    "scroll_y" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(scroll) = value.parse::<f32>() {
                                layout.scroll_y = scroll * scale_factor;
                            }
                        }
                    }
                    "scroll_x" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(scroll) = value.parse::<f32>() {
                                layout.scroll_x = scroll * scale_factor;
                            }
                        }
                    }
                    "display" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(display) = DisplayMode::parse(value, None) {
                                layout.display = display;
                            }
                        }
                    }
                    "reference" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Reference(
                            reference,
                        )) = attr.value
                        {
                            layout.node_ref = Some(reference.0.clone());
                        }
                    }
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = (layout.width != self.width)
            || (layout.height != self.height)
            || (layout.minimum_width != self.minimum_width)
            || (layout.minimum_height != self.minimum_height)
            || (layout.maximum_width != self.maximum_width)
            || (layout.maximum_height != self.maximum_height)
            || (layout.padding != self.padding)
            || (node_view.node_id() != self.node_id)
            || (layout.direction != self.direction)
            || (layout.scroll_x != self.scroll_x)
            || (layout.scroll_y != self.scroll_y)
            || (layout.display != self.display);

        if changed {
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
        }

        *self = layout;
        changed
    }
}