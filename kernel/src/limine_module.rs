use limine::{LimineFile, LimineModuleRequest};

static LIMINE_MODULE_REQUEST: LimineModuleRequest = LimineModuleRequest::new(0);

pub fn get_module(module: &str) -> Option<&LimineFile> {
    let modules = LIMINE_MODULE_REQUEST
        .get_response()
        .get()
        .unwrap()
        .modules();

    for m in modules {
        if m.path.to_str().unwrap().to_str().unwrap() == module {
            return Some(m);
        }
    }

    None
}
