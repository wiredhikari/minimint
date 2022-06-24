use std::convert::Infallible;
use std::sync::Arc;
use rocksdb::Error;
use sled::transaction::TransactionError;
use super::batch::{BatchItem, DbBatch};
use super::{Database, DatabaseError, DecodingError};
use crate::db::PrefixIter;
use tracing::{error, trace};

impl Database for rocksdb::OptimisticTransactionDB {
    fn raw_insert_entry(
        &self,
        key: &[u8],
        value: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, DatabaseError> {
        let val = self.get(key).unwrap();
        self.put(key, value)
            .map_err(|e| DatabaseError::DbError(Box::new(e)))
            .map(|res| val)
    }

    fn raw_get_value(&self, key: &[u8]) -> Result<Option<Vec<u8>>, DatabaseError> {
        self
            .get(key)
            .map_err(|e| DatabaseError::DbError(Box::new(e)))
    }

    fn raw_remove_entry(&self, key: &[u8]) -> Result<Option<Vec<u8>>, DatabaseError> {
        let val = self.get(key).unwrap();
        self.delete(key)
            .map_err(|e| DatabaseError::DbError(Box::new(e)))
            .map(|res| val)
    }

    //Error: 'self' has an anonymous lifeitme `'_` but it needs to satisfy a `'static` lifetime requirement
    fn raw_find_by_prefix(&self, key_prefix: &[u8]) -> PrefixIter {
        Box::new(self.prefix_iterator(key_prefix)
            .map_while(|res| res.0.starts_with(key_prefix).then(|| res))
            .map(|(key_bytes, value_bytes)| (key_bytes.to_vec(), value_bytes.to_vec()))
            .map(Ok::<(Vec<u8>, Vec<u8>), DatabaseError>)
        )
    }

    fn raw_apply_batch(&self, batch: DbBatch) -> Result<(), DatabaseError> {
        let batch: Vec<_> = batch.into();
        let mut tx = self.transaction();

        for change in batch.iter() {
            match change {
                BatchItem::InsertNewElement(element) => {
                    if tx.get(element.key.to_bytes()).unwrap().is_some() {
                        tx.put(element.key.to_bytes(), element.value.to_bytes());
                        error!("Database replaced element! This should not happen!");
                        trace!("Problematic key: {:?}", element.key);
                    } else {
                        tx.put(element.key.to_bytes(), element.value.to_bytes());
                    }
                }
                BatchItem::InsertElement(element) => {
                    tx.put(element.key.to_bytes(), element.value.to_bytes())?;
                }
                BatchItem::DeleteElement(key) => {
                    if tx.get(key.to_bytes()).unwrap().is_none(){
                        tx.delete(key.to_bytes());
                        error!("Database deleted absent element! This should not happen!");
                        trace!("Problematic key: {:?}", key);
                    } else {
                        tx.delete(key.to_bytes());
                    }
                }
                BatchItem::MaybeDeleteElement(key) => {
                    tx.delete(key.to_bytes())?;
                }
            }
            Ok(())
        }
        tx.commit().map_err(|e| DatabaseError::DbError(Box::new(e)));
        return Ok(())
    }
}

impl From<rocksdb::Error> for DatabaseError {
    fn from(e: rocksdb::Error) -> Self {
        DatabaseError::DbError(Box::new(e))
    }
}

#[cfg(test)]
mod tests {
    #[test_log::test]
    fn test_basic_rw() {
        use std::sync::Arc;
        use rocksdb::{DBAccess, OptimisticTransactionDB, Options, SingleThreaded, Transaction, WriteBatch, WriteBatchWithTransaction};
        use crate::db::Database;

        let path = tempdir::TempDir::new("fcb-rocksdb-test").unwrap();
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let mut db: OptimisticTransactionDB<SingleThreaded> =
            OptimisticTransactionDB::open_default(path).unwrap();


    }
}
