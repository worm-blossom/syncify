# Syncify

Two functions for the price of one: a simplistic rust macro for writing async functions once and getting both the async version and a sync version that strips all `async` function modifiers and all `.await` suffixes.

Status: it works, but this is not intended to be a high-quality, general-purpose crate. The macro error messages are bad, the features are barebones. But it does what *we* need it to do, and that suffices.

## Usage

Place the `#[syncify(name_of_sync_mod_variant)]` on a `mod` item. Annotate all `use` items inside that `mod` that should be different in the sync version with `#[syncify_replace(<alternate-use-item>)]`.

Example:

```rust
use syncify::syncify;
use syncify::syncify_replace;

#[syncify(greet_sync)]
mod greet {
    #[syncify_replace(use crate::speaking_sync::speak;)] // A sync function for speaking.
    use crate::speaking::speak; // An async function for speaking.

    pub async fn do_greet(name: &str) -> usize {
        speak(name).await;
        return name.len();
    }
}
```

Expands to:

```rust
use syncify::syncify;
use syncify::syncify_replace;

mod greet {
    use crate::speaking::speak; // An async function for speaking.

    pub async fn do_greet(name: &str) -> usize {
        speak(name).await;
        return name.len();
    }
}

mod greet_sync {
    use crate::speaking_sync::speak; // A sync function for speaking.

    pub fn do_greet(name: &str) -> usize {
        speak(name);
        return name.len();
    }
}
```