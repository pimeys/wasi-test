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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    config.consume_fuel(true);

    let engine = Engine::new(&config)?;

    let component =
        Component::from_file(&engine, "../hello/target/wasm32-wasi/release/hello.wasm")?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .inherit_env()
        .inherit_network()
        .build();

    let state = State {
        ctx: wasi,
        table: ResourceTable::new(),
    };

    let mut store = Store::new(&engine, state);
    store.set_fuel(u64::MAX)?;
    store.fuel_async_yield_interval(Some(10000))?;

    let mut linker = Linker::<State>::new(&engine);
    wasmtime_wasi::add_to_linker_async(&mut linker)?;

    let instance = linker
        .instantiate_async(&mut store, &component)
        .await
        .context("linker")?;

    let fun = instance
        .get_typed_func::<(String,), (String,)>(&mut store, "hello-world")
        .context("get func")?;

    let (result,) = fun
        .call_async(&mut store, (String::from("World"),))
        .await
        .context("call")?;

    dbg!(result);

    Ok(())
}
