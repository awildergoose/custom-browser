use std::sync::Arc;

use crate::{
    WINDOW_HEIGHT, WINDOW_WIDTH,
    capsule::{
        Capsule,
        obj::{BoxedCapsuleObject, CapsuleObject},
    },
    layout::{computed::ComputedStyling, styling::Styling},
};
use stretch::{
    Stretch,
    geometry::Size,
    style::{Dimension, Style},
};

pub fn compute_layout(capsule: &mut Capsule) {
    fn styling_to_stretch(s: &Styling) -> Style {
        Style {
            size: stretch::geometry::Size {
                width: s.width.unwrap_or(stretch::style::Dimension::Auto),
                height: s.height.unwrap_or(stretch::style::Dimension::Auto),
            },
            align_items: s.align,
            justify_content: s.justify,
            flex_direction: s
                .flex_direction
                .unwrap_or(stretch::style::FlexDirection::Row),
            ..Default::default()
        }
    }

    fn build_node(
        stretch: &mut Stretch,
        child: &orx_concurrent_vec::ConcurrentElement<BoxedCapsuleObject>,
    ) -> stretch::node::Node {
        let style_arc: Arc<Styling> = child.map(|c| c.base().style.clone());
        let node_style = styling_to_stretch(&style_arc);

        let children_nodes: Vec<_> = {
            let child_children: Arc<orx_concurrent_vec::ConcurrentVec<BoxedCapsuleObject>> =
                child.map(|c| c.base().children.clone());
            child_children
                .iter()
                .map(|ch| build_node(stretch, ch))
                .collect()
        };

        stretch.new_node(node_style, children_nodes).unwrap()
    }

    fn apply_layout(
        stretch: &Stretch,
        node: stretch::node::Node,
        child: &orx_concurrent_vec::ConcurrentElement<BoxedCapsuleObject>,
        parent_abs_x: f32,
        parent_abs_y: f32,
    ) {
        let layout = stretch.layout(node).unwrap();

        let abs_x = parent_abs_x + layout.location.x;
        let abs_y = parent_abs_y + layout.location.y;

        child.map(|c| {
            let binding = c.base();
            let mut computed = binding.computed_style.write();
            *computed = ComputedStyling {
                x: abs_x,
                y: abs_y,
                width: layout.size.width,
                height: layout.size.height,
            };
        });

        let child_children: Arc<orx_concurrent_vec::ConcurrentVec<BoxedCapsuleObject>> =
            child.map(|c| c.base().children.clone());
        let child_nodes = stretch.children(node).unwrap();

        for (child_node, ch) in child_nodes.into_iter().zip(child_children.iter()) {
            apply_layout(stretch, child_node, ch, abs_x, abs_y);
        }
    }

    let mut stretch = Stretch::new();

    let root_base = capsule.view.base();
    let mut root_children_nodes = Vec::new();
    for child in root_base.children.iter() {
        root_children_nodes.push(build_node(&mut stretch, child));
    }

    let root_node = stretch
        .new_node(
            Style {
                #[allow(clippy::cast_precision_loss)]
                size: Size {
                    width: Dimension::Points(WINDOW_WIDTH as f32),
                    height: Dimension::Points(WINDOW_HEIGHT as f32),
                },
                flex_direction: stretch::style::FlexDirection::Column,
                ..Default::default()
            },
            root_children_nodes,
        )
        .unwrap();

    stretch
        .compute_layout(root_node, Size::undefined())
        .unwrap();

    let root_child_nodes = stretch.children(root_node).unwrap();
    for (child_node, child) in root_child_nodes.into_iter().zip(root_base.children.iter()) {
        apply_layout(&stretch, child_node, child, 0.0, 0.0);
    }
}
