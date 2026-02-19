use orx_concurrent_vec::ConcurrentVec;
use roxmltree::Node;

use crate::{
    capsule::{
        Capsule,
        obj::{BoxedCapsuleObject, CSText, CapsuleMeta, CapsuleObject, CapsuleView},
    },
    layout::styling::Styling,
};

#[must_use]
fn parse_capsule_meta(child: Node) -> CapsuleMeta {
    let mut meta = CapsuleMeta::default();
    assert_eq!(child.tag_name().name(), "meta");

    if let Some(title) = child.children().find(|c| c.tag_name().name() == "title") {
        title.text().unwrap().clone_into(&mut meta.title);
    }

    meta
}

#[must_use]
fn parse_capsule_view(view: Node) -> CapsuleView {
    fn parse_child(child: Node) -> Option<BoxedCapsuleObject> {
        if child.is_text() {
            log::trace!("text, not parsing");
            return None;
        }

        let children = ConcurrentVec::new();

        for child in child.children() {
            let c = parse_child(child);
            if let Some(c) = c {
                children.push(c);
            }
        }

        let mut style = Styling::default();

        if let Some(align) = child.attribute("align") {
            style.align = match align {
                "flexstart" => stretch::style::AlignItems::FlexStart,
                "flexend" => stretch::style::AlignItems::FlexEnd,
                "center" => stretch::style::AlignItems::Center,
                "baseline" => stretch::style::AlignItems::Baseline,
                "stretch" => stretch::style::AlignItems::Stretch,
                _ => panic!("unknown align type"),
            };
        }

        let children = children.into();
        let style = style.into();

        let tag_name = child.tag_name().name();

        Some(match tag_name {
            "text" => Box::new(CSText::new(
                child.text().unwrap().to_string(),
                children,
                style,
            )),
            _ => panic!("unknown node type: '{tag_name}'"),
        })
    }

    let out = CapsuleView::default();
    assert_eq!(view.tag_name().name(), "view");

    for child in view.children() {
        let c = parse_child(child);
        if let Some(c) = c {
            out.base().children.push(c);
        }
    }

    out
}

#[must_use]
pub fn parse_capsule(text: &str) -> Capsule {
    let mut capsule = Capsule::default();
    let xml_document = roxmltree::Document::parse(text).unwrap();

    assert_eq!(xml_document.root_element().tag_name().name(), "capsule");

    let root_children = xml_document.root_element().children();

    for root_child in root_children {
        if root_child.is_text() {
            continue;
        }

        assert!(root_child.tag_name().name() == "meta" || root_child.tag_name().name() == "view");

        if root_child.tag_name().name() == "meta" {
            capsule.meta = parse_capsule_meta(root_child);
        } else {
            capsule.view = parse_capsule_view(root_child);
        }
    }

    capsule
}
