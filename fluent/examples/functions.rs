extern crate fluent;

use fluent::types::FluentValue;
use fluent::FluentBundle;

fn main() {
    let mut bundle = FluentBundle::new(&["en-US"]);

    // Test for a simple function that returns a string
    bundle.add_function("HELLO", |_args, _named_args| {
        return Some("I'm a function!".into());
    });

    // Test for a function that accepts unnamed positional arguments
    bundle.add_function("MEANING_OF_LIFE", |args, _named_args| {
        if let Some(arg0) = args.get(0) {
            if *arg0 == Some(FluentValue::Number(String::from("42"))) {
                return Some("The answer to life, the universe, and everything".into());
            }
        }

        None
    });

    // Test for a function that accepts named arguments
    bundle.add_function("BASE_OWNERSHIP", |_args, named_args| {
        let ownership = named_args.get("ownership").unwrap();

        return match ownership {
            &FluentValue::String(ref string) => {
                Some(format!("All your base belong to {}", string).into())
            }
            _ => None,
        };
    });

    bundle.add_messages("hello-world = Hey there! { HELLO() }");
    bundle.add_messages("meaning-of-life = { MEANING_OF_LIFE(42) }");
    bundle.add_messages("all-your-base = { BASE_OWNERSHIP(hello, ownership: \"us\") }");

    let value = bundle.format("hello-world", None);
    assert_eq!(value, Some("Hey there! I'm a function!".to_string()));

    let value = bundle.format("meaning-of-life", None);
    assert_eq!(
        value,
        Some("The answer to life, the universe, and everything".to_string())
    );

    let value = bundle.format("all-your-base", None);
    assert_eq!(value, Some("All your base belong to us".to_string()));
}