#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub login: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub last_read_date: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(string, tag = "1")]
    pub login: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    #[prost(string, tag = "1")]
    pub login: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
#[doc = r" Generated client implementations."]
pub mod users_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct UsersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UsersServiceClient<tonic::transport::Channel> {
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
    impl<T> UsersServiceClient<T>
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
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::LoginRequest>,
        ) -> Result<tonic::Response<super::LoginResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/users.UsersService/Login");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn register(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterRequest>,
        ) -> Result<tonic::Response<super::RegisterResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/users.UsersService/Register");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for UsersServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for UsersServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UsersServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod users_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UsersServiceServer."]
    #[async_trait]
    pub trait UsersService: Send + Sync + 'static {
        async fn login(
            &self,
            request: tonic::Request<super::LoginRequest>,
        ) -> Result<tonic::Response<super::LoginResponse>, tonic::Status>;
        async fn register(
            &self,
            request: tonic::Request<super::RegisterRequest>,
        ) -> Result<tonic::Response<super::RegisterResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct UsersServiceServer<T: UsersService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: UsersService> UsersServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for UsersServiceServer<T>
    where
        T: UsersService,
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
                "/users.UsersService/Login" => {
                    #[allow(non_camel_case_types)]
                    struct LoginSvc<T: UsersService>(pub Arc<T>);
                    impl<T: UsersService> tonic::server::UnaryService<super::LoginRequest> for LoginSvc<T> {
                        type Response = super::LoginResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoginRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).login(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = LoginSvc(inner);
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
                "/users.UsersService/Register" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterSvc<T: UsersService>(pub Arc<T>);
                    impl<T: UsersService> tonic::server::UnaryService<super::RegisterRequest> for RegisterSvc<T> {
                        type Response = super::RegisterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).register(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RegisterSvc(inner);
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
    impl<T: UsersService> Clone for UsersServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: UsersService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UsersService> tonic::transport::NamedService for UsersServiceServer<T> {
        const NAME: &'static str = "users.UsersService";
    }
}
