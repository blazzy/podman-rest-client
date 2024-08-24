use quote::quote;

use crate::{error::Error, spec::Spec};

pub fn config(spec: &Spec) -> Result<String, Error> {
    let base_path = &spec.base_path;

    let code = quote! {
        use std::future::Future;
        use std::pin::Pin;

        use http::header;
        use http::request::Builder;
        use http::request::Request;
        use http::Response;
        use hyper::body::Incoming;
        use hyper_util::client::legacy::connect::Connect;
        use hyper_util::client::legacy::connect::HttpConnector;
        use hyper_util::client::legacy::Client;
        use hyper_util::client::legacy::Error as HyperError;
        use hyper_util::rt::TokioExecutor;

        pub trait Connector: Connect + Clone + Send + Sync + 'static {}
        impl Connector for HttpConnector {}

        pub trait HasConfig {
            fn get_config(&self) -> &dyn ClientConfig;
        }

        pub trait ClientConfig: Send + Sync {
            fn get_base_path(&self) -> &String;
            fn get_user_agent(&self) -> &Option<String>;
            fn request<'a>(
                &self,
                request: Request<String>,
            ) -> Pin<Box<dyn Future<Output = Result<Response<Incoming>, HyperError>> + 'a + Send>>;

            fn req_builder(&self) -> Builder {
                let mut req_builder = Request::builder();
                if let Some(user_agent) = self.get_user_agent() {
                    req_builder = req_builder.header(header::USER_AGENT, user_agent);
                }
                req_builder
            }
        }

        pub struct Config<C: Connector = HttpConnector> {
            pub base_path: String,
            pub user_agent: Option<String>,
            pub client: Client<C, String>,
        }

        impl Config<HttpConnector> {
            pub fn new() -> Config<HttpConnector> {
                Config::default()
            }
        }

        impl Default for Config<HttpConnector> {
            fn default() -> Self {
                let client = Client::builder(TokioExecutor::new()).build_http();
                Config::with_client(client)
            }
        }

        impl<C: Connector> Config<C> {
            pub fn with_client(client: Client<C, String>) -> Config<C> {
                Config {
                    base_path: #base_path.to_owned(),
                    user_agent: Some("openapi-client-gen".to_owned()),
                    client,
                }
            }
        }

        impl<C: Connector> ClientConfig for Config<C> {
            fn get_base_path(&self) -> &String {
                &self.base_path
            }

            fn get_user_agent(&self) -> &Option<String> {
                &self.user_agent
            }

            fn request<'a>(
                &self,
                request: Request<String>,
            ) -> Pin<Box<dyn Future<Output = Result<Response<Incoming>, HyperError>> + 'a + Send>> {
                Box::pin(self.client.request(request))
            }
        }
    };

    let syn_file: syn::File = syn::parse2(code)?;
    Ok(prettyplease::unparse(&syn_file))
}
