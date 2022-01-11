## XML parser WebAssembly - proof of concept

Run the following:

```
npm i && npm run build & npm run serve
```

Open the browser at localhost:8080, then in the console type the following:

```js
init_panic_hook();
```

This will register the Rust panic handler into console, so we see better error messages.

```js
parse('<page mozabook_version="4.5.8.224"><B>"false"</B></page>');
```

This will parse this toy XML to the desired format.

```js
{
  "mozabook_version": "4.5.8.224",
  "file_version": {
    "B": false
  }
}
```

The schema definition is in `src/lib.rs`:

```rust
#[derive(Serialize, Deserialize)]
pub struct Page {
    pub mozabook_version: String,
    #[serde(rename(deserialize = "$value"))]
    pub file_version: Option<Enum>,
}

#[derive(Serialize, Deserialize)]
pub enum Enum {
    A(String),
    B(bool),
}
```
