use std::sync::Arc;

use orx_concurrent_vec::ConcurrentVec;
use parking_lot::RwLock;
use roxmltree::Node;

use crate::{
    capsule::{
        Capsule,
        obj::{BoxedCapsuleObject, CapsuleMeta, CapsuleObjectCreationContext},
        objs::{obj::CSObj, script::CSScript, text::CSText, view::CSView},
    },
    event::CapsuleObjectEvent,
    layout::{
        capsule::{
            align::COAlignItems, color::COColor, dimension::CODimension, flexdir::COFlexDirection,
            justify::COJustifyContent,
        },
        styling::Styling,
    },
    renderer::constants::BR_LINE_HEIGHT,
};

macro_rules! log_bad_property {
    ($name:expr) => {
        let _tt = stringify!($name);
        log::warn!("bad {_tt} property: '{}'", $name);
    };
}

macro_rules! enum_attr {
    ($child: ident, $style: ident, $name: ident, $type: ty) => {
        if let Some(value) = $child.attribute(stringify!($name)) {
            if let Ok(parsed) = value.parse::<$type>() {
                $style.$name = parsed;
            } else {
                log_bad_property!(value);
            }
        }
    };
}

macro_rules! dimension_attr {
    ($child: ident, $style: ident, $name: ident) => {
        if let Some(value) = $child.attribute(stringify!($name)) {
            if let Some(parsed) = try_parse_dimension(value) {
                $style.$name = Some(parsed);
            } else {
                log_bad_property!(value);
            }
        }
    };
}

macro_rules! color_attr {
    ($child: ident, $style: ident, $name: ident) => {
        if let Some(value) = $child.attribute(stringify!($name)) {
            if let Some(parsed) = try_parse_color(value) {
                $style.$name = Some(parsed);
            } else {
                log_bad_property!(value);
            }
        }
    };
}

macro_rules! primitive_attr {
    ($child: ident, $style: ident, $name: ident, $type: tt) => {
        if let Some(value) = $child.attribute(stringify!($name)) {
            if let Ok(parsed) = value.parse::<$type>() {
                $style.$name = parsed;
            } else {
                log_bad_property!(value);
            }
        }
    };
}

macro_rules! event_attr {
    ($child: ident, $events: ident, $name: ident) => {
        if let Some(value) = $child.attribute(stringify!($name)) {
            $events.push(CapsuleObjectEvent::new(stringify!($name), value));
        }
    };
}

#[must_use]
pub fn try_parse_color(color: &str) -> Option<COColor> {
    if let Some([r, g, b, a]) = parse_color::parse(color) {
        return Some(COColor::from_rgba(r, g, b, a));
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

    Some(COColor::from_rgba(r, g, b, a))
}

#[must_use]
pub fn try_parse_dimension(text: &str) -> Option<CODimension> {
    if text == "auto" {
        return Some(CODimension::Auto);
    } else if text == "undefined" {
        return Some(CODimension::Undefined);
    } else if text.ends_with('%') {
        return Some(CODimension::Percent(
            text.strip_suffix("%").unwrap().parse::<f32>().ok()? / 100.0,
        ));
    }

    Some(CODimension::Points(text.parse::<f32>().ok()?))
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn clean_text(s: String) -> String {
    s.lines().map(str::trim).collect::<Vec<_>>().join("\n")
}

#[must_use]
fn parse_capsule_meta(child: Node) -> CapsuleMeta {
    let mut meta = CapsuleMeta::default();

    for node in child.children() {
        if node.is_text() || node.is_comment() {
            continue;
        }

        let tag_name = node.tag_name().name();
        let text = node
            .text()
            .map(std::string::ToString::to_string)
            .map(clean_text);

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
fn parse_capsule_view(view: Node) -> CSView {
    fn parse_child(child: Node) -> Option<BoxedCapsuleObject> {
        if child.is_text() || child.is_comment() {
            return None;
        }

        let tag_name = child.tag_name().name();
        let child_text = child
            .text()
            .map(std::string::ToString::to_string)
            .map(clean_text);

        let children = ConcurrentVec::new();

        for child in child.children() {
            let c = parse_child(child);
            if let Some(c) = c {
                children.push(c);
            }
        }

        let mut style = Styling::default();
        let events = ConcurrentVec::new();

        primitive_attr!(child, style, font_size, u16);
        dimension_attr!(child, style, width);
        dimension_attr!(child, style, height);
        enum_attr!(child, style, align, COAlignItems);
        enum_attr!(child, style, justify, COJustifyContent);
        enum_attr!(child, style, flexdir, COFlexDirection);
        color_attr!(child, style, color);
        color_attr!(child, style, background_color);
        event_attr!(child, events, onclick);

        let mut style_clone = style.clone();

        let children_arc = children.into();
        let style_arc = RwLock::new(style).into();
        let events_arc = events.into();

        let mut ctx = CapsuleObjectCreationContext::new(children_arc, events_arc, style_arc);

        match tag_name {
            "text" => Some(Arc::new(CSText::new(
                child_text.as_ref().unwrap().clone(),
                ctx,
            ))),
            "obj" => Some(Arc::new(CSObj::new(ctx))),
            "br" => {
                style_clone.width = Some(CODimension::Points(0.0));
                style_clone.height = Some(CODimension::Points(BR_LINE_HEIGHT));
                ctx.style = RwLock::new(style_clone).into();

                Some(Arc::new(CSObj::new(ctx)))
            }
            "script" => Some(Arc::new(CSScript::new(
                child_text.as_ref().unwrap().clone(),
            ))),
            "view" => Some(Arc::new(CSView::new(ctx))),
            _ => {
                log::warn!("unknown node type: '{tag_name}'");
                None
            }
        }
    }

    let out = parse_child(view);
    if out.is_none() {
        let out = CSView::default();
        log::error!("view is not a valid element!");
        return out;
    }
    let out = out.as_ref().unwrap().as_any().downcast_ref::<CSView>();
    if out.is_none() {
        let out = CSView::default();
        log::error!("view is not a valid view element!");
        return out;
    }

    out.unwrap().clone()
}

pub fn parse_capsule(text: &str) -> anyhow::Result<Capsule> {
    let mut capsule = Capsule::default();
    let xml_document = roxmltree::Document::parse(text)?;

    if xml_document.root_element().tag_name().name() != "capsule" {
        return Err(anyhow::anyhow!("Root node is not of tag capsule"));
    }

    let root_children = xml_document.root_element().children();

    for root_child in root_children {
        if root_child.is_text() {
            continue;
        }

        if root_child.tag_name().name() != "meta" && root_child.tag_name().name() != "view" {
            return Err(anyhow::anyhow!("Sub-root node is not of tag meta or view"));
        }

        if root_child.tag_name().name() == "meta" {
            capsule.meta = parse_capsule_meta(root_child);
        } else {
            capsule.view = parse_capsule_view(root_child);
        }
    }

    Ok(capsule)
}
