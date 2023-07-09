use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::{
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;
use accesskit::{NodeId as AccessibilityId};
use crate::{CustomAttributeValues, Role, new_accessibility_id};

#[derive(Clone, Debug, PartialEq, Default, Component)]
pub struct Accessibility {
    pub role: Option<Role>,
    pub id: Option<AccessibilityId>,
    pub focusable: bool,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for Accessibility {
    type ParentDependencies = ();
    type ChildDependencies = ();
    type NodeDependencies = ();
    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&["accessibility", "focusable"]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut state = Self::default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                #[allow(clippy::single_match)]
                match attr.attribute.name.as_str() {
                    "accessibility" => {
                        if let Some(CustomAttributeValues::Role(role)) = attr.value.as_custom() {
                            state.role = Some(role.clone());
                            state.id = Some(new_accessibility_id());
                        } else {
                            // For some of Freya's default elements, it makes sense to
                            // have a default for accessible behavior, such as in the case
                            // if text or image-related elements.
                            // TODO(tropix126): Add fields here for label/alt, etc...
                            match node_view.tag() {
                                Some("text") | Some("label") => {
                                    state.role = Some(Role::Text);
                                    state.id = Some(new_accessibility_id());
                                },
                                Some("image") => {
                                    state.role = Some(Role::Image);
                                    state.id = Some(new_accessibility_id());
                                },
                                _ => {},
                            }
                        }
                    },
                    // TODO(tropix126): change this back to a string attribute
                    "focusable" => {
                        if let Some(value) = attr.value.as_bool() {
                            state.focusable = value;
                        }
                    },
                    _ => {}
                }
            }
        }
        
        let changed = &state != self;

        *self = state;
        changed
    }
}
