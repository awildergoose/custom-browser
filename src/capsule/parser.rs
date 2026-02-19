use macroquad::{color::Color, text::measure_text};
use orx_concurrent_vec::ConcurrentVec;
use roxmltree::Node;
use stretch::style::Dimension;

use crate::{
    capsule::{
        Capsule,
        obj::{BoxedCapsuleObject, CapsuleMeta, CapsuleObject, CapsuleObjectCreationContext},
        objs::{obj::CSObj, script::CSScript, text::CSText, view::CapsuleView},
    },
    event::CapsuleObjectEvent,
    layout::styling::Styling,
    renderer::constants::BR_LINE_HEIGHT,
};

macro_rules! log_bad_property {
    ($name:expr) => {
        let _tt = stringify!($name);
        log::warn!("bad {_tt} property: '{}'", $name);
    };
}

#[must_use]
fn try_parse_color(color: &str) -> Option<Color> {
    if let Some([r, g, b, a]) = parse_color::parse(color) {
        return Some(Color::from_rgba(r, g, b, a));
    }

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
fn try_parse_dimension(text: &str) -> Option<Dimension> {
    if text.ends_with('%') {
        return Some(Dimension::Percent(
            text.strip_suffix("%").unwrap().parse::<f32>().ok()? / 100.0,
        ));
    }

    Some(Dimension::Points(text.parse::<f32>().ok()?))
}

#[must_use]
fn parse_capsule_meta(child: Node) -> CapsuleMeta {
    let mut meta = CapsuleMeta::default();
    assert_eq!(child.tag_name().name(), "meta");

    for node in child.children() {
        if node.is_text() || node.is_comment() {
            continue;
        }

        let tag_name = node.tag_name().name();
        let text = node.text().map(std::string::ToString::to_string);

        match tag_name {
            "title" => {
                meta.title = text.unwrap();
            }
            "script" => {
                meta.scripts.push(CSScript::new(text.unwrap()));
            }
            _ => {
                log::warn!("unknown node type: '{tag_name}'");
            }
        }
    }

    meta
}

#[must_use]
#[allow(clippy::too_many_lines)]
fn parse_capsule_view(view: Node) -> CapsuleView {
    fn parse_child(child: Node) -> Option<BoxedCapsuleObject> {
        if child.is_text() || child.is_comment() {
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

            if let Some(align) = match align {
                "flexstart" => Some(FlexStart),
                "flexend" => Some(FlexEnd),
                "center" => Some(Center),
                "baseline" => Some(Baseline),
                "stretch" => Some(Stretch),
                _ => None,
            } {
                style.align = align;
            } else {
                log_bad_property!(align);
            }
        }

        if let Some(justify) = child.attribute("justify") {
            use stretch::style::JustifyContent::{
                Center, FlexEnd, FlexStart, SpaceAround, SpaceBetween, SpaceEvenly,
            };

            if let Some(justify) = match justify {
                "flexstart" => Some(FlexStart),
                "flexend" => Some(FlexEnd),
                "center" => Some(Center),
                "spacebetween" => Some(SpaceBetween),
                "spacearound" => Some(SpaceAround),
                "spaceevenly" => Some(SpaceEvenly),
                _ => None,
            } {
                style.justify = justify;
            } else {
                log_bad_property!(justify);
            }
        }

        if let Some(flexdir) = child.attribute("flexdir") {
            use stretch::style::FlexDirection::{Column, ColumnReverse, Row, RowReverse};

            if let Some(flexdir) = match flexdir {
                "row" => Some(Row),
                "column" => Some(Column),
                "rowreverse" => Some(RowReverse),
                "columnreverse" => Some(ColumnReverse),
                _ => None,
            } {
                style.flex_direction = Some(flexdir);
            } else {
                log_bad_property!(flexdir);
            }
        }

        if let Some(color) = child.attribute("color") {
            if let Some(color) = try_parse_color(color) {
                style.color = Some(color);
            } else {
                log_bad_property!(color);
            }
        }

        if child.tag_name().name() == "text" {
            let measured = measure_text(child_text.as_ref().unwrap(), None, style.font_size, 1.0);
            style.width = Some(Dimension::Points(measured.width));
            style.height = Some(Dimension::Points(measured.height));
        }

        if let Some(width) = child.attribute("width") {
            if let Some(width) = try_parse_dimension(width) {
                style.width = Some(width);
            } else {
                log_bad_property!(width);
            }
        }

        if let Some(height) = child.attribute("height") {
            if let Some(height) = try_parse_dimension(height) {
                style.height = Some(height);
            } else {
                log_bad_property!(height);
            }
        }

        let events = ConcurrentVec::new();

        if let Some(onclick) = child.attribute("onclick") {
            events.push(CapsuleObjectEvent::new("onclick", onclick));
        }

        let mut style_clone = style.clone();

        let children_arc = children.into();
        let style_arc = style.into();
        let events_arc = events.into();

        let mut ctx = CapsuleObjectCreationContext::new(children_arc, events_arc, style_arc);

        match tag_name {
            "text" => Some(Box::new(CSText::new(
                child_text.as_ref().unwrap().clone(),
                ctx,
            ))),
            "obj" => Some(Box::new(CSObj::new(ctx))),
            "br" => {
                style_clone.width = Some(Dimension::Points(0.0));
                style_clone.height = Some(Dimension::Points(BR_LINE_HEIGHT));
                ctx.style = style_clone.into();

                Some(Box::new(CSObj::new(ctx)))
            }
            "script" => Some(Box::new(CSScript::new(
                child_text.as_ref().unwrap().clone(),
            ))),
            _ => {
                log::warn!("unknown node type: '{tag_name}'");
                None
            }
        }
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

pub fn parse_capsule(text: &str) -> anyhow::Result<Capsule> {
    let mut capsule = Capsule::default();
    let xml_document = roxmltree::Document::parse(text)?;

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

    Ok(capsule)
}
