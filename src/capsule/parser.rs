use macroquad::{color::Color, text::measure_text};
use orx_concurrent_vec::ConcurrentVec;
use roxmltree::Node;

use crate::{
    capsule::{
        Capsule,
        obj::{BoxedCapsuleObject, CapsuleMeta, CapsuleObject, CapsuleView},
        objs::{obj::CSObj, text::CSText},
    },
    layout::styling::Styling,
};

#[must_use]
fn try_parse_color(color: &str) -> Option<Color> {
    let color = color.strip_prefix("0x").unwrap_or(color);
    let color = color.strip_prefix("#").unwrap_or(color);

    if color.len() != 6 && color.len() != 8 {
        return None;
    }

    let r = u8::from_str_radix(&color[0..2], 16).ok()?;
    let g = u8::from_str_radix(&color[2..4], 16).ok()?;
    let b = u8::from_str_radix(&color[4..6], 16).ok()?;
    let a = if color.len() == 8 {
        u8::from_str_radix(&color[6..8], 16).ok()?
    } else {
        255
    };

    Some(Color::from_rgba(r, g, b, a))
}

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
#[allow(clippy::too_many_lines)]
fn parse_capsule_view(view: Node) -> CapsuleView {
    fn parse_child(child: Node) -> Option<BoxedCapsuleObject> {
        if child.is_text() {
            log::trace!("text, not parsing");
            return None;
        }

        let tag_name = child.tag_name().name();
        let child_text = child.text().map(std::string::ToString::to_string);

        let children = ConcurrentVec::new();

        for child in child.children() {
            let c = parse_child(child);
            if let Some(c) = c {
                children.push(c);
            }
        }

        let mut style = Styling::default();

        if let Some(align) = child.attribute("align") {
            use stretch::style::AlignItems::{Baseline, Center, FlexEnd, FlexStart, Stretch};

            style.align = match align {
                "flexstart" => FlexStart,
                "flexend" => FlexEnd,
                "center" => Center,
                "baseline" => Baseline,
                "stretch" => Stretch,
                _ => panic!("bad align property: {align}"),
            };
        }

        if let Some(justify) = child.attribute("justify") {
            use stretch::style::JustifyContent::{
                Center, FlexEnd, FlexStart, SpaceAround, SpaceBetween, SpaceEvenly,
            };

            style.justify = match justify {
                "flexstart" => FlexStart,
                "flexend" => FlexEnd,
                "center" => Center,
                "spacebetween" => SpaceBetween,
                "spacearound" => SpaceAround,
                "spaceevenly" => SpaceEvenly,
                _ => panic!("bad justify property: {justify}"),
            };
        }

        if child.tag_name().name() == "text" {
            let measured = measure_text(child_text.as_ref().unwrap(), None, style.font_size, 1.0);
            style.width = Some(stretch::style::Dimension::Points(measured.width));
            style.height = Some(stretch::style::Dimension::Points(measured.height));
        }

        if let Some(width) = child.attribute("width") {
            if let Ok(width) = width.parse::<f32>() {
                style.width = Some(stretch::style::Dimension::Points(width));
            } else {
                log::warn!("bad width property: '{width}'");
            }
        }

        if let Some(height) = child.attribute("height") {
            if let Ok(height) = height.parse::<f32>() {
                style.height = Some(stretch::style::Dimension::Points(height));
            } else {
                log::warn!("bad height property: '{height}'");
            }
        }

        if let Some(color) = child.attribute("color") {
            style.color = parse_color::parse(color)
                .map_or_else(
                    || try_parse_color(color),
                    |[r, g, b, a]| Some(Color::new(r.into(), g.into(), b.into(), a.into())),
                )
                .unwrap();
        }

        let children = children.into();
        let style = style.into();

        Some(match tag_name {
            "text" => Box::new(CSText::new(
                child_text.as_ref().unwrap().clone(),
                children,
                style,
            )),
            "obj" => Box::new(CSObj::new(children, style)),
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
