use core::ffi::c_void;
use core::ptr::slice_from_raw_parts;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

use chrono::NaiveDateTime;

use crate::app::{ConsumerStrategy, CtpbeeR, ProducerTdApi};
use crate::constants::{Direction, Exchange, Offset, OrderType, Status};
use crate::ctp::sys::*;
use crate::interface::Interface;
use crate::structs::{CancelRequest, LoginForm, OrderData, OrderRequest, TradeData};

/// et the api instance  for the TdCallApi
unsafe fn get_trader_spi<'a>(spi: *mut c_void) -> &'a mut dyn TdCallApi {
    &mut **(spi as *mut *mut dyn TdCallApi)
}

// #[link(name = "thosttraderapi_se")]
// extern "C" {}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnFrontConnected(this: *mut ::std::os::raw::c_void) -> () {
    let x = get_trader_spi(this);
    x.on_front_connected();
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnFrontDisconnected(
    this: *mut ::std::os::raw::c_void,
    nReason: c_int,
) -> () {
    let x = get_trader_spi(this);
    x.on_front_disconnected(nReason);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnHeartBeatWarning(
    this: *mut ::std::os::raw::c_void,
    nTimeLapse: c_int,
) -> () {
    let x = get_trader_spi(this);
    x.on_heart_beat_warning(nTimeLapse);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspAuthenticate(
    this: *mut ::std::os::raw::c_void,
    pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_authenticate(pRspAuthenticateField, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspUserLogin(
    this: *mut ::std::os::raw::c_void,
    pRspUserLogin: *mut CThostFtdcRspUserLoginField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_user_login(pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspUserLogout(
    this: *mut ::std::os::raw::c_void,
    pUserLogout: *mut CThostFtdcUserLogoutField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_user_logout(pUserLogout, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspUserPasswordUpdate(
    this: *mut ::std::os::raw::c_void,
    pUserPasswordUpdate: *mut CThostFtdcUserPasswordUpdateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_user_password_update(pUserPasswordUpdate, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspTradingAccountPasswordUpdate(
    this: *mut ::std::os::raw::c_void,
    pTradingAccountPasswordUpdate: *mut CThostFtdcTradingAccountPasswordUpdateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_trading_account_password_update(
        pTradingAccountPasswordUpdate,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspUserAuthMethod(
    this: *mut ::std::os::raw::c_void,
    pRspUserAuthMethod: *mut CThostFtdcRspUserAuthMethodField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_user_auth_method(pRspUserAuthMethod, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspGenUserCaptcha(
    this: *mut ::std::os::raw::c_void,
    pRspGenUserCaptcha: *mut CThostFtdcRspGenUserCaptchaField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_gen_user_captcha(pRspGenUserCaptcha, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspGenUserText(
    this: *mut ::std::os::raw::c_void,
    pRspGenUserText: *mut CThostFtdcRspGenUserTextField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_gen_user_text(pRspGenUserText, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspOrderInsert(
    this: *mut ::std::os::raw::c_void,
    pInputOrder: *mut CThostFtdcInputOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_order_insert(pInputOrder, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspParkedOrderInsert(
    this: *mut ::std::os::raw::c_void,
    pParkedOrder: *mut CThostFtdcParkedOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_parked_order_insert(pParkedOrder, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspParkedOrderAction(
    this: *mut ::std::os::raw::c_void,
    pParkedOrderAction: *mut CThostFtdcParkedOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_parked_order_action(pParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspOrderAction(
    this: *mut ::std::os::raw::c_void,
    pInputOrderAction: *mut CThostFtdcInputOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_order_action(pInputOrderAction, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQueryMaxOrderVolume(
    this: *mut ::std::os::raw::c_void,
    pQueryMaxOrderVolume: *mut CThostFtdcQueryMaxOrderVolumeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_query_max_order_volume(pQueryMaxOrderVolume, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspSettlementInfoConfirm(
    this: *mut ::std::os::raw::c_void,
    pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_settlement_info_confirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspRemoveParkedOrder(
    this: *mut ::std::os::raw::c_void,
    pRemoveParkedOrder: *mut CThostFtdcRemoveParkedOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_remove_parked_order(pRemoveParkedOrder, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspRemoveParkedOrderAction(
    this: *mut ::std::os::raw::c_void,
    pRemoveParkedOrderAction: *mut CThostFtdcRemoveParkedOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_remove_parked_order_action(pRemoveParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspExecOrderInsert(
    this: *mut ::std::os::raw::c_void,
    pInputExecOrder: *mut CThostFtdcInputExecOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_exec_order_insert(pInputExecOrder, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspExecOrderAction(
    this: *mut ::std::os::raw::c_void,
    pInputExecOrderAction: *mut CThostFtdcInputExecOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_exec_order_action(pInputExecOrderAction, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspForQuoteInsert(
    this: *mut ::std::os::raw::c_void,
    pInputForQuote: *mut CThostFtdcInputForQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_for_quote_insert(pInputForQuote, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQuoteInsert(
    this: *mut ::std::os::raw::c_void,
    pInputQuote: *mut CThostFtdcInputQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_quote_insert(pInputQuote, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQuoteAction(
    this: *mut ::std::os::raw::c_void,
    pInputQuoteAction: *mut CThostFtdcInputQuoteActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_quote_action(pInputQuoteAction, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspBatchOrderAction(
    this: *mut ::std::os::raw::c_void,
    pInputBatchOrderAction: *mut CThostFtdcInputBatchOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_batch_order_action(pInputBatchOrderAction, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspOptionSelfCloseInsert(
    this: *mut ::std::os::raw::c_void,
    pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_option_self_close_insert(pInputOptionSelfClose, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspOptionSelfCloseAction(
    this: *mut ::std::os::raw::c_void,
    pInputOptionSelfCloseAction: *mut CThostFtdcInputOptionSelfCloseActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_option_self_close_action(pInputOptionSelfCloseAction, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspCombActionInsert(
    this: *mut ::std::os::raw::c_void,
    pInputCombAction: *mut CThostFtdcInputCombActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_comb_action_insert(pInputCombAction, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryOrder(
    this: *mut ::std::os::raw::c_void,
    pOrder: *mut CThostFtdcOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_order(pOrder, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTrade(
    this: *mut ::std::os::raw::c_void,
    pTrade: *mut CThostFtdcTradeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_trade(pTrade, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestorPosition(
    this: *mut ::std::os::raw::c_void,
    pInvestorPosition: *mut CThostFtdcInvestorPositionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_investor_position(pInvestorPosition, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTradingAccount(
    this: *mut ::std::os::raw::c_void,
    pTradingAccount: *mut CThostFtdcTradingAccountField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_trading_account(pTradingAccount, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestor(
    this: *mut ::std::os::raw::c_void,
    pInvestor: *mut CThostFtdcInvestorField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_investor(pInvestor, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTradingCode(
    this: *mut ::std::os::raw::c_void,
    pTradingCode: *mut CThostFtdcTradingCodeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_trading_code(pTradingCode, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInstrumentMarginRate(
    this: *mut ::std::os::raw::c_void,
    pInstrumentMarginRate: *mut CThostFtdcInstrumentMarginRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_instrument_margin_rate(pInstrumentMarginRate, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInstrumentCommissionRate(
    this: *mut ::std::os::raw::c_void,
    pInstrumentCommissionRate: *mut CThostFtdcInstrumentCommissionRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_instrument_commission_rate(
        pInstrumentCommissionRate,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryExchange(
    this: *mut ::std::os::raw::c_void,
    pExchange: *mut CThostFtdcExchangeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_exchange(pExchange, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryProduct(
    this: *mut ::std::os::raw::c_void,
    pProduct: *mut CThostFtdcProductField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_product(pProduct, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInstrument(
    this: *mut ::std::os::raw::c_void,
    pInstrument: *mut CThostFtdcInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_instrument(pInstrument, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryDepthMarketData(
    this: *mut ::std::os::raw::c_void,
    pDepthMarketData: *mut CThostFtdcDepthMarketDataField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_depth_market_data(pDepthMarketData, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySettlementInfo(
    this: *mut ::std::os::raw::c_void,
    pSettlementInfo: *mut CThostFtdcSettlementInfoField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_settlement_info(pSettlementInfo, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTransferBank(
    this: *mut ::std::os::raw::c_void,
    pTransferBank: *mut CThostFtdcTransferBankField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_transfer_bank(pTransferBank, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestorPositionDetail(
    this: *mut ::std::os::raw::c_void,
    pInvestorPositionDetail: *mut CThostFtdcInvestorPositionDetailField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_investor_position_detail(pInvestorPositionDetail, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryNotice(
    this: *mut ::std::os::raw::c_void,
    pNotice: *mut CThostFtdcNoticeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_notice(pNotice, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySettlementInfoConfirm(
    this: *mut ::std::os::raw::c_void,
    pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_settlement_info_confirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestorPositionCombineDetail(
    this: *mut ::std::os::raw::c_void,
    pInvestorPositionCombineDetail: *mut CThostFtdcInvestorPositionCombineDetailField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_investor_position_combine_detail(
        pInvestorPositionCombineDetail,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryCFMMCTradingAccountKey(
    this: *mut ::std::os::raw::c_void,
    pCFMMCTradingAccountKey: *mut CThostFtdcCFMMCTradingAccountKeyField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_c_f_m_m_c_trading_account_key(
        pCFMMCTradingAccountKey,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryEWarrantOffset(
    this: *mut ::std::os::raw::c_void,
    pEWarrantOffset: *mut CThostFtdcEWarrantOffsetField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_e_warrant_offset(pEWarrantOffset, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestorProductGroupMargin(
    this: *mut ::std::os::raw::c_void,
    pInvestorProductGroupMargin: *mut CThostFtdcInvestorProductGroupMarginField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_investor_product_group_margin(
        pInvestorProductGroupMargin,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryExchangeMarginRate(
    this: *mut ::std::os::raw::c_void,
    pExchangeMarginRate: *mut CThostFtdcExchangeMarginRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_exchange_margin_rate(pExchangeMarginRate, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryExchangeMarginRateAdjust(
    this: *mut ::std::os::raw::c_void,
    pExchangeMarginRateAdjust: *mut CThostFtdcExchangeMarginRateAdjustField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_exchange_margin_rate_adjust(
        pExchangeMarginRateAdjust,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryExchangeRate(
    this: *mut ::std::os::raw::c_void,
    pExchangeRate: *mut CThostFtdcExchangeRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_exchange_rate(pExchangeRate, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySecAgentACIDMap(
    this: *mut ::std::os::raw::c_void,
    pSecAgentACIDMap: *mut CThostFtdcSecAgentACIDMapField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_sec_agent_a_c_i_d_map(pSecAgentACIDMap, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryProductExchRate(
    this: *mut ::std::os::raw::c_void,
    pProductExchRate: *mut CThostFtdcProductExchRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_product_exch_rate(pProductExchRate, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryProductGroup(
    this: *mut ::std::os::raw::c_void,
    pProductGroup: *mut CThostFtdcProductGroupField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_product_group(pProductGroup, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryMMInstrumentCommissionRate(
    this: *mut ::std::os::raw::c_void,
    pMMInstrumentCommissionRate: *mut CThostFtdcMMInstrumentCommissionRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_m_m_instrument_commission_rate(
        pMMInstrumentCommissionRate,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryMMOptionInstrCommRate(
    this: *mut ::std::os::raw::c_void,
    pMMOptionInstrCommRate: *mut CThostFtdcMMOptionInstrCommRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_m_m_option_instr_comm_rate(pMMOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInstrumentOrderCommRate(
    this: *mut ::std::os::raw::c_void,
    pInstrumentOrderCommRate: *mut CThostFtdcInstrumentOrderCommRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_instrument_order_comm_rate(
        pInstrumentOrderCommRate,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySecAgentTradingAccount(
    this: *mut ::std::os::raw::c_void,
    pTradingAccount: *mut CThostFtdcTradingAccountField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_sec_agent_trading_account(pTradingAccount, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySecAgentCheckMode(
    this: *mut ::std::os::raw::c_void,
    pSecAgentCheckMode: *mut CThostFtdcSecAgentCheckModeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_sec_agent_check_mode(pSecAgentCheckMode, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySecAgentTradeInfo(
    this: *mut ::std::os::raw::c_void,
    pSecAgentTradeInfo: *mut CThostFtdcSecAgentTradeInfoField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_sec_agent_trade_info(pSecAgentTradeInfo, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryOptionInstrTradeCost(
    this: *mut ::std::os::raw::c_void,
    pOptionInstrTradeCost: *mut CThostFtdcOptionInstrTradeCostField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_option_instr_trade_cost(pOptionInstrTradeCost, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryOptionInstrCommRate(
    this: *mut ::std::os::raw::c_void,
    pOptionInstrCommRate: *mut CThostFtdcOptionInstrCommRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_option_instr_comm_rate(pOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryExecOrder(
    this: *mut ::std::os::raw::c_void,
    pExecOrder: *mut CThostFtdcExecOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_exec_order(pExecOrder, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryForQuote(
    this: *mut ::std::os::raw::c_void,
    pForQuote: *mut CThostFtdcForQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_for_quote(pForQuote, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryQuote(
    this: *mut ::std::os::raw::c_void,
    pQuote: *mut CThostFtdcQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_quote(pQuote, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryOptionSelfClose(
    this: *mut ::std::os::raw::c_void,
    pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_option_self_close(pOptionSelfClose, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestUnit(
    this: *mut ::std::os::raw::c_void,
    pInvestUnit: *mut CThostFtdcInvestUnitField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_invest_unit(pInvestUnit, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryCombInstrumentGuard(
    this: *mut ::std::os::raw::c_void,
    pCombInstrumentGuard: *mut CThostFtdcCombInstrumentGuardField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_comb_instrument_guard(pCombInstrumentGuard, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryCombAction(
    this: *mut ::std::os::raw::c_void,
    pCombAction: *mut CThostFtdcCombActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_comb_action(pCombAction, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTransferSerial(
    this: *mut ::std::os::raw::c_void,
    pTransferSerial: *mut CThostFtdcTransferSerialField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_transfer_serial(pTransferSerial, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryAccountregister(
    this: *mut ::std::os::raw::c_void,
    pAccountregister: *mut CThostFtdcAccountregisterField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_accountregister(pAccountregister, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspError(
    this: *mut ::std::os::raw::c_void,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_error(pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnOrder(
    this: *mut ::std::os::raw::c_void,
    pOrder: *mut CThostFtdcOrderField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_order(pOrder);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnTrade(
    this: *mut ::std::os::raw::c_void,
    pTrade: *mut CThostFtdcTradeField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_trade(pTrade);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnOrderInsert(
    this: *mut ::std::os::raw::c_void,
    pInputOrder: *mut CThostFtdcInputOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_order_insert(pInputOrder, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnOrderAction(
    this: *mut ::std::os::raw::c_void,
    pOrderAction: *mut CThostFtdcOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_order_action(pOrderAction, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnInstrumentStatus(
    this: *mut ::std::os::raw::c_void,
    pInstrumentStatus: *mut CThostFtdcInstrumentStatusField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_instrument_status(pInstrumentStatus);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnBulletin(
    this: *mut ::std::os::raw::c_void,
    pBulletin: *mut CThostFtdcBulletinField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_bulletin(pBulletin);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnTradingNotice(
    this: *mut ::std::os::raw::c_void,
    pTradingNoticeInfo: *mut CThostFtdcTradingNoticeInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_trading_notice(pTradingNoticeInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnErrorConditionalOrder(
    this: *mut ::std::os::raw::c_void,
    pErrorConditionalOrder: *mut CThostFtdcErrorConditionalOrderField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_error_conditional_order(pErrorConditionalOrder);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnExecOrder(
    this: *mut ::std::os::raw::c_void,
    pExecOrder: *mut CThostFtdcExecOrderField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_exec_order(pExecOrder);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnExecOrderInsert(
    this: *mut ::std::os::raw::c_void,
    pInputExecOrder: *mut CThostFtdcInputExecOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_exec_order_insert(pInputExecOrder, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnExecOrderAction(
    this: *mut ::std::os::raw::c_void,
    pExecOrderAction: *mut CThostFtdcExecOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_exec_order_action(pExecOrderAction, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnForQuoteInsert(
    this: *mut ::std::os::raw::c_void,
    pInputForQuote: *mut CThostFtdcInputForQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_for_quote_insert(pInputForQuote, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnQuote(
    this: *mut ::std::os::raw::c_void,
    pQuote: *mut CThostFtdcQuoteField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_quote(pQuote);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnQuoteInsert(
    this: *mut ::std::os::raw::c_void,
    pInputQuote: *mut CThostFtdcInputQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_quote_insert(pInputQuote, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnQuoteAction(
    this: *mut ::std::os::raw::c_void,
    pQuoteAction: *mut CThostFtdcQuoteActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_quote_action(pQuoteAction, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnForQuoteRsp(
    this: *mut ::std::os::raw::c_void,
    pForQuoteRsp: *mut CThostFtdcForQuoteRspField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_for_quote_rsp(pForQuoteRsp);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnCFMMCTradingAccountToken(
    this: *mut ::std::os::raw::c_void,
    pCFMMCTradingAccountToken: *mut CThostFtdcCFMMCTradingAccountTokenField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_c_f_m_m_c_trading_account_token(pCFMMCTradingAccountToken);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnBatchOrderAction(
    this: *mut ::std::os::raw::c_void,
    pBatchOrderAction: *mut CThostFtdcBatchOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_batch_order_action(pBatchOrderAction, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnOptionSelfClose(
    this: *mut ::std::os::raw::c_void,
    pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_option_self_close(pOptionSelfClose);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnOptionSelfCloseInsert(
    this: *mut ::std::os::raw::c_void,
    pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_option_self_close_insert(pInputOptionSelfClose, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnOptionSelfCloseAction(
    this: *mut ::std::os::raw::c_void,
    pOptionSelfCloseAction: *mut CThostFtdcOptionSelfCloseActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_option_self_close_action(pOptionSelfCloseAction, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnCombAction(
    this: *mut ::std::os::raw::c_void,
    pCombAction: *mut CThostFtdcCombActionField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_comb_action(pCombAction);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnCombActionInsert(
    this: *mut ::std::os::raw::c_void,
    pInputCombAction: *mut CThostFtdcInputCombActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_comb_action_insert(pInputCombAction, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryContractBank(
    this: *mut ::std::os::raw::c_void,
    pContractBank: *mut CThostFtdcContractBankField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_contract_bank(pContractBank, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryParkedOrder(
    this: *mut ::std::os::raw::c_void,
    pParkedOrder: *mut CThostFtdcParkedOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_parked_order(pParkedOrder, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryParkedOrderAction(
    this: *mut ::std::os::raw::c_void,
    pParkedOrderAction: *mut CThostFtdcParkedOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_parked_order_action(pParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTradingNotice(
    this: *mut ::std::os::raw::c_void,
    pTradingNotice: *mut CThostFtdcTradingNoticeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_trading_notice(pTradingNotice, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryBrokerTradingParams(
    this: *mut ::std::os::raw::c_void,
    pBrokerTradingParams: *mut CThostFtdcBrokerTradingParamsField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_broker_trading_params(pBrokerTradingParams, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryBrokerTradingAlgos(
    this: *mut ::std::os::raw::c_void,
    pBrokerTradingAlgos: *mut CThostFtdcBrokerTradingAlgosField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_qry_broker_trading_algos(pBrokerTradingAlgos, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQueryCFMMCTradingAccountToken(
    this: *mut ::std::os::raw::c_void,
    pQueryCFMMCTradingAccountToken: *mut CThostFtdcQueryCFMMCTradingAccountTokenField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_query_c_f_m_m_c_trading_account_token(
        pQueryCFMMCTradingAccountToken,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnFromBankToFutureByBank(
    this: *mut ::std::os::raw::c_void,
    pRspTransfer: *mut CThostFtdcRspTransferField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_from_bank_to_future_by_bank(pRspTransfer);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnFromFutureToBankByBank(
    this: *mut ::std::os::raw::c_void,
    pRspTransfer: *mut CThostFtdcRspTransferField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_from_future_to_bank_by_bank(pRspTransfer);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromBankToFutureByBank(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_bank_to_future_by_bank(pRspRepeal);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromFutureToBankByBank(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_future_to_bank_by_bank(pRspRepeal);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnFromBankToFutureByFuture(
    this: *mut ::std::os::raw::c_void,
    pRspTransfer: *mut CThostFtdcRspTransferField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_from_bank_to_future_by_future(pRspTransfer);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnFromFutureToBankByFuture(
    this: *mut ::std::os::raw::c_void,
    pRspTransfer: *mut CThostFtdcRspTransferField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_from_future_to_bank_by_future(pRspTransfer);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromBankToFutureByFutureManual(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_bank_to_future_by_future_manual(pRspRepeal);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromFutureToBankByFutureManual(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_future_to_bank_by_future_manual(pRspRepeal);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnQueryBankBalanceByFuture(
    this: *mut ::std::os::raw::c_void,
    pNotifyQueryAccount: *mut CThostFtdcNotifyQueryAccountField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_query_bank_balance_by_future(pNotifyQueryAccount);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnBankToFutureByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqTransfer: *mut CThostFtdcReqTransferField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_bank_to_future_by_future(pReqTransfer, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnFutureToBankByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqTransfer: *mut CThostFtdcReqTransferField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_future_to_bank_by_future(pReqTransfer, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnRepealBankToFutureByFutureManual(
    this: *mut ::std::os::raw::c_void,
    pReqRepeal: *mut CThostFtdcReqRepealField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_repeal_bank_to_future_by_future_manual(pReqRepeal, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnRepealFutureToBankByFutureManual(
    this: *mut ::std::os::raw::c_void,
    pReqRepeal: *mut CThostFtdcReqRepealField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_repeal_future_to_bank_by_future_manual(pReqRepeal, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnQueryBankBalanceByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqQueryAccount: *mut CThostFtdcReqQueryAccountField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) -> () {
    let x = get_trader_spi(this);
    x.on_err_rtn_query_bank_balance_by_future(pReqQueryAccount, pRspInfo);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromBankToFutureByFuture(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_bank_to_future_by_future(pRspRepeal);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromFutureToBankByFuture(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_future_to_bank_by_future(pRspRepeal);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspFromBankToFutureByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqTransfer: *mut CThostFtdcReqTransferField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_from_bank_to_future_by_future(pReqTransfer, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspFromFutureToBankByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqTransfer: *mut CThostFtdcReqTransferField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_from_future_to_bank_by_future(pReqTransfer, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQueryBankAccountMoneyByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqQueryAccount: *mut CThostFtdcReqQueryAccountField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) -> () {
    let x = get_trader_spi(this);
    x.on_rsp_query_bank_account_money_by_future(pReqQueryAccount, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnOpenAccountByBank(
    this: *mut ::std::os::raw::c_void,
    pOpenAccount: *mut CThostFtdcOpenAccountField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_open_account_by_bank(pOpenAccount);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnCancelAccountByBank(
    this: *mut ::std::os::raw::c_void,
    pCancelAccount: *mut CThostFtdcCancelAccountField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_cancel_account_by_bank(pCancelAccount);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnChangeAccountByBank(
    this: *mut ::std::os::raw::c_void,
    pChangeAccount: *mut CThostFtdcChangeAccountField,
) -> () {
    let x = get_trader_spi(this);
    x.on_rtn_change_account_by_bank(pChangeAccount);
}

///  交易回调API
/// 此处应该天有很多地方的回调方法
/// 比如on_front_connected等， 此处的代码应该被python的cpp_generator快速生成
pub trait TdCallApi {
    fn on_front_connected(&mut self) -> () {
        println!("function callback: OnFrontConnected");
    }

    fn on_front_disconnected(&mut self, nReason: c_int) -> () {
        println!("function callback: OnFrontDisconnected");
    }

    fn on_heart_beat_warning(&mut self, nTimeLapse: c_int) -> () {
        println!("function callback: OnHeartBeatWarning");
    }

    fn on_rsp_authenticate(
        &mut self,
        pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspAuthenticate");
    }

    fn on_rsp_user_login(
        &mut self,
        pRspUserLogin: *mut CThostFtdcRspUserLoginField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspUserLogin");
    }

    fn on_rsp_user_logout(
        &mut self,
        pUserLogout: *mut CThostFtdcUserLogoutField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspUserLogout");
    }

    fn on_rsp_user_password_update(
        &mut self,
        pUserPasswordUpdate: *mut CThostFtdcUserPasswordUpdateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspUserPasswordUpdate");
    }

    fn on_rsp_trading_account_password_update(
        &mut self,
        pTradingAccountPasswordUpdate: *mut CThostFtdcTradingAccountPasswordUpdateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspTradingAccountPasswordUpdate");
    }

    fn on_rsp_user_auth_method(
        &mut self,
        pRspUserAuthMethod: *mut CThostFtdcRspUserAuthMethodField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspUserAuthMethod");
    }

    fn on_rsp_gen_user_captcha(
        &mut self,
        pRspGenUserCaptcha: *mut CThostFtdcRspGenUserCaptchaField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspGenUserCaptcha");
    }

    fn on_rsp_gen_user_text(
        &mut self,
        pRspGenUserText: *mut CThostFtdcRspGenUserTextField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspGenUserText");
    }

    fn on_rsp_order_insert(
        &mut self,
        pInputOrder: *mut CThostFtdcInputOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspOrderInsert");
    }

    fn on_rsp_parked_order_insert(
        &mut self,
        pParkedOrder: *mut CThostFtdcParkedOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspParkedOrderInsert");
    }

    fn on_rsp_parked_order_action(
        &mut self,
        pParkedOrderAction: *mut CThostFtdcParkedOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspParkedOrderAction");
    }

    fn on_rsp_order_action(
        &mut self,
        pInputOrderAction: *mut CThostFtdcInputOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspOrderAction");
    }

    fn on_rsp_query_max_order_volume(
        &mut self,
        pQueryMaxOrderVolume: *mut CThostFtdcQueryMaxOrderVolumeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQueryMaxOrderVolume");
    }

    fn on_rsp_settlement_info_confirm(
        &mut self,
        pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspSettlementInfoConfirm");
    }

    fn on_rsp_remove_parked_order(
        &mut self,
        pRemoveParkedOrder: *mut CThostFtdcRemoveParkedOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspRemoveParkedOrder");
    }

    fn on_rsp_remove_parked_order_action(
        &mut self,
        pRemoveParkedOrderAction: *mut CThostFtdcRemoveParkedOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspRemoveParkedOrderAction");
    }

    fn on_rsp_exec_order_insert(
        &mut self,
        pInputExecOrder: *mut CThostFtdcInputExecOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspExecOrderInsert");
    }

    fn on_rsp_exec_order_action(
        &mut self,
        pInputExecOrderAction: *mut CThostFtdcInputExecOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspExecOrderAction");
    }

    fn on_rsp_for_quote_insert(
        &mut self,
        pInputForQuote: *mut CThostFtdcInputForQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspForQuoteInsert");
    }

    fn on_rsp_quote_insert(
        &mut self,
        pInputQuote: *mut CThostFtdcInputQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQuoteInsert");
    }

    fn on_rsp_quote_action(
        &mut self,
        pInputQuoteAction: *mut CThostFtdcInputQuoteActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQuoteAction");
    }

    fn on_rsp_batch_order_action(
        &mut self,
        pInputBatchOrderAction: *mut CThostFtdcInputBatchOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspBatchOrderAction");
    }

    fn on_rsp_option_self_close_insert(
        &mut self,
        pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspOptionSelfCloseInsert");
    }

    fn on_rsp_option_self_close_action(
        &mut self,
        pInputOptionSelfCloseAction: *mut CThostFtdcInputOptionSelfCloseActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspOptionSelfCloseAction");
    }

    fn on_rsp_comb_action_insert(
        &mut self,
        pInputCombAction: *mut CThostFtdcInputCombActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspCombActionInsert");
    }

    fn on_rsp_qry_order(
        &mut self,
        pOrder: *mut CThostFtdcOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryOrder");
    }

    fn on_rsp_qry_trade(
        &mut self,
        pTrade: *mut CThostFtdcTradeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryTrade");
    }

    fn on_rsp_qry_investor_position(
        &mut self,
        pInvestorPosition: *mut CThostFtdcInvestorPositionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryInvestorPosition");
    }

    fn on_rsp_qry_trading_account(
        &mut self,
        pTradingAccount: *mut CThostFtdcTradingAccountField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryTradingAccount");
    }

    fn on_rsp_qry_investor(
        &mut self,
        pInvestor: *mut CThostFtdcInvestorField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryInvestor");
    }

    fn on_rsp_qry_trading_code(
        &mut self,
        pTradingCode: *mut CThostFtdcTradingCodeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryTradingCode");
    }

    fn on_rsp_qry_instrument_margin_rate(
        &mut self,
        pInstrumentMarginRate: *mut CThostFtdcInstrumentMarginRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryInstrumentMarginRate");
    }

    fn on_rsp_qry_instrument_commission_rate(
        &mut self,
        pInstrumentCommissionRate: *mut CThostFtdcInstrumentCommissionRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryInstrumentCommissionRate");
    }

    fn on_rsp_qry_exchange(
        &mut self,
        pExchange: *mut CThostFtdcExchangeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryExchange");
    }

    fn on_rsp_qry_product(
        &mut self,
        pProduct: *mut CThostFtdcProductField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryProduct");
    }

    fn on_rsp_qry_instrument(
        &mut self,
        pInstrument: *mut CThostFtdcInstrumentField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryInstrument");
    }

    fn on_rsp_qry_depth_market_data(
        &mut self,
        pDepthMarketData: *mut CThostFtdcDepthMarketDataField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryDepthMarketData");
    }

    fn on_rsp_qry_settlement_info(
        &mut self,
        pSettlementInfo: *mut CThostFtdcSettlementInfoField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQrySettlementInfo");
    }

    fn on_rsp_qry_transfer_bank(
        &mut self,
        pTransferBank: *mut CThostFtdcTransferBankField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryTransferBank");
    }

    fn on_rsp_qry_investor_position_detail(
        &mut self,
        pInvestorPositionDetail: *mut CThostFtdcInvestorPositionDetailField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryInvestorPositionDetail");
    }

    fn on_rsp_qry_notice(
        &mut self,
        pNotice: *mut CThostFtdcNoticeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryNotice");
    }

    fn on_rsp_qry_settlement_info_confirm(
        &mut self,
        pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQrySettlementInfoConfirm");
    }

    fn on_rsp_qry_investor_position_combine_detail(
        &mut self,
        pInvestorPositionCombineDetail: *mut CThostFtdcInvestorPositionCombineDetailField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryInvestorPositionCombineDetail");
    }

    fn on_rsp_qry_c_f_m_m_c_trading_account_key(
        &mut self,
        pCFMMCTradingAccountKey: *mut CThostFtdcCFMMCTradingAccountKeyField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryCFMMCTradingAccountKey");
    }

    fn on_rsp_qry_e_warrant_offset(
        &mut self,
        pEWarrantOffset: *mut CThostFtdcEWarrantOffsetField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryEWarrantOffset");
    }

    fn on_rsp_qry_investor_product_group_margin(
        &mut self,
        pInvestorProductGroupMargin: *mut CThostFtdcInvestorProductGroupMarginField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryInvestorProductGroupMargin");
    }

    fn on_rsp_qry_exchange_margin_rate(
        &mut self,
        pExchangeMarginRate: *mut CThostFtdcExchangeMarginRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryExchangeMarginRate");
    }

    fn on_rsp_qry_exchange_margin_rate_adjust(
        &mut self,
        pExchangeMarginRateAdjust: *mut CThostFtdcExchangeMarginRateAdjustField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryExchangeMarginRateAdjust");
    }

    fn on_rsp_qry_exchange_rate(
        &mut self,
        pExchangeRate: *mut CThostFtdcExchangeRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryExchangeRate");
    }

    fn on_rsp_qry_sec_agent_a_c_i_d_map(
        &mut self,
        pSecAgentACIDMap: *mut CThostFtdcSecAgentACIDMapField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQrySecAgentACIDMap");
    }

    fn on_rsp_qry_product_exch_rate(
        &mut self,
        pProductExchRate: *mut CThostFtdcProductExchRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryProductExchRate");
    }

    fn on_rsp_qry_product_group(
        &mut self,
        pProductGroup: *mut CThostFtdcProductGroupField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryProductGroup");
    }

    fn on_rsp_qry_m_m_instrument_commission_rate(
        &mut self,
        pMMInstrumentCommissionRate: *mut CThostFtdcMMInstrumentCommissionRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryMMInstrumentCommissionRate");
    }

    fn on_rsp_qry_m_m_option_instr_comm_rate(
        &mut self,
        pMMOptionInstrCommRate: *mut CThostFtdcMMOptionInstrCommRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryMMOptionInstrCommRate");
    }

    fn on_rsp_qry_instrument_order_comm_rate(
        &mut self,
        pInstrumentOrderCommRate: *mut CThostFtdcInstrumentOrderCommRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryInstrumentOrderCommRate");
    }

    fn on_rsp_qry_sec_agent_trading_account(
        &mut self,
        pTradingAccount: *mut CThostFtdcTradingAccountField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQrySecAgentTradingAccount");
    }

    fn on_rsp_qry_sec_agent_check_mode(
        &mut self,
        pSecAgentCheckMode: *mut CThostFtdcSecAgentCheckModeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQrySecAgentCheckMode");
    }

    fn on_rsp_qry_sec_agent_trade_info(
        &mut self,
        pSecAgentTradeInfo: *mut CThostFtdcSecAgentTradeInfoField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQrySecAgentTradeInfo");
    }

    fn on_rsp_qry_option_instr_trade_cost(
        &mut self,
        pOptionInstrTradeCost: *mut CThostFtdcOptionInstrTradeCostField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryOptionInstrTradeCost");
    }

    fn on_rsp_qry_option_instr_comm_rate(
        &mut self,
        pOptionInstrCommRate: *mut CThostFtdcOptionInstrCommRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryOptionInstrCommRate");
    }

    fn on_rsp_qry_exec_order(
        &mut self,
        pExecOrder: *mut CThostFtdcExecOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryExecOrder");
    }

    fn on_rsp_qry_for_quote(
        &mut self,
        pForQuote: *mut CThostFtdcForQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryForQuote");
    }

    fn on_rsp_qry_quote(
        &mut self,
        pQuote: *mut CThostFtdcQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryQuote");
    }

    fn on_rsp_qry_option_self_close(
        &mut self,
        pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryOptionSelfClose");
    }

    fn on_rsp_qry_invest_unit(
        &mut self,
        pInvestUnit: *mut CThostFtdcInvestUnitField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryInvestUnit");
    }

    fn on_rsp_qry_comb_instrument_guard(
        &mut self,
        pCombInstrumentGuard: *mut CThostFtdcCombInstrumentGuardField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryCombInstrumentGuard");
    }

    fn on_rsp_qry_comb_action(
        &mut self,
        pCombAction: *mut CThostFtdcCombActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryCombAction");
    }

    fn on_rsp_qry_transfer_serial(
        &mut self,
        pTransferSerial: *mut CThostFtdcTransferSerialField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryTransferSerial");
    }

    fn on_rsp_qry_accountregister(
        &mut self,
        pAccountregister: *mut CThostFtdcAccountregisterField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryAccountregister");
    }

    fn on_rsp_error(
        &mut self,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspError");
    }

    fn on_rtn_order(&mut self, pOrder: *mut CThostFtdcOrderField) -> () {
        println!("function callback: OnRtnOrder");
    }

    fn on_rtn_trade(&mut self, pTrade: *mut CThostFtdcTradeField) -> () {
        println!("function callback: OnRtnTrade");
    }

    fn on_err_rtn_order_insert(
        &mut self,
        pInputOrder: *mut CThostFtdcInputOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnOrderInsert");
    }

    fn on_err_rtn_order_action(
        &mut self,
        pOrderAction: *mut CThostFtdcOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnOrderAction");
    }

    fn on_rtn_instrument_status(
        &mut self,
        pInstrumentStatus: *mut CThostFtdcInstrumentStatusField,
    ) -> () {
        println!("function callback: OnRtnInstrumentStatus");
    }

    fn on_rtn_bulletin(&mut self, pBulletin: *mut CThostFtdcBulletinField) -> () {
        println!("function callback: OnRtnBulletin");
    }

    fn on_rtn_trading_notice(
        &mut self,
        pTradingNoticeInfo: *mut CThostFtdcTradingNoticeInfoField,
    ) -> () {
    }

    fn on_rtn_error_conditional_order(
        &mut self,
        pErrorConditionalOrder: *mut CThostFtdcErrorConditionalOrderField,
    ) -> () {
        println!("function callback: OnRtnErrorConditionalOrder");
    }

    fn on_rtn_exec_order(&mut self, pExecOrder: *mut CThostFtdcExecOrderField) -> () {
        println!("function callback: OnRtnExecOrder");
    }

    fn on_err_rtn_exec_order_insert(
        &mut self,
        pInputExecOrder: *mut CThostFtdcInputExecOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnExecOrderInsert");
    }

    fn on_err_rtn_exec_order_action(
        &mut self,
        pExecOrderAction: *mut CThostFtdcExecOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnExecOrderAction");
    }

    fn on_err_rtn_for_quote_insert(
        &mut self,
        pInputForQuote: *mut CThostFtdcInputForQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnForQuoteInsert");
    }

    fn on_rtn_quote(&mut self, pQuote: *mut CThostFtdcQuoteField) -> () {
        println!("function callback: OnRtnQuote");
    }

    fn on_err_rtn_quote_insert(
        &mut self,
        pInputQuote: *mut CThostFtdcInputQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnQuoteInsert");
    }

    fn on_err_rtn_quote_action(
        &mut self,
        pQuoteAction: *mut CThostFtdcQuoteActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnQuoteAction");
    }

    fn on_rtn_for_quote_rsp(&mut self, pForQuoteRsp: *mut CThostFtdcForQuoteRspField) -> () {
        println!("function callback: OnRtnForQuoteRsp");
    }

    fn on_rtn_c_f_m_m_c_trading_account_token(
        &mut self,
        pCFMMCTradingAccountToken: *mut CThostFtdcCFMMCTradingAccountTokenField,
    ) -> () {
        println!("function callback: OnRtnCFMMCTradingAccountToken");
    }

    fn on_err_rtn_batch_order_action(
        &mut self,
        pBatchOrderAction: *mut CThostFtdcBatchOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnBatchOrderAction");
    }

    fn on_rtn_option_self_close(
        &mut self,
        pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField,
    ) -> () {
        println!("function callback: OnRtnOptionSelfClose");
    }

    fn on_err_rtn_option_self_close_insert(
        &mut self,
        pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnOptionSelfCloseInsert");
    }

    fn on_err_rtn_option_self_close_action(
        &mut self,
        pOptionSelfCloseAction: *mut CThostFtdcOptionSelfCloseActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnOptionSelfCloseAction");
    }

    fn on_rtn_comb_action(&mut self, pCombAction: *mut CThostFtdcCombActionField) -> () {
        println!("function callback: OnRtnCombAction");
    }

    fn on_err_rtn_comb_action_insert(
        &mut self,
        pInputCombAction: *mut CThostFtdcInputCombActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnCombActionInsert");
    }

    fn on_rsp_qry_contract_bank(
        &mut self,
        pContractBank: *mut CThostFtdcContractBankField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryContractBank");
    }

    fn on_rsp_qry_parked_order(
        &mut self,
        pParkedOrder: *mut CThostFtdcParkedOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryParkedOrder");
    }

    fn on_rsp_qry_parked_order_action(
        &mut self,
        pParkedOrderAction: *mut CThostFtdcParkedOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryParkedOrderAction");
    }

    fn on_rsp_qry_trading_notice(
        &mut self,
        pTradingNotice: *mut CThostFtdcTradingNoticeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryTradingNotice");
    }

    fn on_rsp_qry_broker_trading_params(
        &mut self,
        pBrokerTradingParams: *mut CThostFtdcBrokerTradingParamsField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryBrokerTradingParams");
    }

    fn on_rsp_qry_broker_trading_algos(
        &mut self,
        pBrokerTradingAlgos: *mut CThostFtdcBrokerTradingAlgosField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQryBrokerTradingAlgos");
    }

    fn on_rsp_query_c_f_m_m_c_trading_account_token(
        &mut self,
        pQueryCFMMCTradingAccountToken: *mut CThostFtdcQueryCFMMCTradingAccountTokenField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQueryCFMMCTradingAccountToken");
    }

    fn on_rtn_from_bank_to_future_by_bank(
        &mut self,
        pRspTransfer: *mut CThostFtdcRspTransferField,
    ) -> () {
        println!("function callback: OnRtnFromBankToFutureByBank");
    }

    fn on_rtn_from_future_to_bank_by_bank(
        &mut self,
        pRspTransfer: *mut CThostFtdcRspTransferField,
    ) -> () {
        println!("function callback: OnRtnFromFutureToBankByBank");
    }

    fn on_rtn_repeal_from_bank_to_future_by_bank(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) -> () {
        println!("function callback: OnRtnRepealFromBankToFutureByBank");
    }

    fn on_rtn_repeal_from_future_to_bank_by_bank(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) -> () {
        println!("function callback: OnRtnRepealFromFutureToBankByBank");
    }

    fn on_rtn_from_bank_to_future_by_future(
        &mut self,
        pRspTransfer: *mut CThostFtdcRspTransferField,
    ) -> () {
        println!("function callback: OnRtnFromBankToFutureByFuture");
    }

    fn on_rtn_from_future_to_bank_by_future(
        &mut self,
        pRspTransfer: *mut CThostFtdcRspTransferField,
    ) -> () {
        println!("function callback: OnRtnFromFutureToBankByFuture");
    }

    fn on_rtn_repeal_from_bank_to_future_by_future_manual(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) -> () {
        println!("function callback: OnRtnRepealFromBankToFutureByFutureManual");
    }

    fn on_rtn_repeal_from_future_to_bank_by_future_manual(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) -> () {
        println!("function callback: OnRtnRepealFromFutureToBankByFutureManual");
    }

    fn on_rtn_query_bank_balance_by_future(
        &mut self,
        pNotifyQueryAccount: *mut CThostFtdcNotifyQueryAccountField,
    ) -> () {
        println!("function callback: OnRtnQueryBankBalanceByFuture");
    }

    fn on_err_rtn_bank_to_future_by_future(
        &mut self,
        pReqTransfer: *mut CThostFtdcReqTransferField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnBankToFutureByFuture");
    }

    fn on_err_rtn_future_to_bank_by_future(
        &mut self,
        pReqTransfer: *mut CThostFtdcReqTransferField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnFutureToBankByFuture");
    }

    fn on_err_rtn_repeal_bank_to_future_by_future_manual(
        &mut self,
        pReqRepeal: *mut CThostFtdcReqRepealField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnRepealBankToFutureByFutureManual");
    }

    fn on_err_rtn_repeal_future_to_bank_by_future_manual(
        &mut self,
        pReqRepeal: *mut CThostFtdcReqRepealField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnRepealFutureToBankByFutureManual");
    }

    fn on_err_rtn_query_bank_balance_by_future(
        &mut self,
        pReqQueryAccount: *mut CThostFtdcReqQueryAccountField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) -> () {
        println!("function callback: OnErrRtnQueryBankBalanceByFuture");
    }

    fn on_rtn_repeal_from_bank_to_future_by_future(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) -> () {
        println!("function callback: OnRtnRepealFromBankToFutureByFuture");
    }

    fn on_rtn_repeal_from_future_to_bank_by_future(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) -> () {
        println!("function callback: OnRtnRepealFromFutureToBankByFuture");
    }

    fn on_rsp_from_bank_to_future_by_future(
        &mut self,
        pReqTransfer: *mut CThostFtdcReqTransferField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspFromBankToFutureByFuture");
    }

    fn on_rsp_from_future_to_bank_by_future(
        &mut self,
        pReqTransfer: *mut CThostFtdcReqTransferField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspFromFutureToBankByFuture");
    }

    fn on_rsp_query_bank_account_money_by_future(
        &mut self,
        pReqQueryAccount: *mut CThostFtdcReqQueryAccountField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) -> () {
        println!("function callback: OnRspQueryBankAccountMoneyByFuture");
    }

    fn on_rtn_open_account_by_bank(&mut self, pOpenAccount: *mut CThostFtdcOpenAccountField) -> () {
        println!("function callback: OnRtnOpenAccountByBank");
    }

    fn on_rtn_cancel_account_by_bank(
        &mut self,
        pCancelAccount: *mut CThostFtdcCancelAccountField,
    ) -> () {
        println!("function callback: OnRtnCancelAccountByBank");
    }

    fn on_rtn_change_account_by_bank(
        &mut self,
        pChangeAccount: *mut CThostFtdcChangeAccountField,
    ) -> () {
        println!("function callback: OnRtnChangeAccountByBank");
    }
}

/// 交易API
pub struct TdApi {
    order_ref: i32,
    trader_api: *mut CThostFtdcTraderApi,
    trader_spi: Option<*mut FCtpTdSpi>,
    path: CString,
    producer: ProducerTdApi,
    login_info: Option<LoginForm>,
    request_id: i32,
    frontid: c_int,
    sessionid: c_int,
}

impl TdApi {
    fn login_info(&self) -> &LoginForm {
        self.login_info.as_ref().unwrap()
    }
}

pub struct CallDataCollector<'a> {
    login_status: bool,
    connect_status: bool,
    api: &'a mut TdApi,
}

impl<'a> TdCallApi for CallDataCollector<'a> {
    fn on_front_connected(&mut self) {
        println!(">>> Td Front Connected");
        self.api.auth();
    }
    fn on_rsp_authenticate(
        &mut self,
        pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {
                println!(">>> Td Auth successful");
                self.api.login();
            }
            Err(e) => {
                println!(">>> Td Auth failed, id: {} msg: {}", e.id, e.msg);
            }
        }
    }

    fn on_rsp_user_login(
        &mut self,
        pRspUserLogin: *mut CThostFtdcRspUserLoginField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {
                println!(">>> Td Login successful");
                unsafe {
                    println!(
                        "session {} frontid: {}",
                        (*pRspUserLogin).SessionID as f64,
                        (*pRspUserLogin).FrontID as f64
                    );
                }

                unsafe {
                    self.api.frontid = (*pRspUserLogin).FrontID;
                    self.api.sessionid = (*pRspUserLogin).SessionID;
                }
                self.api.req_settle();
            }
            Err(e) => {
                println!(">>> Td Login failed, id: {} msg: {}", e.id, e.msg);
            }
        }
    }
    fn on_rsp_order_insert(
        &mut self,
        pInputOrder: *mut CThostFtdcInputOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {}
            Err(e) => {
                println!(">>> Order failed, id: {} msg: {}", e.id, e.msg);
            }
        }
    }

    fn on_rsp_settlement_info_confirm(
        &mut self,
        pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {
                println!(">>> Td Confirmed Successful");
            }
            Err(e) => {
                println!(">>> Td Confirmed failed, id: {} msg: {}", e.id, e.msg);
            }
        }
    }
    fn on_rtn_order(&mut self, pOrder: *mut CThostFtdcOrderField) -> () {
        let order = unsafe {
            // todo : we need a fast solution to parse datetime
            let time_string: String = slice_to_string(&(*pOrder).InsertTime);
            let date_string: String = slice_to_string(&(*pOrder).InsertDate);
            let datetime = format!("{:?} {:?}", date_string, time_string);
            let naive = NaiveDateTime::parse_from_str(datetime.as_str(), "\"%Y%m%d\" \"%H:%M:%S\"")
                .unwrap();

            self.api.order_ref += 1;
            OrderData {
                symbol: slice_to_string(&(*pOrder).InstrumentID),
                exchange: Some(Exchange::from((*pOrder).ExchangeID)),
                datetime: Option::from(naive),
                orderid: Option::from(slice_to_string(&(*pOrder).OrderRef)),
                order_type: OrderType::from((*pOrder).OrderPriceType),
                direction: Some(Direction::from((*pOrder).Direction)),
                offset: Offset::from((*pOrder).CombOffsetFlag),
                price: (*pOrder).LimitPrice as f64,
                volume: (*pOrder).VolumeTotalOriginal as f64,
                traded: (*pOrder).VolumeTraded as f64,
                status: Some(Status::from((*pOrder).OrderStatus)),
            }
        };
        // 这里控制接收order data的策略index
        self.api.producer.send_to(order, 0);
    }

    fn on_rtn_trade(&mut self, pTrade: *mut CThostFtdcTradeField) {
        let trade = unsafe {
            let time_string = slice_to_string(&(*pTrade).TradeTime);
            let date_string = slice_to_string(&(*pTrade).TradeDate);
            let datetime = format!("{:?} {:?}", date_string, time_string);
            let naive = NaiveDateTime::parse_from_str(datetime.as_str(), "\"%Y%m%d\" \"%H:%M:%S\"")
                .unwrap();
            TradeData {
                symbol: slice_to_string(&(*pTrade).InstrumentID),
                exchange: Some(Exchange::from((*pTrade).ExchangeID)),
                datetime: Option::from(naive),
                orderid: Option::from(slice_to_string(&(*pTrade).OrderRef)),
                direction: Some(Direction::from((*pTrade).Direction)),
                offset: Some(Offset::from((*pTrade).OffsetFlag)),
                price: (*pTrade).Price as f64,
                volume: (*pTrade).Volume as f64,
                tradeid: Some(slice_to_string(&(*pTrade).TradeID)),
            }
        };
        // 这里控制接收order data的策略index
        self.api.producer.send_to(trade, 0);
    }

    fn on_err_rtn_order_insert(
        &mut self,
        pInputOrder: *mut CThostFtdcInputOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {}
            Err(e) => {
                println!(">>> Order Insert failed, id: {} msg: {}", e.id, e.msg);
            }
        }
    }

    fn on_rtn_instrument_status(
        &mut self,
        pInstrumentStatus: *mut CThostFtdcInstrumentStatusField,
    ) {
        unimplemented!()
    }
}

unsafe impl Send for TdApi {}

fn get_order_type(order: OrderType) -> c_char {
    match order {
        OrderType::LIMIT => THOST_FTDC_OPT_LimitPrice as i8,
        OrderType::MARKET => THOST_FTDC_OPT_AnyPrice as i8,
        _ => panic!("This Interface do not support this order direction"),
    }
}

fn get_order_offset(offset: Offset) -> u8 {
    match offset {
        Offset::CLOSE => THOST_FTDC_OF_Close,
        Offset::CLOSETODAY => THOST_FTDC_OF_CloseToday,
        Offset::CLOSEYESTERDAY => THOST_FTDC_OF_CloseYesterday,
        Offset::OPEN => THOST_FTDC_OF_Open,
        _ => panic!("This Interface do not support this order direction"),
    }
}

fn get_order_exchange(exchange: Exchange) -> String {
    match exchange {
        Exchange::SHFE => "SHFE".to_string(),
        Exchange::DCE => "DCE".to_string(),
        Exchange::CZCE => "CZCE".to_string(),
        Exchange::INE => "INE".to_string(),
        Exchange::CFFEX => "CFFEX".to_string(),
        _ => panic!("This Interface do not support this exchange"),
    }
}

fn get_direction(direction: Direction) -> c_char {
    match direction {
        Direction::LONG => THOST_FTDC_D_Buy as i8,
        Direction::SHORT => THOST_FTDC_D_Sell as i8,
        _ => panic!("This Interface do not support this order direction"),
    }
}

impl From<[i8; 9]> for Exchange {
    fn from(i: [i8; 9]) -> Self {
        let exchange = slice_to_string(&i);
        match exchange.as_ref() {
            "SHFE" => Self::SHFE,
            "INE" => Self::INE,
            "CFFEX" => Self::CFFEX,
            "CZCE" => Self::CZCE,
            "DCE" => Self::DCE,
            _ => panic!("ctp do not support this exchange"),
        }
    }
}

impl From<[i8; 5]> for Offset {
    fn from(offset: [i8; 5]) -> Self {
        let x = *offset.first().unwrap() as u8;
        match x {
            THOST_FTDC_OF_Close => Self::CLOSE,
            THOST_FTDC_OF_CloseToday => Self::CLOSETODAY,
            THOST_FTDC_OF_CloseYesterday => Self::CLOSEYESTERDAY,
            THOST_FTDC_OF_Open => Self::OPEN,
            _ => panic!("ctp do not support this"),
        }
    }
}

impl From<i8> for Offset {
    fn from(offset: i8) -> Self {
        let x = offset as u8;
        match x {
            THOST_FTDC_OF_Close => Self::CLOSE,
            THOST_FTDC_OF_CloseToday => Self::CLOSETODAY,
            THOST_FTDC_OF_CloseYesterday => Self::CLOSEYESTERDAY,
            THOST_FTDC_OF_Open => Self::OPEN,
            _ => panic!("ctp do not support this"),
        }
    }
}

impl From<i8> for OrderType {
    fn from(i: i8) -> Self {
        match i as u8 {
            THOST_FTDC_OPT_LimitPrice => Self::LIMIT,
            THOST_FTDC_OPT_AnyPrice => Self::MARKET,
            _ => panic!("ctp do not support this ordertype"),
        }
    }
}

impl From<i8> for Status {
    fn from(i: i8) -> Self {
        match i as u8 {
            THOST_FTDC_OAS_Submitted => Self::SUBMITTING,
            THOST_FTDC_OAS_Accepted => Self::SUBMITTING,
            THOST_FTDC_OAS_Rejected => Self::REJECTED,
            THOST_FTDC_OST_NoTradeQueueing => Status::NOTTRADED,
            THOST_FTDC_OST_PartTradedQueueing => Status::PARTTRADED,
            THOST_FTDC_OST_AllTraded => Status::ALLTRADED,
            THOST_FTDC_OST_Canceled => Status::CANCELLED,
            _ => panic!("ctp do not support this status"),
        }
    }
}

impl From<i8> for Direction {
    fn from(i: i8) -> Self {
        match i as u8 {
            THOST_FTDC_D_Buy => Self::LONG,
            THOST_FTDC_D_Sell => Self::SHORT,
            _ => panic!("ctp do not support other direction"),
        }
    }
}

impl TdApi {
    pub fn new(path: String, producer: ProducerTdApi) -> TdApi {
        let p = CString::new(path).unwrap();
        let flow_path_ptr = p.as_ptr();
        let api = unsafe { CThostFtdcTraderApi::CreateFtdcTraderApi(flow_path_ptr) };
        TdApi {
            order_ref: 0,
            path: p,
            trader_api: api,
            trader_spi: None,
            producer,
            login_info: None,
            request_id: 0,
            frontid: 0 as c_int,
            sessionid: 0 as c_int,
        }
    }

    pub fn auth(&mut self) {
        self.request_id += 1;
        let form = self.login_info();
        let req = CThostFtdcReqAuthenticateField {
            UserID: form.user_id().to_c_slice(),
            BrokerID: form.broke_id().to_c_slice(),
            AuthCode: form.auth_code().to_c_slice(),
            AppID: form.app_id().to_c_slice(),
            UserProductInfo: form.production_info().to_c_slice(),
        };
        unsafe {
            RustCtpCallReqAuthenticate(
                self.trader_api,
                Box::into_raw(Box::new(req)) as *mut CThostFtdcReqAuthenticateField,
                self.request_id,
            )
        };
    }
    pub fn login(&mut self) {
        self.request_id += 1;
        let form = self.login_info();

        let login_req = CThostFtdcReqUserLoginField {
            BrokerID: form.broke_id().to_c_slice(),
            UserID: form.user_id().to_c_slice(),
            Password: form.password().to_c_slice(),
            UserProductInfo: form.production_info().to_c_slice(),
            ..CThostFtdcReqUserLoginField::default()
        };
        unsafe {
            RustCtpCallReqUserLogin(
                self.trader_api,
                Box::into_raw(Box::new(login_req)),
                self.request_id,
            )
        };
    }

    /// 注册交易前值
    fn register_fronted(&mut self, register_addr: CString) {
        let front_socket_address_ptr = register_addr.into_raw();
        unsafe { RustCtpCallRegisterFront(self.trader_api, front_socket_address_ptr) }
    }

    pub fn init(&mut self) -> bool {
        unsafe { RustCtpCallInit(self.trader_api) };
        true
    }

    fn register_spi(&mut self, login_info: LoginForm) {
        self.login_info = Some(login_info);
        let collector = CallDataCollector {
            login_status: false,
            connect_status: false,
            api: self,
        };
        // rust object
        let trait_object_box: Box<Box<dyn TdCallApi>> = Box::new(Box::new(collector));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn TdCallApi> as *mut c_void;
        // 把 rust对象 传给回调SPI
        let trader_spi = unsafe { FCtpTdSpi::new(trait_object_pointer) };
        let ptr = Box::into_raw(Box::new(trader_spi));
        self.trader_spi = Some(ptr);
        unsafe {
            RustCtpCallSubscribePrivateTopic(self.trader_api, 0);
            RustCtpCallSubscribePublicTopic(self.trader_api, 0);
            RustCtpCallRegisterSpi(self.trader_api, ptr as *mut CThostFtdcTraderSpi)
        };
    }

    fn req_settle(&mut self) {
        self.request_id += 1;
        let form = self.login_info();
        let req = CThostFtdcSettlementInfoConfirmField {
            BrokerID: form.broke_id().to_c_slice(),
            InvestorID: form.user_id().to_c_slice(),
            ..CThostFtdcSettlementInfoConfirmField::default()
        };
        unsafe {
            RustCtpCallReqSettlementInfoConfirm(
                self.trader_api,
                Box::into_raw(Box::new(req)),
                self.request_id,
            );
        }
    }
}

impl Drop for TdApi {
    fn drop(&mut self) {}
}

impl Interface for TdApi {
    fn send_order(&mut self, order: OrderRequest) -> String {
        self.request_id += 1;
        self.order_ref += 1;

        let form = self.login_info();

        let req = CThostFtdcInputOrderField {
            InstrumentID: order.symbol.as_str().to_c_slice(),
            LimitPrice: order.price,
            VolumeTotalOriginal: order.volume as c_int,
            OrderPriceType: get_order_type(order.order_type),
            Direction: get_direction(order.direction),
            UserID: form.user_id().to_c_slice(),
            InvestorID: form.user_id().to_c_slice(),
            BrokerID: form.broke_id().to_c_slice(),
            CombOffsetFlag: String::from_utf8(Vec::from([get_order_offset(order.offset)]))
                .unwrap()
                .to_c_slice(),
            OrderRef: self.order_ref.to_string().to_c_slice(),
            CombHedgeFlag: String::from_utf8(Vec::from([THOST_FTDC_HF_Speculation]))
                .unwrap()
                .to_c_slice(),
            ContingentCondition: THOST_FTDC_CC_Immediately as i8,
            ForceCloseReason: THOST_FTDC_FCC_NotForceClose as i8,
            IsAutoSuspend: 0 as c_int,
            TimeCondition: THOST_FTDC_TC_GFD as i8,
            VolumeCondition: THOST_FTDC_VC_AV as i8,
            MinVolume: 1 as c_int,
            ExchangeID: get_order_exchange(order.exchange).to_c_slice(),
            ..CThostFtdcInputOrderField::default()
        };
        unsafe {
            RustCtpCallReqOrderInsert(
                self.trader_api,
                Box::into_raw(Box::new(req)),
                self.request_id,
            )
        };
        "".to_string()
    }

    fn cancel_order(&mut self, req: CancelRequest) {
        // frontid, sessionid, order_ref = req.order_id.split("_")

        let form = self.login_info();

        let action = CThostFtdcInputOrderActionField {
            // InstrumentID: ,
            OrderRef: req.orderid.to_c_slice(),
            FrontID: self.frontid,
            SessionID: self.sessionid,
            ActionFlag: THOST_FTDC_AF_Delete as i8,
            BrokerID: form.broke_id().to_c_slice(),
            InvestorID: form.user_id().to_c_slice(),
            ExchangeID: get_order_exchange(req.exchange).to_c_slice(),
            ..CThostFtdcInputOrderActionField::default()
        };
        unsafe {
            RustCtpCallReqOrderAction(
                self.trader_api,
                Box::into_raw(Box::new(action)),
                self.request_id,
            );
        }
    }

    fn connect(&mut self, req: &LoginForm) {
        self.register_spi(req.clone());
        let addr = CString::new(req.td_address()).unwrap();
        self.register_fronted(addr);
        self.init();
    }

    fn subscribe(&mut self, _: &[&str]) {
        unimplemented!("This API is not allowed in Trade")
    }

    fn unsubscribe(&mut self, symbol: String) {
        unimplemented!("This API is not allowed in Trade")
    }

    fn exit(&mut self) {
        unimplemented!()
    }
}