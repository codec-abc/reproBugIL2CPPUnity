Project to reproduce bugs with out keyword in IL2CPP

The bug which is happening is in the code generation of IL2CPP when using the C# ```out``` keyword.

## Context:

In the following we assume a 64 bits system. On this system, the Rust ```isize``` type is a 64 bits unsigned integer which is the same as the C# ```long```.
Now, let's say we have a native function that take a pointer (in this case a unsigned 64 bits integer).
We would have something like this:

```rust
#[no_mangle]
pub fn a_function(x : *mut isize) {
    // some code here... that can read and write to this value.
}
```

The C# corresponding code can be 

```csharp
[DllImport("__Internal")] // or whatever dll name we are using
public static extern void a_function (ref long x);
```

or also

```csharp
[DllImport("__Internal")] // or whatever dll name we are using
public static extern void a_function (out long x);
```

Using Mono in the editor both case work fine. Unfortunately, when compiling to iOS only the first case works as expected. The C++ code generation done by IL2CPP is wrong for the ```out``` case.

This is the code generated by IL2CPP when using the ```ref``` keyword:

```c++
extern "C"  void ffi_print_and_change_value_ios_m2255698473 (RuntimeObject * __this /* static, unused */, int64_t* ___x0, const RuntimeMethod* method)
{
	typedef void (DEFAULT_CALL *PInvokeFunc) (int64_t*);

	// Native function invocation
	reinterpret_cast<PInvokeFunc>(print_and_change_value_ios)(___x0);

}
```

And this is the code generated by IL2CPP when using the ```out``` keyword:

```c++
extern "C"  void ffi_print_and_change_value_ios_m2255698473 (RuntimeObject * __this /* static, unused */, int64_t* ___x0, const RuntimeMethod* method)
{
	typedef void (DEFAULT_CALL *PInvokeFunc) (int64_t*);

	// Marshaling of parameter '___x0' to native representation
	int64_t ____x0_empty = 0;

	// Native function invocation
	reinterpret_cast<PInvokeFunc>(print_and_change_value_ios)((&____x0_empty));

	// Marshaling of parameter '___x0' back from native representation
	*___x0 = ____x0_empty;
}
```

As we can see the code create a variable initialized to 0 and pass it to the native code instead of passing the existing value.

## To build the library (optional)

If you wan to build the library install the last stable Rust toolchain for MacOS with rustup. Download also the
```aarch64-apple-ios``` toolchain to cross-compile the library for iOS.

### Dynamic library for MacOS

To build a dynamic library for MacOS, edit the ```Cargo.toml``` file and set the crate type (the ```crate-type``` line) to ```"cdylib"``` under the ```[lib]``` section. Then run ```cargo build --release```to build the dynamic library.

### Static library for iOS

To build a static library for iOS, edit the ```Cargo.toml``` file and set the crate type (the ```crate-type``` line) to ```"staticlib"``` under the ```[lib]``` section. Then run ```cargo build --release --target aarch64-apple-ios``` to build the static iOS library.

## To reproduce the bug

Use the scene ```Assets/main.unity```.

## With the Unity editor

You cannot see the bug with the editor as the code run correctly with Mono either with ref or out. To see what the native function is able to read from memory the result are logged to ```~/result.txt```.

## When building for iOS

You can see the bug using the code when the external function ```print_and_change_value_ios``` is using a out parameter. When looking at the standard output shown in XCode you will see a value of 0 (instead of 11). If you update the signature of ```print_and_change_value_ios``` to use a ```ref``` parameter the code will now correctly and you will be able to see the expected output in the standard output shown in XCode.
