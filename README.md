# Translator
Translator is a rust procedural macro which translates rust structs into other languages for exposure over a rust FFI (or however else you want to use them). Structs that are not ```[repr(C)]``` are ignored. The structs are translated into:

* Python
* C++
* C#

This macro isn't a magic bullet, but it will (hopefully) greatly reduce the cost of creating Rust libraries for use with other languages

## Use
### Input
Say you want to translate the following structs:
```rust
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SomeStruct {
    //pub raw_message: [i16;5],
    pub foo: i32,
    pub bar: Baz,
    pub foobar: [u8;5]
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Baz {
    pub bob: f32
}
```

You would have to add the Translator macro, and the ```Translator``` to the derivations. You would also have to add the 'magic struct' to the end of the struct declarations. It would look like this:

```rust
#[macro_use]
extern crate translator;

#[repr(C)]
#[derive(Clone, Copy, Translate)]
pub struct SomeStruct {
    //pub raw_message: [i16;5],
    pub foo: i32,
    pub bar: Baz,
    pub foobar: [u8;5]
}

#[repr(C)]
#[derive(Clone, Copy, Translate)]
pub struct Baz {
    pub bob: f32
}

#[derive(Translate)]
struct __FinalizeTranslatorStruct__{}
```

When you compile, in the 'target' folder a new folder will be created named 'TranslateOutput' with 3 files (one for each language) with the following contents:

### c++
```c++
typedef struct SomeStructTag {
	int32_t foo;
	Baz bar;
	uint8_t foobar[5];
} SomeStruct;

typedef struct BazTag {
	float bob;
} Baz;
```

### Python
```python
class SomeStruct(Structure):
        _fields_ = [
        ("foo", c_int),
        ("bar", Baz),
        ("foobar", c_ubyte * 5),
        ]

class Baz(Structure):
        _fields_ = [
        ("bob", c_float),
        ]
```

### C#
```csharp
[StructLayout(LayoutKind.Sequential)]
public struct SomeStruct
{
    public int foo;
    public Baz bar;
    [MarshalAs(UnmanagedType.ByValArray, SizeConst = 5)]
    public byte[] foobar;
}

[StructLayout(LayoutKind.Sequential)]
public struct Baz
{
    public float bob;
}
```

<a rel="me" href="https://infosec.exchange/@livinginsyn">Mastodon</a>