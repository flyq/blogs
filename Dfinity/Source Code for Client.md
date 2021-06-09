sudo apt-get install libclang-dev

find /usr -iname "libclang.so" -print
/usr/lib/llvm-10/lib/libclang.so


export LIBCLANG_PATH="/usr/lib/llvm-10/lib/libclang.so"

LIBCLANG_PATH

Compiling lmdb-rkv-sys v0.11.99 (https://github.com/dfinity-lab/lmdb-rs?rev=e45bfaaa9055178ab55f8f31bfece50ebd94f901#e45bfaaa)
The following warnings were emitted during compilation:

warning: couldn't execute `llvm-config --prefix` (error: No such file or directory (os error 2))
warning: set the LLVM_CONFIG_PATH environment variable to the full path to a valid `llvm-config` executable (including the executable itself)

error: failed to run custom build command for `lmdb-rkv-sys v0.11.99 (https://github.com/dfinity-lab/lmdb-rs?rev=e45bfaaa9055178ab55f8f31bfece50ebd94f901#e45bfaaa)`

Caused by:
  process didn't exit successfully: `/home/peter/ic/rs/target/release/build/lmdb-rkv-sys-1ae72409a00fef2f/build-script-build` (exit code: 101)
  --- stdout
  cargo:warning=couldn't execute `llvm-config --prefix` (error: No such file or directory (os error 2))
  cargo:warning=set the LLVM_CONFIG_PATH environment variable to the full path to a valid `llvm-config` executable (including the executable itself)

  --- stderr
  error: header '/root/.cargo/git/checkouts/lmdb-rs-cd739f2dd4762db5/e45bfaa/lmdb-sys/lmdb/libraries/liblmdb/lmdb.h' does not exist.
  thread 'main' panicked at 'Unable to generate bindings: ()', /root/.cargo/git/checkouts/lmdb-rs-cd739f2dd4762db5/e45bfaa/lmdb-sys/bindgen.rs:73:10
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
^C  Building [====================================================>  ] 847/878: librocksdb-sys(build)


https://www.unadulterated-faff.com/articles/2020/02/04/fixing-rust-compilation-issues-caused-by-missing-packages-part-2.html


安装了一堆命令后（参考 Dockerfile，这个能编译通了，但是遇到 

 Compiling ic-replica v0.8.0 (/home/peter/ic/rs/replica)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-Wl,--eh-frame-hdr" "-L" "/root/.rustup/toolchains/nightly-2020-08-03-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/peter/ic/rs/target/release/deps/replica-c927468c32367ae1.replica.17omyrc8-cgu.0.rcgu.o" "/home/peter/ic/rs/target/release/deps/replica-c927468c32367ae1.replica.17omyrc8-cgu.1.rcgu.o" "/home/peter/ic/rs/target/release/deps/replica-c927468c32367ae1.replica.17omyrc8-cgu.10.rcgu.o" "/home/peter/ic/rs/target/release/deps/replica-c927468c32367ae1.replica.17omyrc8-cgu.11.rcgu.o" "/home/peter/ic/rs/target/release/deps/replica-c927468c32367ae1.replica.17omyrc8-cgu.12.rcgu.o" "/home/peter/ic/rs/target/release/deps/replica-c927468c32367ae1.replica.17omyrc8-cgu.13.rcgu.o" "/home/peter/ic/rs/target/release/deps/replica-c927468c32367ae1.replica.17omyrc8-cgu.14.rcgu.o" "/home/peter/ic/rs/target/release/deps/replica-c927468c32367ae1.replica.17omyrc8-cgu.15.rcgu.o" "/home/peter/ic/rs/target/release/deps/replica-c927468c32367ae1