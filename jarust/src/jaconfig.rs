pub const BUFFER_SIZE: usize = 32;

#[derive(Debug)]
pub struct JaConfig {
    pub(crate) uri: String,
    pub(crate) apisecret: Option<String>,
    pub(crate) namespace: String,
    pub(crate) cap: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportType {
    Ws,
}

impl JaConfig {
    pub fn builder() -> JaConfigBuilder<NoUrlTypeState, NoCapTypeState> {
        JaConfigBuilder {
            url: NoUrlTypeState,
            apisecret: None,
            namespace: None,
            cap: NoCapTypeState,
        }
    }
}

pub struct NoUrlTypeState;
pub struct WithUrlTypeState(pub(crate) String);

pub struct NoCapTypeState;
pub struct WithCapTypeState(pub(crate) usize);

pub struct JaConfigBuilder<U, C> {
    pub(crate) url: U,
    pub(crate) apisecret: Option<String>,
    pub(crate) namespace: Option<String>,
    pub(crate) cap: C,
}

impl<C> JaConfigBuilder<NoUrlTypeState, C> {
    pub fn url(self, url: &str) -> JaConfigBuilder<WithUrlTypeState, C> {
        let Self {
            apisecret,
            namespace,
            cap,
            ..
        } = self;
        JaConfigBuilder {
            apisecret,
            namespace,
            cap,
            url: WithUrlTypeState(url.into()),
        }
    }
}

impl<U> JaConfigBuilder<U, NoCapTypeState> {
    pub fn cap(self, cap: usize) -> JaConfigBuilder<U, WithCapTypeState> {
        let Self {
            apisecret,
            namespace,
            url,
            ..
        } = self;
        JaConfigBuilder {
            apisecret,
            namespace,
            url,
            cap: WithCapTypeState(cap),
        }
    }
}

impl<U, C> JaConfigBuilder<U, C> {
    pub fn apisecret(self, apisecret: &str) -> Self {
        let Self { namespace, url, cap, .. } = self;
        JaConfigBuilder {
            apisecret: Some(apisecret.into()),
            namespace,
            url,
            cap,
        }
    }

    pub fn namespace(self, namespace: &str) -> Self {
        let Self { apisecret, url, cap, .. } = self;
        JaConfigBuilder {
            namespace: Some(namespace.into()),
            apisecret,
            url,
            cap,
        }
    }
}

impl JaConfigBuilder<WithUrlTypeState, WithCapTypeState> {
    pub fn build(self) -> JaConfig {
        let Self {
            namespace,
            apisecret,
            url,
            cap,
        } = self;
        let namespace = namespace.unwrap_or(String::from("jarust"));
        JaConfig {
            namespace,
            apisecret,
            uri: url.0,
            cap: cap.0,
        }
    }
}
