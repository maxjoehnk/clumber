use std::fmt::Formatter;
use crate::steps::{BuildContext, BuildStep};
use wasmtime::{Config, Engine, Instance, Linker, Module, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

mod api {
    wit_bindgen_wasmtime::import!("./module.wit");
}

#[derive(Debug)]
pub struct ModuleStep {
    module: ModuleRef
}

impl BuildStep for ModuleStep {
    fn label(&self) -> String {
        todo!()
    }

    fn run(&self, context: &mut BuildContext) -> anyhow::Result<()> {
        todo!()
    }
}

pub struct ModuleLoader {
    engine: Engine,
    linker: Linker<EngineData>,
}

struct EngineData {
    wasi: WasiCtx,
    module: self::api::module::ModuleData,
}

impl EngineData {
    pub fn new() -> Self {
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .build();

        Self {
            wasi,
            module: Default::default(),
        }
    }
}

impl ModuleLoader {
    pub fn new() -> anyhow::Result<Self> {
        let engine = Engine::new(Config::new().wasm_multi_memory(true))?;
        let mut linker = Linker::<EngineData>::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi)?;

        Ok(Self {
            engine,
            linker,
        })
    }

    pub fn load(&mut self, file: &str) -> anyhow::Result<ModuleRef> {
        let module = Module::from_file(&self.engine, file)?;
        let mut store = Store::new(&self.engine, EngineData::new());
        let (module_ref, instance) = api::module::Module::instantiate(&mut store, &module, &mut self.linker, |data| &mut data.module)?;

        Ok(ModuleRef {
            _ref: module_ref,
            instance,
            store,
        })
    }
}

pub struct ModuleRef {
    _ref: api::module::Module<EngineData>,
    instance: Instance,
    store: Store<EngineData>,
}

impl std::fmt::Debug for ModuleRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleRef").finish()
    }
}

impl ModuleRef {
    pub fn describe(&mut self) -> anyhow::Result<api::module::Description> {
        let description = self._ref.describe(&mut self.store)?;

        Ok(description)
    }
}
