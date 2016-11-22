
## Associated types vs type parameters

I started off with

- struct `Property<H, T>` implements trait `trait::Property<H>`.

But really, I should have had

- struct `Property<H, T>` implements trait `trait::Property<Handle=H>`.

which is really obvious in hindsight. This oversight was brought to my attention
when I tried to implement `Debug`

```rust
impl<H: Handle, P: Property<H>> Debug for P
{
    fn fmt(&self, f: &mut Formatter) -> Result { Ok(()) }
}
```
received

```
error[E0207]: the type parameter `H` is not constrained by the impl trait, self type, or predicates
  --> src/property/traits/property.rs:69:6
   |
69 | impl<H: Handle, P> ::std::fmt::Debug for P
   |      ^ unconstrained type parameter
```
It makes sense since `P` could implement `Property<H>` for multiple `H`. So the
handle type should have been an associated type: `Property<Handle = H>`.
(Note that the error above is unrelated to this realization, it just helped me
discover the oversight. Even with `Handle` as an associated type, one runs into
coherence issues and needs to implement `Debug` on the trait object instead.)

Anyway, `traits::Property: Debug` was another alternative, which I'd already
used. I just forgot about it due to working on this off and on.
