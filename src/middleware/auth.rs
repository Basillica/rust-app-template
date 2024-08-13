use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, error::ErrorUnauthorized, Either, Error};
use futures_util::future::LocalBoxFuture;

use crate::utils::jwt::jwt;



pub struct TokenAuth;

impl<S, B> Transform<S, ServiceRequest> for TokenAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TokenAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TokenAuthMiddleware { service }))
    }
}


pub struct TokenAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TokenAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // forward_ready!(service);

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization");

        if let Some(auth_header) = auth_header {
            let auth_token = auth_header.to_str();
            match auth_token {
                Ok(token) => {
                    if !jwt::decode(token) {
                        return Box::pin(async move {
                            Err(ErrorUnauthorized("authorization header is invalid".to_string()))
                        })
                    }
                },
                Err(e) => {
                    return Box::pin(async move {
                        Err(ErrorUnauthorized(e.to_string()))
                    })
                }
            }
        } else {
            return Box::pin(async move {
                Err(ErrorUnauthorized("authorization header was not present in requst".to_string()))
            })
        }

        
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }

    
}