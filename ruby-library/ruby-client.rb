require 'ffi'

module Rust
  extend FFI::Library
  ffi_lib '../rust-library/target/debug/liblibrary.dylib'

  attach_function :hello_world, [], :string
end
