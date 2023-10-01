use std::any::Any;
use std::sync::Arc;
use dyn_clone::DynClone;
use moka::sync::Cache as MokaCache;

/// Trait which defines the behaviour of types that's gonna be stored in Cache
pub trait Cacheable: Send + Sync + 'static + DynClone {
    fn as_any(&self) -> &dyn Any;
}


impl<T> Cacheable for T
    where T: Any + Clone + Send + Sync,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

dyn_clone::clone_trait_object!(Cacheable);

pub struct Cache {
    inner: MokaCache<String, Arc<dyn Cacheable>>,
}

impl std::ops::Deref for Cache {
    type Target = MokaCache<String, Arc<dyn Cacheable>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Cache {
    /// With given `time_to_live` and `time_to_idle` creates a moka cache.
    ///
    /// `time_to_live`: Time in seconds before an object is stored in a caching system before it’s deleted
    /// `time_to_idle`: Time in seconds before a `get` or `insert` operation an object is stored in a caching system before it's deleted
    /// `max_capacity`: Max size in MB's that the cache can hold
    pub fn new(time_to_live: u64, time_to_idle: u64, max_capacity: Option<u64>) -> Self {
        let mut cache_builder = MokaCache::builder()
            .time_to_live(std::time::Duration::from_secs(time_to_live))
            .time_to_idle(std::time::Duration::from_secs(time_to_idle));

        if let Some(capacity) = max_capacity {
            cache_builder = cache_builder.max_capacity(capacity * 1024 * 1024);
        }

        Self {
            inner: cache_builder.build(),
        }
    }

    pub async fn push<T: Cacheable>(&self, key: String, val: Arc<T>) {
        self.inner.insert(key, val);
    }

    pub fn get_val<T: Clone + Cacheable>(&self, key: &str) -> Option<T> {
        // self.inner.get(key)
        //     .map(|metadata| metadata.clone().downcast::<T>().unwrap())
        panic!()
    }
}


type ArcAny = Arc<dyn Any + Send + Sync>;

/// Cache for various metadata about files.
///
/// The cache is keyed by the file path and the type of metadata.
#[derive(Clone)]
pub struct FileMetadataCache {
    cache: Arc<MokaCache<String, ArcAny>>,
}

impl FileMetadataCache {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(MokaCache::new(capacity as u64)),
        }
    }

    pub(crate) fn get<T: Send + Sync + 'static>(&self, key: String) -> Option<Arc<T>> {
        self.cache
            .get(&key)
            .map(|metadata| metadata.clone().downcast::<T>().unwrap())
    }

    pub(crate) fn insert<T: Send + Sync + 'static>(&self, key: String, metadata: Arc<T>) {
        self.cache.insert(key, metadata);
    }
}


#[cfg(test)]
#[cfg(test)]
mod cache_tests {
    use super::*;

    #[derive(Debug)]
    pub struct AdCampaign {
        id: u64,
        name: String,
        price: u64,
    }

    #[derive(Debug)]
    pub struct AppInfo {
        id: u64,
        name: String,
    }


    #[test]
    fn construct_and_get_cache() {
        let cache = Cache::new(1800, 1800, None);
        cache.push("key".to_string(), Arc::new("val".to_string()));
        print!("{:?}", cache.get_val::<String>("key"))
    }

    #[test]
    fn file_meta_cache() {
        let cache = FileMetadataCache::new(1800);
        cache.insert("key".to_string(), Arc::new("val".to_string()));
        cache.insert("i1".to_string(), Arc::new(1u64));
        println!("{:?}", cache.get::<String>("key".to_string()));
        println!("{:?}", cache.get::<u64>("i1".to_string()));

        cache.insert("app1".to_string(), Arc::new(AppInfo {
            id: 1,
            name: "app1".to_string(),
        }));

        cache.insert("cmp1".to_string(), Arc::new(AdCampaign {
            id: 1,
            name: "AdCampaign1".to_string(),
            price: 0,
        }));


        println!("{:?}", cache.get::<AppInfo>("app1".to_string()));
        println!("{:?}", cache.get::<AdCampaign>("cmp1".to_string()));
    }
}