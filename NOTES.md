```
  Windows 10
  
  rustc --version: rustc 1.54.0 (a178d0322 2021-07-26)

```

## Recommended extensions
- I've added 2 extensions to the list, one for Rust tooling and one for the TOML config file.
- 

## Upon first clone, OpenSSL was not resolved
- Potential solution: https://stackoverflow.com/questions/55912871/how-to-work-with-openssl-for-rust-within-a-windows-development-environment
- This happens when I first clone the project and only installed the rustup and solana toolchains without much modifications. 
- This error goes away after I remove the test codes per paul's instructions.
- I decided to keep solana-sdk in dev-dependencies just in case it might be needed, as the guide was a couple months old.

## On code prettier/beautification
- It seems there's no prettier config for rust
- The rust plugin with rustfmt seems helpful, it doesn't enforce certain rules, but it would do for now!
- Rust vscode plugin sucks at auto importing of module or use

## WTF is a BPF program???
- `Berkeley Packet Filter`: https://en.wikipedia.org/wiki/Berkeley_Packet_Filter
- https://blogs.oracle.com/linux/post/bpf-a-tour-of-program-types
- A BPF program is binary code used to filter packet (neworking).

## Rust syntax
- Array.get(..n): This picks n amount of elements from the array
- array.map: this transform each element into another array
- and_then: this is equivalent to a flatmap, it accepts a function instead
- https://fasterthanli.me/articles/a-half-hour-to-learn-rust <- good article to get on track with rust

## Rust debugging
- Using bacon to catch early error:
> cargo install bacon
- https://github.com/Canop/bacon
- Doesn't seem to work on windows...
- Apparently you need to include the module in lib.rs before the compiler complain about it, else it treat the file as draft code.

## Cargo usage
- https://github.com/rust-lang/cargo/issues/5586 <- ?? It takes 3 years and they couldn't finish it because they were wrestling with the TOML format and sorting?? Rust was made by people who doesn't take risk
- use cargo-edit instead: `cargo install cargo-edit`
