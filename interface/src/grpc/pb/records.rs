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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRecordTagRequest {
    #[prost(int32, tag = "1")]
    pub record_id: i32,
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRecordTagResponse {
    #[prost(string, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveRecordTagRequest {
    #[prost(int32, tag = "1")]
    pub record_id: i32,
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveRecordTagResponse {
    #[prost(string, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    #[derive(Debug, Clone)]
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
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RecordsServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            RecordsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
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
        pub async fn add_record_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::AddRecordTagRequest>,
        ) -> Result<tonic::Response<super::AddRecordTagResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/records.RecordsService/AddRecordTag");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_record_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveRecordTagRequest>,
        ) -> Result<tonic::Response<super::RemoveRecordTagResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/records.RecordsService/RemoveRecordTag");
            self.inner.unary(request.into_request(), path, codec).await
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
        async fn add_record_tag(
            &self,
            request: tonic::Request<super::AddRecordTagRequest>,
        ) -> Result<tonic::Response<super::AddRecordTagResponse>, tonic::Status>;
        async fn remove_record_tag(
            &self,
            request: tonic::Request<super::RemoveRecordTagRequest>,
        ) -> Result<tonic::Response<super::RemoveRecordTagResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RecordsServiceServer<T: RecordsService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RecordsService> RecordsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> Service<http::Request<B>> for RecordsServiceServer<T>
    where
        T: RecordsService,
        B: Body + Send + Sync + 'static,
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
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRecordsListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRecordsPreviewSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkRecordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/records.RecordsService/AddRecordTag" => {
                    #[allow(non_camel_case_types)]
                    struct AddRecordTagSvc<T: RecordsService>(pub Arc<T>);
                    impl<T: RecordsService> tonic::server::UnaryService<super::AddRecordTagRequest>
                        for AddRecordTagSvc<T>
                    {
                        type Response = super::AddRecordTagResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddRecordTagRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_record_tag(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddRecordTagSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/records.RecordsService/RemoveRecordTag" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveRecordTagSvc<T: RecordsService>(pub Arc<T>);
                    impl<T: RecordsService>
                        tonic::server::UnaryService<super::RemoveRecordTagRequest>
                        for RemoveRecordTagSvc<T>
                    {
                        type Response = super::RemoveRecordTagResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveRecordTagRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_record_tag(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveRecordTagSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RecordsService> Clone for RecordsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RecordsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
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
