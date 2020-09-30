#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, optional, tag="1")]
    pub init_quote_req: ::std::option::Option<request::InitQuoteRequest>,
}
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitQuoteRequest {
        #[prost(uint32, optional, tag="9")]
        pub timeout: ::std::option::Option<u32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(message, optional, tag="1")]
    pub init_quote_res: ::std::option::Option<response::InitQuoteResponse>,
}
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitQuoteResponse {
        #[prost(uint32, required, tag="1", default="1")]
        pub error_code: u32,
        #[prost(bytes, optional, tag="2")]
        pub target_info: ::std::option::Option<std::vec::Vec<u8>>,
        #[prost(bytes, optional, tag="3")]
        pub gid: ::std::option::Option<std::vec::Vec<u8>>,
    }
}
