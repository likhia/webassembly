
use log::trace;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::str;
//use proxy_wasm::hostcalls;
use std::time::Duration;

use serde_json::Value;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HeaderAppendRootContext{
            namespace : "".to_string(),  whitelist: "".to_string()
        })
    });
}

struct ProxyFilter{
    namespace: String,
    whitelist: String, 
    context_id: u32,
}

impl Context for ProxyFilter {


    fn dispatch_http_call(&self, upstream: &str, headers: Vec<(&str, &str)>, body: Option<&[u8]>,

                          trailers: Vec<(&str, &str)>, timeout: Duration,) -> Result<u32, Status> {
        

         self.send_http_response(403,
            	                 vec![("Powered-By", "proxy-wasm")],
                                 Some(b"Access forbidden.\n"),);
     
         
         self.dispatch_http_call(upstream, headers, body, trailers, timeout)

     }
         
}        

impl HttpContext for ProxyFilter {

    fn on_http_request_headers(&mut self, _: usize) -> Action {
       let mut path = self.get_http_request_header(":path").unwrap();
       let mut external = "".to_string();


       let v: Vec<&str> = self.whitelist.split(',').collect();

       for (name, value) in &self.get_http_request_headers() {
        
            trace!("#{} -> {}: {}", self.context_id, name, value);
            if name == "external" {
	       path = value.to_string();
               external = value.to_string();
            }
        }
        self.add_http_request_header("header-param", path.as_str());

        if v.contains(&external.as_str()) {
           self.send_http_response(403,
                                   vec![("Powered-By", "proxy-wasm")],
                                   Some(b"Access forbidden.\n"),);
        }

        //if path.as_str() == "/country/name/sg" {
        //    self.send_http_response(403,
        //                         vec![("Powered-By", "proxy-wasm")],
        //                         Some(b"Access forbidden.\n"),);

        //}

        Action::Continue
    }

    //fn on_http_response_body(&mut self, _body_size: usize, end_of_stream: bool) -> Action {
        
       // let header  = self.get_http_request_header(":path").unwrap();

        // self.add_http_response_header("custom-header", header.as_str());
    
        // Action::Continue
    //}
 

    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        self.add_http_response_header("namespace", self.namespace.as_str());
        self.add_http_response_header("whitelist", self.whitelist.as_str());

        Action::Continue
    }


}


struct HeaderAppendRootContext {
    namespace: String,
    whitelist: String,
}

impl Context for HeaderAppendRootContext {}

impl RootContext for HeaderAppendRootContext {
    
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        true
    }

    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        if let Some(config_bytes) = self.get_configuration() {
            //self.header_content = str::from_utf8(config_bytes.as_ref()).unwrap().to_owned();

            let cfg: Value = serde_json::from_slice(config_bytes.as_ref()).unwrap();
            self.namespace = cfg.get("namespace").unwrap().as_str().unwrap().to_string();
            self.whitelist = cfg.get("whitelist").unwrap().as_str().unwrap().to_string();
        }
        true
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(ProxyFilter{
            namespace: self.namespace.clone(), whitelist: self.whitelist.clone(),
            context_id,                
        }))
    
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

}
