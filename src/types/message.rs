use crate::structs::{
    AccountData, CancelRequest, ContractData, OrderData, OrderRequest, PositionData, QueryRequest,
    SubscribeRequest, TickData, TradeData,
};

pub enum MdApiMessage {
    TickData(TickData),
    SubscribeRequest(SubscribeRequest),
}

impl From<SubscribeRequest> for MdApiMessage {
    fn from(data: SubscribeRequest) -> Self {
        Self::SubscribeRequest(data)
    }
}

impl From<TickData> for MdApiMessage {
    fn from(data: TickData) -> Self {
        Self::TickData(data)
    }
}

pub enum TdApiMessage {
    OrderData(OrderData),
    TradeData(TradeData),
    AccountData(AccountData),
    PositionData(PositionData),
    ContractData(ContractData),
}

impl From<OrderData> for TdApiMessage {
    fn from(data: OrderData) -> Self {
        Self::OrderData(data)
    }
}

impl From<TradeData> for TdApiMessage {
    fn from(data: TradeData) -> Self {
        Self::TradeData(data)
    }
}

impl From<AccountData> for TdApiMessage {
    fn from(data: AccountData) -> Self {
        Self::AccountData(data)
    }
}

impl From<PositionData> for TdApiMessage {
    fn from(data: PositionData) -> Self {
        Self::PositionData(data)
    }
}

impl From<ContractData> for TdApiMessage {
    fn from(data: ContractData) -> Self {
        Self::ContractData(data)
    }
}

pub enum StrategyMessage {
    OrderRequest(OrderRequest),
    CancelRequest(CancelRequest),
    QueryReq(QueryRequest),
}

impl From<OrderRequest> for StrategyMessage {
    fn from(data: OrderRequest) -> Self {
        Self::OrderRequest(data)
    }
}

impl From<CancelRequest> for StrategyMessage {
    fn from(data: CancelRequest) -> Self {
        Self::CancelRequest(data)
    }
}

impl From<QueryRequest> for StrategyMessage {
    fn from(data: QueryRequest) -> Self {
        Self::QueryReq(data)
    }
}
