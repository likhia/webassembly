use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::str;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HeaderAppendRootContext{
            header_content: "".to_string(),
        })
    });
}

struct HeaderAppendFilter{
    header_content: String
}

impl Context for HeaderAppendFilter {}

impl HttpContext for HeaderAppendFilter {

    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        self.add_http_response_header("custom-header", self.header_content.as_str());

        Action::Continue
    }
}

struct HeaderAppendRootContext {
    header_content: String
}

impl Context for HeaderAppendRootContext {}

impl RootContext for HeaderAppendRootContext {
    
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        true
    }

    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        if let Some(config_bytes) = self.get_configuration() {
            self.header_content = str::from_utf8(config_bytes.as_ref()).unwrap().to_owned()
        }
        true
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HeaderAppendFilter{
            header_content: self.header_content.clone(),
        }))
    
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

}
