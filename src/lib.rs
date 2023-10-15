pub mod error;

use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{self, Poll},
};

use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    lookup_ip::{LookupIp, LookupIpIntoIter},
    TokioAsyncResolver,
};
use hyper::{client::connect::dns::Name, service::Service};

pub struct SocketAddrs(LookupIpIntoIter);

impl Iterator for SocketAddrs {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|ip| SocketAddr::new(ip, 0))
    }
}

impl From<LookupIp> for SocketAddrs {
    fn from(lookup_ip: LookupIp) -> Self {
        Self(lookup_ip.into_iter())
    }
}

#[derive(Clone)]
pub struct HickoryDnsResolver {
    inner: Arc<TokioAsyncResolver>,
}

impl HickoryDnsResolver {
    pub fn with_config_and_options(config: ResolverConfig, opts: ResolverOpts) -> Self {
        let resolver = TokioAsyncResolver::tokio(config, opts);

        Self {
            inner: Arc::new(resolver),
        }
    }

    #[cfg(feature = "system-config")]
    pub fn with_system_config() -> Result<Self, error::Error> {
        let resolver = TokioAsyncResolver::tokio_from_system_conf()?;

        Ok(Self {
            inner: Arc::new(resolver),
        })
    }
}

impl<R> From<R> for HickoryDnsResolver
where
    R: Into<Arc<TokioAsyncResolver>>,
{
    fn from(resolver: R) -> Self {
        Self {
            inner: resolver.into(),
        }
    }
}

impl Service<Name> for HickoryDnsResolver {
    type Response = SocketAddrs;
    type Error = error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, name: Name) -> Self::Future {
        let resolver = self.inner.clone();

        Box::pin(async move {
            let addrs = resolver.lookup_ip(name.as_str()).await?;

            Ok(addrs.into())
        })
    }
}
