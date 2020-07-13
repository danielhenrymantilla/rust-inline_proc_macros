macro_rules! define_strings {(
    $(
        const $VAR:ident = $($s:expr),* $(,)? ;
    )*
) => (
    $(
        #[allow(unused)]
        macro_rules! $VAR { () => (
            concat!($($s),*)
        )}
        #[allow(unused)]
        const $VAR: &str = $VAR!();
    )*
)}

define_strings! {
    const COMPILED_WASM_PATH_TEMPLATE =
        "{out_dir}/inline_proc_macro_{file_id}.wasm"
    ;
}

macro_rules! renv {($name:expr) => (
    &::std::env::var($name)
        .expect(stringify!($name))
)}

pub(in crate)
fn log_stream (ts: impl AsRef<str>)
{
    let ref in_str = ts.as_ref();
    if in_str.len() > 1000 {
        println!(
            "{pre}[...{mid_len} chars...]{post}",
            pre = &in_str[.. 400],
            post = &in_str[(in_str.len() - 400) ..],
            mid_len = in_str.len() - 800,
        )
    } else {
        println!("{}", in_str);
    }
}

macro_rules! mk_debug {(if $debug:expr) => (
    let debug: bool = $debug;
    macro_rules! debug {($expr:expr) => (
        match $expr { expr => {
            if debug {
                eprintln!(
                    "[{}:{}:{}] {} = {:#?}",
                    file!(),
                    line!(),
                    column!(),
                    stringify!($expr),
                    expr,
                );
            }
            expr
        }}
    )}
)}
