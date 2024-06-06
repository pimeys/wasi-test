use anyhow::Context;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

struct State {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

fn main() -> anyhow::Result<()> {
    let engine = Engine::new(Config::new().wasm_component_model(true))?;

    let component =
        Component::from_file(&engine, "../hello/target/wasm32-wasi/release/hello.wasm")?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .inherit_env()
        .build();

    let state = State {
        ctx: wasi,
        table: ResourceTable::new(),
    };

    let mut store = Store::new(&engine, state);

    let mut linker = Linker::<State>::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let instance = linker
        .instantiate(&mut store, &component)
        .context("linker")?;

    let fun = instance
        .get_typed_func::<(String,), (String,)>(&mut store, "hello-world")
        .context("get func")?;

    let result = fun
        .call(&mut store, (String::from("World"),))
        .context("call")?;

    dbg!(result);

    Ok(())
}
