Add very short names to some methods in the core

# Examples
```rust
use short_names::*;

let a = Some(":foo".to_owned());
let b = Some(":bar");
let x = a.ad().o(b).mo("none", |x| &x[1..]);
assert_eq!(x, "foo");
```

```rust
use short_names::*;

let arr = [0, 1, 2, 3, 4, 5];
let vec = arr.iter().co().f(|n| n%2==0).col::<Vec<_>>();
assert_eq!(vec, [0, 2, 4]);
```
