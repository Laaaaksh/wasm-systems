use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Engine, Result, Store};
use wasmtime_wasi::p2::bindings::sync::Command;
use wasmtime_wasi::{FsPerms, WasiCtx, WasiCtxView, WasiView};

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

fn run(component_path: &str, grant_dir: Option<&str>) -> Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::<HostState>::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

    let component = Component::from_file(&engine, component_path)?;

    // The capability decision lives entirely here, in the host, not in the
    // guest. `inherit_stdout` grants stdout unconditionally, matching every
    // other sample in this repo. The filesystem grant is conditional on
    // `grant_dir` - this is the whole demonstration.
    let mut builder = WasiCtx::builder();
    builder.inherit_stdout();
    if let Some(dir) = grant_dir {
        builder.preopened_dir(dir, ".", FsPerms::ReadOnly)?;
    }

    let mut store = Store::new(
        &engine,
        HostState {
            ctx: builder.build(),
            table: ResourceTable::new(),
        },
    );

    let command = Command::instantiate(&mut store, &component, &linker)?;
    match command.wasi_cli_run().call_run(&mut store)? {
        Ok(()) => Ok(()),
        Err(()) => Err(wasmtime::Error::msg("guest returned an error")),
    }
}

fn main() -> Result<()> {
    let component_path = std::env::args()
        .nth(1)
        .expect("usage: host-demo-runner <path-to-guest.wasm> [dir-to-grant]");
    let grant_dir = std::env::args().nth(2);

    println!("--- host: instantiating with no filesystem grant ---");
    run(&component_path, None)?;

    if let Some(dir) = grant_dir {
        println!();
        println!("--- host: instantiating with '{dir}' granted read-only ---");
        run(&component_path, Some(&dir))?;
    }

    Ok(())
}
