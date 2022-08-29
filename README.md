# Self-Reference
A Self-Refernece Helper (Inspired From [Selfie](https://github.com/prokopyl/selfie))

this crate provides safe access to its self-reference.
the main idea is not initializing references when object not pinned(which is not safe and makes many self-referenial crate more complex)
providing only pinned reference makes lot more easier to design self-referential object.
on `self-reference` crate. you only can initialize reference object that has `'static` lifetime means always safe.

# Initializing Reference Object

The major difference from other self-referential crate is initializing referential object.
you only can initialize reference object that has `'static` lifetime

```rust
let mut reference: SelfReference<String, Ref<str>> = SelfReference::new(String::new(), || {
    // you can't get reference of object while initialization.
    // only possible cases are reference that has `'static` lifetime.
    ""
});
pin_mut!(reference);

// this is totally SAFE. this lowers `'static` lifetime into some other lifetime.
reference.get_ref();
```

# Reset Mechanism

The only way to initialize reference object is using `reset` method. remember!! you can use reset method when `SelfReference` object is pinned.

```rust
let mut reference: SelfReference<String, Ref<str>> = SelfReference::new("self-reference".to_string(), || "");
pin_mut!(reference);

// You can reset object to set referencial object to hold object itself.
reference.reset_unpin(|s| &s[..4]);
println!("{}", reference.get_ref()); // prints "self"
```