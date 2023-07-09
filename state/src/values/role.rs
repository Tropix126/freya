use accesskit::{NodeId as AccessibilityId};
use dioxus_hooks::UseSharedState;
use std::num::NonZeroU128;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct UseAccessibility<'a> {
	pub id: AccessibilityId,
    pub focused_id: Option<&'a UseSharedState<Option<AccessibilityId>>>,
	pub role: Role,
}

impl UseAccessibility<'_> {
	/// Get this node's accessibility iD
	pub fn id(&self) -> AccessibilityId {
		self.id
	}

	/// Get this node's current role
	pub fn role(&self) -> Role {
		self.role
	}

	/// Check if this node is currently focused
	pub fn is_focused(&self) -> bool {
        Some(Some(self.id)) == self.focused_id.map(|f| *f.read())
	}
	
    /// Focus this node
    pub fn focus(&self) {
        if let Some(focused_id) = self.focused_id {
            *focused_id.write() = Some(self.id)
        }
    }

	// TODO(tropix126)
	// pub fn focus_node(&self) {}
	// pub fn focus_next(&self) {}

    /// Unfocus the currently focused node.
    pub fn unfocus(&self) {
        if let Some(focused_id) = self.focused_id {
            *focused_id.write() = None;
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum Role {
    Text,
    Button,
    Image,
    Other(accesskit::Role),
    #[default]
	None,
}

pub fn new_accessibility_id() -> AccessibilityId {
    AccessibilityId(NonZeroU128::new(Uuid::new_v4().as_u128()).unwrap())
}