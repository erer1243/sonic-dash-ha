use swss_common::{
    ConsumerStateTable, FieldValues, KeyOpFieldValues, ProducerStateTable, SubscriberStateTable, Table,
    ZmqConsumerStateTable, ZmqProducerStateTable,
};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, derive_more::From)]
pub(crate) enum InputTable {
    ConsumerStateTable(ConsumerStateTable),
    SubscriberStateTable(SubscriberStateTable),
    ZmqConsumerStateTable(ZmqConsumerStateTable),
}

impl InputTable {
    pub(crate) async fn read_data(&mut self) -> std::io::Result<()> {
        match self {
            InputTable::ConsumerStateTable(t) => t.read_data_async().await,
            InputTable::SubscriberStateTable(t) => t.read_data_async().await,
            InputTable::ZmqConsumerStateTable(t) => t.read_data_async().await,
        }
    }

    pub(crate) async fn pops(&mut self) -> swss_common::Result<Vec<KeyOpFieldValues>> {
        match self {
            InputTable::ConsumerStateTable(t) => t.pops_async().await,
            InputTable::SubscriberStateTable(t) => t.pops_async().await,
            InputTable::ZmqConsumerStateTable(t) => t.pops_async().await,
        }
    }
}

#[derive(Debug, derive_more::From)]
pub(crate) enum OutputTable {
    ProducerStateTable(ProducerStateTable),
    ZmqProducerStateTable(ZmqProducerStateTable),
    Table(Table),
}

impl OutputTable {
    pub(crate) async fn set(&mut self, key: &str, fvs: FieldValues) -> swss_common::Result<()> {
        match self {
            OutputTable::ProducerStateTable(t) => t.set_async(key, fvs).await,
            OutputTable::ZmqProducerStateTable(t) => t.set_async(key, fvs).await,
            OutputTable::Table(t) => t.set_async(key, fvs).await,
        }
    }
}
