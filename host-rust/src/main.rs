use anyhow::Context;
use std::collections::HashMap;
use wasmtime::{
    component::{Component, Linker, Resource, ResourceType},
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

#[derive(Default)]
struct Headers {
    inner: HashMap<String, String>,
}

impl Headers {
    fn set(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    fn get(&self, key: String) -> Option<String> {
        self.inner.get(&key).cloned()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    config.consume_fuel(true);

    let engine = Engine::new(&config)?;

    let component = Component::from_file(
        &engine,
        "../guest-rust/target/wasm32-wasi/release/guest_rust.wasm",
    )?;

    exec(&engine, &component).await?;

    Ok(())
}

async fn exec(engine: &Engine, component: &Component) -> anyhow::Result<()> {
    let wasi = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .inherit_env()
        .inherit_network()
        .build();

    let mut table = ResourceTable::new();

    let headers = table.push(Headers::default())?;

    let state = State { ctx: wasi, table };

    let mut store = Store::new(engine, state);
    store.set_fuel(u64::MAX)?;
    store.fuel_async_yield_interval(Some(10000))?;

    let mut linker = Linker::<State>::new(engine);
    wasmtime_wasi::add_to_linker_async(&mut linker)?;

    linker
        .root()
        .resource("headers", ResourceType::host::<Headers>(), |_, _| Ok(()))?;

    linker
        .root()
        .func_wrap::<_, (Resource<Headers>, String, String), ()>(
            "[method]headers.set",
            move |mut store, (resource, key, val)| {
                let headers = store.data_mut().table.get_mut(&resource).unwrap();
                headers.set(key, val);

                Ok(())
            },
        )?;

    linker
        .root()
        .func_wrap::<_, (Resource<Headers>, String), (Option<String>,)>(
            "[method]headers.get",
            move |mut store, (resource, key)| {
                let headers = store.data_mut().table.get_mut(&resource).unwrap();
                let val = headers.get(key);

                Ok((val,))
            },
        )?;

    let instance = linker
        .instantiate_async(&mut store, component)
        .await
        .context("linker")?;

    let fun = instance
        .get_typed_func::<(Resource<Headers>,), (Result<(), String>,)>(
            &mut store,
            "request-callback",
        )
        .context("get func")?;

    let res = fun
        .call_async(&mut store, (headers,))
        .await
        .context("call")?;

    let _ = dbg!(res);

    Ok(())
}
