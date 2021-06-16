#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSourcesListRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceWithMeta {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub origin: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub image: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub last_scrape_time: i64,
    #[prost(string, tag = "7")]
    pub external_link: ::prost::alloc::string::String,
    #[prost(int32, tag = "8")]
    pub folder_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSourcesListResponse {
    #[prost(message, repeated, tag = "1")]
    pub sources: ::prost::alloc::vec::Vec<SourceWithMeta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSourcesRequest {
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub origin: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub image: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub last_scrape_time: i64,
    #[prost(string, tag = "7")]
    pub external_link: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSourcesResponse {
    #[prost(message, repeated, tag = "1")]
    pub sources: ::prost::alloc::vec::Vec<Source>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {
    #[prost(int32, tag = "1")]
    pub source_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeRequest {
    #[prost(int32, tag = "1")]
    pub source_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeResponse {}
#[doc = r" Generated client implementations."]
pub mod sources_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct SourcesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SourcesServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SourcesServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn get_sources_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSourcesListRequest>,
        ) -> Result<tonic::Response<super::GetSourcesListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/sources.SourcesService/GetSourcesList");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn search_sources(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchSourcesRequest>,
        ) -> Result<tonic::Response<super::SearchSourcesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/sources.SourcesService/SearchSources");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRequest>,
        ) -> Result<tonic::Response<super::SubscribeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sources.SourcesService/Subscribe");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn unsubscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::UnsubscribeRequest>,
        ) -> Result<tonic::Response<super::UnsubscribeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sources.SourcesService/Unsubscribe");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SourcesServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SourcesServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SourcesServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod sources_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SourcesServiceServer."]
    #[async_trait]
    pub trait SourcesService: Send + Sync + 'static {
        async fn get_sources_list(
            &self,
            request: tonic::Request<super::GetSourcesListRequest>,
        ) -> Result<tonic::Response<super::GetSourcesListResponse>, tonic::Status>;
        async fn search_sources(
            &self,
            request: tonic::Request<super::SearchSourcesRequest>,
        ) -> Result<tonic::Response<super::SearchSourcesResponse>, tonic::Status>;
        async fn subscribe(
            &self,
            request: tonic::Request<super::SubscribeRequest>,
        ) -> Result<tonic::Response<super::SubscribeResponse>, tonic::Status>;
        async fn unsubscribe(
            &self,
            request: tonic::Request<super::UnsubscribeRequest>,
        ) -> Result<tonic::Response<super::UnsubscribeResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SourcesServiceServer<T: SourcesService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: SourcesService> SourcesServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for SourcesServiceServer<T>
    where
        T: SourcesService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/sources.SourcesService/GetSourcesList" => {
                    #[allow(non_camel_case_types)]
                    struct GetSourcesListSvc<T: SourcesService>(pub Arc<T>);
                    impl<T: SourcesService>
                        tonic::server::UnaryService<super::GetSourcesListRequest>
                        for GetSourcesListSvc<T>
                    {
                        type Response = super::GetSourcesListResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSourcesListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_sources_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSourcesListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sources.SourcesService/SearchSources" => {
                    #[allow(non_camel_case_types)]
                    struct SearchSourcesSvc<T: SourcesService>(pub Arc<T>);
                    impl<T: SourcesService> tonic::server::UnaryService<super::SearchSourcesRequest>
                        for SearchSourcesSvc<T>
                    {
                        type Response = super::SearchSourcesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchSourcesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search_sources(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SearchSourcesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sources.SourcesService/Subscribe" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSvc<T: SourcesService>(pub Arc<T>);
                    impl<T: SourcesService> tonic::server::UnaryService<super::SubscribeRequest> for SubscribeSvc<T> {
                        type Response = super::SubscribeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SubscribeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sources.SourcesService/Unsubscribe" => {
                    #[allow(non_camel_case_types)]
                    struct UnsubscribeSvc<T: SourcesService>(pub Arc<T>);
                    impl<T: SourcesService> tonic::server::UnaryService<super::UnsubscribeRequest>
                        for UnsubscribeSvc<T>
                    {
                        type Response = super::UnsubscribeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnsubscribeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).unsubscribe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UnsubscribeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: SourcesService> Clone for SourcesServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: SourcesService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SourcesService> tonic::transport::NamedService for SourcesServiceServer<T> {
        const NAME: &'static str = "sources.SourcesService";
    }
}
