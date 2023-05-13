# exefs-module-example
An example or PoC Nintendo Switch exefs NSO module, loadable by NX rtld.

## Build
- Requires `aarch64-nintendo-switch-freestanding` target

  (refer to https://github.com/aarch64-switch-rs/setup-guide)
- Requires `linkle` to be installed with `cargo install linkle` 

You can run  `make-nso.bat` to compile elf and compress it to NSO format.

This should work on both Windows and Linux. 

Output file will be named `subsdk6` and you can throw it to `exefs` folder/image 
of any application that has `rtld` module. If it was loaded successfully, it will print 
[`ModuleObject`](https://github.com/marysaka/oss-rtld/blob/master/librtld/include/rtld/ModuleObject.hpp) 
data of each module linked by `rtld` using `OutputDebugString` Syscall. 