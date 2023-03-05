use crate::types::*;
use redis::aio::ConnectionLike;
use redis::{cmd, FromRedisValue, RedisFuture, ToRedisArgs};

/// Provides a high level synchronous API to work with redis time series data types. Uses some abstractions
/// for easier handling of time series related redis command arguments. All commands are directly
/// available on ConnectionLike types from the redis crate.
/// ```rust,no_run
/// # async fn run() -> redis::RedisResult<()> {
/// use redis::AsyncCommands;
/// use redis_ts::{AsyncTsCommands, TsOptions};
///
/// let client = redis::Client::open("redis://127.0.0.1/")?;
/// let mut con = client.get_async_connection().await?;
///
/// let _:() = con.ts_create("my_ts", TsOptions::default()).await?;
/// let ts:u64 = con.ts_add_now("my_ts", 2.0).await?;
/// let v:Option<(u64,f64)> = con.ts_get("my_ts").await?;
/// # Ok(()) }
/// ```
///
pub trait AsyncTsCommands: ConnectionLike + Send + Sized {
    /// Returns information about a redis time series key.
    fn ts_info<'a, K: ToRedisArgs + Send + Sync + 'a>(&'a mut self, key: K) -> RedisFuture<TsInfo> {
        Box::pin(async move { cmd("TS.INFO").arg(key).query_async(self).await })
    }

    /// Creates a new redis time series key.
    fn ts_create<'a, K: ToRedisArgs + Send + Sync + 'a, RV: FromRedisValue>(
        &'a mut self,
        key: K,
        options: TsOptions,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.CREATE")
                .arg(key)
                .arg(options)
                .query_async(self)
                .await
        })
    }

    /// Modifies an existing redis time series configuration.
    fn ts_alter<'a, K: ToRedisArgs + Send + Sync + 'a, RV: FromRedisValue>(
        &'a mut self,
        key: K,
        options: TsOptions,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.ALTER")
                .arg(key)
                .arg(options.uncompressed(false))
                .query_async(self)
                .await
        })
    }

    /// Adds a single time series value with a timestamp to an existing redis time series.
    fn ts_add<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        TS: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        ts: TS,
        value: V,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.ADD")
                .arg(key)
                .arg(ts)
                .arg(value)
                .query_async(self)
                .await
        })
    }

    /// Adds a single time series value to an existing redis time series with redis system
    /// time as timestamp.
    fn ts_add_now<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        value: V,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.ADD")
                .arg(key)
                .arg("*")
                .arg(value)
                .query_async(self)
                .await
        })
    }

    /// Adds a single time series value to a redis time series. If the time series does not
    /// yet exist it will be created with given settings.
    fn ts_add_create<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        TS: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        ts: TS,
        value: V,
        options: TsOptions,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.ADD")
                .arg(key)
                .arg(ts)
                .arg(value)
                .arg(options)
                .query_async(self)
                .await
        })
    }

    /// Adds multiple time series values to an existing redis time series.
    fn ts_madd<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        TS: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        values: &'a [(K, TS, V)],
    ) -> RedisFuture<RV> {
        Box::pin(async move { cmd("TS.MADD").arg(values).query_async(self).await })
    }

    /// Increments a time series value with redis system time.
    fn ts_incrby_now<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        value: V,
    ) -> RedisFuture<RV> {
        Box::pin(async move { cmd("TS.INCRBY").arg(key).arg(value).query_async(self).await })
    }

    /// Increments a time series value with given timestamp.
    fn ts_incrby<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
        TS: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        ts: TS,
        value: V,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.INCRBY")
                .arg(key)
                .arg(value)
                .arg("TIMESTAMP")
                .arg(ts)
                .query_async(self)
                .await
        })
    }

    /// Increments a time series value with timestamp. Time series will be created if it
    /// not already exists.
    fn ts_incrby_create<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
        TS: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        ts: TS,
        value: V,
        options: TsOptions,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.INCRBY")
                .arg(key)
                .arg(value)
                .arg("TIMESTAMP")
                .arg(ts)
                .arg(options)
                .query_async(self)
                .await
        })
    }

    /// Decrements a time series value with redis system time.
    fn ts_decrby_now<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        value: V,
    ) -> RedisFuture<RV> {
        Box::pin(async move { cmd("TS.DECRBY").arg(key).arg(value).query_async(self).await })
    }

    /// Decrements a time series value with given timestamp.
    fn ts_decrby<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
        TS: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        ts: TS,
        value: V,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.DECRBY")
                .arg(key)
                .arg(value)
                .arg("TIMESTAMP")
                .arg(ts)
                .query_async(self)
                .await
        })
    }

    /// Decrements a time series value with timestamp. Time series will be created if it
    /// not already exists.
    fn ts_decrby_create<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
        TS: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        ts: TS,
        value: V,
        options: TsOptions,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.DECRBY")
                .arg(key)
                .arg(value)
                .arg("TIMESTAMP")
                .arg(ts)
                .arg(options)
                .query_async(self)
                .await
        })
    }

    /// Creates a new redis time series compaction rule.
    fn ts_createrule<'a, K: ToRedisArgs + Send + Sync + 'a, RV: FromRedisValue>(
        &'a mut self,
        source_key: K,
        dest_key: K,
        aggregation_type: TsAggregationType,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.CREATERULE")
                .arg(source_key)
                .arg(dest_key)
                .arg(aggregation_type)
                .query_async(self)
                .await
        })
    }

    /// Deletes an existing redis time series compaction rule.
    fn ts_deleterule<'a, K: ToRedisArgs + Send + Sync + 'a, RV: FromRedisValue>(
        &'a mut self,
        source_key: K,
        dest_key: K,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("TS.DELETERULE")
                .arg(source_key)
                .arg(dest_key)
                .query_async(self)
                .await
        })
    }

    /// Returns the latest (current) value in a redis time series.
    fn ts_get<'a, K: ToRedisArgs + Send + Sync + 'a, TS: FromRedisValue, V: FromRedisValue>(
        &'a mut self,
        key: K,
    ) -> RedisFuture<Option<(TS, V)>> {
        Box::pin(async move { cmd("TS.GET").arg(key).query_async(self).await.or(Ok(None)) })
    }

    /// Returns the latest (current) value from multiple redis time series.
    fn ts_mget<
        'a,
        TS: Default + FromRedisValue + 'a,
        V: Default + FromRedisValue + 'a,
    >(
        &mut self,
        filter_options: TsFilterOptions,
    ) -> RedisFuture<TsMget<TS, V>> {
        Box::pin(async move { cmd("TS.MGET").arg(filter_options).query_async(self).await })
    }

    #[doc(hidden)]
    fn range<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        TS: Default + FromRedisValue + Copy,
        V: Default + FromRedisValue + Copy,
    >(
        &'a mut self,
        command: &str,
        key: K,
        query: TsRangeQuery,
    ) -> RedisFuture<TsRange<TS, V>> {
        let mut c = cmd(command);
        c.arg(key).arg(query);
        Box::pin(async move { c.query_async(self).await })
    }

    /// Executes a redis time series range query.
    fn ts_range<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        TS: Default + FromRedisValue + Copy,
        V: Default + FromRedisValue + Copy,
    >(
        &'a mut self,
        key: K,
        query: TsRangeQuery,
    ) -> RedisFuture<TsRange<TS, V>> {
        self.range("TS.RANGE", key, query)
    }

    /// Executes a redis time series revrange query.
    fn ts_revrange<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        TS: Default + FromRedisValue + Copy,
        V: Default + FromRedisValue + Copy,
    >(
        &'a mut self,
        key: K,
        query: TsRangeQuery,
    ) -> RedisFuture<TsRange<TS, V>> {
        self.range("TS.REVRANGE", key, query)
    }

    #[doc(hidden)]
    fn mrange<
        'a,
        TS: Default + FromRedisValue + Copy,
        V: Default + FromRedisValue + Copy,
    >(
        &mut self,
        command: &str,
        query: TsRangeQuery,
        filter_options: TsFilterOptions,
    ) -> RedisFuture<TsMrange<TS, V>> {
        let mut c = cmd(command);
        c.arg(query).arg(filter_options);

        Box::pin(async move { c.query_async(self).await })
    }

    /// Executes multiple redis time series range queries.
    fn ts_mrange<
        'a,
        TS: Default + FromRedisValue + Copy,
        V: Default + FromRedisValue + Copy,
    >(
        &mut self,
        query: TsRangeQuery,
        filter_options: TsFilterOptions,
    ) -> RedisFuture<TsMrange<TS, V>> {
        self.mrange("TS.MRANGE", query, filter_options)
    }

    /// Executes multiple redis time series revrange queries.
    fn ts_mrevrange<
        'a,
        TS: Default + FromRedisValue + Copy,
        V: Default + FromRedisValue + Copy,
    >(
        &mut self,
        query: TsRangeQuery,
        filter_options: TsFilterOptions,
    ) -> RedisFuture<TsMrange<TS, V>> {
        self.mrange("TS.MREVRANGE", query, filter_options)
    }

    /// Returns a filtered list of redis time series keys.
    fn ts_queryindex(&mut self, filter_options: TsFilterOptions) -> RedisFuture<Vec<String>> {
        Box::pin(async move {
            cmd("TS.QUERYINDEX")
                .arg(filter_options.get_filters())
                .query_async(self)
                .await
        })
    }
}

impl<T> AsyncTsCommands for T where T: Send + ConnectionLike {}
