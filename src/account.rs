use chrono::{Date, Utc};
use std::borrow::Cow;

use crate::constants::{Direction, Offset};
use crate::structs::PositionData;
use crate::structs::{DailyResult, OrderData, Params, TickData, TradeData};
use crate::util::hash::HashMap;

pub trait PositionManager {
    fn get_all_positions(&self) -> Vec<PositionData>;
}

impl PositionManager for Account {
    fn get_all_positions(&self) -> Vec<PositionData> {
        vec![]
    }
}

/// Account Instance
/// It provides most public ctp to Accept data or solve data
pub struct Account {
    name: String,
    pre_balance: f64,
    // 手续费占用率
    pub commission_ratio: HashMap<String, f64>,
    // 保证金占用率
    pub margin_ratio: HashMap<String, f64>,
    pub size_map: HashMap<String, f64>,
    // 冻结的手续费
    pub frozen_fee: HashMap<String, f64>,
    // 手续费
    pub fee: HashMap<Cow<'static, str>, f64>,
    // 平仓盈亏
    pub close_profit: HashMap<Cow<'static, str>, f64>,
    // 计数器
    pub count: f64,
    // 持仓冻结
    pub margin_frozen_container: HashMap<String, f64>,
    pub pre_close: HashMap<String, f64>,
    pub price_mapping: HashMap<String, f64>,
    pub date: Date<Utc>,
}

impl Account {
    pub(crate) fn new() -> Self {
        Account {
            name: "FlashFunk".to_owned(),
            pre_balance: 0.0,
            commission_ratio: Default::default(),
            margin_ratio: Default::default(),
            size_map: Default::default(),
            frozen_fee: Default::default(),
            fee: Default::default(),
            close_profit: Default::default(),
            count: 0.0,
            margin_frozen_container: Default::default(),
            pre_close: Default::default(),
            price_mapping: Default::default(),
            date: Utc::today(),
        }
    }

    pub fn balance(&self) -> f64 {
        self.available() + self.margin()
    }
    pub fn available(&self) -> f64 {
        self.pre_balance + self.float_pnl() + self.get_close_profit_sum()
            - self.get_frozen_fee_sum()
            - self.get_fee_sum()
            - self.margin()
            - self.frozen_margin()
    }
    pub fn get_fee_sum(&self) -> f64 {
        self.fee.values().into_iter().sum()
    }

    pub fn get_frozen_fee_sum(&self) -> f64 {
        self.frozen_fee.values().into_iter().sum()
    }

    pub fn get_close_profit_sum(&self) -> f64 {
        self.close_profit.values().into_iter().copied().sum()
    }
    /// update trade
    /// 1. add fee to actual fee
    /// 2.remove frozen_fee if exist
    /// 3.remove frozen if exist
    /// 4. add close_profit
    pub fn update_trade(&mut self, data: TradeData) {
        let symbol = data.symbol.clone();
        // calculate fee for trade_data
        let commision = data.volume * data.price * self.get_commission_ratio(symbol.as_ref());

        // Check the orderid if has been frozen
        if let Some(order_id) = &data.orderid {
            // remove会先查找再删除
            self.frozen_fee.remove(order_id);
            // if self.frozen_fee.contains_key(order_id) {
            //     self.frozen_fee.remove(order_id);
            // }
        }
        // insert fee to fact
        match self.fee.get_mut(symbol.as_ref()) {
            Some(t) => *t += commision,
            None => {
                let _ = self.fee.insert(symbol.clone(), commision);
            }
        }

        // update margin_frozen if open else add close_profit for close action
        match data.offset.unwrap() {
            Offset::OPEN => {
                self.count += 1.0;
                if let Some(order_id) = &data.orderid {
                    self.margin_frozen_container.remove(order_id);
                }
            }
            _ => {
                // todo : let pos =
                let close_profit = match data.direction.unwrap() {
                    Direction::LONG => {
                        //  replace 0.0 with  position avg price
                        (0.0 - data.price) * data.volume * self.get_size_map(&symbol)
                    }
                    Direction::SHORT => {
                        (data.price - 0.0) * data.volume * self.get_size_map(&symbol)
                    }
                    _ => 0.0,
                };

                match self.close_profit.get_mut(symbol.as_ref()) {
                    Some(t) => *t += close_profit,
                    None => {
                        let _ = self.close_profit.insert(symbol.clone(), close_profit);
                    }
                }
            }
        }
    }
    /// return size by passed symbol
    fn get_size_map(&self, symbol: &str) -> f64 {
        self.size_map.get(symbol).copied().unwrap_or(0.0)
    }
    /// return commission_ration by passed symbol
    fn get_commission_ratio(&self, symbol: &str) -> f64 {
        self.commission_ratio.get(symbol).copied().unwrap_or(0.0)
    }
    /// return margin_ratio by passed symbol
    fn get_margin_ratio(&self, symbol: &str) -> f64 {
        self.margin_ratio.get(symbol).copied().unwrap_or(0.0)
    }
    /// update order
    /// 1.add frozen fee if open
    /// 2.add margin_frozen if open
    pub fn update_order(&mut self, data: OrderData) {
        let symbol = data.symbol.as_str();
        let commission_ratio = self.get_commission_ratio(&symbol);

        match data.offset {
            Offset::OPEN => {
                // Add Margin frozen
                let ratio = self.get_margin_ratio(&symbol);
                self.margin_frozen_container
                    .insert(data.orderid.unwrap(), data.volume * data.price * ratio);
            }
            _ => {}
        }

        self.frozen_fee
            .insert(data.symbol, commission_ratio * data.volume * data.price);
    }
    /// update position by tick
    /// refresh pnl in time
    pub fn update_tick(&mut self, tick: TickData) {
        unimplemented!()
    }
    /// Get the float pnl for account!
    /// so the most important things is to calculate the float pnl. It should be regard as a  import things
    /// And we should use replace the price  with pre_close_price  for yd `position`, and `price` for today volume,
    /// so we had to maintain the pre_close and real_price both in looper or realtime trade
    pub fn float_pnl(&self) -> f64 {
        self.get_all_positions()
            .iter()
            .map(|x| {
                let real_price = self.get_real_price(x.symbol.as_ref());
                match x.direction.unwrap() {
                    Direction::LONG => {
                        if x.yd_volume.eq(&0.0) {
                            x.volume * (real_price - x.price) * self.get_size_map(x.symbol.as_ref())
                        } else {
                            let today = x.volume - x.yd_volume;
                            today * (real_price - x.price) * self.get_size_map(x.symbol.as_ref())
                                + x.yd_volume
                                    * (real_price - self.get_pre_price(x.symbol.as_ref()))
                                    * self.get_size_map(x.symbol.as_ref())
                        }
                    }
                    Direction::SHORT => {
                        if x.yd_volume.eq(&0.0) {
                            x.volume * (x.price - real_price) * self.get_size_map(x.symbol.as_ref())
                        } else {
                            let today = x.volume - x.yd_volume;
                            today * (x.price - real_price) * self.get_size_map(x.symbol.as_ref())
                                + x.yd_volume
                                    * (self.get_pre_price(x.symbol.as_ref()) - real_price)
                                    * self.get_size_map(x.symbol.as_ref())
                        }
                    }
                    _ => panic!("暂不支持"),
                }
            })
            .sum()
    }
    /// 获取实时价格
    fn get_real_price(&self, symbol: &str) -> f64 {
        *self.price_mapping.get(symbol).unwrap_or(&0.0)
    }
    /// 获取昨日收盘价
    fn get_pre_price(&self, symbol: &str) -> f64 {
        *self.pre_close.get(symbol).unwrap_or(&0.0)
    }
    ///  get the margin of position for the account
    pub fn margin(&self) -> f64 {
        let rs = 0.0;

        self.get_all_positions()
            .into_iter()
            .fold(0.0, |mut rs, pos| {
                let symbol = pos.symbol.as_ref();
                rs += pos.price
                    * pos.volume
                    * self.get_margin_ratio(symbol)
                    * self.get_size_map(symbol);

                rs
            })
    }
    /// settle the account by passed a datetime
    pub fn settle(&mut self, date: Date<Utc>) -> bool {
        if self.date == date {
            false
        } else {
            let p = self.generate_self();
            self.date = date;
            true
        }
    }
    /// update the params by pass a Params
    /// it looks like hard to understand
    fn update_params(&mut self, params: Params) {
        unimplemented!()
    }
    /// get the frozen , when day,end ,it will zero
    pub fn frozen_margin(&self) -> f64 {
        self.margin_frozen_container
            .values()
            .into_iter()
            .copied()
            .sum()
    }
    /// generator a Account object named DailyResult, it will be written into database
    fn generate_self(&self) -> DailyResult {
        DailyResult {
            available: self.available(),
            balance: self.balance(),
            fee: self.get_fee_sum(),
            margin: self.margin(),
            date: self.date.to_string(),
        }
    }
}

impl From<HashMap<String, f64>> for Account {
    fn from(_: HashMap<String, f64>) -> Self {
        unimplemented!()
    }
}
