wit_bindgen_rust::export!("../../module.wit");

impl module::Module for Module {
    fn describe() -> module::Description {
        module::Description {
            name: "Docker".into(),
            version: "0.1.0".into(),
            blocks: vec![
                module::Block {
                    identifier: "docker".into(),
                    description: "Run children in docker container".into(),
                    blocktype: module::Blocktype::Step,
                    args: vec![module::Blockarg {
                        name: "image".into(),
                        argtype: module::Argtype::Text,
                        required: true,
                    }],
                },
                module::Block {
                    identifier: "dockerImage".into(),
                    description: "Build and push a docker image".into(),
                    blocktype: module::Blocktype::Step,
                    args: vec![],
                }
            ]
        }
    }
}

struct Module;
