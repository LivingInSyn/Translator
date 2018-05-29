# Translator
Translator is a rust procedural macro which translates rust structs into other languages for exposure over a rust FFI (or however else you want to use them). Structs that are not ```[repr(C)]``` are ignored. The structs are translated into:

* Python
* C++
* C#

This macro isn't a magic bullet, but it will (hopefully) greatly reduce the cost of creating Rust libraries for use with other languages

## Sample
### Input
This rust input:
```rust
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
```

Would be translated into:

### c++
```c++
typedef struct SomeStructTag {
	int foo;
	Baz bar;
	unsigned char foobar[5];
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

