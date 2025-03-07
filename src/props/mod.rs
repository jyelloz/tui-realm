//! ## Props
//!
//! `Props` is the module which defines properties for layout components

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// ext
use tui::style::{Color, Modifier};

// modules
pub mod borders;
pub mod builder;
pub mod texts;

// Exports
pub use borders::{Borders, BordersProps};
pub use builder::{GenericPropsBuilder, PropsBuilder};
pub use texts::{Table, TableBuilder, TextParts, TextSpan, TextSpanBuilder};

use std::collections::{HashMap, LinkedList};

// -- Props

/// ## Props
///
/// Props holds all the possible properties for a layout component
#[derive(Clone)]
pub struct Props {
    // Values
    pub visible: bool,         // Is the element visible ON CREATE?
    pub foreground: Color,     // Foreground color
    pub background: Color,     // Background color
    pub borders: BordersProps, // Borders
    pub modifiers: Modifier,
    pub palette: HashMap<&'static str, Color>, // Use palette to store extra colors
    pub texts: TextParts,                      // text parts
    pub own: HashMap<&'static str, PropPayload>, // Own properties (extra)
}

impl Default for Props {
    fn default() -> Self {
        Self {
            // Values
            visible: true,
            foreground: Color::Reset,
            background: Color::Reset,
            borders: BordersProps::default(),
            modifiers: Modifier::empty(),
            palette: HashMap::new(),
            texts: TextParts::default(),
            own: HashMap::new(),
        }
    }
}

// -- Prop value

/// ## PropPayload
///
/// Payload describes a property initial value payload, which contains the actual value in different kind of storage
#[derive(Debug, PartialEq, Clone)]
pub enum PropPayload {
    One(PropValue),
    Tup2((PropValue, PropValue)),
    Tup3((PropValue, PropValue, PropValue)),
    Tup4((PropValue, PropValue, PropValue, PropValue)),
    Vec(Vec<PropValue>),
    Map(HashMap<String, PropValue>),
    Linked(LinkedList<PropPayload>),
    None,
}

/// ## PropValue
///
/// Value describes the value contained in a `PropPayload`
#[derive(Debug, PartialEq, Clone)]
pub enum PropValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    F64(f64),
    F32(f32),
    Str(String),
    Color(Color),
    InputType(InputType),
}

// -- Input Type

/// ## InputType
///
/// Input type for text inputs
#[derive(Clone, Copy, PartialEq, std::fmt::Debug)]
pub enum InputType {
    Text,
    Number,
    Password,
}

#[cfg(test)]
mod tests {

    use super::borders::BorderType;
    use super::*;

    #[test]
    fn test_props_default() {
        let props: Props = Props::default();
        assert_eq!(props.visible, true);
        assert_eq!(props.background, Color::Reset);
        assert_eq!(props.foreground, Color::Reset);
        assert_eq!(props.borders.borders, Borders::ALL);
        assert_eq!(props.borders.color, Color::Reset);
        assert_eq!(props.borders.variant, BorderType::Plain);
        assert_eq!(props.modifiers, Modifier::empty());
        assert_eq!(props.palette.len(), 0);
        assert!(props.texts.title.is_none());
        assert_eq!(props.own.len(), 0);
        assert!(props.texts.spans.is_none());
    }

    #[test]
    fn test_props_values() {
        PropPayload::One(PropValue::Usize(2));
        PropPayload::Tup2((PropValue::Bool(true), PropValue::Usize(128)));
        PropPayload::Tup3((
            PropValue::Bool(true),
            PropValue::Usize(128),
            PropValue::Str(String::from("omar")),
        ));
        PropPayload::Tup4((
            PropValue::Bool(true),
            PropValue::U8(128),
            PropValue::Str(String::from("pippo")),
            PropValue::Isize(-2),
        ));
        PropPayload::Vec(vec![
            PropValue::U16(1),
            PropValue::U32(2),
            PropValue::U64(3),
            PropValue::U128(4),
        ]);
        let mut map: HashMap<String, PropValue> = HashMap::new();
        map.insert(String::from("a"), PropValue::I8(4));
        assert_eq!(*map.get("a").unwrap(), PropValue::I8(4));
        map.insert(String::from("b"), PropValue::I16(-8));
        assert_eq!(*map.get("b").unwrap(), PropValue::I16(-8));
        map.insert(String::from("c"), PropValue::I32(16));
        assert_eq!(*map.get("c").unwrap(), PropValue::I32(16));
        map.insert(String::from("d"), PropValue::I64(-32));
        assert_eq!(*map.get("d").unwrap(), PropValue::I64(-32));
        map.insert(String::from("e"), PropValue::I128(64));
        assert_eq!(*map.get("e").unwrap(), PropValue::I128(64));
        map.insert(String::from("f"), PropValue::Color(Color::Red));
        assert_eq!(*map.get("f").unwrap(), PropValue::Color(Color::Red));
        map.insert(String::from("g"), PropValue::InputType(InputType::Number));
        assert_eq!(
            *map.get("g").unwrap(),
            PropValue::InputType(InputType::Number)
        );
        map.insert(String::from("h"), PropValue::U8(0));
        assert_eq!(*map.get("h").unwrap(), PropValue::U8(0));
        map.insert(String::from("i"), PropValue::Bool(true));
        assert_eq!(*map.get("i").unwrap(), PropValue::Bool(true));
        map.insert(String::from("j"), PropValue::U16(256));
        assert_eq!(*map.get("j").unwrap(), PropValue::U16(256));
        map.insert(String::from("k"), PropValue::U32(65536));
        assert_eq!(*map.get("k").unwrap(), PropValue::U32(65536));
        map.insert(String::from("l"), PropValue::U64(10000000000));
        assert_eq!(*map.get("l").unwrap(), PropValue::U64(10000000000));
        map.insert(String::from("m"), PropValue::Isize(200));
        assert_eq!(*map.get("m").unwrap(), PropValue::Isize(200));
        map.insert(String::from("n"), PropValue::F32(0.23));
        assert_eq!(*map.get("n").unwrap(), PropValue::F32(0.23));
        map.insert(String::from("s"), PropValue::InputType(InputType::Number));
        assert_eq!(
            *map.get("s").unwrap(),
            PropValue::InputType(InputType::Number)
        );
        PropPayload::Map(map);
        let mut link: LinkedList<PropPayload> = LinkedList::new();
        link.push_back(PropPayload::One(PropValue::Usize(1)));
        link.push_back(PropPayload::Tup2((
            PropValue::Usize(2),
            PropValue::Usize(4),
        )));
        PropPayload::Linked(link);
    }
}
