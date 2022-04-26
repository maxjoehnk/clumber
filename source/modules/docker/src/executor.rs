wit_bindgen_rust::export!("../../module_executor.wit");

struct Executor;

impl module_executor::Executor for Executor {
    fn new() -> wit_bindgen_rust::Handle<Self> {
        todo!()
    }
    fn os(&self) -> module_executor::Os {
        module_executor::Os::Linux
    }
    fn spawn(&self, command: String) {
        todo!()
    }
}

struct ModuleExecutor;

impl module_executor::ModuleExecutor for ModuleExecutor {
}
