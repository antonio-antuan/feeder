#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Record {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub source_record_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub source_id: i32,
    #[prost(string, tag = "5")]
    pub content: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub date: i64,
    #[prost(string, tag = "7")]
    pub image: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordWithMeta {
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<Record>,
    #[prost(bool, tag = "2")]
    pub starred: bool,
    #[prost(string, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecordsListRequest {
    #[prost(int32, tag = "1")]
    pub source_id: i32,
    #[prost(int32, tag = "2")]
    pub record_id: i32,
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    #[prost(uint32, tag = "4")]
    pub offset: u32,
    #[prost(enumeration = "RecordsQuery", tag = "5")]
    pub query: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecordsListResponse {
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<RecordWithMeta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecordsPreviewRequest {
    #[prost(int32, tag = "1")]
    pub source_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecordsPreviewResponse {
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<Record>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkRecordRequest {
    #[prost(int32, tag = "1")]
    pub record_id: i32,
    #[prost(bool, tag = "2")]
    pub starred: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkRecordResponse {
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<RecordWithMeta>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RecordsQuery {
    Undefined = 0,
    All = 1,
    Starred = 2,
}
#[doc = r" Generated client implementations."]
pub mod records_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct RecordsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RecordsServiceClient<tonic::transport::Channel> {
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
    impl<T> RecordsServiceClient<T>
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
        pub async fn get_records_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecordsListRequest>,
        ) -> Result<tonic::Response<super::GetRecordsListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/records.RecordsService/GetRecordsList");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_records_preview(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecordsPreviewRequest>,
        ) -> Result<tonic::Response<super::GetRecordsPreviewResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/records.RecordsService/GetRecordsPreview");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn mark_record(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkRecordRequest>,
        ) -> Result<tonic::Response<super::MarkRecordResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/records.RecordsService/MarkRecord");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RecordsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RecordsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RecordsServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod records_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RecordsServiceServer."]
    #[async_trait]
    pub trait RecordsService: Send + Sync + 'static {
        async fn get_records_list(
            &self,
            request: tonic::Request<super::GetRecordsListRequest>,
        ) -> Result<tonic::Response<super::GetRecordsListResponse>, tonic::Status>;
        async fn get_records_preview(
            &self,
            request: tonic::Request<super::GetRecordsPreviewRequest>,
        ) -> Result<tonic::Response<super::GetRecordsPreviewResponse>, tonic::Status>;
        async fn mark_record(
            &self,
            request: tonic::Request<super::MarkRecordRequest>,
        ) -> Result<tonic::Response<super::MarkRecordResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RecordsServiceServer<T: RecordsService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: RecordsService> RecordsServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for RecordsServiceServer<T>
    where
        T: RecordsService,
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
                "/records.RecordsService/GetRecordsList" => {
                    #[allow(non_camel_case_types)]
                    struct GetRecordsListSvc<T: RecordsService>(pub Arc<T>);
                    impl<T: RecordsService>
                        tonic::server::UnaryService<super::GetRecordsListRequest>
                        for GetRecordsListSvc<T>
                    {
                        type Response = super::GetRecordsListResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRecordsListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_records_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetRecordsListSvc(inner);
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
                "/records.RecordsService/GetRecordsPreview" => {
                    #[allow(non_camel_case_types)]
                    struct GetRecordsPreviewSvc<T: RecordsService>(pub Arc<T>);
                    impl<T: RecordsService>
                        tonic::server::UnaryService<super::GetRecordsPreviewRequest>
                        for GetRecordsPreviewSvc<T>
                    {
                        type Response = super::GetRecordsPreviewResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRecordsPreviewRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_records_preview(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetRecordsPreviewSvc(inner);
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
                "/records.RecordsService/MarkRecord" => {
                    #[allow(non_camel_case_types)]
                    struct MarkRecordSvc<T: RecordsService>(pub Arc<T>);
                    impl<T: RecordsService> tonic::server::UnaryService<super::MarkRecordRequest> for MarkRecordSvc<T> {
                        type Response = super::MarkRecordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkRecordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mark_record(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MarkRecordSvc(inner);
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
    impl<T: RecordsService> Clone for RecordsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: RecordsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RecordsService> tonic::transport::NamedService for RecordsServiceServer<T> {
        const NAME: &'static str = "records.RecordsService";
    }
}
