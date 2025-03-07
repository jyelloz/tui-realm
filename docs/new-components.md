# Implement new components

- [Implement new components](#implement-new-components)
  - [Introduction](#introduction)
  - [Setup](#setup)
  - [States](#states)
  - [Properties](#properties)
    - [About properties](#about-properties)
      - [Texts](#texts)
      - [PropPayload](#proppayload)
  - [Implement Component trait](#implement-component-trait)
    - [Render](#render)
    - [Update](#update)
    - [Get Props](#get-props)
    - [On](#on)
    - [Get State](#get-state)
    - [Focus](#focus)
    - [To summarize](#to-summarize)
  - [What's next](#whats-next)

---

## Introduction

This document describes how to implement a new component in tui-realm. This procedure is both valid to extend the standard library in a pull request and to implement your own private components.
Tui-Realm has been designed to make as simpler as possible to implement new components, so let's see in what this procedure consists:

- Define the states of your component if you need any
- Define the properties you're going to use
- Implement the prop builder for your component (*optional - you can use the GenericPropsBuilder, but I strongly suggest to implement your own*)
- Implement the **Component** trait for your component

If you're new to tui-realm, I strongly suggest to read the [component lifecycle guide](lifecycle.md), which also explains each part of a component in details ☺.

Okay, let's start then!

## Setup

We'll only need one file for this, so let's say we want to implement a simple `Counter` component: this component will increment a state when the user presses Submit and will show a button with a customizable text and the counter value.
So let's say we're going to work in a file `counter.rs`.
The first thing we need to do is to import what we need:

```rust
extern crate tuirealm;

use tuirealm::components::utils::get_block;
use tuirealm::event::{Event, KeyCode};
use tuirealm::props::{BordersProps, PropPayload, PropValue, Props, PropsBuilder, TextParts};
use tuirealm::tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tuirealm::{Canvas, Component, Msg, Payload, Value};
```

## States

Let's define the states we need. In this case we only need to define a counter value which will be an `usize`.

```rust
// -- states

struct OwnStates {
    counter: usize,
    focus: bool,
}

impl Default for OwnStates {

    fn default() -> Self {
        OwnStates {
            counter: 0,
            focus: false,
        }
    }

}

impl OwnStates {

    pub fn incr(&mut self) {
        self.counter += 1;
    }

}

```

The struct must be private, and by convention is called `OwnStates`. We define a counter value and the focus state. The focus flag is used to change the component color when is active, in order to give the user the feedback it is enabled.

## Properties

Let's define the properties we'll be using and after that we'll make the props builder for it.
I don't want to make it complicated, so let's say we only need these properties:

- foreground color
- background color
- label string
- border properties
- initial value

First of all we always need to implement these traits for the props builder:

```rust
// -- Props

pub struct CounterPropsBuilder {
    props: Option<Props>,
}

impl Default for CounterPropsBuilder {
    fn default() -> Self {
        let mut builder = CounterPropsBuilder {
            props: Some(Props::default()),
        };
        builder
    }
}

impl PropsBuilder for CounterPropsBuilder {
    fn build(&mut self) -> Props {
        self.props.take().unwrap()
    }

    fn hidden(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = false;
        }
        self
    }

    fn visible(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = true;
        }
        self
    }
}

impl From<Props> for CounterPropsBuilder {
    fn from(props: Props) -> Self {
        CounterPropsBuilder { props: Some(props) }
    }
}

```

Then let's define the property setters:

```rust
impl CounterPropsBuilder {
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    pub fn with_borders(
        &mut self,
        borders: Borders,
        variant: BorderType,
        color: Color,
    ) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.borders = BordersProps {
                borders,
                variant,
                color,
            }
        }
        self
    }

    pub fn with_label(&mut self, label: String) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.texts = TextParts::new(Some(label), None);
        }
        self
    }

    pub fn with_value(&mut self, counter: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert("value", PropPayload::One(PropValue::Usize(counter)));
        }
        self
    }
}
```

### About properties

The property struct contains basically everything you need to create the best components. When implementing a builder you can work with these attributes:

- **visible**: a boolean which should be used to indicate whether the component must be rendered or not
- **foreground**: the foreground color. Usually each tui widget has a foreground
- **background**: the background color. Usually each tui widget has a background
- **borders**: defines the style and the properties for the widget block
- **modifiers**: the modifiers for the text (default). To differentiate modifiers across text parts, define a style for the spans.
- **palette**: defines the color palette. Useful if you  want to define more colors, than foreground and background. You could also use `own`, but own wasn't there when palette was defined for the first time.
- **texts**: contains the texts for the components. For more details read [texts](#texts)
- **own**: an key-value storage to store custom values. The values must use the `PropPayload` syntax. For more details read [PropPayload](#proppayload)

#### Texts

Component texts are defined inside a struct called `TextParts` which is defined as:

```rust
pub struct TextParts {
    pub title: Option<String>,
    pub spans: Option<Vec<TextSpan>>,
    pub table: Option<Table>, // First vector is rows, inner vec is column
}
```

where:

- **title** should describe the title for the container
- **spans** is a multi-part text with its own style
- **table** should be used to create a row x cols text

In general spans and table should never been used at the same time.

To build text parts there are some helpers such as `TextSpanBuilder` and `TableBuilder`.

#### PropPayload

The PropPayload is very similiar to `Payload`, but this one is used only to store properties into the component.
The payload supports the same data types as `Payload`:

```rust
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
```

while values are somehow different from `Value`:

```rust
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
```

## Implement Component trait

Finally we just need to implement the Component trait for our component, but first we need to define it:

```rust
// -- Component

pub struct Counter {
    props: Props,
    states: OwnStates,
}

impl Counter {
    pub fn new(props: Props) -> Self {
        let mut states: OwnStates = OwnStates::default();
        // Init counter
        if let Some(PropPayload::One(PropValue::Unsigned(val))) = props.own.get("input") {
            states.counter = *val;
        }
        Counter { props, states }
    }
}
```

Then let's define each method for the component

```rust
impl Component for Counter {
  // ...
}
```

### Render

The render method must, indeed, render the component into the canvas.
To do so, we'll need to define colors etc. I will make it very simple here. Unfortunately I cannot help you to implement yours, but try to give a look at the tui documentation 😄.

```rust
    fn render(&self, render: &mut Canvas, area: Rect) {
        // Make a Span - THIS IS VERY IMPORTANT!!!
        if self.props.visible {
            // Make text
            let prefix: String = match self.props.texts.title.as_ref() {
                None => String::new(),
                Some(t) => t.clone(),
            };
            let text: String = format!("{} ({})", prefix, self.states.counter);
            let block: Block = get_block(&self.props.borders, &None, self.states.focus);
            let (fg, bg) = match self.states.focus {
                true => (self.props.foreground, self.props.background),
                false => (Color::Reset, Color::Reset),
            };
            render.render_widget(
                Paragraph::new(text).block(block).style(
                    Style::default()
                        .fg(fg)
                        .bg(bg)
                        .add_modifier(self.props.modifiers),
                ),
                area,
            );
        }
    }
```

### Update

Update, as you might know must update the component properties. This also returns a Msg; in our case we'll return a `OnChange` message if the value of the counter is changed.

```rust
    fn update(&mut self, props: Props) -> Msg {
        let prev_value = self.states.counter;
        // Get value
        if let Some(PropPayload::One(PropValue::Unsigned(val))) = props.own.get("input") {
            self.states.counter = *val;
        }
        self.props = props;
        // Msg none
        if prev_value != self.states.counter {
            Msg::OnChange(self.get_state())
        } else {
            Msg::None
        }
    }
```

### Get Props

Get props is kinda standard, since just clones the properties.

```rust
    fn get_props(&self) -> Props {
        self.props.clone()
    }
```

### On

On must handle an input event received from crossterm. In this case I'm just going to return a OnChange after `Enter` is pressed, while `OnKey` will be returned for all the other keys. The counter will be obviously incremented before.

```rust
    fn on(&mut self, ev: Event) -> Msg {
        // Match event
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Enter => {
                    // Increment first
                    self.states.incr();
                    // Return OnChange
                    Msg::OnChange(self.get_state())
                }
                _ => {
                    // Return key event to activity
                    Msg::OnKey(key)
                }
            }
        } else {
            // Ignore event
            Msg::None
        }
    }
```

### Get State

Get state just exposes a meaningful state to the application. In this case it is obviously the counter value, so we'll return a `Payload::Unsigned`:

```rust
    fn get_state(&self) -> Payload {
        Payload::One(Value::Usize(self.states.counter))
    }
```

### Focus

Finally we just need to implement `blur` and `active` for the component:

```rust
    fn blur(&mut self) {
        self.states.focus = false;
    }

    fn active(&mut self) {
        self.states.focus = true;
    }
```

### To summarize

You finally know how to implement a component in tui realm. As you've probably seen tuirealm gives you the possibility to use the `tui` crate with `tuirealm::tui`.
So basically it's not that complicated and the process just consists in:

1. Implement states
2. Implement props builder
3. Implement the Component trait

you can now see the component we've just implemented running the example:

```sh
cargo run --features="with-components" --example counter
```

---

## What's next

The only thing remained to do now, is to implement your application. There's nothing else you need to learn about tui-realm to build an excelent application 🦄.
Remember that tui-realm already provides some standard components; check out [here](std-components.md)!
