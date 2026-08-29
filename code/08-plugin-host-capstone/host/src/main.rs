use std::path::Path;

use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Engine, Result, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};

struct HostState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

// No bindgen! for a fixed "plugin" world here on purpose: this host looks
// up `name` and `transform` by string at instantiation time, so it can
// load a plugin it has never seen the WIT for, as long as that plugin
// happens to export functions matching this shape. That's the entire
// point of a plugin host - the contract is a convention the host checks
// at runtime, not a type the compiler checked in advance.
fn run_plugin(engine: &Engine, path: &Path, input: &str) -> Result<()> {
    let mut linker = Linker::<HostState>::new(engine);
    // Every plugin built by cargo-component today imports the full
    // wasi:cli world regardless of what its own WIT declares (see
    // README.md) - add_to_linker_sync satisfies that import requirement.
    // What actually matters for the "no ambient capability" story is the
    // *empty* WasiCtx below: nothing is inherited, not even stdout.
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

    let component = Component::from_file(engine, path)?;
    let mut store = Store::new(
        engine,
        HostState {
            ctx: WasiCtx::builder().build(),
            table: ResourceTable::new(),
        },
    );

    let instance = linker.instantiate(&mut store, &component)?;

    let name_fn = instance.get_typed_func::<(), (String,)>(&mut store, "name")?;
    let (name,) = name_fn.call(&mut store, ())?;

    let transform_fn =
        instance.get_typed_func::<(&str,), (String,)>(&mut store, "transform")?;
    let (output,) = transform_fn.call(&mut store, (input,))?;

    println!("{name}: {input:?} -> {output:?}");
    Ok(())
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let plugin_dir = args.next().expect("usage: host <plugin-dir> <input>");
    let input = args.next().unwrap_or_else(|| "Hello, plugins!".to_string());

    let engine = Engine::default();

    let mut entries: Vec<_> = std::fs::read_dir(&plugin_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "wasm"))
        .collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        run_plugin(&engine, &entry.path(), &input)?;
    }

    Ok(())
}
