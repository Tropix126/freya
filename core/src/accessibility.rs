use accesskit::{
    Action, DefaultActionVerb, Node, NodeBuilder, NodeClassSet, NodeId as AccessibilityId, Rect,
    Role, Tree, TreeUpdate,
};
use dioxus_native_core::{
    prelude::{NodeType, TextNode},
    real_dom::NodeImmutable,
    NodeId,
};
use freya_dom::prelude::{DioxusDOM, DioxusNode};
use freya_layout::Layers;
use freya_node_state::{Accessibility, Fill, FontStyle, Style};
use std::slice::Iter;
use tokio::sync::watch;
use torin::{prelude::NodeAreas, torin::Torin};

/// Direction for the next Accessibility Node to be focused.
#[derive(PartialEq)]
pub enum AccessibilityFocusDirection {
    Forward,
    Backward,
}

pub trait AccessibilityProvider {
    /// Add a Dioxus Node to the Accessibility Tree, applying properties as needed.
    fn add_node(
        &mut self,
        dioxus_node: &DioxusNode,
        node_areas: &NodeAreas,
        node_accessibility: &Accessibility,
    ) {
        let node_style = &*dioxus_node.get::<Style>().unwrap();
        let node_font_style = &*dioxus_node.get::<FontStyle>().unwrap();

        // Make sure that there's valid data to add.
        let mut builder = NodeBuilder::new(Role::Unknown);

        // Generic properties
        // These are accessibility properties that are common to all nodes, and can thus be
        // inferred from them automatically.
        let area = node_areas.area.to_f64();
        builder.set_bounds(Rect {
            x0: area.min_x(),
            x1: area.max_x(),
            y0: area.min_y(),
            y1: area.max_y(),
        });

        // Color/styling-related properties
        // Do these only apply for text-related roles? Does it matter if they're applied to
        // a node where they don't matter on?
        let foreground = node_font_style.color;
        builder.set_foreground_color(u32::from_be_bytes([
            foreground.r(),
            foreground.g(),
            foreground.b(),
            foreground.a(),
        ]));
        if let Fill::Color(background) = node_style.background {
            builder.set_background_color(u32::from_be_bytes([
                background.r(),
                background.g(),
                background.b(),
                background.a(),
            ]));
        }

        // We don't support RTL yet.
        builder.set_text_direction(accesskit::TextDirection::LeftToRight);

        // NOTE: Probably not needed, but could be inferred.
        // builder.set_aria_role();

        // Set focusable action
        if node_accessibility.focusable {
            builder.add_action(Action::Focus);
            // TODO
            // builder.add_action(Action::Blur);
        } else {
            builder.add_action(Action::Default);
            builder.set_default_action_verb(DefaultActionVerb::Click);
        }

        builder.set_children(dioxus_node.get_accessible_children());

        // Insert the node into the Tree
        if let Some(id) = node_accessibility.id {
            let node = builder.build(self.node_classes());
            self.push_node(id, node);
        }
    }

    /// Push a Node into the Accesibility Tree.
    fn push_node(&mut self, id: AccessibilityId, node: Node);

    /// Mutable reference to the NodeClassSet.
    fn node_classes(&mut self) -> &mut NodeClassSet;

    /// Iterator over the Accessibility Tree of Nodes.
    fn nodes(&self) -> Iter<(AccessibilityId, Node)>;

    /// Get the currently focused Node's ID.
    fn focus_id(&self) -> Option<AccessibilityId>;

    /// Update the focused Node ID.
    fn set_focus(&mut self, new_focus_id: Option<AccessibilityId>);

    /// Update the focused Node ID and generate a TreeUpdate if necessary.
    fn set_focus_with_update(
        &mut self,
        new_focus_id: Option<AccessibilityId>,
    ) -> Option<TreeUpdate> {
        self.set_focus(new_focus_id);

        // Only focus the element if it exists
        let node_focused_exists = self.nodes().any(|node| Some(node.0) == new_focus_id);
        if node_focused_exists {
            Some(TreeUpdate {
                nodes: Vec::new(),
                tree: None,
                focus: self.focus_id(),
            })
        } else {
            None
        }
    }

    /// Create the root Accessibility Node.
    fn build_root(&mut self, root_name: &str) -> Node {
        let mut builder = NodeBuilder::new(Role::Window);
        builder.set_name(root_name.to_string());
        builder.set_children(
            self.nodes()
                .map(|(id, _)| *id)
                .collect::<Vec<AccessibilityId>>(),
        );

        builder.build(self.node_classes())
    }

    /// Process the Nodes accessibility Tree
    fn process(&mut self, root_id: AccessibilityId, root_name: &str) -> TreeUpdate {
        let root = self.build_root(root_name);
        let mut nodes = vec![(root_id, root)];
        nodes.extend(self.nodes().cloned());
        nodes.reverse();

        let focus = self.nodes().find_map(|node| {
            if Some(node.0) == self.focus_id() {
                Some(node.0)
            } else {
                None
            }
        });

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(root_id)),
            focus,
        }
    }

    /// Focus the next/previous Node starting from the currently focused Node.
    fn set_focus_on_next_node(
        &mut self,
        direction: AccessibilityFocusDirection,
        focus_sender: &watch::Sender<Option<AccessibilityId>>,
    ) -> Option<TreeUpdate> {
        if let Some(focused_node_id) = self.focus_id() {
            let current_node = self
                .nodes()
                .enumerate()
                .find(|(_, node)| node.0 == focused_node_id)
                .map(|(i, _)| i);

            if let Some(node_index) = current_node {
                let target_node_index = if direction == AccessibilityFocusDirection::Forward {
                    // Find the next Node
                    if node_index == self.nodes().len() - 1 {
                        0
                    } else {
                        node_index + 1
                    }
                } else {
                    // Find the previous Node
                    if node_index == 0 {
                        self.nodes().len() - 1
                    } else {
                        node_index - 1
                    }
                };

                let target_node = self
                    .nodes()
                    .enumerate()
                    .find(|(i, _)| *i == target_node_index)
                    .map(|(_, node)| node.0);

                self.set_focus(target_node);
            } else {
                // Select the first Node
                self.set_focus(self.nodes().next().map(|(id, _)| *id))
            }

            focus_sender.send(self.focus_id()).ok();

            Some(TreeUpdate {
                nodes: Vec::new(),
                tree: None,
                focus: self.focus_id(),
            })
        } else {
            None
        }
    }
}

/// Shortcut functions to retrieve Acessibility info from a Dioxus Node
trait NodeAccessibility {
    /// Return the first TextNode from this Node
    fn get_inner_texts(&self) -> Option<String>;

    /// Collect all the AccessibilityIDs from a Node's children
    fn get_accessible_children(&self) -> Vec<AccessibilityId>;
}

impl NodeAccessibility for DioxusNode<'_> {
    /// Return the first TextNode from this Node
    fn get_inner_texts(&self) -> Option<String> {
        let children = self.children();
        let first_child = children.first()?;
        let node_type = first_child.node_type();
        if let NodeType::Text(TextNode { text, .. }) = &*node_type {
            Some(text.to_owned())
        } else {
            None
        }
    }

    /// Collect all the AccessibilityIDs from a Node's children
    fn get_accessible_children(&self) -> Vec<AccessibilityId> {
        self.children()
            .iter()
            .filter_map(|child| {
                let node_accessibility = &*child.get::<Accessibility>().unwrap();
                node_accessibility.id
            })
            .collect::<Vec<AccessibilityId>>()
    }
}

pub fn process_accessibility(
    layers: &Layers,
    layout: &Torin<NodeId>,
    rdom: &DioxusDOM,
    access_provider: &mut impl AccessibilityProvider,
) {
    for layer in layers.layers.values() {
        for node_id in layer {
            let node_areas = layout.get(*node_id).unwrap();
            let dioxus_node = rdom.get(*node_id);
            if let Some(dioxus_node) = dioxus_node {
                let node_accessibility = &*dioxus_node.get::<Accessibility>().unwrap();

                if node_accessibility.id.is_some() {
                    access_provider.add_node(&dioxus_node, node_areas, node_accessibility);
                }
            }
        }
    }
}
