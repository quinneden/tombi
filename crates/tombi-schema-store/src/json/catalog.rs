use super::JsonCatalogSchema;

pub const DEFAULT_CATALOG_URL: &str = "https://schemastore.org/api/json/catalog.json";

#[derive(Debug, Clone, serde::Deserialize)]
pub struct JsonCatalog {
    pub schemas: Vec<JsonCatalogSchema>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatalogUrl(url::Url);

impl CatalogUrl {
    #[inline]
    pub fn new(url: url::Url) -> Self {
        Self(url)
    }
}

impl std::ops::Deref for CatalogUrl {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for CatalogUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CatalogUrl> for url::Url {
    fn from(catalog_url: CatalogUrl) -> Self {
        catalog_url.0
    }
}
