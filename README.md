# thiserror + error-stack 
# Introduction
This repository is a sample of how to use [thiserror](https://github.com/dtolnay/thiserror) along with [error-stack](https://github.com/hashintel/hash/tree/main/libs/error-stack) to create a custom error type with a stack trace.

# Example
Running a test command:
```bash
cargo test --package transform --lib -- tests::transform_file_error --exact --nocapture 
```

Ouptut:

```bash
running 1 test
[TransformError::TransformString] Error when transform string
├╴at transform/src/lib.rs:112:18
│
├─▶ [TransformError::AddAgeGroup] Error when add age group to person: Person {
│       name: "Jane",
│       age: 200,
│       age_group: None,
│   }
│   ╰╴at transform/src/lib.rs:120:14
│
╰─▶ [TransformError::ExceptionalAge] Invalid age: 200
    ├╴at transform/src/lib.rs:46:59
    ╰╴backtrace (1)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

backtrace no. 1
   0: std::backtrace_rs::backtrace::libunwind::trace
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2: std::backtrace::Backtrace::create
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/backtrace.rs:332:13
   3: error_stack::report::Report<C>::from_frame
             at .cargo/registry/src/github.com-1ecc6299db9ec823/error-stack-0.3.1/src/report.rs:294:30
   4: error_stack::report::Report<C>::new
             at .cargo/registry/src/github.com-1ecc6299db9ec823/error-stack-0.3.1/src/report.rs:274:9
   5: error_stack::context::<impl core::convert::From<C> for error_stack::report::Report<C>>::from
             at .cargo/registry/src/github.com-1ecc6299db9ec823/error-stack-0.3.1/src/context.rs:83:9
   6: <core::result::Result<T,E> as error_stack::result::IntoReport>::into_report
             at .cargo/registry/src/github.com-1ecc6299db9ec823/error-stack-0.3.1/src/result.rs:203:31
   7: transform::AgeGroup::from_age
             at ./src/lib.rs:46:18
   8: transform::Transform::add_age_group
             at ./src/lib.rs:119:28
   9: transform::Transform::transform_string
             at ./src/lib.rs:111:13
  10: transform::tests::transform_file_error::transform_file_error
             at ./src/lib.rs:152:19
  11: transform::tests::transform_file_error
             at ./src/lib.rs:149:5
  12: transform::tests::transform_file_error::{{closure}}
             at ./src/lib.rs:149:5
  13: core::ops::function::FnOnce::call_once
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:250:5
  14: core::ops::function::FnOnce::call_once
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:250:5
  15: test::__rust_begin_short_backtrace
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/test/src/lib.rs:656:18
  16: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/alloc/src/boxed.rs:1987:9
  17: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panic/unwind_safe.rs:271:9
  18: std::panicking::try::do_call
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  19: std::panicking::try
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  20: std::panic::catch_unwind
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  21: test::run_test_in_process
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/test/src/lib.rs:679:27
  22: test::run_test::run_test_inner::{{closure}}
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/test/src/lib.rs:573:39
  23: test::run_test::run_test_inner::{{closure}}
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/test/src/lib.rs:600:37
  24: std::sys_common::backtrace::__rust_begin_short_backtrace
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:134:18
  25: std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/thread/mod.rs:560:17
  26: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panic/unwind_safe.rs:271:9
  27: std::panicking::try::do_call
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  28: std::panicking::try
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  29: std::panic::catch_unwind
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  30: std::thread::Builder::spawn_unchecked_::{{closure}}
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/thread/mod.rs:559:30
  31: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:250:5
  32: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/alloc/src/boxed.rs:1987:9
  33: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/alloc/src/boxed.rs:1987:9
  34: std::sys::unix::thread::Thread::new::thread_start
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys/unix/thread.rs:108:17
  35: __pthread_deallocate
test tests::transform_file_error ... ok
```