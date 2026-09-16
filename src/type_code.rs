//! Uniform BAI2 balance reporting type codes.
//!
//! Generated from Appendix A ("Uniform BAI Balance Reporting Type Codes
//! and Type Code Ranges") of the BAI Cash Management Balance Reporting
//! Specifications, Version 2. See `docs/TYPE_CODE.md` for the technical spec.

/// Whether a type code applies to a debit, a credit, or neither.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Transaction {
    /// Not applicable — used by Account Status codes and non-monetary codes.
    Na,
    /// Debit.
    Db,
    /// Credit.
    Cr,
}

/// Whether a type code is an account status, an activity summary, or a
/// transaction detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    /// Describes the status of the account (03 record and 88 continuations only).
    Status,
    /// Summarizes account credit/debit activity (03 record and 88 continuations only).
    Summary,
    /// Details an individual credit or debit (16 record only).
    Detail,
}

/// A uniform BAI2 type code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeCode {
    /// Opening Ledger (010, NA, Status).
    OpeningLedger,
    /// Average Opening Ledger MTD (011, NA, Status).
    AverageOpeningLedgerMtd,
    /// Average Opening Ledger YTD (012, NA, Status).
    AverageOpeningLedgerYtd,
    /// Closing Ledger (015, NA, Status).
    ClosingLedger,
    /// Average Closing Ledger MTD (020, NA, Status).
    AverageClosingLedgerMtd,
    /// Average Closing Ledger - Previous Month (021, NA, Status).
    AverageClosingLedgerPreviousMonth,
    /// Aggregate Balance Adjustments (022, NA, Status).
    AggregateBalanceAdjustments,
    /// Average Closing Ledger YTD - Previous Month (024, NA, Status).
    AverageClosingLedgerYtdPreviousMonth,
    /// Average Closing Ledger YTD (025, NA, Status).
    AverageClosingLedgerYtd,
    /// Current Ledger (030, NA, Status).
    CurrentLedger,
    /// ACH Net Position (037, NA, Status).
    AchNetPosition,
    /// Opening Available + Total Same-Day ACH DTC Deposit (039, NA, Status).
    OpeningAvailableTotalSameDayAchDtcDeposit,
    /// Opening Available (040, NA, Status).
    OpeningAvailable,
    /// Average Opening Available MTD (041, NA, Status).
    AverageOpeningAvailableMtd,
    /// Average Opening Available YTD (042, NA, Status).
    AverageOpeningAvailableYtd,
    /// Average Available - Previous Month (043, NA, Status).
    AverageAvailablePreviousMonth,
    /// Disbursing Opening Available Balance (044, NA, Status).
    DisbursingOpeningAvailableBalance,
    /// Closing Available (045, NA, Status).
    ClosingAvailable,
    /// Average Closing Available MTD (050, NA, Status).
    AverageClosingAvailableMtd,
    /// Average Closing Available - Last Month (051, NA, Status).
    AverageClosingAvailableLastMonth,
    /// Average Closing Available YTD - Last Month (054, NA, Status).
    AverageClosingAvailableYtdLastMonth,
    /// Average Closing Available YTD (055, NA, Status).
    AverageClosingAvailableYtd,
    /// Loan Balance (056, NA, Status).
    LoanBalance,
    /// Total Investment Position (057, NA, Status).
    TotalInvestmentPosition,
    /// Current Available (CRS Supressed) (059, NA, Status).
    CurrentAvailableCrsSupressed,
    /// Current Available (060, NA, Status).
    CurrentAvailable,
    /// Average Current Available MTD (061, NA, Status).
    AverageCurrentAvailableMtd,
    /// Average Current Available YTD (062, NA, Status).
    AverageCurrentAvailableYtd,
    /// Total Float (063, NA, Status).
    TotalFloat,
    /// Target Balance (065, NA, Status).
    TargetBalance,
    /// Adjusted Balance (066, NA, Status).
    AdjustedBalance,
    /// Adjusted Balance MTD (067, NA, Status).
    AdjustedBalanceMtd,
    /// Adjusted Balance YTD (068, NA, Status).
    AdjustedBalanceYtd,
    /// 0-Day Float (070, NA, Status).
    N0DayFloat,
    /// 1-Day Float (072, NA, Status).
    N1DayFloat,
    /// Float Adjustment (073, NA, Status).
    FloatAdjustmentNa,
    /// 2 or More Days Float (074, NA, Status).
    N2OrMoreDaysFloat,
    /// 3 or More Days Float (075, NA, Status).
    N3OrMoreDaysFloat,
    /// Adjustment to Balances (076, NA, Status).
    AdjustmentToBalances,
    /// Average Adjustment to Balances MTD (077, NA, Status).
    AverageAdjustmentToBalancesMtd,
    /// Average Adjustment to Balances YTD (078, NA, Status).
    AverageAdjustmentToBalancesYtd,
    /// 4-Day Float (079, NA, Status).
    N4DayFloat,
    /// 5-Day Float (080, NA, Status).
    N5DayFloat,
    /// 6-Day Float (081, NA, Status).
    N6DayFloat,
    /// Average 1-Day Float MTD (082, NA, Status).
    Average1DayFloatMtd,
    /// Average 1-Day Float YTD (083, NA, Status).
    Average1DayFloatYtd,
    /// Average 2-Day Float MTD (084, NA, Status).
    Average2DayFloatMtd,
    /// Average 2-Day Float YTD (085, NA, Status).
    Average2DayFloatYtd,
    /// Transfer Calculation (086, NA, Status).
    TransferCalculation,
    /// Total Credits (100, CR, Summary).
    TotalCredits,
    /// Total Credit Amount MTD (101, CR, Summary).
    TotalCreditAmountMtd,
    /// Credits Not Detailed (105, CR, Summary).
    CreditsNotDetailed,
    /// Deposits Subject to Float (106, CR, Summary).
    DepositsSubjectToFloat,
    /// Total Adjustment Credits YTD (107, CR, Summary).
    TotalAdjustmentCreditsYtd,
    /// Credit (Any Type) (108, CR, Detail).
    CreditAnyType,
    /// Current Day Total Lockbox Deposits (109, CR, Summary).
    CurrentDayTotalLockboxDeposits,
    /// Total Lockbox Deposits (110, CR, Summary).
    TotalLockboxDeposits,
    /// Lockbox Deposit (115, CR, Detail).
    LockboxDeposit,
    /// Item in Lockbox Deposit (116, CR, Detail).
    ItemInLockboxDeposit,
    /// Lockbox Adjustment Credit (118, CR, Detail).
    LockboxAdjustmentCredit,
    /// EDI Transaction Credit (120, CR, Summary).
    EdiTransactionCreditCrSummary,
    /// EDI Transaction Credit (121, CR, Detail).
    EdiTransactionCreditCrDetail,
    /// EDIBANX Credit Received (122, CR, Detail).
    EdibanxCreditReceived,
    /// EDIBANX Credit Return (123, CR, Detail).
    EdibanxCreditReturn,
    /// Total Concentration Credits (130, CR, Summary).
    TotalConcentrationCredits,
    /// Total DTC Credits (131, CR, Summary).
    TotalDtcCredits,
    /// DTC Concentration Credit (135, CR, Detail).
    DtcConcentrationCredit,
    /// Item in DTC Deposit (136, CR, Detail).
    ItemInDtcDeposit,
    /// Total ACH Credits (140, CR, Summary).
    TotalAchCredits,
    /// ACH Credit Received (142, CR, Detail).
    AchCreditReceived,
    /// Item in ACH Deposit (143, CR, Detail).
    ItemInAchDeposit,
    /// ACH Concentration Credit (145, CR, Detail).
    AchConcentrationCredit,
    /// Total Bank Card Deposits (146, CR, Summary).
    TotalBankCardDeposits,
    /// Individual Bank Card Deposit (147, CR, Detail).
    IndividualBankCardDeposit,
    /// Total Preauthorized Payment Credits (150, CR, Summary).
    TotalPreauthorizedPaymentCredits,
    /// Preauthorized Draft Credit (155, CR, Detail).
    PreauthorizedDraftCredit,
    /// Item in PAC Deposit (156, CR, Detail).
    ItemInPacDeposit,
    /// Total ACH Disbursing Funding Credits (160, CR, Summary).
    TotalAchDisbursingFundingCredits,
    /// Corporate Trade Payment Settlement (162, CR, Summary).
    CorporateTradePaymentSettlementCr,
    /// Corporate Trade Payment Credits (163, CR, Summary).
    CorporateTradePaymentCredits,
    /// Corporate Trade Payment Credit (164, CR, Detail).
    CorporateTradePaymentCredit,
    /// Preauthorized ACH Credit (165, CR, Detail).
    PreauthorizedAchCredit,
    /// ACH Settlement (166, CR, Detail).
    AchSettlementCr,
    /// ACH Settlement Credits (167, CR, Summary).
    AchSettlementCredits,
    /// ACH Return Item or Adjustment Settlement (168, CR, Detail).
    AchReturnItemOrAdjustmentSettlementCr,
    /// Miscellaneous ACH Credit (169, CR, Detail).
    MiscellaneousAchCredit,
    /// Total Other Check Deposits (170, CR, Summary).
    TotalOtherCheckDeposits,
    /// Individual Loan Deposit (171, CR, Detail).
    IndividualLoanDeposit,
    /// Deposit Correction (172, CR, Detail).
    DepositCorrection,
    /// Bank-Prepared Deposit (173, CR, Detail).
    BankPreparedDeposit,
    /// Other Deposit (174, CR, Detail).
    OtherDeposit,
    /// Check Deposit Package (175, CR, Detail).
    CheckDepositPackage,
    /// Re-presented Check Deposit (176, CR, Detail).
    RePresentedCheckDeposit,
    /// List Post Credits (178, CR, Summary).
    ListPostCredits,
    /// Total Loan Proceeds (180, CR, Summary).
    TotalLoanProceeds,
    /// Total Bank-Prepared Deposits (182, CR, Summary).
    TotalBankPreparedDeposits,
    /// Draft Deposit (184, CR, Detail).
    DraftDeposit,
    /// Total Miscellaneous Deposits (185, CR, Summary).
    TotalMiscellaneousDeposits,
    /// Total Cash Letter Credits (186, CR, Summary).
    TotalCashLetterCredits,
    /// Cash Letter Credit (187, CR, Detail).
    CashLetterCredit,
    /// Total Cash Letter Adjustments (188, CR, Summary).
    TotalCashLetterAdjustments,
    /// Cash Letter Adjustment (189, CR, Detail).
    CashLetterAdjustmentCr,
    /// Total Incoming Money Transfers (190, CR, Summary).
    TotalIncomingMoneyTransfers,
    /// Individual Incoming Internal Money Transfer (191, CR, Detail).
    IndividualIncomingInternalMoneyTransfer,
    /// Incoming Money Transfer (195, CR, Detail).
    IncomingMoneyTransfer,
    /// Money Transfer Adjustment (196, CR, Detail).
    MoneyTransferAdjustmentCr,
    /// Compensation (198, CR, Detail).
    CompensationCr,
    /// Total Automatic Transfer Credits (200, CR, Summary).
    TotalAutomaticTransferCredits,
    /// Individual Automatic Transfer Credit (201, CR, Detail).
    IndividualAutomaticTransferCredit,
    /// Bond Operations Credit (202, CR, Detail).
    BondOperationsCredit,
    /// Total Book Transfer Credits (205, CR, Summary).
    TotalBookTransferCredits,
    /// Book Transfer Credit (206, CR, Detail).
    BookTransferCredit,
    /// Total International Money Transfer Credits (207, CR, Summary).
    TotalInternationalMoneyTransferCredits,
    /// Individual International Money Transfer Credit (208, CR, Detail).
    IndividualInternationalMoneyTransferCredit,
    /// Total International Credits (210, CR, Summary).
    TotalInternationalCredits,
    /// Foreign Letter of Credit (212, CR, Detail).
    ForeignLetterOfCredit,
    /// Letter of Credit (213, CR, Detail).
    LetterOfCreditCr,
    /// Foreign Exchange of Credit (214, CR, Detail).
    ForeignExchangeOfCredit,
    /// Total Letters of Credit (215, CR, Summary).
    TotalLettersOfCreditCr,
    /// Foreign Remittance Credit (216, CR, Detail).
    ForeignRemittanceCredit,
    /// Foreign Collection Credit (218, CR, Detail).
    ForeignCollectionCredit,
    /// Foreign Check Purchase (221, CR, Detail).
    ForeignCheckPurchase,
    /// Foreign Checks Deposited (222, CR, Detail).
    ForeignChecksDeposited,
    /// Commission (224, CR, Detail).
    CommissionCr,
    /// International Money Market Trading (226, CR, Detail).
    InternationalMoneyMarketTradingCr,
    /// Standing Order (227, CR, Detail).
    StandingOrderCr,
    /// Miscellaneous International Credit (229, CR, Detail).
    MiscellaneousInternationalCredit,
    /// Total Security Credits (230, CR, Summary).
    TotalSecurityCredits,
    /// Total Collection Credits (231, CR, Summary).
    TotalCollectionCredits,
    /// Sale of Debt Security (232, CR, Detail).
    SaleOfDebtSecurity,
    /// Securities Sold (233, CR, Detail).
    SecuritiesSold,
    /// Sale of Equity Security (234, CR, Detail).
    SaleOfEquitySecurity,
    /// Matured Reverse Repurchase Order (235, CR, Detail).
    MaturedReverseRepurchaseOrder,
    /// Maturity of Debt Security (236, CR, Detail).
    MaturityOfDebtSecurity,
    /// Individual Collection Credit (237, CR, Detail).
    IndividualCollectionCredit,
    /// Collection of Dividends (238, CR, Detail).
    CollectionOfDividends,
    /// Total Bankers' Acceptance Credits (239, CR, Summary).
    TotalBankersAcceptanceCredits,
    /// Coupon Collections - Banks (240, CR, Detail).
    CouponCollectionsBanks,
    /// Bankers' Acceptances (241, CR, Detail).
    BankersAcceptancesCr,
    /// Collection of Interest Income (242, CR, Detail).
    CollectionOfInterestIncome,
    /// Matured Fed Funds Purchased (243, CR, Detail).
    MaturedFedFundsPurchased,
    /// Interest/Matured Principal Payment (244, CR, Detail).
    InterestMaturedPrincipalPaymentCr,
    /// Monthly Dividends (245, CR, Summary).
    MonthlyDividends,
    /// Commercial Paper (246, CR, Detail).
    CommercialPaperCr,
    /// Capital Change (247, CR, Detail).
    CapitalChangeCr,
    /// Savings Bonds Sales Adjustment (248, CR, Detail).
    SavingsBondsSalesAdjustmentCr,
    /// Miscellaneous Security Credit (249, CR, Detail).
    MiscellaneousSecurityCredit,
    /// Total Checks Posted and Returned (250, CR, Summary).
    TotalChecksPostedAndReturned,
    /// Total Debit Reversals (251, CR, Summary).
    TotalDebitReversals,
    /// Debit Reversal (252, CR, Detail).
    DebitReversal,
    /// Posting Error Correction Credit (254, CR, Detail).
    PostingErrorCorrectionCredit,
    /// Check Posted and Returned (255, CR, Detail).
    CheckPostedAndReturned,
    /// Total ACH Return Items (256, CR, Summary).
    TotalAchReturnItemsCr,
    /// Individual ACH Return Item (257, CR, Detail).
    IndividualAchReturnItemCr,
    /// ACH Reversal Credit (258, CR, Detail).
    AchReversalCredit,
    /// Total Rejected Credits (260, CR, Summary).
    TotalRejectedCredits,
    /// Individual Rejected Credit (261, CR, Detail).
    IndividualRejectedCredit,
    /// Overdraft (263, CR, Detail).
    OverdraftCr,
    /// Return Item (266, CR, Detail).
    ReturnItemCr,
    /// Return Item Adjustment (268, CR, Detail).
    ReturnItemAdjustmentCr,
    /// Total ZBA Credits (270, CR, Summary).
    TotalZbaCredits,
    /// Net Zero-Balance Amount (271, CR, Summary).
    NetZeroBalanceAmount,
    /// Cumulative ZBA or Disbursement Credits (274, CR, Detail).
    CumulativeZbaOrDisbursementCredits,
    /// ZBA Credit (275, CR, Detail).
    ZbaCredit,
    /// ZBA Float Adjustment (276, CR, Detail).
    ZbaFloatAdjustment,
    /// ZBA Credit Transfer (277, CR, Detail).
    ZbaCreditTransfer,
    /// ZBA Credit Adjustment (278, CR, Detail).
    ZbaCreditAdjustment,
    /// Total Controlled Disbursing Credits (280, CR, Summary).
    TotalControlledDisbursingCredits,
    /// Individual Controlled Disbursing Credit (281, CR, Detail).
    IndividualControlledDisbursingCredit,
    /// Total DTC Disbursing Credits (285, CR, Summary).
    TotalDtcDisbursingCredits,
    /// Individual DTC Disbursing Credit (286, CR, Detail).
    IndividualDtcDisbursingCredit,
    /// Total ATM Credits (294, CR, Summary).
    TotalAtmCredits,
    /// ATM Credit (295, CR, Detail).
    AtmCredit,
    /// Commercial Deposit (301, CR, Detail).
    CommercialDeposit,
    /// Correspondent Bank Deposit (302, CR, Summary).
    CorrespondentBankDeposit,
    /// Total Wire Transfers In - FF (303, CR, Summary).
    TotalWireTransfersInFf,
    /// Total Wire Transfers In - CHF (304, CR, Summary).
    TotalWireTransfersInChf,
    /// Total Fed Funds Sold (305, CR, Summary).
    TotalFedFundsSold,
    /// Fed Funds Sold (306, CR, Detail).
    FedFundsSold,
    /// Total Trust Credits (307, CR, Summary).
    TotalTrustCredits,
    /// Trust Credit (308, CR, Detail).
    TrustCredit,
    /// Total Value - Dated Funds (309, CR, Summary).
    TotalValueDatedFunds,
    /// Total Commercial Deposits (310, CR, Summary).
    TotalCommercialDeposits,
    /// Total International Credits - FF (315, CR, Summary).
    TotalInternationalCreditsFf,
    /// Total International Credits - CHF (316, CR, Summary).
    TotalInternationalCreditsChf,
    /// Total Foreign Check Purchased (318, CR, Summary).
    TotalForeignCheckPurchased,
    /// Late Deposit (319, CR, Summary).
    LateDeposit,
    /// Total Securities Sold - FF (320, CR, Summary).
    TotalSecuritiesSoldFf,
    /// Total Securities Sold - CHF (321, CR, Summary).
    TotalSecuritiesSoldChf,
    /// Total Securities Matured - FF (324, CR, Summary).
    TotalSecuritiesMaturedFf,
    /// Total Securities Matured - CHF (325, CR, Summary).
    TotalSecuritiesMaturedChf,
    /// Total Securities Interest (326, CR, Summary).
    TotalSecuritiesInterest,
    /// Total Securities Matured (327, CR, Summary).
    TotalSecuritiesMatured,
    /// Total Securities Interest - FF (328, CR, Summary).
    TotalSecuritiesInterestFf,
    /// Total Securities Interest - CHF (329, CR, Summary).
    TotalSecuritiesInterestChf,
    /// Total Escrow Credits (330, CR, Summary).
    TotalEscrowCredits,
    /// Individual Escrow Credit (331, CR, Detail).
    IndividualEscrowCredit,
    /// Total Miscellaneous Securities Credits - FF (332, CR, Summary).
    TotalMiscellaneousSecuritiesCreditsFf,
    /// Total Miscellaneous Securities Credits - CHF (336, CR, Summary).
    TotalMiscellaneousSecuritiesCreditsChf,
    /// Total Securities Sold (338, CR, Summary).
    TotalSecuritiesSold,
    /// Total Broker Deposits (340, CR, Summary).
    TotalBrokerDeposits,
    /// Total Broker Deposits - FF (341, CR, Summary).
    TotalBrokerDepositsFf,
    /// Broker Deposit (342, CR, Detail).
    BrokerDeposit,
    /// Total Broker Deposits - CHF (343, CR, Summary).
    TotalBrokerDepositsChf,
    /// Individual Back Value Credit (344, CR, Detail).
    IndividualBackValueCredit,
    /// Item in Brokers Deposit (345, CR, Detail).
    ItemInBrokersDeposit,
    /// Sweep Interest Income (346, CR, Detail).
    SweepInterestIncome,
    /// Sweep Principal Sell (347, CR, Detail).
    SweepPrincipalSell,
    /// Futures Credit (348, CR, Detail).
    FuturesCredit,
    /// Principal Payments Credit (349, CR, Detail).
    PrincipalPaymentsCredit,
    /// Investment Sold (350, CR, Summary).
    InvestmentSold,
    /// Individual Investment Sold (351, CR, Detail).
    IndividualInvestmentSold,
    /// Total Cash Center Credits (352, CR, Summary).
    TotalCashCenterCredits,
    /// Cash Center Credit (353, CR, Detail).
    CashCenterCredit,
    /// Interest Credit (354, CR, Detail).
    InterestCredit,
    /// Investment Interest (355, CR, Summary).
    InvestmentInterest,
    /// Total Credit Adjustment (356, CR, Summary).
    TotalCreditAdjustment,
    /// Credit Adjustment (357, CR, Detail).
    CreditAdjustment,
    /// YTD Adjustment Credit (358, CR, Detail).
    YtdAdjustmentCredit,
    /// Interest Adjustment Credit (359, CR, Detail).
    InterestAdjustmentCredit,
    /// Total Credits Less Wire Transfer and Returned Checks (360, CR, Summary).
    TotalCreditsLessWireTransferAndReturnedChecks,
    /// Grand Total Credits Less Grand Total Debits (361, CR, Summary).
    GrandTotalCreditsLessGrandTotalDebits,
    /// Correspondent Collection (362, CR, Detail).
    CorrespondentCollection,
    /// Correspondent Collection Adjustment (363, CR, Detail).
    CorrespondentCollectionAdjustmentCr,
    /// Loan Participation (364, CR, Detail).
    LoanParticipationCr,
    /// Currency and Coin Deposited (366, CR, Detail).
    CurrencyAndCoinDeposited,
    /// Food Stamp Letter (367, CR, Detail).
    FoodStampLetterCr,
    /// Food Stamp Adjustment (368, CR, Detail).
    FoodStampAdjustmentCr,
    /// Clearing Settlement Credit (369, CR, Detail).
    ClearingSettlementCredit,
    /// Total Back Value Credits (370, CR, Summary).
    TotalBackValueCredits,
    /// Back Value Adjustment (372, CR, Detail).
    BackValueAdjustmentCr,
    /// Customer Payroll (373, CR, Detail).
    CustomerPayrollCr,
    /// FRB Statement Recap (374, CR, Detail).
    FrbStatementRecapCr,
    /// Savings Bond Letter or Adjustment (376, CR, Detail).
    SavingsBondLetterOrAdjustmentCr,
    /// Treasury Tax and Loan Credit (377, CR, Detail).
    TreasuryTaxAndLoanCredit,
    /// Transfer of Treasury Credit (378, CR, Detail).
    TransferOfTreasuryCredit,
    /// FRB Government Checks Cash Letter Credit (379, CR, Detail).
    FrbGovernmentChecksCashLetterCredit,
    /// FRB Government Check Adjustment (381, CR, Detail).
    FrbGovernmentCheckAdjustmentCr,
    /// FRB Postal Money Order Credit (382, CR, Detail).
    FrbPostalMoneyOrderCredit,
    /// FRB Postal Money Order Adjustment (383, CR, Detail).
    FrbPostalMoneyOrderAdjustmentCr,
    /// FRB Cash Letter Auto Charge Credit (384, CR, Detail).
    FrbCashLetterAutoChargeCredit,
    /// Total Universal Credits (385, CR, Summary).
    TotalUniversalCredits,
    /// FRB Cash Letter Auto Charge Adjustment (386, CR, Detail).
    FrbCashLetterAutoChargeAdjustmentCr,
    /// FRB Fine-Sort Cash Letter Credit (387, CR, Detail).
    FrbFineSortCashLetterCredit,
    /// FRB Fine-Sort Adjustment (388, CR, Detail).
    FrbFineSortAdjustmentCr,
    /// Total Freight Payment Credits (389, CR, Summary).
    TotalFreightPaymentCredits,
    /// Total Miscellaneous Credits (390, CR, Summary).
    TotalMiscellaneousCredits,
    /// Universal Credit (391, CR, Detail).
    UniversalCredit,
    /// Freight Payment Credit (392, CR, Detail).
    FreightPaymentCredit,
    /// Itemized Credit Over $10 (393, CR, Detail).
    ItemizedCreditOver10,
    /// Cumulative Credits (394, CR, Detail).
    CumulativeCredits,
    /// Check Reversal (395, CR, Detail).
    CheckReversal,
    /// Float Adjustment (397, CR, Detail).
    FloatAdjustmentCr,
    /// Miscellaneous Fee Refund (398, CR, Detail).
    MiscellaneousFeeRefund,
    /// Miscellaneous Credit (399, CR, Detail).
    MiscellaneousCredit,
    /// Total Debits (400, DB, Summary).
    TotalDebits,
    /// Total Debit Amount MTD (401, DB, Summary).
    TotalDebitAmountMtd,
    /// Today's Total Debits (403, DB, Summary).
    TodaySTotalDebits,
    /// Total Debit Less Wire Transfers and Charge-Backs (405, DB, Summary).
    TotalDebitLessWireTransfersAndChargeBacks,
    /// Debits not Detailed (406, DB, Summary).
    DebitsNotDetailed,
    /// Float Adjustment (408, DB, Detail).
    FloatAdjustmentDb,
    /// Debit (Any Type) (409, DB, Detail).
    DebitAnyType,
    /// Total YTD Adjustment (410, DB, Summary).
    TotalYtdAdjustment,
    /// Total Debits (Excluding Returned Items) (412, DB, Summary).
    TotalDebitsExcludingReturnedItems,
    /// Lockbox Debit (415, DB, Detail).
    LockboxDebit,
    /// Total Lockbox Debits (416, DB, Summary).
    TotalLockboxDebits,
    /// EDI Transaction Debits (420, DB, Summary).
    EdiTransactionDebits,
    /// EDI Transaction Debit (421, DB, Detail).
    EdiTransactionDebit,
    /// EDIBANX Settlement Debit (422, DB, Detail).
    EdibanxSettlementDebit,
    /// EDIBANX Return Item Debit (423, DB, Detail).
    EdibanxReturnItemDebit,
    /// Total Payable-Through Drafts (430, DB, Summary).
    TotalPayableThroughDrafts,
    /// Payable-Through Draft (435, DB, Detail).
    PayableThroughDraft,
    /// ACH Concentration Debit (445, DB, Detail).
    AchConcentrationDebit,
    /// Total ACH Disbursement Funding Debits (446, DB, Summary).
    TotalAchDisbursementFundingDebits,
    /// ACH Disbursement Funding Debit (447, DB, Detail).
    AchDisbursementFundingDebit,
    /// Total ACH Debits (450, DB, Summary).
    TotalAchDebits,
    /// ACH Debit Received (451, DB, Detail).
    AchDebitReceived,
    /// Item in ACH Disbursement or Debit (452, DB, Detail).
    ItemInAchDisbursementOrDebit,
    /// Preauthorized ACH Debit (455, DB, Detail).
    PreauthorizedAchDebit,
    /// Account Holder Initiated ACH Debit (462, DB, Detail).
    AccountHolderInitiatedAchDebit,
    /// Corporate Trade Payment Debits (463, DB, Summary).
    CorporateTradePaymentDebits,
    /// Corporate Trade Payment Debit (464, DB, Detail).
    CorporateTradePaymentDebit,
    /// Corporate Trade Payment Settlement (465, DB, Summary).
    CorporateTradePaymentSettlementDb,
    /// ACH Settlement (466, DB, Detail).
    AchSettlementDb,
    /// ACH Settlement Debits (467, DB, Summary).
    AchSettlementDebits,
    /// ACH Return Item or Adjustment Settlement (468, DB, Detail).
    AchReturnItemOrAdjustmentSettlementDb,
    /// Miscellaneous ACH Debit (469, DB, Detail).
    MiscellaneousAchDebit,
    /// Total Check Paid (470, DB, Summary).
    TotalCheckPaid,
    /// Total Check Paid - Cumulative MTD (471, DB, Summary).
    TotalCheckPaidCumulativeMtd,
    /// Cumulative Checks Paid (472, DB, Detail).
    CumulativeChecksPaid,
    /// Certified Check Debit (474, DB, Detail).
    CertifiedCheckDebit,
    /// Check Paid (475, DB, Detail).
    CheckPaid,
    /// Federal Reserve Bank Letter Debit (476, DB, Detail).
    FederalReserveBankLetterDebit,
    /// Bank Originated Debit (477, DB, Detail).
    BankOriginatedDebit,
    /// List Post Debits (478, DB, Summary).
    ListPostDebits,
    /// List Post Debit (479, DB, Detail).
    ListPostDebit,
    /// Total Loan Payments (480, DB, Summary).
    TotalLoanPayments,
    /// Individual Loan Payment (481, DB, Detail).
    IndividualLoanPayment,
    /// Total Bank-Originated Debits (482, DB, Summary).
    TotalBankOriginatedDebits,
    /// Draft (484, DB, Detail).
    Draft,
    /// DTC Debit (485, DB, Detail).
    DtcDebit,
    /// Total Cash Letter Debits (486, DB, Summary).
    TotalCashLetterDebits,
    /// Cash Letter Debit (487, DB, Detail).
    CashLetterDebit,
    /// Cash Letter Adjustment (489, DB, Detail).
    CashLetterAdjustmentDb,
    /// Total Outgoing Money Transfers (490, DB, Summary).
    TotalOutgoingMoneyTransfers,
    /// Individual Outgoing Internal Money Transfer (491, DB, Detail).
    IndividualOutgoingInternalMoneyTransfer,
    /// Customer Terminal Initiated Money Transfer (493, DB, Detail).
    CustomerTerminalInitiatedMoneyTransfer,
    /// Outgoing Money Transfer (495, DB, Detail).
    OutgoingMoneyTransfer,
    /// Money Transfer Adjustment (496, DB, Detail).
    MoneyTransferAdjustmentDb,
    /// Compensation (498, DB, Detail).
    CompensationDb,
    /// Total Automatic Transfer Debits (500, DB, Summary).
    TotalAutomaticTransferDebits,
    /// Individual Automatic Transfer Debit (501, DB, Detail).
    IndividualAutomaticTransferDebit,
    /// Bond Operations Debit (502, DB, Detail).
    BondOperationsDebit,
    /// Total Book Transfer Debits (505, DB, Summary).
    TotalBookTransferDebits,
    /// Book Transfer Debit (506, DB, Detail).
    BookTransferDebit,
    /// Total International Money Transfer Debits (507, DB, Summary).
    TotalInternationalMoneyTransferDebits,
    /// Individual International Money Transfer Debits (508, DB, Detail).
    IndividualInternationalMoneyTransferDebits,
    /// Total International Debits (510, DB, Summary).
    TotalInternationalDebits,
    /// Letter of Credit Debit (512, DB, Detail).
    LetterOfCreditDebit,
    /// Letter of Credit (513, DB, Detail).
    LetterOfCreditDb,
    /// Foreign Exchange Debit (514, DB, Detail).
    ForeignExchangeDebit,
    /// Total Letters of Credit (515, DB, Summary).
    TotalLettersOfCreditDb,
    /// Foreign Remittance Debit (516, DB, Detail).
    ForeignRemittanceDebit,
    /// Foreign Collection Debit (518, DB, Detail).
    ForeignCollectionDebit,
    /// Foreign Checks Paid (522, DB, Detail).
    ForeignChecksPaid,
    /// Commission (524, DB, Detail).
    CommissionDb,
    /// International Money Market Trading (526, DB, Detail).
    InternationalMoneyMarketTradingDb,
    /// Standing Order (527, DB, Detail).
    StandingOrderDb,
    /// Miscellaneous International Debit (529, DB, Detail).
    MiscellaneousInternationalDebit,
    /// Total Security Debits (530, DB, Summary).
    TotalSecurityDebits,
    /// Securities Purchased (531, DB, Detail).
    SecuritiesPurchased,
    /// Total Amount of Securities Purchased (532, DB, Summary).
    TotalAmountOfSecuritiesPurchased,
    /// Security Collection Debit (533, DB, Detail).
    SecurityCollectionDebit,
    /// Total Miscellaneous Securities DB - FF (534, DB, Summary).
    TotalMiscellaneousSecuritiesDbFf,
    /// Purchase of Equity Securities (535, DB, Detail).
    PurchaseOfEquitySecurities,
    /// Total Miscellaneous Securities Debit - CHF (536, DB, Summary).
    TotalMiscellaneousSecuritiesDebitChf,
    /// Total Collection Debit (537, DB, Summary).
    TotalCollectionDebit,
    /// Matured Repurchase Order (538, DB, Detail).
    MaturedRepurchaseOrder,
    /// Total Bankers' Acceptances Debit (539, DB, Summary).
    TotalBankersAcceptancesDebit,
    /// Coupon Collection Debit (540, DB, Detail).
    CouponCollectionDebit,
    /// Bankers' Acceptances (541, DB, Detail).
    BankersAcceptancesDb,
    /// Purchase of Debt Securities (542, DB, Detail).
    PurchaseOfDebtSecurities,
    /// Domestic Collection (543, DB, Detail).
    DomesticCollection,
    /// Interest/Matured Principal Payment (544, DB, Detail).
    InterestMaturedPrincipalPaymentDb,
    /// Commercial paper (546, DB, Detail).
    CommercialPaperDb,
    /// Capital Change (547, DB, Detail).
    CapitalChangeDb,
    /// Savings Bonds Sales Adjustment (548, DB, Detail).
    SavingsBondsSalesAdjustmentDb,
    /// Miscellaneous Security Debit (549, DB, Detail).
    MiscellaneousSecurityDebit,
    /// Total Deposited Items Returned (550, DB, Summary).
    TotalDepositedItemsReturned,
    /// Total Credit Reversals (551, DB, Summary).
    TotalCreditReversals,
    /// Credit Reversal (552, DB, Detail).
    CreditReversal,
    /// Posting Error Correction Debit (554, DB, Detail).
    PostingErrorCorrectionDebit,
    /// Deposited Item Returned (555, DB, Detail).
    DepositedItemReturned,
    /// Total ACH Return Items (556, DB, Summary).
    TotalAchReturnItemsDb,
    /// Individual ACH Return Item (557, DB, Detail).
    IndividualAchReturnItemDb,
    /// ACH Reversal Debit (558, DB, Detail).
    AchReversalDebit,
    /// Total Rejected Debits (560, DB, Summary).
    TotalRejectedDebits,
    /// Individual Rejected Debit (561, DB, Detail).
    IndividualRejectedDebit,
    /// Overdraft (563, DB, Detail).
    OverdraftDb,
    /// Overdraft Fee (564, DB, Detail).
    OverdraftFee,
    /// Return Item (566, DB, Detail).
    ReturnItemDb,
    /// Return Item Fee (567, DB, Detail).
    ReturnItemFee,
    /// Return Item Adjustment (568, DB, Detail).
    ReturnItemAdjustmentDb,
    /// Total ZBA Debits (570, DB, Summary).
    TotalZbaDebits,
    /// Cumulative ZBA Debits (574, DB, Detail).
    CumulativeZbaDebits,
    /// ZBA Debit (575, DB, Detail).
    ZbaDebit,
    /// ZBA Debit Transfer (577, DB, Detail).
    ZbaDebitTransfer,
    /// ZBA Debit Adjustment (578, DB, Detail).
    ZbaDebitAdjustment,
    /// Total Controlled Disbursing Debits (580, DB, Summary).
    TotalControlledDisbursingDebits,
    /// Individual Controlled Disbursing Debit (581, DB, Detail).
    IndividualControlledDisbursingDebit,
    /// Total Disbursing Checks Paid - Early Amount (583, DB, Summary).
    TotalDisbursingChecksPaidEarlyAmount,
    /// Total Disbursing Checks Paid - Later Amount (584, DB, Summary).
    TotalDisbursingChecksPaidLaterAmount,
    /// Disbursing Funding Requirement (585, DB, Summary).
    DisbursingFundingRequirement,
    /// FRB Presentment Estimate (Fed Estimate) (586, DB, Summary).
    FrbPresentmentEstimateFedEstimate,
    /// Late Debits (After Notification) (587, DB, Summary).
    LateDebitsAfterNotification,
    /// Total Disbursing Checks Paid-Last Amount (588, DB, Summary).
    TotalDisbursingChecksPaidLastAmount,
    /// Total DTC Debits (590, DB, Summary).
    TotalDtcDebits,
    /// Total ATM Debits (594, DB, Summary).
    TotalAtmDebits,
    /// ATM Debit (595, DB, Detail).
    AtmDebit,
    /// Total APR Debits (596, DB, Summary).
    TotalAprDebits,
    /// ARP Debit (597, DB, Detail).
    ArpDebit,
    /// Estimated Total Disbursement (601, DB, Summary).
    EstimatedTotalDisbursement,
    /// Adjusted Total Disbursement (602, DB, Summary).
    AdjustedTotalDisbursement,
    /// Total Funds Required (610, DB, Summary).
    TotalFundsRequired,
    /// Total Wire Transfers Out- CHF (611, DB, Summary).
    TotalWireTransfersOutChf,
    /// Total Wire Transfers Out - FF (612, DB, Summary).
    TotalWireTransfersOutFf,
    /// Total International Debit - CHF (613, DB, Summary).
    TotalInternationalDebitChf,
    /// Total International Debit - FF (614, DB, Summary).
    TotalInternationalDebitFf,
    /// Total Federal Reserve Bank - Commercial Bank Debit (615, DB, Summary).
    TotalFederalReserveBankCommercialBankDebit,
    /// Federal Reserve Bank - Commercial Bank Debit (616, DB, Detail).
    FederalReserveBankCommercialBankDebit,
    /// Total Securities Purchased - CHF (617, DB, Summary).
    TotalSecuritiesPurchasedChf,
    /// Total Securities Purchased - FF (618, DB, Summary).
    TotalSecuritiesPurchasedFf,
    /// Total Broker Debits - CHF (621, DB, Summary).
    TotalBrokerDebitsChf,
    /// Broker Debit (622, DB, Detail).
    BrokerDebit,
    /// Total Broker Debits - FF (623, DB, Summary).
    TotalBrokerDebitsFf,
    /// Total Broker Debits (625, DB, Summary).
    TotalBrokerDebits,
    /// Total Fed Funds Purchased (626, DB, Summary).
    TotalFedFundsPurchased,
    /// Fed Funds Purchased (627, DB, Detail).
    FedFundsPurchased,
    /// Total Cash Center Debits (628, DB, Summary).
    TotalCashCenterDebits,
    /// Cash Center Debit (629, DB, Detail).
    CashCenterDebit,
    /// Total Debit Adjustments (630, DB, Summary).
    TotalDebitAdjustments,
    /// Debit Adjustment (631, DB, Detail).
    DebitAdjustment,
    /// Total Trust Debits (632, DB, Summary).
    TotalTrustDebits,
    /// Trust Debit (633, DB, Detail).
    TrustDebit,
    /// YTD Adjustment Debit (634, DB, Detail).
    YtdAdjustmentDebit,
    /// Total Escrow Debits (640, DB, Summary).
    TotalEscrowDebits,
    /// Individual Escrow Debit (641, DB, Detail).
    IndividualEscrowDebit,
    /// Individual Back Value Debit (644, DB, Detail).
    IndividualBackValueDebit,
    /// Transfer Calculation Debit (646, DB, Summary).
    TransferCalculationDebit,
    /// Investments Purchased (650, DB, Summary).
    InvestmentsPurchased,
    /// Individual Investment purchased (651, DB, Detail).
    IndividualInvestmentPurchased,
    /// Interest Debit (654, DB, Detail).
    InterestDebit,
    /// Total Investment Interest Debits (655, DB, Summary).
    TotalInvestmentInterestDebits,
    /// Sweep Principal Buy (656, DB, Detail).
    SweepPrincipalBuy,
    /// Futures Debit (657, DB, Detail).
    FuturesDebit,
    /// Principal Payments Debit (658, DB, Detail).
    PrincipalPaymentsDebit,
    /// Interest Adjustment Debit (659, DB, Detail).
    InterestAdjustmentDebit,
    /// Account Analysis Fee (661, DB, Detail).
    AccountAnalysisFee,
    /// Correspondent Collection Debit (662, DB, Detail).
    CorrespondentCollectionDebit,
    /// Correspondent Collection Adjustment (663, DB, Detail).
    CorrespondentCollectionAdjustmentDb,
    /// Loan Participation (664, DB, Detail).
    LoanParticipationDb,
    /// Intercept Debits (665, DB, Summary).
    InterceptDebits,
    /// Currency and Coin Shipped (666, DB, Detail).
    CurrencyAndCoinShipped,
    /// Food Stamp Letter (667, DB, Detail).
    FoodStampLetterDb,
    /// Food Stamp Adjustment (668, DB, Detail).
    FoodStampAdjustmentDb,
    /// Clearing Settlement Debit (669, DB, Detail).
    ClearingSettlementDebit,
    /// Total Back Value Debits (670, DB, Summary).
    TotalBackValueDebits,
    /// Back Value Adjustment (672, DB, Detail).
    BackValueAdjustmentDb,
    /// Customer Payroll (673, DB, Detail).
    CustomerPayrollDb,
    /// FRB Statement Recap (674, DB, Detail).
    FrbStatementRecapDb,
    /// Savings Bond Letter or Adjustment (676, DB, Detail).
    SavingsBondLetterOrAdjustmentDb,
    /// Treasury Tax and Loan Debit (677, DB, Detail).
    TreasuryTaxAndLoanDebit,
    /// Transfer of Treasury Debit (678, DB, Detail).
    TransferOfTreasuryDebit,
    /// FRB Government Checks Cash Letter Debit (679, DB, Detail).
    FrbGovernmentChecksCashLetterDebit,
    /// FRB Government Check Adjustment (681, DB, Detail).
    FrbGovernmentCheckAdjustmentDb,
    /// FRB Postal Money Order Debit (682, DB, Detail).
    FrbPostalMoneyOrderDebit,
    /// FRB Postal Money Order Adjustment (683, DB, Detail).
    FrbPostalMoneyOrderAdjustmentDb,
    /// FRB Cash Letter Auto Charge Debit (684, DB, Detail).
    FrbCashLetterAutoChargeDebit,
    /// Total Universal Debits (685, DB, Summary).
    TotalUniversalDebits,
    /// FRB Cash Letter Auto Charge Adjustment (686, DB, Detail).
    FrbCashLetterAutoChargeAdjustmentDb,
    /// FRB Fine-Sort Cash Letter Debit (687, DB, Detail).
    FrbFineSortCashLetterDebit,
    /// FRB Fine-Sort Adjustment (688, DB, Detail).
    FrbFineSortAdjustmentDb,
    /// FRB Freight Payment Debits (689, DB, Summary).
    FrbFreightPaymentDebits,
    /// Total Miscellaneous Debits (690, DB, Summary).
    TotalMiscellaneousDebits,
    /// Universal Debit (691, DB, Detail).
    UniversalDebit,
    /// Freight Payment Debit (692, DB, Detail).
    FreightPaymentDebit,
    /// Itemized Debit Over $10 (693, DB, Detail).
    ItemizedDebitOver10,
    /// Deposit Reversal (694, DB, Detail).
    DepositReversal,
    /// Deposit Correction Debit (695, DB, Detail).
    DepositCorrectionDebit,
    /// Regular Collection Debit (696, DB, Detail).
    RegularCollectionDebit,
    /// Cumulative Debits (697, DB, Detail).
    CumulativeDebits,
    /// Miscellaneous Fees (698, DB, Detail).
    MiscellaneousFees,
    /// Miscellaneous Debit (699, DB, Detail).
    MiscellaneousDebit,
    /// Principal Loan Balance (701, NA, Status).
    PrincipalLoanBalance,
    /// Available Commitment Amount (703, NA, Status).
    AvailableCommitmentAmount,
    /// Payment Amount Due (705, NA, Status).
    PaymentAmountDue,
    /// Principal Amount Past Due (707, NA, Status).
    PrincipalAmountPastDue,
    /// Interest Amount Past Due (709, NA, Status).
    InterestAmountPastDue,
    /// Total Loan Payment (720, CR, Summary).
    TotalLoanPayment,
    /// Amount Applied to Interest (721, CR, Detail).
    AmountAppliedToInterest,
    /// Amount Applied to Principal (722, CR, Detail).
    AmountAppliedToPrincipal,
    /// Amount Applied to Escrow (723, CR, Detail).
    AmountAppliedToEscrow,
    /// Amount Applied to Late Charges (724, CR, Detail).
    AmountAppliedToLateCharges,
    /// Amount Applied to Buydown (725, CR, Detail).
    AmountAppliedToBuydown,
    /// Amount Applied to Misc. Fees (726, CR, Detail).
    AmountAppliedToMiscFees,
    /// Amount Applied to Deferred Interest Detail (727, CR, Detail).
    AmountAppliedToDeferredInterestDetail,
    /// Amount Applied to Service Charge (728, CR, Detail).
    AmountAppliedToServiceCharge,
    /// Loan Disbursement (760, DB, Summary).
    LoanDisbursement,
    /// Contains Non-monetary Information (890, NA, Detail).
    ContainsNonMonetaryInformation,
    /// A type code not defined in the Uniform BAI Balance Reporting Type Codes appendix.
    Unknown,
}

impl TypeCode {
    /// The three-digit BAI2 type code string, e.g. "010". Empty for `Unknown`.
    pub fn code(self) -> &'static str {
        match self {
            Self::OpeningLedger => "010",
            Self::AverageOpeningLedgerMtd => "011",
            Self::AverageOpeningLedgerYtd => "012",
            Self::ClosingLedger => "015",
            Self::AverageClosingLedgerMtd => "020",
            Self::AverageClosingLedgerPreviousMonth => "021",
            Self::AggregateBalanceAdjustments => "022",
            Self::AverageClosingLedgerYtdPreviousMonth => "024",
            Self::AverageClosingLedgerYtd => "025",
            Self::CurrentLedger => "030",
            Self::AchNetPosition => "037",
            Self::OpeningAvailableTotalSameDayAchDtcDeposit => "039",
            Self::OpeningAvailable => "040",
            Self::AverageOpeningAvailableMtd => "041",
            Self::AverageOpeningAvailableYtd => "042",
            Self::AverageAvailablePreviousMonth => "043",
            Self::DisbursingOpeningAvailableBalance => "044",
            Self::ClosingAvailable => "045",
            Self::AverageClosingAvailableMtd => "050",
            Self::AverageClosingAvailableLastMonth => "051",
            Self::AverageClosingAvailableYtdLastMonth => "054",
            Self::AverageClosingAvailableYtd => "055",
            Self::LoanBalance => "056",
            Self::TotalInvestmentPosition => "057",
            Self::CurrentAvailableCrsSupressed => "059",
            Self::CurrentAvailable => "060",
            Self::AverageCurrentAvailableMtd => "061",
            Self::AverageCurrentAvailableYtd => "062",
            Self::TotalFloat => "063",
            Self::TargetBalance => "065",
            Self::AdjustedBalance => "066",
            Self::AdjustedBalanceMtd => "067",
            Self::AdjustedBalanceYtd => "068",
            Self::N0DayFloat => "070",
            Self::N1DayFloat => "072",
            Self::FloatAdjustmentNa => "073",
            Self::N2OrMoreDaysFloat => "074",
            Self::N3OrMoreDaysFloat => "075",
            Self::AdjustmentToBalances => "076",
            Self::AverageAdjustmentToBalancesMtd => "077",
            Self::AverageAdjustmentToBalancesYtd => "078",
            Self::N4DayFloat => "079",
            Self::N5DayFloat => "080",
            Self::N6DayFloat => "081",
            Self::Average1DayFloatMtd => "082",
            Self::Average1DayFloatYtd => "083",
            Self::Average2DayFloatMtd => "084",
            Self::Average2DayFloatYtd => "085",
            Self::TransferCalculation => "086",
            Self::TotalCredits => "100",
            Self::TotalCreditAmountMtd => "101",
            Self::CreditsNotDetailed => "105",
            Self::DepositsSubjectToFloat => "106",
            Self::TotalAdjustmentCreditsYtd => "107",
            Self::CreditAnyType => "108",
            Self::CurrentDayTotalLockboxDeposits => "109",
            Self::TotalLockboxDeposits => "110",
            Self::LockboxDeposit => "115",
            Self::ItemInLockboxDeposit => "116",
            Self::LockboxAdjustmentCredit => "118",
            Self::EdiTransactionCreditCrSummary => "120",
            Self::EdiTransactionCreditCrDetail => "121",
            Self::EdibanxCreditReceived => "122",
            Self::EdibanxCreditReturn => "123",
            Self::TotalConcentrationCredits => "130",
            Self::TotalDtcCredits => "131",
            Self::DtcConcentrationCredit => "135",
            Self::ItemInDtcDeposit => "136",
            Self::TotalAchCredits => "140",
            Self::AchCreditReceived => "142",
            Self::ItemInAchDeposit => "143",
            Self::AchConcentrationCredit => "145",
            Self::TotalBankCardDeposits => "146",
            Self::IndividualBankCardDeposit => "147",
            Self::TotalPreauthorizedPaymentCredits => "150",
            Self::PreauthorizedDraftCredit => "155",
            Self::ItemInPacDeposit => "156",
            Self::TotalAchDisbursingFundingCredits => "160",
            Self::CorporateTradePaymentSettlementCr => "162",
            Self::CorporateTradePaymentCredits => "163",
            Self::CorporateTradePaymentCredit => "164",
            Self::PreauthorizedAchCredit => "165",
            Self::AchSettlementCr => "166",
            Self::AchSettlementCredits => "167",
            Self::AchReturnItemOrAdjustmentSettlementCr => "168",
            Self::MiscellaneousAchCredit => "169",
            Self::TotalOtherCheckDeposits => "170",
            Self::IndividualLoanDeposit => "171",
            Self::DepositCorrection => "172",
            Self::BankPreparedDeposit => "173",
            Self::OtherDeposit => "174",
            Self::CheckDepositPackage => "175",
            Self::RePresentedCheckDeposit => "176",
            Self::ListPostCredits => "178",
            Self::TotalLoanProceeds => "180",
            Self::TotalBankPreparedDeposits => "182",
            Self::DraftDeposit => "184",
            Self::TotalMiscellaneousDeposits => "185",
            Self::TotalCashLetterCredits => "186",
            Self::CashLetterCredit => "187",
            Self::TotalCashLetterAdjustments => "188",
            Self::CashLetterAdjustmentCr => "189",
            Self::TotalIncomingMoneyTransfers => "190",
            Self::IndividualIncomingInternalMoneyTransfer => "191",
            Self::IncomingMoneyTransfer => "195",
            Self::MoneyTransferAdjustmentCr => "196",
            Self::CompensationCr => "198",
            Self::TotalAutomaticTransferCredits => "200",
            Self::IndividualAutomaticTransferCredit => "201",
            Self::BondOperationsCredit => "202",
            Self::TotalBookTransferCredits => "205",
            Self::BookTransferCredit => "206",
            Self::TotalInternationalMoneyTransferCredits => "207",
            Self::IndividualInternationalMoneyTransferCredit => "208",
            Self::TotalInternationalCredits => "210",
            Self::ForeignLetterOfCredit => "212",
            Self::LetterOfCreditCr => "213",
            Self::ForeignExchangeOfCredit => "214",
            Self::TotalLettersOfCreditCr => "215",
            Self::ForeignRemittanceCredit => "216",
            Self::ForeignCollectionCredit => "218",
            Self::ForeignCheckPurchase => "221",
            Self::ForeignChecksDeposited => "222",
            Self::CommissionCr => "224",
            Self::InternationalMoneyMarketTradingCr => "226",
            Self::StandingOrderCr => "227",
            Self::MiscellaneousInternationalCredit => "229",
            Self::TotalSecurityCredits => "230",
            Self::TotalCollectionCredits => "231",
            Self::SaleOfDebtSecurity => "232",
            Self::SecuritiesSold => "233",
            Self::SaleOfEquitySecurity => "234",
            Self::MaturedReverseRepurchaseOrder => "235",
            Self::MaturityOfDebtSecurity => "236",
            Self::IndividualCollectionCredit => "237",
            Self::CollectionOfDividends => "238",
            Self::TotalBankersAcceptanceCredits => "239",
            Self::CouponCollectionsBanks => "240",
            Self::BankersAcceptancesCr => "241",
            Self::CollectionOfInterestIncome => "242",
            Self::MaturedFedFundsPurchased => "243",
            Self::InterestMaturedPrincipalPaymentCr => "244",
            Self::MonthlyDividends => "245",
            Self::CommercialPaperCr => "246",
            Self::CapitalChangeCr => "247",
            Self::SavingsBondsSalesAdjustmentCr => "248",
            Self::MiscellaneousSecurityCredit => "249",
            Self::TotalChecksPostedAndReturned => "250",
            Self::TotalDebitReversals => "251",
            Self::DebitReversal => "252",
            Self::PostingErrorCorrectionCredit => "254",
            Self::CheckPostedAndReturned => "255",
            Self::TotalAchReturnItemsCr => "256",
            Self::IndividualAchReturnItemCr => "257",
            Self::AchReversalCredit => "258",
            Self::TotalRejectedCredits => "260",
            Self::IndividualRejectedCredit => "261",
            Self::OverdraftCr => "263",
            Self::ReturnItemCr => "266",
            Self::ReturnItemAdjustmentCr => "268",
            Self::TotalZbaCredits => "270",
            Self::NetZeroBalanceAmount => "271",
            Self::CumulativeZbaOrDisbursementCredits => "274",
            Self::ZbaCredit => "275",
            Self::ZbaFloatAdjustment => "276",
            Self::ZbaCreditTransfer => "277",
            Self::ZbaCreditAdjustment => "278",
            Self::TotalControlledDisbursingCredits => "280",
            Self::IndividualControlledDisbursingCredit => "281",
            Self::TotalDtcDisbursingCredits => "285",
            Self::IndividualDtcDisbursingCredit => "286",
            Self::TotalAtmCredits => "294",
            Self::AtmCredit => "295",
            Self::CommercialDeposit => "301",
            Self::CorrespondentBankDeposit => "302",
            Self::TotalWireTransfersInFf => "303",
            Self::TotalWireTransfersInChf => "304",
            Self::TotalFedFundsSold => "305",
            Self::FedFundsSold => "306",
            Self::TotalTrustCredits => "307",
            Self::TrustCredit => "308",
            Self::TotalValueDatedFunds => "309",
            Self::TotalCommercialDeposits => "310",
            Self::TotalInternationalCreditsFf => "315",
            Self::TotalInternationalCreditsChf => "316",
            Self::TotalForeignCheckPurchased => "318",
            Self::LateDeposit => "319",
            Self::TotalSecuritiesSoldFf => "320",
            Self::TotalSecuritiesSoldChf => "321",
            Self::TotalSecuritiesMaturedFf => "324",
            Self::TotalSecuritiesMaturedChf => "325",
            Self::TotalSecuritiesInterest => "326",
            Self::TotalSecuritiesMatured => "327",
            Self::TotalSecuritiesInterestFf => "328",
            Self::TotalSecuritiesInterestChf => "329",
            Self::TotalEscrowCredits => "330",
            Self::IndividualEscrowCredit => "331",
            Self::TotalMiscellaneousSecuritiesCreditsFf => "332",
            Self::TotalMiscellaneousSecuritiesCreditsChf => "336",
            Self::TotalSecuritiesSold => "338",
            Self::TotalBrokerDeposits => "340",
            Self::TotalBrokerDepositsFf => "341",
            Self::BrokerDeposit => "342",
            Self::TotalBrokerDepositsChf => "343",
            Self::IndividualBackValueCredit => "344",
            Self::ItemInBrokersDeposit => "345",
            Self::SweepInterestIncome => "346",
            Self::SweepPrincipalSell => "347",
            Self::FuturesCredit => "348",
            Self::PrincipalPaymentsCredit => "349",
            Self::InvestmentSold => "350",
            Self::IndividualInvestmentSold => "351",
            Self::TotalCashCenterCredits => "352",
            Self::CashCenterCredit => "353",
            Self::InterestCredit => "354",
            Self::InvestmentInterest => "355",
            Self::TotalCreditAdjustment => "356",
            Self::CreditAdjustment => "357",
            Self::YtdAdjustmentCredit => "358",
            Self::InterestAdjustmentCredit => "359",
            Self::TotalCreditsLessWireTransferAndReturnedChecks => "360",
            Self::GrandTotalCreditsLessGrandTotalDebits => "361",
            Self::CorrespondentCollection => "362",
            Self::CorrespondentCollectionAdjustmentCr => "363",
            Self::LoanParticipationCr => "364",
            Self::CurrencyAndCoinDeposited => "366",
            Self::FoodStampLetterCr => "367",
            Self::FoodStampAdjustmentCr => "368",
            Self::ClearingSettlementCredit => "369",
            Self::TotalBackValueCredits => "370",
            Self::BackValueAdjustmentCr => "372",
            Self::CustomerPayrollCr => "373",
            Self::FrbStatementRecapCr => "374",
            Self::SavingsBondLetterOrAdjustmentCr => "376",
            Self::TreasuryTaxAndLoanCredit => "377",
            Self::TransferOfTreasuryCredit => "378",
            Self::FrbGovernmentChecksCashLetterCredit => "379",
            Self::FrbGovernmentCheckAdjustmentCr => "381",
            Self::FrbPostalMoneyOrderCredit => "382",
            Self::FrbPostalMoneyOrderAdjustmentCr => "383",
            Self::FrbCashLetterAutoChargeCredit => "384",
            Self::TotalUniversalCredits => "385",
            Self::FrbCashLetterAutoChargeAdjustmentCr => "386",
            Self::FrbFineSortCashLetterCredit => "387",
            Self::FrbFineSortAdjustmentCr => "388",
            Self::TotalFreightPaymentCredits => "389",
            Self::TotalMiscellaneousCredits => "390",
            Self::UniversalCredit => "391",
            Self::FreightPaymentCredit => "392",
            Self::ItemizedCreditOver10 => "393",
            Self::CumulativeCredits => "394",
            Self::CheckReversal => "395",
            Self::FloatAdjustmentCr => "397",
            Self::MiscellaneousFeeRefund => "398",
            Self::MiscellaneousCredit => "399",
            Self::TotalDebits => "400",
            Self::TotalDebitAmountMtd => "401",
            Self::TodaySTotalDebits => "403",
            Self::TotalDebitLessWireTransfersAndChargeBacks => "405",
            Self::DebitsNotDetailed => "406",
            Self::FloatAdjustmentDb => "408",
            Self::DebitAnyType => "409",
            Self::TotalYtdAdjustment => "410",
            Self::TotalDebitsExcludingReturnedItems => "412",
            Self::LockboxDebit => "415",
            Self::TotalLockboxDebits => "416",
            Self::EdiTransactionDebits => "420",
            Self::EdiTransactionDebit => "421",
            Self::EdibanxSettlementDebit => "422",
            Self::EdibanxReturnItemDebit => "423",
            Self::TotalPayableThroughDrafts => "430",
            Self::PayableThroughDraft => "435",
            Self::AchConcentrationDebit => "445",
            Self::TotalAchDisbursementFundingDebits => "446",
            Self::AchDisbursementFundingDebit => "447",
            Self::TotalAchDebits => "450",
            Self::AchDebitReceived => "451",
            Self::ItemInAchDisbursementOrDebit => "452",
            Self::PreauthorizedAchDebit => "455",
            Self::AccountHolderInitiatedAchDebit => "462",
            Self::CorporateTradePaymentDebits => "463",
            Self::CorporateTradePaymentDebit => "464",
            Self::CorporateTradePaymentSettlementDb => "465",
            Self::AchSettlementDb => "466",
            Self::AchSettlementDebits => "467",
            Self::AchReturnItemOrAdjustmentSettlementDb => "468",
            Self::MiscellaneousAchDebit => "469",
            Self::TotalCheckPaid => "470",
            Self::TotalCheckPaidCumulativeMtd => "471",
            Self::CumulativeChecksPaid => "472",
            Self::CertifiedCheckDebit => "474",
            Self::CheckPaid => "475",
            Self::FederalReserveBankLetterDebit => "476",
            Self::BankOriginatedDebit => "477",
            Self::ListPostDebits => "478",
            Self::ListPostDebit => "479",
            Self::TotalLoanPayments => "480",
            Self::IndividualLoanPayment => "481",
            Self::TotalBankOriginatedDebits => "482",
            Self::Draft => "484",
            Self::DtcDebit => "485",
            Self::TotalCashLetterDebits => "486",
            Self::CashLetterDebit => "487",
            Self::CashLetterAdjustmentDb => "489",
            Self::TotalOutgoingMoneyTransfers => "490",
            Self::IndividualOutgoingInternalMoneyTransfer => "491",
            Self::CustomerTerminalInitiatedMoneyTransfer => "493",
            Self::OutgoingMoneyTransfer => "495",
            Self::MoneyTransferAdjustmentDb => "496",
            Self::CompensationDb => "498",
            Self::TotalAutomaticTransferDebits => "500",
            Self::IndividualAutomaticTransferDebit => "501",
            Self::BondOperationsDebit => "502",
            Self::TotalBookTransferDebits => "505",
            Self::BookTransferDebit => "506",
            Self::TotalInternationalMoneyTransferDebits => "507",
            Self::IndividualInternationalMoneyTransferDebits => "508",
            Self::TotalInternationalDebits => "510",
            Self::LetterOfCreditDebit => "512",
            Self::LetterOfCreditDb => "513",
            Self::ForeignExchangeDebit => "514",
            Self::TotalLettersOfCreditDb => "515",
            Self::ForeignRemittanceDebit => "516",
            Self::ForeignCollectionDebit => "518",
            Self::ForeignChecksPaid => "522",
            Self::CommissionDb => "524",
            Self::InternationalMoneyMarketTradingDb => "526",
            Self::StandingOrderDb => "527",
            Self::MiscellaneousInternationalDebit => "529",
            Self::TotalSecurityDebits => "530",
            Self::SecuritiesPurchased => "531",
            Self::TotalAmountOfSecuritiesPurchased => "532",
            Self::SecurityCollectionDebit => "533",
            Self::TotalMiscellaneousSecuritiesDbFf => "534",
            Self::PurchaseOfEquitySecurities => "535",
            Self::TotalMiscellaneousSecuritiesDebitChf => "536",
            Self::TotalCollectionDebit => "537",
            Self::MaturedRepurchaseOrder => "538",
            Self::TotalBankersAcceptancesDebit => "539",
            Self::CouponCollectionDebit => "540",
            Self::BankersAcceptancesDb => "541",
            Self::PurchaseOfDebtSecurities => "542",
            Self::DomesticCollection => "543",
            Self::InterestMaturedPrincipalPaymentDb => "544",
            Self::CommercialPaperDb => "546",
            Self::CapitalChangeDb => "547",
            Self::SavingsBondsSalesAdjustmentDb => "548",
            Self::MiscellaneousSecurityDebit => "549",
            Self::TotalDepositedItemsReturned => "550",
            Self::TotalCreditReversals => "551",
            Self::CreditReversal => "552",
            Self::PostingErrorCorrectionDebit => "554",
            Self::DepositedItemReturned => "555",
            Self::TotalAchReturnItemsDb => "556",
            Self::IndividualAchReturnItemDb => "557",
            Self::AchReversalDebit => "558",
            Self::TotalRejectedDebits => "560",
            Self::IndividualRejectedDebit => "561",
            Self::OverdraftDb => "563",
            Self::OverdraftFee => "564",
            Self::ReturnItemDb => "566",
            Self::ReturnItemFee => "567",
            Self::ReturnItemAdjustmentDb => "568",
            Self::TotalZbaDebits => "570",
            Self::CumulativeZbaDebits => "574",
            Self::ZbaDebit => "575",
            Self::ZbaDebitTransfer => "577",
            Self::ZbaDebitAdjustment => "578",
            Self::TotalControlledDisbursingDebits => "580",
            Self::IndividualControlledDisbursingDebit => "581",
            Self::TotalDisbursingChecksPaidEarlyAmount => "583",
            Self::TotalDisbursingChecksPaidLaterAmount => "584",
            Self::DisbursingFundingRequirement => "585",
            Self::FrbPresentmentEstimateFedEstimate => "586",
            Self::LateDebitsAfterNotification => "587",
            Self::TotalDisbursingChecksPaidLastAmount => "588",
            Self::TotalDtcDebits => "590",
            Self::TotalAtmDebits => "594",
            Self::AtmDebit => "595",
            Self::TotalAprDebits => "596",
            Self::ArpDebit => "597",
            Self::EstimatedTotalDisbursement => "601",
            Self::AdjustedTotalDisbursement => "602",
            Self::TotalFundsRequired => "610",
            Self::TotalWireTransfersOutChf => "611",
            Self::TotalWireTransfersOutFf => "612",
            Self::TotalInternationalDebitChf => "613",
            Self::TotalInternationalDebitFf => "614",
            Self::TotalFederalReserveBankCommercialBankDebit => "615",
            Self::FederalReserveBankCommercialBankDebit => "616",
            Self::TotalSecuritiesPurchasedChf => "617",
            Self::TotalSecuritiesPurchasedFf => "618",
            Self::TotalBrokerDebitsChf => "621",
            Self::BrokerDebit => "622",
            Self::TotalBrokerDebitsFf => "623",
            Self::TotalBrokerDebits => "625",
            Self::TotalFedFundsPurchased => "626",
            Self::FedFundsPurchased => "627",
            Self::TotalCashCenterDebits => "628",
            Self::CashCenterDebit => "629",
            Self::TotalDebitAdjustments => "630",
            Self::DebitAdjustment => "631",
            Self::TotalTrustDebits => "632",
            Self::TrustDebit => "633",
            Self::YtdAdjustmentDebit => "634",
            Self::TotalEscrowDebits => "640",
            Self::IndividualEscrowDebit => "641",
            Self::IndividualBackValueDebit => "644",
            Self::TransferCalculationDebit => "646",
            Self::InvestmentsPurchased => "650",
            Self::IndividualInvestmentPurchased => "651",
            Self::InterestDebit => "654",
            Self::TotalInvestmentInterestDebits => "655",
            Self::SweepPrincipalBuy => "656",
            Self::FuturesDebit => "657",
            Self::PrincipalPaymentsDebit => "658",
            Self::InterestAdjustmentDebit => "659",
            Self::AccountAnalysisFee => "661",
            Self::CorrespondentCollectionDebit => "662",
            Self::CorrespondentCollectionAdjustmentDb => "663",
            Self::LoanParticipationDb => "664",
            Self::InterceptDebits => "665",
            Self::CurrencyAndCoinShipped => "666",
            Self::FoodStampLetterDb => "667",
            Self::FoodStampAdjustmentDb => "668",
            Self::ClearingSettlementDebit => "669",
            Self::TotalBackValueDebits => "670",
            Self::BackValueAdjustmentDb => "672",
            Self::CustomerPayrollDb => "673",
            Self::FrbStatementRecapDb => "674",
            Self::SavingsBondLetterOrAdjustmentDb => "676",
            Self::TreasuryTaxAndLoanDebit => "677",
            Self::TransferOfTreasuryDebit => "678",
            Self::FrbGovernmentChecksCashLetterDebit => "679",
            Self::FrbGovernmentCheckAdjustmentDb => "681",
            Self::FrbPostalMoneyOrderDebit => "682",
            Self::FrbPostalMoneyOrderAdjustmentDb => "683",
            Self::FrbCashLetterAutoChargeDebit => "684",
            Self::TotalUniversalDebits => "685",
            Self::FrbCashLetterAutoChargeAdjustmentDb => "686",
            Self::FrbFineSortCashLetterDebit => "687",
            Self::FrbFineSortAdjustmentDb => "688",
            Self::FrbFreightPaymentDebits => "689",
            Self::TotalMiscellaneousDebits => "690",
            Self::UniversalDebit => "691",
            Self::FreightPaymentDebit => "692",
            Self::ItemizedDebitOver10 => "693",
            Self::DepositReversal => "694",
            Self::DepositCorrectionDebit => "695",
            Self::RegularCollectionDebit => "696",
            Self::CumulativeDebits => "697",
            Self::MiscellaneousFees => "698",
            Self::MiscellaneousDebit => "699",
            Self::PrincipalLoanBalance => "701",
            Self::AvailableCommitmentAmount => "703",
            Self::PaymentAmountDue => "705",
            Self::PrincipalAmountPastDue => "707",
            Self::InterestAmountPastDue => "709",
            Self::TotalLoanPayment => "720",
            Self::AmountAppliedToInterest => "721",
            Self::AmountAppliedToPrincipal => "722",
            Self::AmountAppliedToEscrow => "723",
            Self::AmountAppliedToLateCharges => "724",
            Self::AmountAppliedToBuydown => "725",
            Self::AmountAppliedToMiscFees => "726",
            Self::AmountAppliedToDeferredInterestDetail => "727",
            Self::AmountAppliedToServiceCharge => "728",
            Self::LoanDisbursement => "760",
            Self::ContainsNonMonetaryInformation => "890",
            Self::Unknown => "",
        }
    }

    /// The human-readable description from the BAI2 specification.
    pub fn description(self) -> &'static str {
        match self {
            Self::OpeningLedger => "Opening Ledger",
            Self::AverageOpeningLedgerMtd => "Average Opening Ledger MTD",
            Self::AverageOpeningLedgerYtd => "Average Opening Ledger YTD",
            Self::ClosingLedger => "Closing Ledger",
            Self::AverageClosingLedgerMtd => "Average Closing Ledger MTD",
            Self::AverageClosingLedgerPreviousMonth => "Average Closing Ledger - Previous Month",
            Self::AggregateBalanceAdjustments => "Aggregate Balance Adjustments",
            Self::AverageClosingLedgerYtdPreviousMonth => {
                "Average Closing Ledger YTD - Previous Month"
            }
            Self::AverageClosingLedgerYtd => "Average Closing Ledger YTD",
            Self::CurrentLedger => "Current Ledger",
            Self::AchNetPosition => "ACH Net Position",
            Self::OpeningAvailableTotalSameDayAchDtcDeposit => {
                "Opening Available + Total Same-Day ACH DTC Deposit"
            }
            Self::OpeningAvailable => "Opening Available",
            Self::AverageOpeningAvailableMtd => "Average Opening Available MTD",
            Self::AverageOpeningAvailableYtd => "Average Opening Available YTD",
            Self::AverageAvailablePreviousMonth => "Average Available - Previous Month",
            Self::DisbursingOpeningAvailableBalance => "Disbursing Opening Available Balance",
            Self::ClosingAvailable => "Closing Available",
            Self::AverageClosingAvailableMtd => "Average Closing Available MTD",
            Self::AverageClosingAvailableLastMonth => "Average Closing Available - Last Month",
            Self::AverageClosingAvailableYtdLastMonth => {
                "Average Closing Available YTD - Last Month"
            }
            Self::AverageClosingAvailableYtd => "Average Closing Available YTD",
            Self::LoanBalance => "Loan Balance",
            Self::TotalInvestmentPosition => "Total Investment Position",
            Self::CurrentAvailableCrsSupressed => "Current Available (CRS Supressed)",
            Self::CurrentAvailable => "Current Available",
            Self::AverageCurrentAvailableMtd => "Average Current Available MTD",
            Self::AverageCurrentAvailableYtd => "Average Current Available YTD",
            Self::TotalFloat => "Total Float",
            Self::TargetBalance => "Target Balance",
            Self::AdjustedBalance => "Adjusted Balance",
            Self::AdjustedBalanceMtd => "Adjusted Balance MTD",
            Self::AdjustedBalanceYtd => "Adjusted Balance YTD",
            Self::N0DayFloat => "0-Day Float",
            Self::N1DayFloat => "1-Day Float",
            Self::FloatAdjustmentNa => "Float Adjustment",
            Self::N2OrMoreDaysFloat => "2 or More Days Float",
            Self::N3OrMoreDaysFloat => "3 or More Days Float",
            Self::AdjustmentToBalances => "Adjustment to Balances",
            Self::AverageAdjustmentToBalancesMtd => "Average Adjustment to Balances MTD",
            Self::AverageAdjustmentToBalancesYtd => "Average Adjustment to Balances YTD",
            Self::N4DayFloat => "4-Day Float",
            Self::N5DayFloat => "5-Day Float",
            Self::N6DayFloat => "6-Day Float",
            Self::Average1DayFloatMtd => "Average 1-Day Float MTD",
            Self::Average1DayFloatYtd => "Average 1-Day Float YTD",
            Self::Average2DayFloatMtd => "Average 2-Day Float MTD",
            Self::Average2DayFloatYtd => "Average 2-Day Float YTD",
            Self::TransferCalculation => "Transfer Calculation",
            Self::TotalCredits => "Total Credits",
            Self::TotalCreditAmountMtd => "Total Credit Amount MTD",
            Self::CreditsNotDetailed => "Credits Not Detailed",
            Self::DepositsSubjectToFloat => "Deposits Subject to Float",
            Self::TotalAdjustmentCreditsYtd => "Total Adjustment Credits YTD",
            Self::CreditAnyType => "Credit (Any Type)",
            Self::CurrentDayTotalLockboxDeposits => "Current Day Total Lockbox Deposits",
            Self::TotalLockboxDeposits => "Total Lockbox Deposits",
            Self::LockboxDeposit => "Lockbox Deposit",
            Self::ItemInLockboxDeposit => "Item in Lockbox Deposit",
            Self::LockboxAdjustmentCredit => "Lockbox Adjustment Credit",
            Self::EdiTransactionCreditCrSummary => "EDI Transaction Credit",
            Self::EdiTransactionCreditCrDetail => "EDI Transaction Credit",
            Self::EdibanxCreditReceived => "EDIBANX Credit Received",
            Self::EdibanxCreditReturn => "EDIBANX Credit Return",
            Self::TotalConcentrationCredits => "Total Concentration Credits",
            Self::TotalDtcCredits => "Total DTC Credits",
            Self::DtcConcentrationCredit => "DTC Concentration Credit",
            Self::ItemInDtcDeposit => "Item in DTC Deposit",
            Self::TotalAchCredits => "Total ACH Credits",
            Self::AchCreditReceived => "ACH Credit Received",
            Self::ItemInAchDeposit => "Item in ACH Deposit",
            Self::AchConcentrationCredit => "ACH Concentration Credit",
            Self::TotalBankCardDeposits => "Total Bank Card Deposits",
            Self::IndividualBankCardDeposit => "Individual Bank Card Deposit",
            Self::TotalPreauthorizedPaymentCredits => "Total Preauthorized Payment Credits",
            Self::PreauthorizedDraftCredit => "Preauthorized Draft Credit",
            Self::ItemInPacDeposit => "Item in PAC Deposit",
            Self::TotalAchDisbursingFundingCredits => "Total ACH Disbursing Funding Credits",
            Self::CorporateTradePaymentSettlementCr => "Corporate Trade Payment Settlement",
            Self::CorporateTradePaymentCredits => "Corporate Trade Payment Credits",
            Self::CorporateTradePaymentCredit => "Corporate Trade Payment Credit",
            Self::PreauthorizedAchCredit => "Preauthorized ACH Credit",
            Self::AchSettlementCr => "ACH Settlement",
            Self::AchSettlementCredits => "ACH Settlement Credits",
            Self::AchReturnItemOrAdjustmentSettlementCr => {
                "ACH Return Item or Adjustment Settlement"
            }
            Self::MiscellaneousAchCredit => "Miscellaneous ACH Credit",
            Self::TotalOtherCheckDeposits => "Total Other Check Deposits",
            Self::IndividualLoanDeposit => "Individual Loan Deposit",
            Self::DepositCorrection => "Deposit Correction",
            Self::BankPreparedDeposit => "Bank-Prepared Deposit",
            Self::OtherDeposit => "Other Deposit",
            Self::CheckDepositPackage => "Check Deposit Package",
            Self::RePresentedCheckDeposit => "Re-presented Check Deposit",
            Self::ListPostCredits => "List Post Credits",
            Self::TotalLoanProceeds => "Total Loan Proceeds",
            Self::TotalBankPreparedDeposits => "Total Bank-Prepared Deposits",
            Self::DraftDeposit => "Draft Deposit",
            Self::TotalMiscellaneousDeposits => "Total Miscellaneous Deposits",
            Self::TotalCashLetterCredits => "Total Cash Letter Credits",
            Self::CashLetterCredit => "Cash Letter Credit",
            Self::TotalCashLetterAdjustments => "Total Cash Letter Adjustments",
            Self::CashLetterAdjustmentCr => "Cash Letter Adjustment",
            Self::TotalIncomingMoneyTransfers => "Total Incoming Money Transfers",
            Self::IndividualIncomingInternalMoneyTransfer => {
                "Individual Incoming Internal Money Transfer"
            }
            Self::IncomingMoneyTransfer => "Incoming Money Transfer",
            Self::MoneyTransferAdjustmentCr => "Money Transfer Adjustment",
            Self::CompensationCr => "Compensation",
            Self::TotalAutomaticTransferCredits => "Total Automatic Transfer Credits",
            Self::IndividualAutomaticTransferCredit => "Individual Automatic Transfer Credit",
            Self::BondOperationsCredit => "Bond Operations Credit",
            Self::TotalBookTransferCredits => "Total Book Transfer Credits",
            Self::BookTransferCredit => "Book Transfer Credit",
            Self::TotalInternationalMoneyTransferCredits => {
                "Total International Money Transfer Credits"
            }
            Self::IndividualInternationalMoneyTransferCredit => {
                "Individual International Money Transfer Credit"
            }
            Self::TotalInternationalCredits => "Total International Credits",
            Self::ForeignLetterOfCredit => "Foreign Letter of Credit",
            Self::LetterOfCreditCr => "Letter of Credit",
            Self::ForeignExchangeOfCredit => "Foreign Exchange of Credit",
            Self::TotalLettersOfCreditCr => "Total Letters of Credit",
            Self::ForeignRemittanceCredit => "Foreign Remittance Credit",
            Self::ForeignCollectionCredit => "Foreign Collection Credit",
            Self::ForeignCheckPurchase => "Foreign Check Purchase",
            Self::ForeignChecksDeposited => "Foreign Checks Deposited",
            Self::CommissionCr => "Commission",
            Self::InternationalMoneyMarketTradingCr => "International Money Market Trading",
            Self::StandingOrderCr => "Standing Order",
            Self::MiscellaneousInternationalCredit => "Miscellaneous International Credit",
            Self::TotalSecurityCredits => "Total Security Credits",
            Self::TotalCollectionCredits => "Total Collection Credits",
            Self::SaleOfDebtSecurity => "Sale of Debt Security",
            Self::SecuritiesSold => "Securities Sold",
            Self::SaleOfEquitySecurity => "Sale of Equity Security",
            Self::MaturedReverseRepurchaseOrder => "Matured Reverse Repurchase Order",
            Self::MaturityOfDebtSecurity => "Maturity of Debt Security",
            Self::IndividualCollectionCredit => "Individual Collection Credit",
            Self::CollectionOfDividends => "Collection of Dividends",
            Self::TotalBankersAcceptanceCredits => "Total Bankers' Acceptance Credits",
            Self::CouponCollectionsBanks => "Coupon Collections - Banks",
            Self::BankersAcceptancesCr => "Bankers' Acceptances",
            Self::CollectionOfInterestIncome => "Collection of Interest Income",
            Self::MaturedFedFundsPurchased => "Matured Fed Funds Purchased",
            Self::InterestMaturedPrincipalPaymentCr => "Interest/Matured Principal Payment",
            Self::MonthlyDividends => "Monthly Dividends",
            Self::CommercialPaperCr => "Commercial Paper",
            Self::CapitalChangeCr => "Capital Change",
            Self::SavingsBondsSalesAdjustmentCr => "Savings Bonds Sales Adjustment",
            Self::MiscellaneousSecurityCredit => "Miscellaneous Security Credit",
            Self::TotalChecksPostedAndReturned => "Total Checks Posted and Returned",
            Self::TotalDebitReversals => "Total Debit Reversals",
            Self::DebitReversal => "Debit Reversal",
            Self::PostingErrorCorrectionCredit => "Posting Error Correction Credit",
            Self::CheckPostedAndReturned => "Check Posted and Returned",
            Self::TotalAchReturnItemsCr => "Total ACH Return Items",
            Self::IndividualAchReturnItemCr => "Individual ACH Return Item",
            Self::AchReversalCredit => "ACH Reversal Credit",
            Self::TotalRejectedCredits => "Total Rejected Credits",
            Self::IndividualRejectedCredit => "Individual Rejected Credit",
            Self::OverdraftCr => "Overdraft",
            Self::ReturnItemCr => "Return Item",
            Self::ReturnItemAdjustmentCr => "Return Item Adjustment",
            Self::TotalZbaCredits => "Total ZBA Credits",
            Self::NetZeroBalanceAmount => "Net Zero-Balance Amount",
            Self::CumulativeZbaOrDisbursementCredits => "Cumulative ZBA or Disbursement Credits",
            Self::ZbaCredit => "ZBA Credit",
            Self::ZbaFloatAdjustment => "ZBA Float Adjustment",
            Self::ZbaCreditTransfer => "ZBA Credit Transfer",
            Self::ZbaCreditAdjustment => "ZBA Credit Adjustment",
            Self::TotalControlledDisbursingCredits => "Total Controlled Disbursing Credits",
            Self::IndividualControlledDisbursingCredit => "Individual Controlled Disbursing Credit",
            Self::TotalDtcDisbursingCredits => "Total DTC Disbursing Credits",
            Self::IndividualDtcDisbursingCredit => "Individual DTC Disbursing Credit",
            Self::TotalAtmCredits => "Total ATM Credits",
            Self::AtmCredit => "ATM Credit",
            Self::CommercialDeposit => "Commercial Deposit",
            Self::CorrespondentBankDeposit => "Correspondent Bank Deposit",
            Self::TotalWireTransfersInFf => "Total Wire Transfers In - FF",
            Self::TotalWireTransfersInChf => "Total Wire Transfers In - CHF",
            Self::TotalFedFundsSold => "Total Fed Funds Sold",
            Self::FedFundsSold => "Fed Funds Sold",
            Self::TotalTrustCredits => "Total Trust Credits",
            Self::TrustCredit => "Trust Credit",
            Self::TotalValueDatedFunds => "Total Value - Dated Funds",
            Self::TotalCommercialDeposits => "Total Commercial Deposits",
            Self::TotalInternationalCreditsFf => "Total International Credits - FF",
            Self::TotalInternationalCreditsChf => "Total International Credits - CHF",
            Self::TotalForeignCheckPurchased => "Total Foreign Check Purchased",
            Self::LateDeposit => "Late Deposit",
            Self::TotalSecuritiesSoldFf => "Total Securities Sold - FF",
            Self::TotalSecuritiesSoldChf => "Total Securities Sold - CHF",
            Self::TotalSecuritiesMaturedFf => "Total Securities Matured - FF",
            Self::TotalSecuritiesMaturedChf => "Total Securities Matured - CHF",
            Self::TotalSecuritiesInterest => "Total Securities Interest",
            Self::TotalSecuritiesMatured => "Total Securities Matured",
            Self::TotalSecuritiesInterestFf => "Total Securities Interest - FF",
            Self::TotalSecuritiesInterestChf => "Total Securities Interest - CHF",
            Self::TotalEscrowCredits => "Total Escrow Credits",
            Self::IndividualEscrowCredit => "Individual Escrow Credit",
            Self::TotalMiscellaneousSecuritiesCreditsFf => {
                "Total Miscellaneous Securities Credits - FF"
            }
            Self::TotalMiscellaneousSecuritiesCreditsChf => {
                "Total Miscellaneous Securities Credits - CHF"
            }
            Self::TotalSecuritiesSold => "Total Securities Sold",
            Self::TotalBrokerDeposits => "Total Broker Deposits",
            Self::TotalBrokerDepositsFf => "Total Broker Deposits - FF",
            Self::BrokerDeposit => "Broker Deposit",
            Self::TotalBrokerDepositsChf => "Total Broker Deposits - CHF",
            Self::IndividualBackValueCredit => "Individual Back Value Credit",
            Self::ItemInBrokersDeposit => "Item in Brokers Deposit",
            Self::SweepInterestIncome => "Sweep Interest Income",
            Self::SweepPrincipalSell => "Sweep Principal Sell",
            Self::FuturesCredit => "Futures Credit",
            Self::PrincipalPaymentsCredit => "Principal Payments Credit",
            Self::InvestmentSold => "Investment Sold",
            Self::IndividualInvestmentSold => "Individual Investment Sold",
            Self::TotalCashCenterCredits => "Total Cash Center Credits",
            Self::CashCenterCredit => "Cash Center Credit",
            Self::InterestCredit => "Interest Credit",
            Self::InvestmentInterest => "Investment Interest",
            Self::TotalCreditAdjustment => "Total Credit Adjustment",
            Self::CreditAdjustment => "Credit Adjustment",
            Self::YtdAdjustmentCredit => "YTD Adjustment Credit",
            Self::InterestAdjustmentCredit => "Interest Adjustment Credit",
            Self::TotalCreditsLessWireTransferAndReturnedChecks => {
                "Total Credits Less Wire Transfer and Returned Checks"
            }
            Self::GrandTotalCreditsLessGrandTotalDebits => {
                "Grand Total Credits Less Grand Total Debits"
            }
            Self::CorrespondentCollection => "Correspondent Collection",
            Self::CorrespondentCollectionAdjustmentCr => "Correspondent Collection Adjustment",
            Self::LoanParticipationCr => "Loan Participation",
            Self::CurrencyAndCoinDeposited => "Currency and Coin Deposited",
            Self::FoodStampLetterCr => "Food Stamp Letter",
            Self::FoodStampAdjustmentCr => "Food Stamp Adjustment",
            Self::ClearingSettlementCredit => "Clearing Settlement Credit",
            Self::TotalBackValueCredits => "Total Back Value Credits",
            Self::BackValueAdjustmentCr => "Back Value Adjustment",
            Self::CustomerPayrollCr => "Customer Payroll",
            Self::FrbStatementRecapCr => "FRB Statement Recap",
            Self::SavingsBondLetterOrAdjustmentCr => "Savings Bond Letter or Adjustment",
            Self::TreasuryTaxAndLoanCredit => "Treasury Tax and Loan Credit",
            Self::TransferOfTreasuryCredit => "Transfer of Treasury Credit",
            Self::FrbGovernmentChecksCashLetterCredit => "FRB Government Checks Cash Letter Credit",
            Self::FrbGovernmentCheckAdjustmentCr => "FRB Government Check Adjustment",
            Self::FrbPostalMoneyOrderCredit => "FRB Postal Money Order Credit",
            Self::FrbPostalMoneyOrderAdjustmentCr => "FRB Postal Money Order Adjustment",
            Self::FrbCashLetterAutoChargeCredit => "FRB Cash Letter Auto Charge Credit",
            Self::TotalUniversalCredits => "Total Universal Credits",
            Self::FrbCashLetterAutoChargeAdjustmentCr => "FRB Cash Letter Auto Charge Adjustment",
            Self::FrbFineSortCashLetterCredit => "FRB Fine-Sort Cash Letter Credit",
            Self::FrbFineSortAdjustmentCr => "FRB Fine-Sort Adjustment",
            Self::TotalFreightPaymentCredits => "Total Freight Payment Credits",
            Self::TotalMiscellaneousCredits => "Total Miscellaneous Credits",
            Self::UniversalCredit => "Universal Credit",
            Self::FreightPaymentCredit => "Freight Payment Credit",
            Self::ItemizedCreditOver10 => "Itemized Credit Over $10",
            Self::CumulativeCredits => "Cumulative Credits",
            Self::CheckReversal => "Check Reversal",
            Self::FloatAdjustmentCr => "Float Adjustment",
            Self::MiscellaneousFeeRefund => "Miscellaneous Fee Refund",
            Self::MiscellaneousCredit => "Miscellaneous Credit",
            Self::TotalDebits => "Total Debits",
            Self::TotalDebitAmountMtd => "Total Debit Amount MTD",
            Self::TodaySTotalDebits => "Today's Total Debits",
            Self::TotalDebitLessWireTransfersAndChargeBacks => {
                "Total Debit Less Wire Transfers and Charge-Backs"
            }
            Self::DebitsNotDetailed => "Debits not Detailed",
            Self::FloatAdjustmentDb => "Float Adjustment",
            Self::DebitAnyType => "Debit (Any Type)",
            Self::TotalYtdAdjustment => "Total YTD Adjustment",
            Self::TotalDebitsExcludingReturnedItems => "Total Debits (Excluding Returned Items)",
            Self::LockboxDebit => "Lockbox Debit",
            Self::TotalLockboxDebits => "Total Lockbox Debits",
            Self::EdiTransactionDebits => "EDI Transaction Debits",
            Self::EdiTransactionDebit => "EDI Transaction Debit",
            Self::EdibanxSettlementDebit => "EDIBANX Settlement Debit",
            Self::EdibanxReturnItemDebit => "EDIBANX Return Item Debit",
            Self::TotalPayableThroughDrafts => "Total Payable-Through Drafts",
            Self::PayableThroughDraft => "Payable-Through Draft",
            Self::AchConcentrationDebit => "ACH Concentration Debit",
            Self::TotalAchDisbursementFundingDebits => "Total ACH Disbursement Funding Debits",
            Self::AchDisbursementFundingDebit => "ACH Disbursement Funding Debit",
            Self::TotalAchDebits => "Total ACH Debits",
            Self::AchDebitReceived => "ACH Debit Received",
            Self::ItemInAchDisbursementOrDebit => "Item in ACH Disbursement or Debit",
            Self::PreauthorizedAchDebit => "Preauthorized ACH Debit",
            Self::AccountHolderInitiatedAchDebit => "Account Holder Initiated ACH Debit",
            Self::CorporateTradePaymentDebits => "Corporate Trade Payment Debits",
            Self::CorporateTradePaymentDebit => "Corporate Trade Payment Debit",
            Self::CorporateTradePaymentSettlementDb => "Corporate Trade Payment Settlement",
            Self::AchSettlementDb => "ACH Settlement",
            Self::AchSettlementDebits => "ACH Settlement Debits",
            Self::AchReturnItemOrAdjustmentSettlementDb => {
                "ACH Return Item or Adjustment Settlement"
            }
            Self::MiscellaneousAchDebit => "Miscellaneous ACH Debit",
            Self::TotalCheckPaid => "Total Check Paid",
            Self::TotalCheckPaidCumulativeMtd => "Total Check Paid - Cumulative MTD",
            Self::CumulativeChecksPaid => "Cumulative Checks Paid",
            Self::CertifiedCheckDebit => "Certified Check Debit",
            Self::CheckPaid => "Check Paid",
            Self::FederalReserveBankLetterDebit => "Federal Reserve Bank Letter Debit",
            Self::BankOriginatedDebit => "Bank Originated Debit",
            Self::ListPostDebits => "List Post Debits",
            Self::ListPostDebit => "List Post Debit",
            Self::TotalLoanPayments => "Total Loan Payments",
            Self::IndividualLoanPayment => "Individual Loan Payment",
            Self::TotalBankOriginatedDebits => "Total Bank-Originated Debits",
            Self::Draft => "Draft",
            Self::DtcDebit => "DTC Debit",
            Self::TotalCashLetterDebits => "Total Cash Letter Debits",
            Self::CashLetterDebit => "Cash Letter Debit",
            Self::CashLetterAdjustmentDb => "Cash Letter Adjustment",
            Self::TotalOutgoingMoneyTransfers => "Total Outgoing Money Transfers",
            Self::IndividualOutgoingInternalMoneyTransfer => {
                "Individual Outgoing Internal Money Transfer"
            }
            Self::CustomerTerminalInitiatedMoneyTransfer => {
                "Customer Terminal Initiated Money Transfer"
            }
            Self::OutgoingMoneyTransfer => "Outgoing Money Transfer",
            Self::MoneyTransferAdjustmentDb => "Money Transfer Adjustment",
            Self::CompensationDb => "Compensation",
            Self::TotalAutomaticTransferDebits => "Total Automatic Transfer Debits",
            Self::IndividualAutomaticTransferDebit => "Individual Automatic Transfer Debit",
            Self::BondOperationsDebit => "Bond Operations Debit",
            Self::TotalBookTransferDebits => "Total Book Transfer Debits",
            Self::BookTransferDebit => "Book Transfer Debit",
            Self::TotalInternationalMoneyTransferDebits => {
                "Total International Money Transfer Debits"
            }
            Self::IndividualInternationalMoneyTransferDebits => {
                "Individual International Money Transfer Debits"
            }
            Self::TotalInternationalDebits => "Total International Debits",
            Self::LetterOfCreditDebit => "Letter of Credit Debit",
            Self::LetterOfCreditDb => "Letter of Credit",
            Self::ForeignExchangeDebit => "Foreign Exchange Debit",
            Self::TotalLettersOfCreditDb => "Total Letters of Credit",
            Self::ForeignRemittanceDebit => "Foreign Remittance Debit",
            Self::ForeignCollectionDebit => "Foreign Collection Debit",
            Self::ForeignChecksPaid => "Foreign Checks Paid",
            Self::CommissionDb => "Commission",
            Self::InternationalMoneyMarketTradingDb => "International Money Market Trading",
            Self::StandingOrderDb => "Standing Order",
            Self::MiscellaneousInternationalDebit => "Miscellaneous International Debit",
            Self::TotalSecurityDebits => "Total Security Debits",
            Self::SecuritiesPurchased => "Securities Purchased",
            Self::TotalAmountOfSecuritiesPurchased => "Total Amount of Securities Purchased",
            Self::SecurityCollectionDebit => "Security Collection Debit",
            Self::TotalMiscellaneousSecuritiesDbFf => "Total Miscellaneous Securities DB - FF",
            Self::PurchaseOfEquitySecurities => "Purchase of Equity Securities",
            Self::TotalMiscellaneousSecuritiesDebitChf => {
                "Total Miscellaneous Securities Debit - CHF"
            }
            Self::TotalCollectionDebit => "Total Collection Debit",
            Self::MaturedRepurchaseOrder => "Matured Repurchase Order",
            Self::TotalBankersAcceptancesDebit => "Total Bankers' Acceptances Debit",
            Self::CouponCollectionDebit => "Coupon Collection Debit",
            Self::BankersAcceptancesDb => "Bankers' Acceptances",
            Self::PurchaseOfDebtSecurities => "Purchase of Debt Securities",
            Self::DomesticCollection => "Domestic Collection",
            Self::InterestMaturedPrincipalPaymentDb => "Interest/Matured Principal Payment",
            Self::CommercialPaperDb => "Commercial paper",
            Self::CapitalChangeDb => "Capital Change",
            Self::SavingsBondsSalesAdjustmentDb => "Savings Bonds Sales Adjustment",
            Self::MiscellaneousSecurityDebit => "Miscellaneous Security Debit",
            Self::TotalDepositedItemsReturned => "Total Deposited Items Returned",
            Self::TotalCreditReversals => "Total Credit Reversals",
            Self::CreditReversal => "Credit Reversal",
            Self::PostingErrorCorrectionDebit => "Posting Error Correction Debit",
            Self::DepositedItemReturned => "Deposited Item Returned",
            Self::TotalAchReturnItemsDb => "Total ACH Return Items",
            Self::IndividualAchReturnItemDb => "Individual ACH Return Item",
            Self::AchReversalDebit => "ACH Reversal Debit",
            Self::TotalRejectedDebits => "Total Rejected Debits",
            Self::IndividualRejectedDebit => "Individual Rejected Debit",
            Self::OverdraftDb => "Overdraft",
            Self::OverdraftFee => "Overdraft Fee",
            Self::ReturnItemDb => "Return Item",
            Self::ReturnItemFee => "Return Item Fee",
            Self::ReturnItemAdjustmentDb => "Return Item Adjustment",
            Self::TotalZbaDebits => "Total ZBA Debits",
            Self::CumulativeZbaDebits => "Cumulative ZBA Debits",
            Self::ZbaDebit => "ZBA Debit",
            Self::ZbaDebitTransfer => "ZBA Debit Transfer",
            Self::ZbaDebitAdjustment => "ZBA Debit Adjustment",
            Self::TotalControlledDisbursingDebits => "Total Controlled Disbursing Debits",
            Self::IndividualControlledDisbursingDebit => "Individual Controlled Disbursing Debit",
            Self::TotalDisbursingChecksPaidEarlyAmount => {
                "Total Disbursing Checks Paid - Early Amount"
            }
            Self::TotalDisbursingChecksPaidLaterAmount => {
                "Total Disbursing Checks Paid - Later Amount"
            }
            Self::DisbursingFundingRequirement => "Disbursing Funding Requirement",
            Self::FrbPresentmentEstimateFedEstimate => "FRB Presentment Estimate (Fed Estimate)",
            Self::LateDebitsAfterNotification => "Late Debits (After Notification)",
            Self::TotalDisbursingChecksPaidLastAmount => "Total Disbursing Checks Paid-Last Amount",
            Self::TotalDtcDebits => "Total DTC Debits",
            Self::TotalAtmDebits => "Total ATM Debits",
            Self::AtmDebit => "ATM Debit",
            Self::TotalAprDebits => "Total APR Debits",
            Self::ArpDebit => "ARP Debit",
            Self::EstimatedTotalDisbursement => "Estimated Total Disbursement",
            Self::AdjustedTotalDisbursement => "Adjusted Total Disbursement",
            Self::TotalFundsRequired => "Total Funds Required",
            Self::TotalWireTransfersOutChf => "Total Wire Transfers Out- CHF",
            Self::TotalWireTransfersOutFf => "Total Wire Transfers Out - FF",
            Self::TotalInternationalDebitChf => "Total International Debit - CHF",
            Self::TotalInternationalDebitFf => "Total International Debit - FF",
            Self::TotalFederalReserveBankCommercialBankDebit => {
                "Total Federal Reserve Bank - Commercial Bank Debit"
            }
            Self::FederalReserveBankCommercialBankDebit => {
                "Federal Reserve Bank - Commercial Bank Debit"
            }
            Self::TotalSecuritiesPurchasedChf => "Total Securities Purchased - CHF",
            Self::TotalSecuritiesPurchasedFf => "Total Securities Purchased - FF",
            Self::TotalBrokerDebitsChf => "Total Broker Debits - CHF",
            Self::BrokerDebit => "Broker Debit",
            Self::TotalBrokerDebitsFf => "Total Broker Debits - FF",
            Self::TotalBrokerDebits => "Total Broker Debits",
            Self::TotalFedFundsPurchased => "Total Fed Funds Purchased",
            Self::FedFundsPurchased => "Fed Funds Purchased",
            Self::TotalCashCenterDebits => "Total Cash Center Debits",
            Self::CashCenterDebit => "Cash Center Debit",
            Self::TotalDebitAdjustments => "Total Debit Adjustments",
            Self::DebitAdjustment => "Debit Adjustment",
            Self::TotalTrustDebits => "Total Trust Debits",
            Self::TrustDebit => "Trust Debit",
            Self::YtdAdjustmentDebit => "YTD Adjustment Debit",
            Self::TotalEscrowDebits => "Total Escrow Debits",
            Self::IndividualEscrowDebit => "Individual Escrow Debit",
            Self::IndividualBackValueDebit => "Individual Back Value Debit",
            Self::TransferCalculationDebit => "Transfer Calculation Debit",
            Self::InvestmentsPurchased => "Investments Purchased",
            Self::IndividualInvestmentPurchased => "Individual Investment purchased",
            Self::InterestDebit => "Interest Debit",
            Self::TotalInvestmentInterestDebits => "Total Investment Interest Debits",
            Self::SweepPrincipalBuy => "Sweep Principal Buy",
            Self::FuturesDebit => "Futures Debit",
            Self::PrincipalPaymentsDebit => "Principal Payments Debit",
            Self::InterestAdjustmentDebit => "Interest Adjustment Debit",
            Self::AccountAnalysisFee => "Account Analysis Fee",
            Self::CorrespondentCollectionDebit => "Correspondent Collection Debit",
            Self::CorrespondentCollectionAdjustmentDb => "Correspondent Collection Adjustment",
            Self::LoanParticipationDb => "Loan Participation",
            Self::InterceptDebits => "Intercept Debits",
            Self::CurrencyAndCoinShipped => "Currency and Coin Shipped",
            Self::FoodStampLetterDb => "Food Stamp Letter",
            Self::FoodStampAdjustmentDb => "Food Stamp Adjustment",
            Self::ClearingSettlementDebit => "Clearing Settlement Debit",
            Self::TotalBackValueDebits => "Total Back Value Debits",
            Self::BackValueAdjustmentDb => "Back Value Adjustment",
            Self::CustomerPayrollDb => "Customer Payroll",
            Self::FrbStatementRecapDb => "FRB Statement Recap",
            Self::SavingsBondLetterOrAdjustmentDb => "Savings Bond Letter or Adjustment",
            Self::TreasuryTaxAndLoanDebit => "Treasury Tax and Loan Debit",
            Self::TransferOfTreasuryDebit => "Transfer of Treasury Debit",
            Self::FrbGovernmentChecksCashLetterDebit => "FRB Government Checks Cash Letter Debit",
            Self::FrbGovernmentCheckAdjustmentDb => "FRB Government Check Adjustment",
            Self::FrbPostalMoneyOrderDebit => "FRB Postal Money Order Debit",
            Self::FrbPostalMoneyOrderAdjustmentDb => "FRB Postal Money Order Adjustment",
            Self::FrbCashLetterAutoChargeDebit => "FRB Cash Letter Auto Charge Debit",
            Self::TotalUniversalDebits => "Total Universal Debits",
            Self::FrbCashLetterAutoChargeAdjustmentDb => "FRB Cash Letter Auto Charge Adjustment",
            Self::FrbFineSortCashLetterDebit => "FRB Fine-Sort Cash Letter Debit",
            Self::FrbFineSortAdjustmentDb => "FRB Fine-Sort Adjustment",
            Self::FrbFreightPaymentDebits => "FRB Freight Payment Debits",
            Self::TotalMiscellaneousDebits => "Total Miscellaneous Debits",
            Self::UniversalDebit => "Universal Debit",
            Self::FreightPaymentDebit => "Freight Payment Debit",
            Self::ItemizedDebitOver10 => "Itemized Debit Over $10",
            Self::DepositReversal => "Deposit Reversal",
            Self::DepositCorrectionDebit => "Deposit Correction Debit",
            Self::RegularCollectionDebit => "Regular Collection Debit",
            Self::CumulativeDebits => "Cumulative Debits",
            Self::MiscellaneousFees => "Miscellaneous Fees",
            Self::MiscellaneousDebit => "Miscellaneous Debit",
            Self::PrincipalLoanBalance => "Principal Loan Balance",
            Self::AvailableCommitmentAmount => "Available Commitment Amount",
            Self::PaymentAmountDue => "Payment Amount Due",
            Self::PrincipalAmountPastDue => "Principal Amount Past Due",
            Self::InterestAmountPastDue => "Interest Amount Past Due",
            Self::TotalLoanPayment => "Total Loan Payment",
            Self::AmountAppliedToInterest => "Amount Applied to Interest",
            Self::AmountAppliedToPrincipal => "Amount Applied to Principal",
            Self::AmountAppliedToEscrow => "Amount Applied to Escrow",
            Self::AmountAppliedToLateCharges => "Amount Applied to Late Charges",
            Self::AmountAppliedToBuydown => "Amount Applied to Buydown",
            Self::AmountAppliedToMiscFees => "Amount Applied to Misc. Fees",
            Self::AmountAppliedToDeferredInterestDetail => {
                "Amount Applied to Deferred Interest Detail"
            }
            Self::AmountAppliedToServiceCharge => "Amount Applied to Service Charge",
            Self::LoanDisbursement => "Loan Disbursement",
            Self::ContainsNonMonetaryInformation => "Contains Non-monetary Information",
            Self::Unknown => "Unknown type code",
        }
    }

    /// The debit/credit classification, or `None` for `Unknown`.
    pub fn transaction(self) -> Option<Transaction> {
        match self {
            Self::OpeningLedger => Some(Transaction::Na),
            Self::AverageOpeningLedgerMtd => Some(Transaction::Na),
            Self::AverageOpeningLedgerYtd => Some(Transaction::Na),
            Self::ClosingLedger => Some(Transaction::Na),
            Self::AverageClosingLedgerMtd => Some(Transaction::Na),
            Self::AverageClosingLedgerPreviousMonth => Some(Transaction::Na),
            Self::AggregateBalanceAdjustments => Some(Transaction::Na),
            Self::AverageClosingLedgerYtdPreviousMonth => Some(Transaction::Na),
            Self::AverageClosingLedgerYtd => Some(Transaction::Na),
            Self::CurrentLedger => Some(Transaction::Na),
            Self::AchNetPosition => Some(Transaction::Na),
            Self::OpeningAvailableTotalSameDayAchDtcDeposit => Some(Transaction::Na),
            Self::OpeningAvailable => Some(Transaction::Na),
            Self::AverageOpeningAvailableMtd => Some(Transaction::Na),
            Self::AverageOpeningAvailableYtd => Some(Transaction::Na),
            Self::AverageAvailablePreviousMonth => Some(Transaction::Na),
            Self::DisbursingOpeningAvailableBalance => Some(Transaction::Na),
            Self::ClosingAvailable => Some(Transaction::Na),
            Self::AverageClosingAvailableMtd => Some(Transaction::Na),
            Self::AverageClosingAvailableLastMonth => Some(Transaction::Na),
            Self::AverageClosingAvailableYtdLastMonth => Some(Transaction::Na),
            Self::AverageClosingAvailableYtd => Some(Transaction::Na),
            Self::LoanBalance => Some(Transaction::Na),
            Self::TotalInvestmentPosition => Some(Transaction::Na),
            Self::CurrentAvailableCrsSupressed => Some(Transaction::Na),
            Self::CurrentAvailable => Some(Transaction::Na),
            Self::AverageCurrentAvailableMtd => Some(Transaction::Na),
            Self::AverageCurrentAvailableYtd => Some(Transaction::Na),
            Self::TotalFloat => Some(Transaction::Na),
            Self::TargetBalance => Some(Transaction::Na),
            Self::AdjustedBalance => Some(Transaction::Na),
            Self::AdjustedBalanceMtd => Some(Transaction::Na),
            Self::AdjustedBalanceYtd => Some(Transaction::Na),
            Self::N0DayFloat => Some(Transaction::Na),
            Self::N1DayFloat => Some(Transaction::Na),
            Self::FloatAdjustmentNa => Some(Transaction::Na),
            Self::N2OrMoreDaysFloat => Some(Transaction::Na),
            Self::N3OrMoreDaysFloat => Some(Transaction::Na),
            Self::AdjustmentToBalances => Some(Transaction::Na),
            Self::AverageAdjustmentToBalancesMtd => Some(Transaction::Na),
            Self::AverageAdjustmentToBalancesYtd => Some(Transaction::Na),
            Self::N4DayFloat => Some(Transaction::Na),
            Self::N5DayFloat => Some(Transaction::Na),
            Self::N6DayFloat => Some(Transaction::Na),
            Self::Average1DayFloatMtd => Some(Transaction::Na),
            Self::Average1DayFloatYtd => Some(Transaction::Na),
            Self::Average2DayFloatMtd => Some(Transaction::Na),
            Self::Average2DayFloatYtd => Some(Transaction::Na),
            Self::TransferCalculation => Some(Transaction::Na),
            Self::TotalCredits => Some(Transaction::Cr),
            Self::TotalCreditAmountMtd => Some(Transaction::Cr),
            Self::CreditsNotDetailed => Some(Transaction::Cr),
            Self::DepositsSubjectToFloat => Some(Transaction::Cr),
            Self::TotalAdjustmentCreditsYtd => Some(Transaction::Cr),
            Self::CreditAnyType => Some(Transaction::Cr),
            Self::CurrentDayTotalLockboxDeposits => Some(Transaction::Cr),
            Self::TotalLockboxDeposits => Some(Transaction::Cr),
            Self::LockboxDeposit => Some(Transaction::Cr),
            Self::ItemInLockboxDeposit => Some(Transaction::Cr),
            Self::LockboxAdjustmentCredit => Some(Transaction::Cr),
            Self::EdiTransactionCreditCrSummary => Some(Transaction::Cr),
            Self::EdiTransactionCreditCrDetail => Some(Transaction::Cr),
            Self::EdibanxCreditReceived => Some(Transaction::Cr),
            Self::EdibanxCreditReturn => Some(Transaction::Cr),
            Self::TotalConcentrationCredits => Some(Transaction::Cr),
            Self::TotalDtcCredits => Some(Transaction::Cr),
            Self::DtcConcentrationCredit => Some(Transaction::Cr),
            Self::ItemInDtcDeposit => Some(Transaction::Cr),
            Self::TotalAchCredits => Some(Transaction::Cr),
            Self::AchCreditReceived => Some(Transaction::Cr),
            Self::ItemInAchDeposit => Some(Transaction::Cr),
            Self::AchConcentrationCredit => Some(Transaction::Cr),
            Self::TotalBankCardDeposits => Some(Transaction::Cr),
            Self::IndividualBankCardDeposit => Some(Transaction::Cr),
            Self::TotalPreauthorizedPaymentCredits => Some(Transaction::Cr),
            Self::PreauthorizedDraftCredit => Some(Transaction::Cr),
            Self::ItemInPacDeposit => Some(Transaction::Cr),
            Self::TotalAchDisbursingFundingCredits => Some(Transaction::Cr),
            Self::CorporateTradePaymentSettlementCr => Some(Transaction::Cr),
            Self::CorporateTradePaymentCredits => Some(Transaction::Cr),
            Self::CorporateTradePaymentCredit => Some(Transaction::Cr),
            Self::PreauthorizedAchCredit => Some(Transaction::Cr),
            Self::AchSettlementCr => Some(Transaction::Cr),
            Self::AchSettlementCredits => Some(Transaction::Cr),
            Self::AchReturnItemOrAdjustmentSettlementCr => Some(Transaction::Cr),
            Self::MiscellaneousAchCredit => Some(Transaction::Cr),
            Self::TotalOtherCheckDeposits => Some(Transaction::Cr),
            Self::IndividualLoanDeposit => Some(Transaction::Cr),
            Self::DepositCorrection => Some(Transaction::Cr),
            Self::BankPreparedDeposit => Some(Transaction::Cr),
            Self::OtherDeposit => Some(Transaction::Cr),
            Self::CheckDepositPackage => Some(Transaction::Cr),
            Self::RePresentedCheckDeposit => Some(Transaction::Cr),
            Self::ListPostCredits => Some(Transaction::Cr),
            Self::TotalLoanProceeds => Some(Transaction::Cr),
            Self::TotalBankPreparedDeposits => Some(Transaction::Cr),
            Self::DraftDeposit => Some(Transaction::Cr),
            Self::TotalMiscellaneousDeposits => Some(Transaction::Cr),
            Self::TotalCashLetterCredits => Some(Transaction::Cr),
            Self::CashLetterCredit => Some(Transaction::Cr),
            Self::TotalCashLetterAdjustments => Some(Transaction::Cr),
            Self::CashLetterAdjustmentCr => Some(Transaction::Cr),
            Self::TotalIncomingMoneyTransfers => Some(Transaction::Cr),
            Self::IndividualIncomingInternalMoneyTransfer => Some(Transaction::Cr),
            Self::IncomingMoneyTransfer => Some(Transaction::Cr),
            Self::MoneyTransferAdjustmentCr => Some(Transaction::Cr),
            Self::CompensationCr => Some(Transaction::Cr),
            Self::TotalAutomaticTransferCredits => Some(Transaction::Cr),
            Self::IndividualAutomaticTransferCredit => Some(Transaction::Cr),
            Self::BondOperationsCredit => Some(Transaction::Cr),
            Self::TotalBookTransferCredits => Some(Transaction::Cr),
            Self::BookTransferCredit => Some(Transaction::Cr),
            Self::TotalInternationalMoneyTransferCredits => Some(Transaction::Cr),
            Self::IndividualInternationalMoneyTransferCredit => Some(Transaction::Cr),
            Self::TotalInternationalCredits => Some(Transaction::Cr),
            Self::ForeignLetterOfCredit => Some(Transaction::Cr),
            Self::LetterOfCreditCr => Some(Transaction::Cr),
            Self::ForeignExchangeOfCredit => Some(Transaction::Cr),
            Self::TotalLettersOfCreditCr => Some(Transaction::Cr),
            Self::ForeignRemittanceCredit => Some(Transaction::Cr),
            Self::ForeignCollectionCredit => Some(Transaction::Cr),
            Self::ForeignCheckPurchase => Some(Transaction::Cr),
            Self::ForeignChecksDeposited => Some(Transaction::Cr),
            Self::CommissionCr => Some(Transaction::Cr),
            Self::InternationalMoneyMarketTradingCr => Some(Transaction::Cr),
            Self::StandingOrderCr => Some(Transaction::Cr),
            Self::MiscellaneousInternationalCredit => Some(Transaction::Cr),
            Self::TotalSecurityCredits => Some(Transaction::Cr),
            Self::TotalCollectionCredits => Some(Transaction::Cr),
            Self::SaleOfDebtSecurity => Some(Transaction::Cr),
            Self::SecuritiesSold => Some(Transaction::Cr),
            Self::SaleOfEquitySecurity => Some(Transaction::Cr),
            Self::MaturedReverseRepurchaseOrder => Some(Transaction::Cr),
            Self::MaturityOfDebtSecurity => Some(Transaction::Cr),
            Self::IndividualCollectionCredit => Some(Transaction::Cr),
            Self::CollectionOfDividends => Some(Transaction::Cr),
            Self::TotalBankersAcceptanceCredits => Some(Transaction::Cr),
            Self::CouponCollectionsBanks => Some(Transaction::Cr),
            Self::BankersAcceptancesCr => Some(Transaction::Cr),
            Self::CollectionOfInterestIncome => Some(Transaction::Cr),
            Self::MaturedFedFundsPurchased => Some(Transaction::Cr),
            Self::InterestMaturedPrincipalPaymentCr => Some(Transaction::Cr),
            Self::MonthlyDividends => Some(Transaction::Cr),
            Self::CommercialPaperCr => Some(Transaction::Cr),
            Self::CapitalChangeCr => Some(Transaction::Cr),
            Self::SavingsBondsSalesAdjustmentCr => Some(Transaction::Cr),
            Self::MiscellaneousSecurityCredit => Some(Transaction::Cr),
            Self::TotalChecksPostedAndReturned => Some(Transaction::Cr),
            Self::TotalDebitReversals => Some(Transaction::Cr),
            Self::DebitReversal => Some(Transaction::Cr),
            Self::PostingErrorCorrectionCredit => Some(Transaction::Cr),
            Self::CheckPostedAndReturned => Some(Transaction::Cr),
            Self::TotalAchReturnItemsCr => Some(Transaction::Cr),
            Self::IndividualAchReturnItemCr => Some(Transaction::Cr),
            Self::AchReversalCredit => Some(Transaction::Cr),
            Self::TotalRejectedCredits => Some(Transaction::Cr),
            Self::IndividualRejectedCredit => Some(Transaction::Cr),
            Self::OverdraftCr => Some(Transaction::Cr),
            Self::ReturnItemCr => Some(Transaction::Cr),
            Self::ReturnItemAdjustmentCr => Some(Transaction::Cr),
            Self::TotalZbaCredits => Some(Transaction::Cr),
            Self::NetZeroBalanceAmount => Some(Transaction::Cr),
            Self::CumulativeZbaOrDisbursementCredits => Some(Transaction::Cr),
            Self::ZbaCredit => Some(Transaction::Cr),
            Self::ZbaFloatAdjustment => Some(Transaction::Cr),
            Self::ZbaCreditTransfer => Some(Transaction::Cr),
            Self::ZbaCreditAdjustment => Some(Transaction::Cr),
            Self::TotalControlledDisbursingCredits => Some(Transaction::Cr),
            Self::IndividualControlledDisbursingCredit => Some(Transaction::Cr),
            Self::TotalDtcDisbursingCredits => Some(Transaction::Cr),
            Self::IndividualDtcDisbursingCredit => Some(Transaction::Cr),
            Self::TotalAtmCredits => Some(Transaction::Cr),
            Self::AtmCredit => Some(Transaction::Cr),
            Self::CommercialDeposit => Some(Transaction::Cr),
            Self::CorrespondentBankDeposit => Some(Transaction::Cr),
            Self::TotalWireTransfersInFf => Some(Transaction::Cr),
            Self::TotalWireTransfersInChf => Some(Transaction::Cr),
            Self::TotalFedFundsSold => Some(Transaction::Cr),
            Self::FedFundsSold => Some(Transaction::Cr),
            Self::TotalTrustCredits => Some(Transaction::Cr),
            Self::TrustCredit => Some(Transaction::Cr),
            Self::TotalValueDatedFunds => Some(Transaction::Cr),
            Self::TotalCommercialDeposits => Some(Transaction::Cr),
            Self::TotalInternationalCreditsFf => Some(Transaction::Cr),
            Self::TotalInternationalCreditsChf => Some(Transaction::Cr),
            Self::TotalForeignCheckPurchased => Some(Transaction::Cr),
            Self::LateDeposit => Some(Transaction::Cr),
            Self::TotalSecuritiesSoldFf => Some(Transaction::Cr),
            Self::TotalSecuritiesSoldChf => Some(Transaction::Cr),
            Self::TotalSecuritiesMaturedFf => Some(Transaction::Cr),
            Self::TotalSecuritiesMaturedChf => Some(Transaction::Cr),
            Self::TotalSecuritiesInterest => Some(Transaction::Cr),
            Self::TotalSecuritiesMatured => Some(Transaction::Cr),
            Self::TotalSecuritiesInterestFf => Some(Transaction::Cr),
            Self::TotalSecuritiesInterestChf => Some(Transaction::Cr),
            Self::TotalEscrowCredits => Some(Transaction::Cr),
            Self::IndividualEscrowCredit => Some(Transaction::Cr),
            Self::TotalMiscellaneousSecuritiesCreditsFf => Some(Transaction::Cr),
            Self::TotalMiscellaneousSecuritiesCreditsChf => Some(Transaction::Cr),
            Self::TotalSecuritiesSold => Some(Transaction::Cr),
            Self::TotalBrokerDeposits => Some(Transaction::Cr),
            Self::TotalBrokerDepositsFf => Some(Transaction::Cr),
            Self::BrokerDeposit => Some(Transaction::Cr),
            Self::TotalBrokerDepositsChf => Some(Transaction::Cr),
            Self::IndividualBackValueCredit => Some(Transaction::Cr),
            Self::ItemInBrokersDeposit => Some(Transaction::Cr),
            Self::SweepInterestIncome => Some(Transaction::Cr),
            Self::SweepPrincipalSell => Some(Transaction::Cr),
            Self::FuturesCredit => Some(Transaction::Cr),
            Self::PrincipalPaymentsCredit => Some(Transaction::Cr),
            Self::InvestmentSold => Some(Transaction::Cr),
            Self::IndividualInvestmentSold => Some(Transaction::Cr),
            Self::TotalCashCenterCredits => Some(Transaction::Cr),
            Self::CashCenterCredit => Some(Transaction::Cr),
            Self::InterestCredit => Some(Transaction::Cr),
            Self::InvestmentInterest => Some(Transaction::Cr),
            Self::TotalCreditAdjustment => Some(Transaction::Cr),
            Self::CreditAdjustment => Some(Transaction::Cr),
            Self::YtdAdjustmentCredit => Some(Transaction::Cr),
            Self::InterestAdjustmentCredit => Some(Transaction::Cr),
            Self::TotalCreditsLessWireTransferAndReturnedChecks => Some(Transaction::Cr),
            Self::GrandTotalCreditsLessGrandTotalDebits => Some(Transaction::Cr),
            Self::CorrespondentCollection => Some(Transaction::Cr),
            Self::CorrespondentCollectionAdjustmentCr => Some(Transaction::Cr),
            Self::LoanParticipationCr => Some(Transaction::Cr),
            Self::CurrencyAndCoinDeposited => Some(Transaction::Cr),
            Self::FoodStampLetterCr => Some(Transaction::Cr),
            Self::FoodStampAdjustmentCr => Some(Transaction::Cr),
            Self::ClearingSettlementCredit => Some(Transaction::Cr),
            Self::TotalBackValueCredits => Some(Transaction::Cr),
            Self::BackValueAdjustmentCr => Some(Transaction::Cr),
            Self::CustomerPayrollCr => Some(Transaction::Cr),
            Self::FrbStatementRecapCr => Some(Transaction::Cr),
            Self::SavingsBondLetterOrAdjustmentCr => Some(Transaction::Cr),
            Self::TreasuryTaxAndLoanCredit => Some(Transaction::Cr),
            Self::TransferOfTreasuryCredit => Some(Transaction::Cr),
            Self::FrbGovernmentChecksCashLetterCredit => Some(Transaction::Cr),
            Self::FrbGovernmentCheckAdjustmentCr => Some(Transaction::Cr),
            Self::FrbPostalMoneyOrderCredit => Some(Transaction::Cr),
            Self::FrbPostalMoneyOrderAdjustmentCr => Some(Transaction::Cr),
            Self::FrbCashLetterAutoChargeCredit => Some(Transaction::Cr),
            Self::TotalUniversalCredits => Some(Transaction::Cr),
            Self::FrbCashLetterAutoChargeAdjustmentCr => Some(Transaction::Cr),
            Self::FrbFineSortCashLetterCredit => Some(Transaction::Cr),
            Self::FrbFineSortAdjustmentCr => Some(Transaction::Cr),
            Self::TotalFreightPaymentCredits => Some(Transaction::Cr),
            Self::TotalMiscellaneousCredits => Some(Transaction::Cr),
            Self::UniversalCredit => Some(Transaction::Cr),
            Self::FreightPaymentCredit => Some(Transaction::Cr),
            Self::ItemizedCreditOver10 => Some(Transaction::Cr),
            Self::CumulativeCredits => Some(Transaction::Cr),
            Self::CheckReversal => Some(Transaction::Cr),
            Self::FloatAdjustmentCr => Some(Transaction::Cr),
            Self::MiscellaneousFeeRefund => Some(Transaction::Cr),
            Self::MiscellaneousCredit => Some(Transaction::Cr),
            Self::TotalDebits => Some(Transaction::Db),
            Self::TotalDebitAmountMtd => Some(Transaction::Db),
            Self::TodaySTotalDebits => Some(Transaction::Db),
            Self::TotalDebitLessWireTransfersAndChargeBacks => Some(Transaction::Db),
            Self::DebitsNotDetailed => Some(Transaction::Db),
            Self::FloatAdjustmentDb => Some(Transaction::Db),
            Self::DebitAnyType => Some(Transaction::Db),
            Self::TotalYtdAdjustment => Some(Transaction::Db),
            Self::TotalDebitsExcludingReturnedItems => Some(Transaction::Db),
            Self::LockboxDebit => Some(Transaction::Db),
            Self::TotalLockboxDebits => Some(Transaction::Db),
            Self::EdiTransactionDebits => Some(Transaction::Db),
            Self::EdiTransactionDebit => Some(Transaction::Db),
            Self::EdibanxSettlementDebit => Some(Transaction::Db),
            Self::EdibanxReturnItemDebit => Some(Transaction::Db),
            Self::TotalPayableThroughDrafts => Some(Transaction::Db),
            Self::PayableThroughDraft => Some(Transaction::Db),
            Self::AchConcentrationDebit => Some(Transaction::Db),
            Self::TotalAchDisbursementFundingDebits => Some(Transaction::Db),
            Self::AchDisbursementFundingDebit => Some(Transaction::Db),
            Self::TotalAchDebits => Some(Transaction::Db),
            Self::AchDebitReceived => Some(Transaction::Db),
            Self::ItemInAchDisbursementOrDebit => Some(Transaction::Db),
            Self::PreauthorizedAchDebit => Some(Transaction::Db),
            Self::AccountHolderInitiatedAchDebit => Some(Transaction::Db),
            Self::CorporateTradePaymentDebits => Some(Transaction::Db),
            Self::CorporateTradePaymentDebit => Some(Transaction::Db),
            Self::CorporateTradePaymentSettlementDb => Some(Transaction::Db),
            Self::AchSettlementDb => Some(Transaction::Db),
            Self::AchSettlementDebits => Some(Transaction::Db),
            Self::AchReturnItemOrAdjustmentSettlementDb => Some(Transaction::Db),
            Self::MiscellaneousAchDebit => Some(Transaction::Db),
            Self::TotalCheckPaid => Some(Transaction::Db),
            Self::TotalCheckPaidCumulativeMtd => Some(Transaction::Db),
            Self::CumulativeChecksPaid => Some(Transaction::Db),
            Self::CertifiedCheckDebit => Some(Transaction::Db),
            Self::CheckPaid => Some(Transaction::Db),
            Self::FederalReserveBankLetterDebit => Some(Transaction::Db),
            Self::BankOriginatedDebit => Some(Transaction::Db),
            Self::ListPostDebits => Some(Transaction::Db),
            Self::ListPostDebit => Some(Transaction::Db),
            Self::TotalLoanPayments => Some(Transaction::Db),
            Self::IndividualLoanPayment => Some(Transaction::Db),
            Self::TotalBankOriginatedDebits => Some(Transaction::Db),
            Self::Draft => Some(Transaction::Db),
            Self::DtcDebit => Some(Transaction::Db),
            Self::TotalCashLetterDebits => Some(Transaction::Db),
            Self::CashLetterDebit => Some(Transaction::Db),
            Self::CashLetterAdjustmentDb => Some(Transaction::Db),
            Self::TotalOutgoingMoneyTransfers => Some(Transaction::Db),
            Self::IndividualOutgoingInternalMoneyTransfer => Some(Transaction::Db),
            Self::CustomerTerminalInitiatedMoneyTransfer => Some(Transaction::Db),
            Self::OutgoingMoneyTransfer => Some(Transaction::Db),
            Self::MoneyTransferAdjustmentDb => Some(Transaction::Db),
            Self::CompensationDb => Some(Transaction::Db),
            Self::TotalAutomaticTransferDebits => Some(Transaction::Db),
            Self::IndividualAutomaticTransferDebit => Some(Transaction::Db),
            Self::BondOperationsDebit => Some(Transaction::Db),
            Self::TotalBookTransferDebits => Some(Transaction::Db),
            Self::BookTransferDebit => Some(Transaction::Db),
            Self::TotalInternationalMoneyTransferDebits => Some(Transaction::Db),
            Self::IndividualInternationalMoneyTransferDebits => Some(Transaction::Db),
            Self::TotalInternationalDebits => Some(Transaction::Db),
            Self::LetterOfCreditDebit => Some(Transaction::Db),
            Self::LetterOfCreditDb => Some(Transaction::Db),
            Self::ForeignExchangeDebit => Some(Transaction::Db),
            Self::TotalLettersOfCreditDb => Some(Transaction::Db),
            Self::ForeignRemittanceDebit => Some(Transaction::Db),
            Self::ForeignCollectionDebit => Some(Transaction::Db),
            Self::ForeignChecksPaid => Some(Transaction::Db),
            Self::CommissionDb => Some(Transaction::Db),
            Self::InternationalMoneyMarketTradingDb => Some(Transaction::Db),
            Self::StandingOrderDb => Some(Transaction::Db),
            Self::MiscellaneousInternationalDebit => Some(Transaction::Db),
            Self::TotalSecurityDebits => Some(Transaction::Db),
            Self::SecuritiesPurchased => Some(Transaction::Db),
            Self::TotalAmountOfSecuritiesPurchased => Some(Transaction::Db),
            Self::SecurityCollectionDebit => Some(Transaction::Db),
            Self::TotalMiscellaneousSecuritiesDbFf => Some(Transaction::Db),
            Self::PurchaseOfEquitySecurities => Some(Transaction::Db),
            Self::TotalMiscellaneousSecuritiesDebitChf => Some(Transaction::Db),
            Self::TotalCollectionDebit => Some(Transaction::Db),
            Self::MaturedRepurchaseOrder => Some(Transaction::Db),
            Self::TotalBankersAcceptancesDebit => Some(Transaction::Db),
            Self::CouponCollectionDebit => Some(Transaction::Db),
            Self::BankersAcceptancesDb => Some(Transaction::Db),
            Self::PurchaseOfDebtSecurities => Some(Transaction::Db),
            Self::DomesticCollection => Some(Transaction::Db),
            Self::InterestMaturedPrincipalPaymentDb => Some(Transaction::Db),
            Self::CommercialPaperDb => Some(Transaction::Db),
            Self::CapitalChangeDb => Some(Transaction::Db),
            Self::SavingsBondsSalesAdjustmentDb => Some(Transaction::Db),
            Self::MiscellaneousSecurityDebit => Some(Transaction::Db),
            Self::TotalDepositedItemsReturned => Some(Transaction::Db),
            Self::TotalCreditReversals => Some(Transaction::Db),
            Self::CreditReversal => Some(Transaction::Db),
            Self::PostingErrorCorrectionDebit => Some(Transaction::Db),
            Self::DepositedItemReturned => Some(Transaction::Db),
            Self::TotalAchReturnItemsDb => Some(Transaction::Db),
            Self::IndividualAchReturnItemDb => Some(Transaction::Db),
            Self::AchReversalDebit => Some(Transaction::Db),
            Self::TotalRejectedDebits => Some(Transaction::Db),
            Self::IndividualRejectedDebit => Some(Transaction::Db),
            Self::OverdraftDb => Some(Transaction::Db),
            Self::OverdraftFee => Some(Transaction::Db),
            Self::ReturnItemDb => Some(Transaction::Db),
            Self::ReturnItemFee => Some(Transaction::Db),
            Self::ReturnItemAdjustmentDb => Some(Transaction::Db),
            Self::TotalZbaDebits => Some(Transaction::Db),
            Self::CumulativeZbaDebits => Some(Transaction::Db),
            Self::ZbaDebit => Some(Transaction::Db),
            Self::ZbaDebitTransfer => Some(Transaction::Db),
            Self::ZbaDebitAdjustment => Some(Transaction::Db),
            Self::TotalControlledDisbursingDebits => Some(Transaction::Db),
            Self::IndividualControlledDisbursingDebit => Some(Transaction::Db),
            Self::TotalDisbursingChecksPaidEarlyAmount => Some(Transaction::Db),
            Self::TotalDisbursingChecksPaidLaterAmount => Some(Transaction::Db),
            Self::DisbursingFundingRequirement => Some(Transaction::Db),
            Self::FrbPresentmentEstimateFedEstimate => Some(Transaction::Db),
            Self::LateDebitsAfterNotification => Some(Transaction::Db),
            Self::TotalDisbursingChecksPaidLastAmount => Some(Transaction::Db),
            Self::TotalDtcDebits => Some(Transaction::Db),
            Self::TotalAtmDebits => Some(Transaction::Db),
            Self::AtmDebit => Some(Transaction::Db),
            Self::TotalAprDebits => Some(Transaction::Db),
            Self::ArpDebit => Some(Transaction::Db),
            Self::EstimatedTotalDisbursement => Some(Transaction::Db),
            Self::AdjustedTotalDisbursement => Some(Transaction::Db),
            Self::TotalFundsRequired => Some(Transaction::Db),
            Self::TotalWireTransfersOutChf => Some(Transaction::Db),
            Self::TotalWireTransfersOutFf => Some(Transaction::Db),
            Self::TotalInternationalDebitChf => Some(Transaction::Db),
            Self::TotalInternationalDebitFf => Some(Transaction::Db),
            Self::TotalFederalReserveBankCommercialBankDebit => Some(Transaction::Db),
            Self::FederalReserveBankCommercialBankDebit => Some(Transaction::Db),
            Self::TotalSecuritiesPurchasedChf => Some(Transaction::Db),
            Self::TotalSecuritiesPurchasedFf => Some(Transaction::Db),
            Self::TotalBrokerDebitsChf => Some(Transaction::Db),
            Self::BrokerDebit => Some(Transaction::Db),
            Self::TotalBrokerDebitsFf => Some(Transaction::Db),
            Self::TotalBrokerDebits => Some(Transaction::Db),
            Self::TotalFedFundsPurchased => Some(Transaction::Db),
            Self::FedFundsPurchased => Some(Transaction::Db),
            Self::TotalCashCenterDebits => Some(Transaction::Db),
            Self::CashCenterDebit => Some(Transaction::Db),
            Self::TotalDebitAdjustments => Some(Transaction::Db),
            Self::DebitAdjustment => Some(Transaction::Db),
            Self::TotalTrustDebits => Some(Transaction::Db),
            Self::TrustDebit => Some(Transaction::Db),
            Self::YtdAdjustmentDebit => Some(Transaction::Db),
            Self::TotalEscrowDebits => Some(Transaction::Db),
            Self::IndividualEscrowDebit => Some(Transaction::Db),
            Self::IndividualBackValueDebit => Some(Transaction::Db),
            Self::TransferCalculationDebit => Some(Transaction::Db),
            Self::InvestmentsPurchased => Some(Transaction::Db),
            Self::IndividualInvestmentPurchased => Some(Transaction::Db),
            Self::InterestDebit => Some(Transaction::Db),
            Self::TotalInvestmentInterestDebits => Some(Transaction::Db),
            Self::SweepPrincipalBuy => Some(Transaction::Db),
            Self::FuturesDebit => Some(Transaction::Db),
            Self::PrincipalPaymentsDebit => Some(Transaction::Db),
            Self::InterestAdjustmentDebit => Some(Transaction::Db),
            Self::AccountAnalysisFee => Some(Transaction::Db),
            Self::CorrespondentCollectionDebit => Some(Transaction::Db),
            Self::CorrespondentCollectionAdjustmentDb => Some(Transaction::Db),
            Self::LoanParticipationDb => Some(Transaction::Db),
            Self::InterceptDebits => Some(Transaction::Db),
            Self::CurrencyAndCoinShipped => Some(Transaction::Db),
            Self::FoodStampLetterDb => Some(Transaction::Db),
            Self::FoodStampAdjustmentDb => Some(Transaction::Db),
            Self::ClearingSettlementDebit => Some(Transaction::Db),
            Self::TotalBackValueDebits => Some(Transaction::Db),
            Self::BackValueAdjustmentDb => Some(Transaction::Db),
            Self::CustomerPayrollDb => Some(Transaction::Db),
            Self::FrbStatementRecapDb => Some(Transaction::Db),
            Self::SavingsBondLetterOrAdjustmentDb => Some(Transaction::Db),
            Self::TreasuryTaxAndLoanDebit => Some(Transaction::Db),
            Self::TransferOfTreasuryDebit => Some(Transaction::Db),
            Self::FrbGovernmentChecksCashLetterDebit => Some(Transaction::Db),
            Self::FrbGovernmentCheckAdjustmentDb => Some(Transaction::Db),
            Self::FrbPostalMoneyOrderDebit => Some(Transaction::Db),
            Self::FrbPostalMoneyOrderAdjustmentDb => Some(Transaction::Db),
            Self::FrbCashLetterAutoChargeDebit => Some(Transaction::Db),
            Self::TotalUniversalDebits => Some(Transaction::Db),
            Self::FrbCashLetterAutoChargeAdjustmentDb => Some(Transaction::Db),
            Self::FrbFineSortCashLetterDebit => Some(Transaction::Db),
            Self::FrbFineSortAdjustmentDb => Some(Transaction::Db),
            Self::FrbFreightPaymentDebits => Some(Transaction::Db),
            Self::TotalMiscellaneousDebits => Some(Transaction::Db),
            Self::UniversalDebit => Some(Transaction::Db),
            Self::FreightPaymentDebit => Some(Transaction::Db),
            Self::ItemizedDebitOver10 => Some(Transaction::Db),
            Self::DepositReversal => Some(Transaction::Db),
            Self::DepositCorrectionDebit => Some(Transaction::Db),
            Self::RegularCollectionDebit => Some(Transaction::Db),
            Self::CumulativeDebits => Some(Transaction::Db),
            Self::MiscellaneousFees => Some(Transaction::Db),
            Self::MiscellaneousDebit => Some(Transaction::Db),
            Self::PrincipalLoanBalance => Some(Transaction::Na),
            Self::AvailableCommitmentAmount => Some(Transaction::Na),
            Self::PaymentAmountDue => Some(Transaction::Na),
            Self::PrincipalAmountPastDue => Some(Transaction::Na),
            Self::InterestAmountPastDue => Some(Transaction::Na),
            Self::TotalLoanPayment => Some(Transaction::Cr),
            Self::AmountAppliedToInterest => Some(Transaction::Cr),
            Self::AmountAppliedToPrincipal => Some(Transaction::Cr),
            Self::AmountAppliedToEscrow => Some(Transaction::Cr),
            Self::AmountAppliedToLateCharges => Some(Transaction::Cr),
            Self::AmountAppliedToBuydown => Some(Transaction::Cr),
            Self::AmountAppliedToMiscFees => Some(Transaction::Cr),
            Self::AmountAppliedToDeferredInterestDetail => Some(Transaction::Cr),
            Self::AmountAppliedToServiceCharge => Some(Transaction::Cr),
            Self::LoanDisbursement => Some(Transaction::Db),
            Self::ContainsNonMonetaryInformation => Some(Transaction::Na),
            Self::Unknown => None,
        }
    }

    /// The status/summary/detail classification, or `None` for `Unknown`.
    pub fn level(self) -> Option<Level> {
        match self {
            Self::OpeningLedger => Some(Level::Status),
            Self::AverageOpeningLedgerMtd => Some(Level::Status),
            Self::AverageOpeningLedgerYtd => Some(Level::Status),
            Self::ClosingLedger => Some(Level::Status),
            Self::AverageClosingLedgerMtd => Some(Level::Status),
            Self::AverageClosingLedgerPreviousMonth => Some(Level::Status),
            Self::AggregateBalanceAdjustments => Some(Level::Status),
            Self::AverageClosingLedgerYtdPreviousMonth => Some(Level::Status),
            Self::AverageClosingLedgerYtd => Some(Level::Status),
            Self::CurrentLedger => Some(Level::Status),
            Self::AchNetPosition => Some(Level::Status),
            Self::OpeningAvailableTotalSameDayAchDtcDeposit => Some(Level::Status),
            Self::OpeningAvailable => Some(Level::Status),
            Self::AverageOpeningAvailableMtd => Some(Level::Status),
            Self::AverageOpeningAvailableYtd => Some(Level::Status),
            Self::AverageAvailablePreviousMonth => Some(Level::Status),
            Self::DisbursingOpeningAvailableBalance => Some(Level::Status),
            Self::ClosingAvailable => Some(Level::Status),
            Self::AverageClosingAvailableMtd => Some(Level::Status),
            Self::AverageClosingAvailableLastMonth => Some(Level::Status),
            Self::AverageClosingAvailableYtdLastMonth => Some(Level::Status),
            Self::AverageClosingAvailableYtd => Some(Level::Status),
            Self::LoanBalance => Some(Level::Status),
            Self::TotalInvestmentPosition => Some(Level::Status),
            Self::CurrentAvailableCrsSupressed => Some(Level::Status),
            Self::CurrentAvailable => Some(Level::Status),
            Self::AverageCurrentAvailableMtd => Some(Level::Status),
            Self::AverageCurrentAvailableYtd => Some(Level::Status),
            Self::TotalFloat => Some(Level::Status),
            Self::TargetBalance => Some(Level::Status),
            Self::AdjustedBalance => Some(Level::Status),
            Self::AdjustedBalanceMtd => Some(Level::Status),
            Self::AdjustedBalanceYtd => Some(Level::Status),
            Self::N0DayFloat => Some(Level::Status),
            Self::N1DayFloat => Some(Level::Status),
            Self::FloatAdjustmentNa => Some(Level::Status),
            Self::N2OrMoreDaysFloat => Some(Level::Status),
            Self::N3OrMoreDaysFloat => Some(Level::Status),
            Self::AdjustmentToBalances => Some(Level::Status),
            Self::AverageAdjustmentToBalancesMtd => Some(Level::Status),
            Self::AverageAdjustmentToBalancesYtd => Some(Level::Status),
            Self::N4DayFloat => Some(Level::Status),
            Self::N5DayFloat => Some(Level::Status),
            Self::N6DayFloat => Some(Level::Status),
            Self::Average1DayFloatMtd => Some(Level::Status),
            Self::Average1DayFloatYtd => Some(Level::Status),
            Self::Average2DayFloatMtd => Some(Level::Status),
            Self::Average2DayFloatYtd => Some(Level::Status),
            Self::TransferCalculation => Some(Level::Status),
            Self::TotalCredits => Some(Level::Summary),
            Self::TotalCreditAmountMtd => Some(Level::Summary),
            Self::CreditsNotDetailed => Some(Level::Summary),
            Self::DepositsSubjectToFloat => Some(Level::Summary),
            Self::TotalAdjustmentCreditsYtd => Some(Level::Summary),
            Self::CreditAnyType => Some(Level::Detail),
            Self::CurrentDayTotalLockboxDeposits => Some(Level::Summary),
            Self::TotalLockboxDeposits => Some(Level::Summary),
            Self::LockboxDeposit => Some(Level::Detail),
            Self::ItemInLockboxDeposit => Some(Level::Detail),
            Self::LockboxAdjustmentCredit => Some(Level::Detail),
            Self::EdiTransactionCreditCrSummary => Some(Level::Summary),
            Self::EdiTransactionCreditCrDetail => Some(Level::Detail),
            Self::EdibanxCreditReceived => Some(Level::Detail),
            Self::EdibanxCreditReturn => Some(Level::Detail),
            Self::TotalConcentrationCredits => Some(Level::Summary),
            Self::TotalDtcCredits => Some(Level::Summary),
            Self::DtcConcentrationCredit => Some(Level::Detail),
            Self::ItemInDtcDeposit => Some(Level::Detail),
            Self::TotalAchCredits => Some(Level::Summary),
            Self::AchCreditReceived => Some(Level::Detail),
            Self::ItemInAchDeposit => Some(Level::Detail),
            Self::AchConcentrationCredit => Some(Level::Detail),
            Self::TotalBankCardDeposits => Some(Level::Summary),
            Self::IndividualBankCardDeposit => Some(Level::Detail),
            Self::TotalPreauthorizedPaymentCredits => Some(Level::Summary),
            Self::PreauthorizedDraftCredit => Some(Level::Detail),
            Self::ItemInPacDeposit => Some(Level::Detail),
            Self::TotalAchDisbursingFundingCredits => Some(Level::Summary),
            Self::CorporateTradePaymentSettlementCr => Some(Level::Summary),
            Self::CorporateTradePaymentCredits => Some(Level::Summary),
            Self::CorporateTradePaymentCredit => Some(Level::Detail),
            Self::PreauthorizedAchCredit => Some(Level::Detail),
            Self::AchSettlementCr => Some(Level::Detail),
            Self::AchSettlementCredits => Some(Level::Summary),
            Self::AchReturnItemOrAdjustmentSettlementCr => Some(Level::Detail),
            Self::MiscellaneousAchCredit => Some(Level::Detail),
            Self::TotalOtherCheckDeposits => Some(Level::Summary),
            Self::IndividualLoanDeposit => Some(Level::Detail),
            Self::DepositCorrection => Some(Level::Detail),
            Self::BankPreparedDeposit => Some(Level::Detail),
            Self::OtherDeposit => Some(Level::Detail),
            Self::CheckDepositPackage => Some(Level::Detail),
            Self::RePresentedCheckDeposit => Some(Level::Detail),
            Self::ListPostCredits => Some(Level::Summary),
            Self::TotalLoanProceeds => Some(Level::Summary),
            Self::TotalBankPreparedDeposits => Some(Level::Summary),
            Self::DraftDeposit => Some(Level::Detail),
            Self::TotalMiscellaneousDeposits => Some(Level::Summary),
            Self::TotalCashLetterCredits => Some(Level::Summary),
            Self::CashLetterCredit => Some(Level::Detail),
            Self::TotalCashLetterAdjustments => Some(Level::Summary),
            Self::CashLetterAdjustmentCr => Some(Level::Detail),
            Self::TotalIncomingMoneyTransfers => Some(Level::Summary),
            Self::IndividualIncomingInternalMoneyTransfer => Some(Level::Detail),
            Self::IncomingMoneyTransfer => Some(Level::Detail),
            Self::MoneyTransferAdjustmentCr => Some(Level::Detail),
            Self::CompensationCr => Some(Level::Detail),
            Self::TotalAutomaticTransferCredits => Some(Level::Summary),
            Self::IndividualAutomaticTransferCredit => Some(Level::Detail),
            Self::BondOperationsCredit => Some(Level::Detail),
            Self::TotalBookTransferCredits => Some(Level::Summary),
            Self::BookTransferCredit => Some(Level::Detail),
            Self::TotalInternationalMoneyTransferCredits => Some(Level::Summary),
            Self::IndividualInternationalMoneyTransferCredit => Some(Level::Detail),
            Self::TotalInternationalCredits => Some(Level::Summary),
            Self::ForeignLetterOfCredit => Some(Level::Detail),
            Self::LetterOfCreditCr => Some(Level::Detail),
            Self::ForeignExchangeOfCredit => Some(Level::Detail),
            Self::TotalLettersOfCreditCr => Some(Level::Summary),
            Self::ForeignRemittanceCredit => Some(Level::Detail),
            Self::ForeignCollectionCredit => Some(Level::Detail),
            Self::ForeignCheckPurchase => Some(Level::Detail),
            Self::ForeignChecksDeposited => Some(Level::Detail),
            Self::CommissionCr => Some(Level::Detail),
            Self::InternationalMoneyMarketTradingCr => Some(Level::Detail),
            Self::StandingOrderCr => Some(Level::Detail),
            Self::MiscellaneousInternationalCredit => Some(Level::Detail),
            Self::TotalSecurityCredits => Some(Level::Summary),
            Self::TotalCollectionCredits => Some(Level::Summary),
            Self::SaleOfDebtSecurity => Some(Level::Detail),
            Self::SecuritiesSold => Some(Level::Detail),
            Self::SaleOfEquitySecurity => Some(Level::Detail),
            Self::MaturedReverseRepurchaseOrder => Some(Level::Detail),
            Self::MaturityOfDebtSecurity => Some(Level::Detail),
            Self::IndividualCollectionCredit => Some(Level::Detail),
            Self::CollectionOfDividends => Some(Level::Detail),
            Self::TotalBankersAcceptanceCredits => Some(Level::Summary),
            Self::CouponCollectionsBanks => Some(Level::Detail),
            Self::BankersAcceptancesCr => Some(Level::Detail),
            Self::CollectionOfInterestIncome => Some(Level::Detail),
            Self::MaturedFedFundsPurchased => Some(Level::Detail),
            Self::InterestMaturedPrincipalPaymentCr => Some(Level::Detail),
            Self::MonthlyDividends => Some(Level::Summary),
            Self::CommercialPaperCr => Some(Level::Detail),
            Self::CapitalChangeCr => Some(Level::Detail),
            Self::SavingsBondsSalesAdjustmentCr => Some(Level::Detail),
            Self::MiscellaneousSecurityCredit => Some(Level::Detail),
            Self::TotalChecksPostedAndReturned => Some(Level::Summary),
            Self::TotalDebitReversals => Some(Level::Summary),
            Self::DebitReversal => Some(Level::Detail),
            Self::PostingErrorCorrectionCredit => Some(Level::Detail),
            Self::CheckPostedAndReturned => Some(Level::Detail),
            Self::TotalAchReturnItemsCr => Some(Level::Summary),
            Self::IndividualAchReturnItemCr => Some(Level::Detail),
            Self::AchReversalCredit => Some(Level::Detail),
            Self::TotalRejectedCredits => Some(Level::Summary),
            Self::IndividualRejectedCredit => Some(Level::Detail),
            Self::OverdraftCr => Some(Level::Detail),
            Self::ReturnItemCr => Some(Level::Detail),
            Self::ReturnItemAdjustmentCr => Some(Level::Detail),
            Self::TotalZbaCredits => Some(Level::Summary),
            Self::NetZeroBalanceAmount => Some(Level::Summary),
            Self::CumulativeZbaOrDisbursementCredits => Some(Level::Detail),
            Self::ZbaCredit => Some(Level::Detail),
            Self::ZbaFloatAdjustment => Some(Level::Detail),
            Self::ZbaCreditTransfer => Some(Level::Detail),
            Self::ZbaCreditAdjustment => Some(Level::Detail),
            Self::TotalControlledDisbursingCredits => Some(Level::Summary),
            Self::IndividualControlledDisbursingCredit => Some(Level::Detail),
            Self::TotalDtcDisbursingCredits => Some(Level::Summary),
            Self::IndividualDtcDisbursingCredit => Some(Level::Detail),
            Self::TotalAtmCredits => Some(Level::Summary),
            Self::AtmCredit => Some(Level::Detail),
            Self::CommercialDeposit => Some(Level::Detail),
            Self::CorrespondentBankDeposit => Some(Level::Summary),
            Self::TotalWireTransfersInFf => Some(Level::Summary),
            Self::TotalWireTransfersInChf => Some(Level::Summary),
            Self::TotalFedFundsSold => Some(Level::Summary),
            Self::FedFundsSold => Some(Level::Detail),
            Self::TotalTrustCredits => Some(Level::Summary),
            Self::TrustCredit => Some(Level::Detail),
            Self::TotalValueDatedFunds => Some(Level::Summary),
            Self::TotalCommercialDeposits => Some(Level::Summary),
            Self::TotalInternationalCreditsFf => Some(Level::Summary),
            Self::TotalInternationalCreditsChf => Some(Level::Summary),
            Self::TotalForeignCheckPurchased => Some(Level::Summary),
            Self::LateDeposit => Some(Level::Summary),
            Self::TotalSecuritiesSoldFf => Some(Level::Summary),
            Self::TotalSecuritiesSoldChf => Some(Level::Summary),
            Self::TotalSecuritiesMaturedFf => Some(Level::Summary),
            Self::TotalSecuritiesMaturedChf => Some(Level::Summary),
            Self::TotalSecuritiesInterest => Some(Level::Summary),
            Self::TotalSecuritiesMatured => Some(Level::Summary),
            Self::TotalSecuritiesInterestFf => Some(Level::Summary),
            Self::TotalSecuritiesInterestChf => Some(Level::Summary),
            Self::TotalEscrowCredits => Some(Level::Summary),
            Self::IndividualEscrowCredit => Some(Level::Detail),
            Self::TotalMiscellaneousSecuritiesCreditsFf => Some(Level::Summary),
            Self::TotalMiscellaneousSecuritiesCreditsChf => Some(Level::Summary),
            Self::TotalSecuritiesSold => Some(Level::Summary),
            Self::TotalBrokerDeposits => Some(Level::Summary),
            Self::TotalBrokerDepositsFf => Some(Level::Summary),
            Self::BrokerDeposit => Some(Level::Detail),
            Self::TotalBrokerDepositsChf => Some(Level::Summary),
            Self::IndividualBackValueCredit => Some(Level::Detail),
            Self::ItemInBrokersDeposit => Some(Level::Detail),
            Self::SweepInterestIncome => Some(Level::Detail),
            Self::SweepPrincipalSell => Some(Level::Detail),
            Self::FuturesCredit => Some(Level::Detail),
            Self::PrincipalPaymentsCredit => Some(Level::Detail),
            Self::InvestmentSold => Some(Level::Summary),
            Self::IndividualInvestmentSold => Some(Level::Detail),
            Self::TotalCashCenterCredits => Some(Level::Summary),
            Self::CashCenterCredit => Some(Level::Detail),
            Self::InterestCredit => Some(Level::Detail),
            Self::InvestmentInterest => Some(Level::Summary),
            Self::TotalCreditAdjustment => Some(Level::Summary),
            Self::CreditAdjustment => Some(Level::Detail),
            Self::YtdAdjustmentCredit => Some(Level::Detail),
            Self::InterestAdjustmentCredit => Some(Level::Detail),
            Self::TotalCreditsLessWireTransferAndReturnedChecks => Some(Level::Summary),
            Self::GrandTotalCreditsLessGrandTotalDebits => Some(Level::Summary),
            Self::CorrespondentCollection => Some(Level::Detail),
            Self::CorrespondentCollectionAdjustmentCr => Some(Level::Detail),
            Self::LoanParticipationCr => Some(Level::Detail),
            Self::CurrencyAndCoinDeposited => Some(Level::Detail),
            Self::FoodStampLetterCr => Some(Level::Detail),
            Self::FoodStampAdjustmentCr => Some(Level::Detail),
            Self::ClearingSettlementCredit => Some(Level::Detail),
            Self::TotalBackValueCredits => Some(Level::Summary),
            Self::BackValueAdjustmentCr => Some(Level::Detail),
            Self::CustomerPayrollCr => Some(Level::Detail),
            Self::FrbStatementRecapCr => Some(Level::Detail),
            Self::SavingsBondLetterOrAdjustmentCr => Some(Level::Detail),
            Self::TreasuryTaxAndLoanCredit => Some(Level::Detail),
            Self::TransferOfTreasuryCredit => Some(Level::Detail),
            Self::FrbGovernmentChecksCashLetterCredit => Some(Level::Detail),
            Self::FrbGovernmentCheckAdjustmentCr => Some(Level::Detail),
            Self::FrbPostalMoneyOrderCredit => Some(Level::Detail),
            Self::FrbPostalMoneyOrderAdjustmentCr => Some(Level::Detail),
            Self::FrbCashLetterAutoChargeCredit => Some(Level::Detail),
            Self::TotalUniversalCredits => Some(Level::Summary),
            Self::FrbCashLetterAutoChargeAdjustmentCr => Some(Level::Detail),
            Self::FrbFineSortCashLetterCredit => Some(Level::Detail),
            Self::FrbFineSortAdjustmentCr => Some(Level::Detail),
            Self::TotalFreightPaymentCredits => Some(Level::Summary),
            Self::TotalMiscellaneousCredits => Some(Level::Summary),
            Self::UniversalCredit => Some(Level::Detail),
            Self::FreightPaymentCredit => Some(Level::Detail),
            Self::ItemizedCreditOver10 => Some(Level::Detail),
            Self::CumulativeCredits => Some(Level::Detail),
            Self::CheckReversal => Some(Level::Detail),
            Self::FloatAdjustmentCr => Some(Level::Detail),
            Self::MiscellaneousFeeRefund => Some(Level::Detail),
            Self::MiscellaneousCredit => Some(Level::Detail),
            Self::TotalDebits => Some(Level::Summary),
            Self::TotalDebitAmountMtd => Some(Level::Summary),
            Self::TodaySTotalDebits => Some(Level::Summary),
            Self::TotalDebitLessWireTransfersAndChargeBacks => Some(Level::Summary),
            Self::DebitsNotDetailed => Some(Level::Summary),
            Self::FloatAdjustmentDb => Some(Level::Detail),
            Self::DebitAnyType => Some(Level::Detail),
            Self::TotalYtdAdjustment => Some(Level::Summary),
            Self::TotalDebitsExcludingReturnedItems => Some(Level::Summary),
            Self::LockboxDebit => Some(Level::Detail),
            Self::TotalLockboxDebits => Some(Level::Summary),
            Self::EdiTransactionDebits => Some(Level::Summary),
            Self::EdiTransactionDebit => Some(Level::Detail),
            Self::EdibanxSettlementDebit => Some(Level::Detail),
            Self::EdibanxReturnItemDebit => Some(Level::Detail),
            Self::TotalPayableThroughDrafts => Some(Level::Summary),
            Self::PayableThroughDraft => Some(Level::Detail),
            Self::AchConcentrationDebit => Some(Level::Detail),
            Self::TotalAchDisbursementFundingDebits => Some(Level::Summary),
            Self::AchDisbursementFundingDebit => Some(Level::Detail),
            Self::TotalAchDebits => Some(Level::Summary),
            Self::AchDebitReceived => Some(Level::Detail),
            Self::ItemInAchDisbursementOrDebit => Some(Level::Detail),
            Self::PreauthorizedAchDebit => Some(Level::Detail),
            Self::AccountHolderInitiatedAchDebit => Some(Level::Detail),
            Self::CorporateTradePaymentDebits => Some(Level::Summary),
            Self::CorporateTradePaymentDebit => Some(Level::Detail),
            Self::CorporateTradePaymentSettlementDb => Some(Level::Summary),
            Self::AchSettlementDb => Some(Level::Detail),
            Self::AchSettlementDebits => Some(Level::Summary),
            Self::AchReturnItemOrAdjustmentSettlementDb => Some(Level::Detail),
            Self::MiscellaneousAchDebit => Some(Level::Detail),
            Self::TotalCheckPaid => Some(Level::Summary),
            Self::TotalCheckPaidCumulativeMtd => Some(Level::Summary),
            Self::CumulativeChecksPaid => Some(Level::Detail),
            Self::CertifiedCheckDebit => Some(Level::Detail),
            Self::CheckPaid => Some(Level::Detail),
            Self::FederalReserveBankLetterDebit => Some(Level::Detail),
            Self::BankOriginatedDebit => Some(Level::Detail),
            Self::ListPostDebits => Some(Level::Summary),
            Self::ListPostDebit => Some(Level::Detail),
            Self::TotalLoanPayments => Some(Level::Summary),
            Self::IndividualLoanPayment => Some(Level::Detail),
            Self::TotalBankOriginatedDebits => Some(Level::Summary),
            Self::Draft => Some(Level::Detail),
            Self::DtcDebit => Some(Level::Detail),
            Self::TotalCashLetterDebits => Some(Level::Summary),
            Self::CashLetterDebit => Some(Level::Detail),
            Self::CashLetterAdjustmentDb => Some(Level::Detail),
            Self::TotalOutgoingMoneyTransfers => Some(Level::Summary),
            Self::IndividualOutgoingInternalMoneyTransfer => Some(Level::Detail),
            Self::CustomerTerminalInitiatedMoneyTransfer => Some(Level::Detail),
            Self::OutgoingMoneyTransfer => Some(Level::Detail),
            Self::MoneyTransferAdjustmentDb => Some(Level::Detail),
            Self::CompensationDb => Some(Level::Detail),
            Self::TotalAutomaticTransferDebits => Some(Level::Summary),
            Self::IndividualAutomaticTransferDebit => Some(Level::Detail),
            Self::BondOperationsDebit => Some(Level::Detail),
            Self::TotalBookTransferDebits => Some(Level::Summary),
            Self::BookTransferDebit => Some(Level::Detail),
            Self::TotalInternationalMoneyTransferDebits => Some(Level::Summary),
            Self::IndividualInternationalMoneyTransferDebits => Some(Level::Detail),
            Self::TotalInternationalDebits => Some(Level::Summary),
            Self::LetterOfCreditDebit => Some(Level::Detail),
            Self::LetterOfCreditDb => Some(Level::Detail),
            Self::ForeignExchangeDebit => Some(Level::Detail),
            Self::TotalLettersOfCreditDb => Some(Level::Summary),
            Self::ForeignRemittanceDebit => Some(Level::Detail),
            Self::ForeignCollectionDebit => Some(Level::Detail),
            Self::ForeignChecksPaid => Some(Level::Detail),
            Self::CommissionDb => Some(Level::Detail),
            Self::InternationalMoneyMarketTradingDb => Some(Level::Detail),
            Self::StandingOrderDb => Some(Level::Detail),
            Self::MiscellaneousInternationalDebit => Some(Level::Detail),
            Self::TotalSecurityDebits => Some(Level::Summary),
            Self::SecuritiesPurchased => Some(Level::Detail),
            Self::TotalAmountOfSecuritiesPurchased => Some(Level::Summary),
            Self::SecurityCollectionDebit => Some(Level::Detail),
            Self::TotalMiscellaneousSecuritiesDbFf => Some(Level::Summary),
            Self::PurchaseOfEquitySecurities => Some(Level::Detail),
            Self::TotalMiscellaneousSecuritiesDebitChf => Some(Level::Summary),
            Self::TotalCollectionDebit => Some(Level::Summary),
            Self::MaturedRepurchaseOrder => Some(Level::Detail),
            Self::TotalBankersAcceptancesDebit => Some(Level::Summary),
            Self::CouponCollectionDebit => Some(Level::Detail),
            Self::BankersAcceptancesDb => Some(Level::Detail),
            Self::PurchaseOfDebtSecurities => Some(Level::Detail),
            Self::DomesticCollection => Some(Level::Detail),
            Self::InterestMaturedPrincipalPaymentDb => Some(Level::Detail),
            Self::CommercialPaperDb => Some(Level::Detail),
            Self::CapitalChangeDb => Some(Level::Detail),
            Self::SavingsBondsSalesAdjustmentDb => Some(Level::Detail),
            Self::MiscellaneousSecurityDebit => Some(Level::Detail),
            Self::TotalDepositedItemsReturned => Some(Level::Summary),
            Self::TotalCreditReversals => Some(Level::Summary),
            Self::CreditReversal => Some(Level::Detail),
            Self::PostingErrorCorrectionDebit => Some(Level::Detail),
            Self::DepositedItemReturned => Some(Level::Detail),
            Self::TotalAchReturnItemsDb => Some(Level::Summary),
            Self::IndividualAchReturnItemDb => Some(Level::Detail),
            Self::AchReversalDebit => Some(Level::Detail),
            Self::TotalRejectedDebits => Some(Level::Summary),
            Self::IndividualRejectedDebit => Some(Level::Detail),
            Self::OverdraftDb => Some(Level::Detail),
            Self::OverdraftFee => Some(Level::Detail),
            Self::ReturnItemDb => Some(Level::Detail),
            Self::ReturnItemFee => Some(Level::Detail),
            Self::ReturnItemAdjustmentDb => Some(Level::Detail),
            Self::TotalZbaDebits => Some(Level::Summary),
            Self::CumulativeZbaDebits => Some(Level::Detail),
            Self::ZbaDebit => Some(Level::Detail),
            Self::ZbaDebitTransfer => Some(Level::Detail),
            Self::ZbaDebitAdjustment => Some(Level::Detail),
            Self::TotalControlledDisbursingDebits => Some(Level::Summary),
            Self::IndividualControlledDisbursingDebit => Some(Level::Detail),
            Self::TotalDisbursingChecksPaidEarlyAmount => Some(Level::Summary),
            Self::TotalDisbursingChecksPaidLaterAmount => Some(Level::Summary),
            Self::DisbursingFundingRequirement => Some(Level::Summary),
            Self::FrbPresentmentEstimateFedEstimate => Some(Level::Summary),
            Self::LateDebitsAfterNotification => Some(Level::Summary),
            Self::TotalDisbursingChecksPaidLastAmount => Some(Level::Summary),
            Self::TotalDtcDebits => Some(Level::Summary),
            Self::TotalAtmDebits => Some(Level::Summary),
            Self::AtmDebit => Some(Level::Detail),
            Self::TotalAprDebits => Some(Level::Summary),
            Self::ArpDebit => Some(Level::Detail),
            Self::EstimatedTotalDisbursement => Some(Level::Summary),
            Self::AdjustedTotalDisbursement => Some(Level::Summary),
            Self::TotalFundsRequired => Some(Level::Summary),
            Self::TotalWireTransfersOutChf => Some(Level::Summary),
            Self::TotalWireTransfersOutFf => Some(Level::Summary),
            Self::TotalInternationalDebitChf => Some(Level::Summary),
            Self::TotalInternationalDebitFf => Some(Level::Summary),
            Self::TotalFederalReserveBankCommercialBankDebit => Some(Level::Summary),
            Self::FederalReserveBankCommercialBankDebit => Some(Level::Detail),
            Self::TotalSecuritiesPurchasedChf => Some(Level::Summary),
            Self::TotalSecuritiesPurchasedFf => Some(Level::Summary),
            Self::TotalBrokerDebitsChf => Some(Level::Summary),
            Self::BrokerDebit => Some(Level::Detail),
            Self::TotalBrokerDebitsFf => Some(Level::Summary),
            Self::TotalBrokerDebits => Some(Level::Summary),
            Self::TotalFedFundsPurchased => Some(Level::Summary),
            Self::FedFundsPurchased => Some(Level::Detail),
            Self::TotalCashCenterDebits => Some(Level::Summary),
            Self::CashCenterDebit => Some(Level::Detail),
            Self::TotalDebitAdjustments => Some(Level::Summary),
            Self::DebitAdjustment => Some(Level::Detail),
            Self::TotalTrustDebits => Some(Level::Summary),
            Self::TrustDebit => Some(Level::Detail),
            Self::YtdAdjustmentDebit => Some(Level::Detail),
            Self::TotalEscrowDebits => Some(Level::Summary),
            Self::IndividualEscrowDebit => Some(Level::Detail),
            Self::IndividualBackValueDebit => Some(Level::Detail),
            Self::TransferCalculationDebit => Some(Level::Summary),
            Self::InvestmentsPurchased => Some(Level::Summary),
            Self::IndividualInvestmentPurchased => Some(Level::Detail),
            Self::InterestDebit => Some(Level::Detail),
            Self::TotalInvestmentInterestDebits => Some(Level::Summary),
            Self::SweepPrincipalBuy => Some(Level::Detail),
            Self::FuturesDebit => Some(Level::Detail),
            Self::PrincipalPaymentsDebit => Some(Level::Detail),
            Self::InterestAdjustmentDebit => Some(Level::Detail),
            Self::AccountAnalysisFee => Some(Level::Detail),
            Self::CorrespondentCollectionDebit => Some(Level::Detail),
            Self::CorrespondentCollectionAdjustmentDb => Some(Level::Detail),
            Self::LoanParticipationDb => Some(Level::Detail),
            Self::InterceptDebits => Some(Level::Summary),
            Self::CurrencyAndCoinShipped => Some(Level::Detail),
            Self::FoodStampLetterDb => Some(Level::Detail),
            Self::FoodStampAdjustmentDb => Some(Level::Detail),
            Self::ClearingSettlementDebit => Some(Level::Detail),
            Self::TotalBackValueDebits => Some(Level::Summary),
            Self::BackValueAdjustmentDb => Some(Level::Detail),
            Self::CustomerPayrollDb => Some(Level::Detail),
            Self::FrbStatementRecapDb => Some(Level::Detail),
            Self::SavingsBondLetterOrAdjustmentDb => Some(Level::Detail),
            Self::TreasuryTaxAndLoanDebit => Some(Level::Detail),
            Self::TransferOfTreasuryDebit => Some(Level::Detail),
            Self::FrbGovernmentChecksCashLetterDebit => Some(Level::Detail),
            Self::FrbGovernmentCheckAdjustmentDb => Some(Level::Detail),
            Self::FrbPostalMoneyOrderDebit => Some(Level::Detail),
            Self::FrbPostalMoneyOrderAdjustmentDb => Some(Level::Detail),
            Self::FrbCashLetterAutoChargeDebit => Some(Level::Detail),
            Self::TotalUniversalDebits => Some(Level::Summary),
            Self::FrbCashLetterAutoChargeAdjustmentDb => Some(Level::Detail),
            Self::FrbFineSortCashLetterDebit => Some(Level::Detail),
            Self::FrbFineSortAdjustmentDb => Some(Level::Detail),
            Self::FrbFreightPaymentDebits => Some(Level::Summary),
            Self::TotalMiscellaneousDebits => Some(Level::Summary),
            Self::UniversalDebit => Some(Level::Detail),
            Self::FreightPaymentDebit => Some(Level::Detail),
            Self::ItemizedDebitOver10 => Some(Level::Detail),
            Self::DepositReversal => Some(Level::Detail),
            Self::DepositCorrectionDebit => Some(Level::Detail),
            Self::RegularCollectionDebit => Some(Level::Detail),
            Self::CumulativeDebits => Some(Level::Detail),
            Self::MiscellaneousFees => Some(Level::Detail),
            Self::MiscellaneousDebit => Some(Level::Detail),
            Self::PrincipalLoanBalance => Some(Level::Status),
            Self::AvailableCommitmentAmount => Some(Level::Status),
            Self::PaymentAmountDue => Some(Level::Status),
            Self::PrincipalAmountPastDue => Some(Level::Status),
            Self::InterestAmountPastDue => Some(Level::Status),
            Self::TotalLoanPayment => Some(Level::Summary),
            Self::AmountAppliedToInterest => Some(Level::Detail),
            Self::AmountAppliedToPrincipal => Some(Level::Detail),
            Self::AmountAppliedToEscrow => Some(Level::Detail),
            Self::AmountAppliedToLateCharges => Some(Level::Detail),
            Self::AmountAppliedToBuydown => Some(Level::Detail),
            Self::AmountAppliedToMiscFees => Some(Level::Detail),
            Self::AmountAppliedToDeferredInterestDetail => Some(Level::Detail),
            Self::AmountAppliedToServiceCharge => Some(Level::Detail),
            Self::LoanDisbursement => Some(Level::Summary),
            Self::ContainsNonMonetaryInformation => Some(Level::Detail),
            Self::Unknown => None,
        }
    }
}

/// Lookup table indexed by the numeric value of a 3-digit type code.
///
/// Used by `TypeCode::from(&str)` to turn parsing into a single array index
/// instead of a string-comparison chain, since every defined code is a
/// zero-padded 3-digit number in `000..=999`.
const TYPE_CODE_TABLE: [TypeCode; 1000] = [
    TypeCode::Unknown,                                       // 000
    TypeCode::Unknown,                                       // 001
    TypeCode::Unknown,                                       // 002
    TypeCode::Unknown,                                       // 003
    TypeCode::Unknown,                                       // 004
    TypeCode::Unknown,                                       // 005
    TypeCode::Unknown,                                       // 006
    TypeCode::Unknown,                                       // 007
    TypeCode::Unknown,                                       // 008
    TypeCode::Unknown,                                       // 009
    TypeCode::OpeningLedger,                                 // 010
    TypeCode::AverageOpeningLedgerMtd,                       // 011
    TypeCode::AverageOpeningLedgerYtd,                       // 012
    TypeCode::Unknown,                                       // 013
    TypeCode::Unknown,                                       // 014
    TypeCode::ClosingLedger,                                 // 015
    TypeCode::Unknown,                                       // 016
    TypeCode::Unknown,                                       // 017
    TypeCode::Unknown,                                       // 018
    TypeCode::Unknown,                                       // 019
    TypeCode::AverageClosingLedgerMtd,                       // 020
    TypeCode::AverageClosingLedgerPreviousMonth,             // 021
    TypeCode::AggregateBalanceAdjustments,                   // 022
    TypeCode::Unknown,                                       // 023
    TypeCode::AverageClosingLedgerYtdPreviousMonth,          // 024
    TypeCode::AverageClosingLedgerYtd,                       // 025
    TypeCode::Unknown,                                       // 026
    TypeCode::Unknown,                                       // 027
    TypeCode::Unknown,                                       // 028
    TypeCode::Unknown,                                       // 029
    TypeCode::CurrentLedger,                                 // 030
    TypeCode::Unknown,                                       // 031
    TypeCode::Unknown,                                       // 032
    TypeCode::Unknown,                                       // 033
    TypeCode::Unknown,                                       // 034
    TypeCode::Unknown,                                       // 035
    TypeCode::Unknown,                                       // 036
    TypeCode::AchNetPosition,                                // 037
    TypeCode::Unknown,                                       // 038
    TypeCode::OpeningAvailableTotalSameDayAchDtcDeposit,     // 039
    TypeCode::OpeningAvailable,                              // 040
    TypeCode::AverageOpeningAvailableMtd,                    // 041
    TypeCode::AverageOpeningAvailableYtd,                    // 042
    TypeCode::AverageAvailablePreviousMonth,                 // 043
    TypeCode::DisbursingOpeningAvailableBalance,             // 044
    TypeCode::ClosingAvailable,                              // 045
    TypeCode::Unknown,                                       // 046
    TypeCode::Unknown,                                       // 047
    TypeCode::Unknown,                                       // 048
    TypeCode::Unknown,                                       // 049
    TypeCode::AverageClosingAvailableMtd,                    // 050
    TypeCode::AverageClosingAvailableLastMonth,              // 051
    TypeCode::Unknown,                                       // 052
    TypeCode::Unknown,                                       // 053
    TypeCode::AverageClosingAvailableYtdLastMonth,           // 054
    TypeCode::AverageClosingAvailableYtd,                    // 055
    TypeCode::LoanBalance,                                   // 056
    TypeCode::TotalInvestmentPosition,                       // 057
    TypeCode::Unknown,                                       // 058
    TypeCode::CurrentAvailableCrsSupressed,                  // 059
    TypeCode::CurrentAvailable,                              // 060
    TypeCode::AverageCurrentAvailableMtd,                    // 061
    TypeCode::AverageCurrentAvailableYtd,                    // 062
    TypeCode::TotalFloat,                                    // 063
    TypeCode::Unknown,                                       // 064
    TypeCode::TargetBalance,                                 // 065
    TypeCode::AdjustedBalance,                               // 066
    TypeCode::AdjustedBalanceMtd,                            // 067
    TypeCode::AdjustedBalanceYtd,                            // 068
    TypeCode::Unknown,                                       // 069
    TypeCode::N0DayFloat,                                    // 070
    TypeCode::Unknown,                                       // 071
    TypeCode::N1DayFloat,                                    // 072
    TypeCode::FloatAdjustmentNa,                             // 073
    TypeCode::N2OrMoreDaysFloat,                             // 074
    TypeCode::N3OrMoreDaysFloat,                             // 075
    TypeCode::AdjustmentToBalances,                          // 076
    TypeCode::AverageAdjustmentToBalancesMtd,                // 077
    TypeCode::AverageAdjustmentToBalancesYtd,                // 078
    TypeCode::N4DayFloat,                                    // 079
    TypeCode::N5DayFloat,                                    // 080
    TypeCode::N6DayFloat,                                    // 081
    TypeCode::Average1DayFloatMtd,                           // 082
    TypeCode::Average1DayFloatYtd,                           // 083
    TypeCode::Average2DayFloatMtd,                           // 084
    TypeCode::Average2DayFloatYtd,                           // 085
    TypeCode::TransferCalculation,                           // 086
    TypeCode::Unknown,                                       // 087
    TypeCode::Unknown,                                       // 088
    TypeCode::Unknown,                                       // 089
    TypeCode::Unknown,                                       // 090
    TypeCode::Unknown,                                       // 091
    TypeCode::Unknown,                                       // 092
    TypeCode::Unknown,                                       // 093
    TypeCode::Unknown,                                       // 094
    TypeCode::Unknown,                                       // 095
    TypeCode::Unknown,                                       // 096
    TypeCode::Unknown,                                       // 097
    TypeCode::Unknown,                                       // 098
    TypeCode::Unknown,                                       // 099
    TypeCode::TotalCredits,                                  // 100
    TypeCode::TotalCreditAmountMtd,                          // 101
    TypeCode::Unknown,                                       // 102
    TypeCode::Unknown,                                       // 103
    TypeCode::Unknown,                                       // 104
    TypeCode::CreditsNotDetailed,                            // 105
    TypeCode::DepositsSubjectToFloat,                        // 106
    TypeCode::TotalAdjustmentCreditsYtd,                     // 107
    TypeCode::CreditAnyType,                                 // 108
    TypeCode::CurrentDayTotalLockboxDeposits,                // 109
    TypeCode::TotalLockboxDeposits,                          // 110
    TypeCode::Unknown,                                       // 111
    TypeCode::Unknown,                                       // 112
    TypeCode::Unknown,                                       // 113
    TypeCode::Unknown,                                       // 114
    TypeCode::LockboxDeposit,                                // 115
    TypeCode::ItemInLockboxDeposit,                          // 116
    TypeCode::Unknown,                                       // 117
    TypeCode::LockboxAdjustmentCredit,                       // 118
    TypeCode::Unknown,                                       // 119
    TypeCode::EdiTransactionCreditCrSummary,                 // 120
    TypeCode::EdiTransactionCreditCrDetail,                  // 121
    TypeCode::EdibanxCreditReceived,                         // 122
    TypeCode::EdibanxCreditReturn,                           // 123
    TypeCode::Unknown,                                       // 124
    TypeCode::Unknown,                                       // 125
    TypeCode::Unknown,                                       // 126
    TypeCode::Unknown,                                       // 127
    TypeCode::Unknown,                                       // 128
    TypeCode::Unknown,                                       // 129
    TypeCode::TotalConcentrationCredits,                     // 130
    TypeCode::TotalDtcCredits,                               // 131
    TypeCode::Unknown,                                       // 132
    TypeCode::Unknown,                                       // 133
    TypeCode::Unknown,                                       // 134
    TypeCode::DtcConcentrationCredit,                        // 135
    TypeCode::ItemInDtcDeposit,                              // 136
    TypeCode::Unknown,                                       // 137
    TypeCode::Unknown,                                       // 138
    TypeCode::Unknown,                                       // 139
    TypeCode::TotalAchCredits,                               // 140
    TypeCode::Unknown,                                       // 141
    TypeCode::AchCreditReceived,                             // 142
    TypeCode::ItemInAchDeposit,                              // 143
    TypeCode::Unknown,                                       // 144
    TypeCode::AchConcentrationCredit,                        // 145
    TypeCode::TotalBankCardDeposits,                         // 146
    TypeCode::IndividualBankCardDeposit,                     // 147
    TypeCode::Unknown,                                       // 148
    TypeCode::Unknown,                                       // 149
    TypeCode::TotalPreauthorizedPaymentCredits,              // 150
    TypeCode::Unknown,                                       // 151
    TypeCode::Unknown,                                       // 152
    TypeCode::Unknown,                                       // 153
    TypeCode::Unknown,                                       // 154
    TypeCode::PreauthorizedDraftCredit,                      // 155
    TypeCode::ItemInPacDeposit,                              // 156
    TypeCode::Unknown,                                       // 157
    TypeCode::Unknown,                                       // 158
    TypeCode::Unknown,                                       // 159
    TypeCode::TotalAchDisbursingFundingCredits,              // 160
    TypeCode::Unknown,                                       // 161
    TypeCode::CorporateTradePaymentSettlementCr,             // 162
    TypeCode::CorporateTradePaymentCredits,                  // 163
    TypeCode::CorporateTradePaymentCredit,                   // 164
    TypeCode::PreauthorizedAchCredit,                        // 165
    TypeCode::AchSettlementCr,                               // 166
    TypeCode::AchSettlementCredits,                          // 167
    TypeCode::AchReturnItemOrAdjustmentSettlementCr,         // 168
    TypeCode::MiscellaneousAchCredit,                        // 169
    TypeCode::TotalOtherCheckDeposits,                       // 170
    TypeCode::IndividualLoanDeposit,                         // 171
    TypeCode::DepositCorrection,                             // 172
    TypeCode::BankPreparedDeposit,                           // 173
    TypeCode::OtherDeposit,                                  // 174
    TypeCode::CheckDepositPackage,                           // 175
    TypeCode::RePresentedCheckDeposit,                       // 176
    TypeCode::Unknown,                                       // 177
    TypeCode::ListPostCredits,                               // 178
    TypeCode::Unknown,                                       // 179
    TypeCode::TotalLoanProceeds,                             // 180
    TypeCode::Unknown,                                       // 181
    TypeCode::TotalBankPreparedDeposits,                     // 182
    TypeCode::Unknown,                                       // 183
    TypeCode::DraftDeposit,                                  // 184
    TypeCode::TotalMiscellaneousDeposits,                    // 185
    TypeCode::TotalCashLetterCredits,                        // 186
    TypeCode::CashLetterCredit,                              // 187
    TypeCode::TotalCashLetterAdjustments,                    // 188
    TypeCode::CashLetterAdjustmentCr,                        // 189
    TypeCode::TotalIncomingMoneyTransfers,                   // 190
    TypeCode::IndividualIncomingInternalMoneyTransfer,       // 191
    TypeCode::Unknown,                                       // 192
    TypeCode::Unknown,                                       // 193
    TypeCode::Unknown,                                       // 194
    TypeCode::IncomingMoneyTransfer,                         // 195
    TypeCode::MoneyTransferAdjustmentCr,                     // 196
    TypeCode::Unknown,                                       // 197
    TypeCode::CompensationCr,                                // 198
    TypeCode::Unknown,                                       // 199
    TypeCode::TotalAutomaticTransferCredits,                 // 200
    TypeCode::IndividualAutomaticTransferCredit,             // 201
    TypeCode::BondOperationsCredit,                          // 202
    TypeCode::Unknown,                                       // 203
    TypeCode::Unknown,                                       // 204
    TypeCode::TotalBookTransferCredits,                      // 205
    TypeCode::BookTransferCredit,                            // 206
    TypeCode::TotalInternationalMoneyTransferCredits,        // 207
    TypeCode::IndividualInternationalMoneyTransferCredit,    // 208
    TypeCode::Unknown,                                       // 209
    TypeCode::TotalInternationalCredits,                     // 210
    TypeCode::Unknown,                                       // 211
    TypeCode::ForeignLetterOfCredit,                         // 212
    TypeCode::LetterOfCreditCr,                              // 213
    TypeCode::ForeignExchangeOfCredit,                       // 214
    TypeCode::TotalLettersOfCreditCr,                        // 215
    TypeCode::ForeignRemittanceCredit,                       // 216
    TypeCode::Unknown,                                       // 217
    TypeCode::ForeignCollectionCredit,                       // 218
    TypeCode::Unknown,                                       // 219
    TypeCode::Unknown,                                       // 220
    TypeCode::ForeignCheckPurchase,                          // 221
    TypeCode::ForeignChecksDeposited,                        // 222
    TypeCode::Unknown,                                       // 223
    TypeCode::CommissionCr,                                  // 224
    TypeCode::Unknown,                                       // 225
    TypeCode::InternationalMoneyMarketTradingCr,             // 226
    TypeCode::StandingOrderCr,                               // 227
    TypeCode::Unknown,                                       // 228
    TypeCode::MiscellaneousInternationalCredit,              // 229
    TypeCode::TotalSecurityCredits,                          // 230
    TypeCode::TotalCollectionCredits,                        // 231
    TypeCode::SaleOfDebtSecurity,                            // 232
    TypeCode::SecuritiesSold,                                // 233
    TypeCode::SaleOfEquitySecurity,                          // 234
    TypeCode::MaturedReverseRepurchaseOrder,                 // 235
    TypeCode::MaturityOfDebtSecurity,                        // 236
    TypeCode::IndividualCollectionCredit,                    // 237
    TypeCode::CollectionOfDividends,                         // 238
    TypeCode::TotalBankersAcceptanceCredits,                 // 239
    TypeCode::CouponCollectionsBanks,                        // 240
    TypeCode::BankersAcceptancesCr,                          // 241
    TypeCode::CollectionOfInterestIncome,                    // 242
    TypeCode::MaturedFedFundsPurchased,                      // 243
    TypeCode::InterestMaturedPrincipalPaymentCr,             // 244
    TypeCode::MonthlyDividends,                              // 245
    TypeCode::CommercialPaperCr,                             // 246
    TypeCode::CapitalChangeCr,                               // 247
    TypeCode::SavingsBondsSalesAdjustmentCr,                 // 248
    TypeCode::MiscellaneousSecurityCredit,                   // 249
    TypeCode::TotalChecksPostedAndReturned,                  // 250
    TypeCode::TotalDebitReversals,                           // 251
    TypeCode::DebitReversal,                                 // 252
    TypeCode::Unknown,                                       // 253
    TypeCode::PostingErrorCorrectionCredit,                  // 254
    TypeCode::CheckPostedAndReturned,                        // 255
    TypeCode::TotalAchReturnItemsCr,                         // 256
    TypeCode::IndividualAchReturnItemCr,                     // 257
    TypeCode::AchReversalCredit,                             // 258
    TypeCode::Unknown,                                       // 259
    TypeCode::TotalRejectedCredits,                          // 260
    TypeCode::IndividualRejectedCredit,                      // 261
    TypeCode::Unknown,                                       // 262
    TypeCode::OverdraftCr,                                   // 263
    TypeCode::Unknown,                                       // 264
    TypeCode::Unknown,                                       // 265
    TypeCode::ReturnItemCr,                                  // 266
    TypeCode::Unknown,                                       // 267
    TypeCode::ReturnItemAdjustmentCr,                        // 268
    TypeCode::Unknown,                                       // 269
    TypeCode::TotalZbaCredits,                               // 270
    TypeCode::NetZeroBalanceAmount,                          // 271
    TypeCode::Unknown,                                       // 272
    TypeCode::Unknown,                                       // 273
    TypeCode::CumulativeZbaOrDisbursementCredits,            // 274
    TypeCode::ZbaCredit,                                     // 275
    TypeCode::ZbaFloatAdjustment,                            // 276
    TypeCode::ZbaCreditTransfer,                             // 277
    TypeCode::ZbaCreditAdjustment,                           // 278
    TypeCode::Unknown,                                       // 279
    TypeCode::TotalControlledDisbursingCredits,              // 280
    TypeCode::IndividualControlledDisbursingCredit,          // 281
    TypeCode::Unknown,                                       // 282
    TypeCode::Unknown,                                       // 283
    TypeCode::Unknown,                                       // 284
    TypeCode::TotalDtcDisbursingCredits,                     // 285
    TypeCode::IndividualDtcDisbursingCredit,                 // 286
    TypeCode::Unknown,                                       // 287
    TypeCode::Unknown,                                       // 288
    TypeCode::Unknown,                                       // 289
    TypeCode::Unknown,                                       // 290
    TypeCode::Unknown,                                       // 291
    TypeCode::Unknown,                                       // 292
    TypeCode::Unknown,                                       // 293
    TypeCode::TotalAtmCredits,                               // 294
    TypeCode::AtmCredit,                                     // 295
    TypeCode::Unknown,                                       // 296
    TypeCode::Unknown,                                       // 297
    TypeCode::Unknown,                                       // 298
    TypeCode::Unknown,                                       // 299
    TypeCode::Unknown,                                       // 300
    TypeCode::CommercialDeposit,                             // 301
    TypeCode::CorrespondentBankDeposit,                      // 302
    TypeCode::TotalWireTransfersInFf,                        // 303
    TypeCode::TotalWireTransfersInChf,                       // 304
    TypeCode::TotalFedFundsSold,                             // 305
    TypeCode::FedFundsSold,                                  // 306
    TypeCode::TotalTrustCredits,                             // 307
    TypeCode::TrustCredit,                                   // 308
    TypeCode::TotalValueDatedFunds,                          // 309
    TypeCode::TotalCommercialDeposits,                       // 310
    TypeCode::Unknown,                                       // 311
    TypeCode::Unknown,                                       // 312
    TypeCode::Unknown,                                       // 313
    TypeCode::Unknown,                                       // 314
    TypeCode::TotalInternationalCreditsFf,                   // 315
    TypeCode::TotalInternationalCreditsChf,                  // 316
    TypeCode::Unknown,                                       // 317
    TypeCode::TotalForeignCheckPurchased,                    // 318
    TypeCode::LateDeposit,                                   // 319
    TypeCode::TotalSecuritiesSoldFf,                         // 320
    TypeCode::TotalSecuritiesSoldChf,                        // 321
    TypeCode::Unknown,                                       // 322
    TypeCode::Unknown,                                       // 323
    TypeCode::TotalSecuritiesMaturedFf,                      // 324
    TypeCode::TotalSecuritiesMaturedChf,                     // 325
    TypeCode::TotalSecuritiesInterest,                       // 326
    TypeCode::TotalSecuritiesMatured,                        // 327
    TypeCode::TotalSecuritiesInterestFf,                     // 328
    TypeCode::TotalSecuritiesInterestChf,                    // 329
    TypeCode::TotalEscrowCredits,                            // 330
    TypeCode::IndividualEscrowCredit,                        // 331
    TypeCode::TotalMiscellaneousSecuritiesCreditsFf,         // 332
    TypeCode::Unknown,                                       // 333
    TypeCode::Unknown,                                       // 334
    TypeCode::Unknown,                                       // 335
    TypeCode::TotalMiscellaneousSecuritiesCreditsChf,        // 336
    TypeCode::Unknown,                                       // 337
    TypeCode::TotalSecuritiesSold,                           // 338
    TypeCode::Unknown,                                       // 339
    TypeCode::TotalBrokerDeposits,                           // 340
    TypeCode::TotalBrokerDepositsFf,                         // 341
    TypeCode::BrokerDeposit,                                 // 342
    TypeCode::TotalBrokerDepositsChf,                        // 343
    TypeCode::IndividualBackValueCredit,                     // 344
    TypeCode::ItemInBrokersDeposit,                          // 345
    TypeCode::SweepInterestIncome,                           // 346
    TypeCode::SweepPrincipalSell,                            // 347
    TypeCode::FuturesCredit,                                 // 348
    TypeCode::PrincipalPaymentsCredit,                       // 349
    TypeCode::InvestmentSold,                                // 350
    TypeCode::IndividualInvestmentSold,                      // 351
    TypeCode::TotalCashCenterCredits,                        // 352
    TypeCode::CashCenterCredit,                              // 353
    TypeCode::InterestCredit,                                // 354
    TypeCode::InvestmentInterest,                            // 355
    TypeCode::TotalCreditAdjustment,                         // 356
    TypeCode::CreditAdjustment,                              // 357
    TypeCode::YtdAdjustmentCredit,                           // 358
    TypeCode::InterestAdjustmentCredit,                      // 359
    TypeCode::TotalCreditsLessWireTransferAndReturnedChecks, // 360
    TypeCode::GrandTotalCreditsLessGrandTotalDebits,         // 361
    TypeCode::CorrespondentCollection,                       // 362
    TypeCode::CorrespondentCollectionAdjustmentCr,           // 363
    TypeCode::LoanParticipationCr,                           // 364
    TypeCode::Unknown,                                       // 365
    TypeCode::CurrencyAndCoinDeposited,                      // 366
    TypeCode::FoodStampLetterCr,                             // 367
    TypeCode::FoodStampAdjustmentCr,                         // 368
    TypeCode::ClearingSettlementCredit,                      // 369
    TypeCode::TotalBackValueCredits,                         // 370
    TypeCode::Unknown,                                       // 371
    TypeCode::BackValueAdjustmentCr,                         // 372
    TypeCode::CustomerPayrollCr,                             // 373
    TypeCode::FrbStatementRecapCr,                           // 374
    TypeCode::Unknown,                                       // 375
    TypeCode::SavingsBondLetterOrAdjustmentCr,               // 376
    TypeCode::TreasuryTaxAndLoanCredit,                      // 377
    TypeCode::TransferOfTreasuryCredit,                      // 378
    TypeCode::FrbGovernmentChecksCashLetterCredit,           // 379
    TypeCode::Unknown,                                       // 380
    TypeCode::FrbGovernmentCheckAdjustmentCr,                // 381
    TypeCode::FrbPostalMoneyOrderCredit,                     // 382
    TypeCode::FrbPostalMoneyOrderAdjustmentCr,               // 383
    TypeCode::FrbCashLetterAutoChargeCredit,                 // 384
    TypeCode::TotalUniversalCredits,                         // 385
    TypeCode::FrbCashLetterAutoChargeAdjustmentCr,           // 386
    TypeCode::FrbFineSortCashLetterCredit,                   // 387
    TypeCode::FrbFineSortAdjustmentCr,                       // 388
    TypeCode::TotalFreightPaymentCredits,                    // 389
    TypeCode::TotalMiscellaneousCredits,                     // 390
    TypeCode::UniversalCredit,                               // 391
    TypeCode::FreightPaymentCredit,                          // 392
    TypeCode::ItemizedCreditOver10,                          // 393
    TypeCode::CumulativeCredits,                             // 394
    TypeCode::CheckReversal,                                 // 395
    TypeCode::Unknown,                                       // 396
    TypeCode::FloatAdjustmentCr,                             // 397
    TypeCode::MiscellaneousFeeRefund,                        // 398
    TypeCode::MiscellaneousCredit,                           // 399
    TypeCode::TotalDebits,                                   // 400
    TypeCode::TotalDebitAmountMtd,                           // 401
    TypeCode::Unknown,                                       // 402
    TypeCode::TodaySTotalDebits,                             // 403
    TypeCode::Unknown,                                       // 404
    TypeCode::TotalDebitLessWireTransfersAndChargeBacks,     // 405
    TypeCode::DebitsNotDetailed,                             // 406
    TypeCode::Unknown,                                       // 407
    TypeCode::FloatAdjustmentDb,                             // 408
    TypeCode::DebitAnyType,                                  // 409
    TypeCode::TotalYtdAdjustment,                            // 410
    TypeCode::Unknown,                                       // 411
    TypeCode::TotalDebitsExcludingReturnedItems,             // 412
    TypeCode::Unknown,                                       // 413
    TypeCode::Unknown,                                       // 414
    TypeCode::LockboxDebit,                                  // 415
    TypeCode::TotalLockboxDebits,                            // 416
    TypeCode::Unknown,                                       // 417
    TypeCode::Unknown,                                       // 418
    TypeCode::Unknown,                                       // 419
    TypeCode::EdiTransactionDebits,                          // 420
    TypeCode::EdiTransactionDebit,                           // 421
    TypeCode::EdibanxSettlementDebit,                        // 422
    TypeCode::EdibanxReturnItemDebit,                        // 423
    TypeCode::Unknown,                                       // 424
    TypeCode::Unknown,                                       // 425
    TypeCode::Unknown,                                       // 426
    TypeCode::Unknown,                                       // 427
    TypeCode::Unknown,                                       // 428
    TypeCode::Unknown,                                       // 429
    TypeCode::TotalPayableThroughDrafts,                     // 430
    TypeCode::Unknown,                                       // 431
    TypeCode::Unknown,                                       // 432
    TypeCode::Unknown,                                       // 433
    TypeCode::Unknown,                                       // 434
    TypeCode::PayableThroughDraft,                           // 435
    TypeCode::Unknown,                                       // 436
    TypeCode::Unknown,                                       // 437
    TypeCode::Unknown,                                       // 438
    TypeCode::Unknown,                                       // 439
    TypeCode::Unknown,                                       // 440
    TypeCode::Unknown,                                       // 441
    TypeCode::Unknown,                                       // 442
    TypeCode::Unknown,                                       // 443
    TypeCode::Unknown,                                       // 444
    TypeCode::AchConcentrationDebit,                         // 445
    TypeCode::TotalAchDisbursementFundingDebits,             // 446
    TypeCode::AchDisbursementFundingDebit,                   // 447
    TypeCode::Unknown,                                       // 448
    TypeCode::Unknown,                                       // 449
    TypeCode::TotalAchDebits,                                // 450
    TypeCode::AchDebitReceived,                              // 451
    TypeCode::ItemInAchDisbursementOrDebit,                  // 452
    TypeCode::Unknown,                                       // 453
    TypeCode::Unknown,                                       // 454
    TypeCode::PreauthorizedAchDebit,                         // 455
    TypeCode::Unknown,                                       // 456
    TypeCode::Unknown,                                       // 457
    TypeCode::Unknown,                                       // 458
    TypeCode::Unknown,                                       // 459
    TypeCode::Unknown,                                       // 460
    TypeCode::Unknown,                                       // 461
    TypeCode::AccountHolderInitiatedAchDebit,                // 462
    TypeCode::CorporateTradePaymentDebits,                   // 463
    TypeCode::CorporateTradePaymentDebit,                    // 464
    TypeCode::CorporateTradePaymentSettlementDb,             // 465
    TypeCode::AchSettlementDb,                               // 466
    TypeCode::AchSettlementDebits,                           // 467
    TypeCode::AchReturnItemOrAdjustmentSettlementDb,         // 468
    TypeCode::MiscellaneousAchDebit,                         // 469
    TypeCode::TotalCheckPaid,                                // 470
    TypeCode::TotalCheckPaidCumulativeMtd,                   // 471
    TypeCode::CumulativeChecksPaid,                          // 472
    TypeCode::Unknown,                                       // 473
    TypeCode::CertifiedCheckDebit,                           // 474
    TypeCode::CheckPaid,                                     // 475
    TypeCode::FederalReserveBankLetterDebit,                 // 476
    TypeCode::BankOriginatedDebit,                           // 477
    TypeCode::ListPostDebits,                                // 478
    TypeCode::ListPostDebit,                                 // 479
    TypeCode::TotalLoanPayments,                             // 480
    TypeCode::IndividualLoanPayment,                         // 481
    TypeCode::TotalBankOriginatedDebits,                     // 482
    TypeCode::Unknown,                                       // 483
    TypeCode::Draft,                                         // 484
    TypeCode::DtcDebit,                                      // 485
    TypeCode::TotalCashLetterDebits,                         // 486
    TypeCode::CashLetterDebit,                               // 487
    TypeCode::Unknown,                                       // 488
    TypeCode::CashLetterAdjustmentDb,                        // 489
    TypeCode::TotalOutgoingMoneyTransfers,                   // 490
    TypeCode::IndividualOutgoingInternalMoneyTransfer,       // 491
    TypeCode::Unknown,                                       // 492
    TypeCode::CustomerTerminalInitiatedMoneyTransfer,        // 493
    TypeCode::Unknown,                                       // 494
    TypeCode::OutgoingMoneyTransfer,                         // 495
    TypeCode::MoneyTransferAdjustmentDb,                     // 496
    TypeCode::Unknown,                                       // 497
    TypeCode::CompensationDb,                                // 498
    TypeCode::Unknown,                                       // 499
    TypeCode::TotalAutomaticTransferDebits,                  // 500
    TypeCode::IndividualAutomaticTransferDebit,              // 501
    TypeCode::BondOperationsDebit,                           // 502
    TypeCode::Unknown,                                       // 503
    TypeCode::Unknown,                                       // 504
    TypeCode::TotalBookTransferDebits,                       // 505
    TypeCode::BookTransferDebit,                             // 506
    TypeCode::TotalInternationalMoneyTransferDebits,         // 507
    TypeCode::IndividualInternationalMoneyTransferDebits,    // 508
    TypeCode::Unknown,                                       // 509
    TypeCode::TotalInternationalDebits,                      // 510
    TypeCode::Unknown,                                       // 511
    TypeCode::LetterOfCreditDebit,                           // 512
    TypeCode::LetterOfCreditDb,                              // 513
    TypeCode::ForeignExchangeDebit,                          // 514
    TypeCode::TotalLettersOfCreditDb,                        // 515
    TypeCode::ForeignRemittanceDebit,                        // 516
    TypeCode::Unknown,                                       // 517
    TypeCode::ForeignCollectionDebit,                        // 518
    TypeCode::Unknown,                                       // 519
    TypeCode::Unknown,                                       // 520
    TypeCode::Unknown,                                       // 521
    TypeCode::ForeignChecksPaid,                             // 522
    TypeCode::Unknown,                                       // 523
    TypeCode::CommissionDb,                                  // 524
    TypeCode::Unknown,                                       // 525
    TypeCode::InternationalMoneyMarketTradingDb,             // 526
    TypeCode::StandingOrderDb,                               // 527
    TypeCode::Unknown,                                       // 528
    TypeCode::MiscellaneousInternationalDebit,               // 529
    TypeCode::TotalSecurityDebits,                           // 530
    TypeCode::SecuritiesPurchased,                           // 531
    TypeCode::TotalAmountOfSecuritiesPurchased,              // 532
    TypeCode::SecurityCollectionDebit,                       // 533
    TypeCode::TotalMiscellaneousSecuritiesDbFf,              // 534
    TypeCode::PurchaseOfEquitySecurities,                    // 535
    TypeCode::TotalMiscellaneousSecuritiesDebitChf,          // 536
    TypeCode::TotalCollectionDebit,                          // 537
    TypeCode::MaturedRepurchaseOrder,                        // 538
    TypeCode::TotalBankersAcceptancesDebit,                  // 539
    TypeCode::CouponCollectionDebit,                         // 540
    TypeCode::BankersAcceptancesDb,                          // 541
    TypeCode::PurchaseOfDebtSecurities,                      // 542
    TypeCode::DomesticCollection,                            // 543
    TypeCode::InterestMaturedPrincipalPaymentDb,             // 544
    TypeCode::Unknown,                                       // 545
    TypeCode::CommercialPaperDb,                             // 546
    TypeCode::CapitalChangeDb,                               // 547
    TypeCode::SavingsBondsSalesAdjustmentDb,                 // 548
    TypeCode::MiscellaneousSecurityDebit,                    // 549
    TypeCode::TotalDepositedItemsReturned,                   // 550
    TypeCode::TotalCreditReversals,                          // 551
    TypeCode::CreditReversal,                                // 552
    TypeCode::Unknown,                                       // 553
    TypeCode::PostingErrorCorrectionDebit,                   // 554
    TypeCode::DepositedItemReturned,                         // 555
    TypeCode::TotalAchReturnItemsDb,                         // 556
    TypeCode::IndividualAchReturnItemDb,                     // 557
    TypeCode::AchReversalDebit,                              // 558
    TypeCode::Unknown,                                       // 559
    TypeCode::TotalRejectedDebits,                           // 560
    TypeCode::IndividualRejectedDebit,                       // 561
    TypeCode::Unknown,                                       // 562
    TypeCode::OverdraftDb,                                   // 563
    TypeCode::OverdraftFee,                                  // 564
    TypeCode::Unknown,                                       // 565
    TypeCode::ReturnItemDb,                                  // 566
    TypeCode::ReturnItemFee,                                 // 567
    TypeCode::ReturnItemAdjustmentDb,                        // 568
    TypeCode::Unknown,                                       // 569
    TypeCode::TotalZbaDebits,                                // 570
    TypeCode::Unknown,                                       // 571
    TypeCode::Unknown,                                       // 572
    TypeCode::Unknown,                                       // 573
    TypeCode::CumulativeZbaDebits,                           // 574
    TypeCode::ZbaDebit,                                      // 575
    TypeCode::Unknown,                                       // 576
    TypeCode::ZbaDebitTransfer,                              // 577
    TypeCode::ZbaDebitAdjustment,                            // 578
    TypeCode::Unknown,                                       // 579
    TypeCode::TotalControlledDisbursingDebits,               // 580
    TypeCode::IndividualControlledDisbursingDebit,           // 581
    TypeCode::Unknown,                                       // 582
    TypeCode::TotalDisbursingChecksPaidEarlyAmount,          // 583
    TypeCode::TotalDisbursingChecksPaidLaterAmount,          // 584
    TypeCode::DisbursingFundingRequirement,                  // 585
    TypeCode::FrbPresentmentEstimateFedEstimate,             // 586
    TypeCode::LateDebitsAfterNotification,                   // 587
    TypeCode::TotalDisbursingChecksPaidLastAmount,           // 588
    TypeCode::Unknown,                                       // 589
    TypeCode::TotalDtcDebits,                                // 590
    TypeCode::Unknown,                                       // 591
    TypeCode::Unknown,                                       // 592
    TypeCode::Unknown,                                       // 593
    TypeCode::TotalAtmDebits,                                // 594
    TypeCode::AtmDebit,                                      // 595
    TypeCode::TotalAprDebits,                                // 596
    TypeCode::ArpDebit,                                      // 597
    TypeCode::Unknown,                                       // 598
    TypeCode::Unknown,                                       // 599
    TypeCode::Unknown,                                       // 600
    TypeCode::EstimatedTotalDisbursement,                    // 601
    TypeCode::AdjustedTotalDisbursement,                     // 602
    TypeCode::Unknown,                                       // 603
    TypeCode::Unknown,                                       // 604
    TypeCode::Unknown,                                       // 605
    TypeCode::Unknown,                                       // 606
    TypeCode::Unknown,                                       // 607
    TypeCode::Unknown,                                       // 608
    TypeCode::Unknown,                                       // 609
    TypeCode::TotalFundsRequired,                            // 610
    TypeCode::TotalWireTransfersOutChf,                      // 611
    TypeCode::TotalWireTransfersOutFf,                       // 612
    TypeCode::TotalInternationalDebitChf,                    // 613
    TypeCode::TotalInternationalDebitFf,                     // 614
    TypeCode::TotalFederalReserveBankCommercialBankDebit,    // 615
    TypeCode::FederalReserveBankCommercialBankDebit,         // 616
    TypeCode::TotalSecuritiesPurchasedChf,                   // 617
    TypeCode::TotalSecuritiesPurchasedFf,                    // 618
    TypeCode::Unknown,                                       // 619
    TypeCode::Unknown,                                       // 620
    TypeCode::TotalBrokerDebitsChf,                          // 621
    TypeCode::BrokerDebit,                                   // 622
    TypeCode::TotalBrokerDebitsFf,                           // 623
    TypeCode::Unknown,                                       // 624
    TypeCode::TotalBrokerDebits,                             // 625
    TypeCode::TotalFedFundsPurchased,                        // 626
    TypeCode::FedFundsPurchased,                             // 627
    TypeCode::TotalCashCenterDebits,                         // 628
    TypeCode::CashCenterDebit,                               // 629
    TypeCode::TotalDebitAdjustments,                         // 630
    TypeCode::DebitAdjustment,                               // 631
    TypeCode::TotalTrustDebits,                              // 632
    TypeCode::TrustDebit,                                    // 633
    TypeCode::YtdAdjustmentDebit,                            // 634
    TypeCode::Unknown,                                       // 635
    TypeCode::Unknown,                                       // 636
    TypeCode::Unknown,                                       // 637
    TypeCode::Unknown,                                       // 638
    TypeCode::Unknown,                                       // 639
    TypeCode::TotalEscrowDebits,                             // 640
    TypeCode::IndividualEscrowDebit,                         // 641
    TypeCode::Unknown,                                       // 642
    TypeCode::Unknown,                                       // 643
    TypeCode::IndividualBackValueDebit,                      // 644
    TypeCode::Unknown,                                       // 645
    TypeCode::TransferCalculationDebit,                      // 646
    TypeCode::Unknown,                                       // 647
    TypeCode::Unknown,                                       // 648
    TypeCode::Unknown,                                       // 649
    TypeCode::InvestmentsPurchased,                          // 650
    TypeCode::IndividualInvestmentPurchased,                 // 651
    TypeCode::Unknown,                                       // 652
    TypeCode::Unknown,                                       // 653
    TypeCode::InterestDebit,                                 // 654
    TypeCode::TotalInvestmentInterestDebits,                 // 655
    TypeCode::SweepPrincipalBuy,                             // 656
    TypeCode::FuturesDebit,                                  // 657
    TypeCode::PrincipalPaymentsDebit,                        // 658
    TypeCode::InterestAdjustmentDebit,                       // 659
    TypeCode::Unknown,                                       // 660
    TypeCode::AccountAnalysisFee,                            // 661
    TypeCode::CorrespondentCollectionDebit,                  // 662
    TypeCode::CorrespondentCollectionAdjustmentDb,           // 663
    TypeCode::LoanParticipationDb,                           // 664
    TypeCode::InterceptDebits,                               // 665
    TypeCode::CurrencyAndCoinShipped,                        // 666
    TypeCode::FoodStampLetterDb,                             // 667
    TypeCode::FoodStampAdjustmentDb,                         // 668
    TypeCode::ClearingSettlementDebit,                       // 669
    TypeCode::TotalBackValueDebits,                          // 670
    TypeCode::Unknown,                                       // 671
    TypeCode::BackValueAdjustmentDb,                         // 672
    TypeCode::CustomerPayrollDb,                             // 673
    TypeCode::FrbStatementRecapDb,                           // 674
    TypeCode::Unknown,                                       // 675
    TypeCode::SavingsBondLetterOrAdjustmentDb,               // 676
    TypeCode::TreasuryTaxAndLoanDebit,                       // 677
    TypeCode::TransferOfTreasuryDebit,                       // 678
    TypeCode::FrbGovernmentChecksCashLetterDebit,            // 679
    TypeCode::Unknown,                                       // 680
    TypeCode::FrbGovernmentCheckAdjustmentDb,                // 681
    TypeCode::FrbPostalMoneyOrderDebit,                      // 682
    TypeCode::FrbPostalMoneyOrderAdjustmentDb,               // 683
    TypeCode::FrbCashLetterAutoChargeDebit,                  // 684
    TypeCode::TotalUniversalDebits,                          // 685
    TypeCode::FrbCashLetterAutoChargeAdjustmentDb,           // 686
    TypeCode::FrbFineSortCashLetterDebit,                    // 687
    TypeCode::FrbFineSortAdjustmentDb,                       // 688
    TypeCode::FrbFreightPaymentDebits,                       // 689
    TypeCode::TotalMiscellaneousDebits,                      // 690
    TypeCode::UniversalDebit,                                // 691
    TypeCode::FreightPaymentDebit,                           // 692
    TypeCode::ItemizedDebitOver10,                           // 693
    TypeCode::DepositReversal,                               // 694
    TypeCode::DepositCorrectionDebit,                        // 695
    TypeCode::RegularCollectionDebit,                        // 696
    TypeCode::CumulativeDebits,                              // 697
    TypeCode::MiscellaneousFees,                             // 698
    TypeCode::MiscellaneousDebit,                            // 699
    TypeCode::Unknown,                                       // 700
    TypeCode::PrincipalLoanBalance,                          // 701
    TypeCode::Unknown,                                       // 702
    TypeCode::AvailableCommitmentAmount,                     // 703
    TypeCode::Unknown,                                       // 704
    TypeCode::PaymentAmountDue,                              // 705
    TypeCode::Unknown,                                       // 706
    TypeCode::PrincipalAmountPastDue,                        // 707
    TypeCode::Unknown,                                       // 708
    TypeCode::InterestAmountPastDue,                         // 709
    TypeCode::Unknown,                                       // 710
    TypeCode::Unknown,                                       // 711
    TypeCode::Unknown,                                       // 712
    TypeCode::Unknown,                                       // 713
    TypeCode::Unknown,                                       // 714
    TypeCode::Unknown,                                       // 715
    TypeCode::Unknown,                                       // 716
    TypeCode::Unknown,                                       // 717
    TypeCode::Unknown,                                       // 718
    TypeCode::Unknown,                                       // 719
    TypeCode::TotalLoanPayment,                              // 720
    TypeCode::AmountAppliedToInterest,                       // 721
    TypeCode::AmountAppliedToPrincipal,                      // 722
    TypeCode::AmountAppliedToEscrow,                         // 723
    TypeCode::AmountAppliedToLateCharges,                    // 724
    TypeCode::AmountAppliedToBuydown,                        // 725
    TypeCode::AmountAppliedToMiscFees,                       // 726
    TypeCode::AmountAppliedToDeferredInterestDetail,         // 727
    TypeCode::AmountAppliedToServiceCharge,                  // 728
    TypeCode::Unknown,                                       // 729
    TypeCode::Unknown,                                       // 730
    TypeCode::Unknown,                                       // 731
    TypeCode::Unknown,                                       // 732
    TypeCode::Unknown,                                       // 733
    TypeCode::Unknown,                                       // 734
    TypeCode::Unknown,                                       // 735
    TypeCode::Unknown,                                       // 736
    TypeCode::Unknown,                                       // 737
    TypeCode::Unknown,                                       // 738
    TypeCode::Unknown,                                       // 739
    TypeCode::Unknown,                                       // 740
    TypeCode::Unknown,                                       // 741
    TypeCode::Unknown,                                       // 742
    TypeCode::Unknown,                                       // 743
    TypeCode::Unknown,                                       // 744
    TypeCode::Unknown,                                       // 745
    TypeCode::Unknown,                                       // 746
    TypeCode::Unknown,                                       // 747
    TypeCode::Unknown,                                       // 748
    TypeCode::Unknown,                                       // 749
    TypeCode::Unknown,                                       // 750
    TypeCode::Unknown,                                       // 751
    TypeCode::Unknown,                                       // 752
    TypeCode::Unknown,                                       // 753
    TypeCode::Unknown,                                       // 754
    TypeCode::Unknown,                                       // 755
    TypeCode::Unknown,                                       // 756
    TypeCode::Unknown,                                       // 757
    TypeCode::Unknown,                                       // 758
    TypeCode::Unknown,                                       // 759
    TypeCode::LoanDisbursement,                              // 760
    TypeCode::Unknown,                                       // 761
    TypeCode::Unknown,                                       // 762
    TypeCode::Unknown,                                       // 763
    TypeCode::Unknown,                                       // 764
    TypeCode::Unknown,                                       // 765
    TypeCode::Unknown,                                       // 766
    TypeCode::Unknown,                                       // 767
    TypeCode::Unknown,                                       // 768
    TypeCode::Unknown,                                       // 769
    TypeCode::Unknown,                                       // 770
    TypeCode::Unknown,                                       // 771
    TypeCode::Unknown,                                       // 772
    TypeCode::Unknown,                                       // 773
    TypeCode::Unknown,                                       // 774
    TypeCode::Unknown,                                       // 775
    TypeCode::Unknown,                                       // 776
    TypeCode::Unknown,                                       // 777
    TypeCode::Unknown,                                       // 778
    TypeCode::Unknown,                                       // 779
    TypeCode::Unknown,                                       // 780
    TypeCode::Unknown,                                       // 781
    TypeCode::Unknown,                                       // 782
    TypeCode::Unknown,                                       // 783
    TypeCode::Unknown,                                       // 784
    TypeCode::Unknown,                                       // 785
    TypeCode::Unknown,                                       // 786
    TypeCode::Unknown,                                       // 787
    TypeCode::Unknown,                                       // 788
    TypeCode::Unknown,                                       // 789
    TypeCode::Unknown,                                       // 790
    TypeCode::Unknown,                                       // 791
    TypeCode::Unknown,                                       // 792
    TypeCode::Unknown,                                       // 793
    TypeCode::Unknown,                                       // 794
    TypeCode::Unknown,                                       // 795
    TypeCode::Unknown,                                       // 796
    TypeCode::Unknown,                                       // 797
    TypeCode::Unknown,                                       // 798
    TypeCode::Unknown,                                       // 799
    TypeCode::Unknown,                                       // 800
    TypeCode::Unknown,                                       // 801
    TypeCode::Unknown,                                       // 802
    TypeCode::Unknown,                                       // 803
    TypeCode::Unknown,                                       // 804
    TypeCode::Unknown,                                       // 805
    TypeCode::Unknown,                                       // 806
    TypeCode::Unknown,                                       // 807
    TypeCode::Unknown,                                       // 808
    TypeCode::Unknown,                                       // 809
    TypeCode::Unknown,                                       // 810
    TypeCode::Unknown,                                       // 811
    TypeCode::Unknown,                                       // 812
    TypeCode::Unknown,                                       // 813
    TypeCode::Unknown,                                       // 814
    TypeCode::Unknown,                                       // 815
    TypeCode::Unknown,                                       // 816
    TypeCode::Unknown,                                       // 817
    TypeCode::Unknown,                                       // 818
    TypeCode::Unknown,                                       // 819
    TypeCode::Unknown,                                       // 820
    TypeCode::Unknown,                                       // 821
    TypeCode::Unknown,                                       // 822
    TypeCode::Unknown,                                       // 823
    TypeCode::Unknown,                                       // 824
    TypeCode::Unknown,                                       // 825
    TypeCode::Unknown,                                       // 826
    TypeCode::Unknown,                                       // 827
    TypeCode::Unknown,                                       // 828
    TypeCode::Unknown,                                       // 829
    TypeCode::Unknown,                                       // 830
    TypeCode::Unknown,                                       // 831
    TypeCode::Unknown,                                       // 832
    TypeCode::Unknown,                                       // 833
    TypeCode::Unknown,                                       // 834
    TypeCode::Unknown,                                       // 835
    TypeCode::Unknown,                                       // 836
    TypeCode::Unknown,                                       // 837
    TypeCode::Unknown,                                       // 838
    TypeCode::Unknown,                                       // 839
    TypeCode::Unknown,                                       // 840
    TypeCode::Unknown,                                       // 841
    TypeCode::Unknown,                                       // 842
    TypeCode::Unknown,                                       // 843
    TypeCode::Unknown,                                       // 844
    TypeCode::Unknown,                                       // 845
    TypeCode::Unknown,                                       // 846
    TypeCode::Unknown,                                       // 847
    TypeCode::Unknown,                                       // 848
    TypeCode::Unknown,                                       // 849
    TypeCode::Unknown,                                       // 850
    TypeCode::Unknown,                                       // 851
    TypeCode::Unknown,                                       // 852
    TypeCode::Unknown,                                       // 853
    TypeCode::Unknown,                                       // 854
    TypeCode::Unknown,                                       // 855
    TypeCode::Unknown,                                       // 856
    TypeCode::Unknown,                                       // 857
    TypeCode::Unknown,                                       // 858
    TypeCode::Unknown,                                       // 859
    TypeCode::Unknown,                                       // 860
    TypeCode::Unknown,                                       // 861
    TypeCode::Unknown,                                       // 862
    TypeCode::Unknown,                                       // 863
    TypeCode::Unknown,                                       // 864
    TypeCode::Unknown,                                       // 865
    TypeCode::Unknown,                                       // 866
    TypeCode::Unknown,                                       // 867
    TypeCode::Unknown,                                       // 868
    TypeCode::Unknown,                                       // 869
    TypeCode::Unknown,                                       // 870
    TypeCode::Unknown,                                       // 871
    TypeCode::Unknown,                                       // 872
    TypeCode::Unknown,                                       // 873
    TypeCode::Unknown,                                       // 874
    TypeCode::Unknown,                                       // 875
    TypeCode::Unknown,                                       // 876
    TypeCode::Unknown,                                       // 877
    TypeCode::Unknown,                                       // 878
    TypeCode::Unknown,                                       // 879
    TypeCode::Unknown,                                       // 880
    TypeCode::Unknown,                                       // 881
    TypeCode::Unknown,                                       // 882
    TypeCode::Unknown,                                       // 883
    TypeCode::Unknown,                                       // 884
    TypeCode::Unknown,                                       // 885
    TypeCode::Unknown,                                       // 886
    TypeCode::Unknown,                                       // 887
    TypeCode::Unknown,                                       // 888
    TypeCode::Unknown,                                       // 889
    TypeCode::ContainsNonMonetaryInformation,                // 890
    TypeCode::Unknown,                                       // 891
    TypeCode::Unknown,                                       // 892
    TypeCode::Unknown,                                       // 893
    TypeCode::Unknown,                                       // 894
    TypeCode::Unknown,                                       // 895
    TypeCode::Unknown,                                       // 896
    TypeCode::Unknown,                                       // 897
    TypeCode::Unknown,                                       // 898
    TypeCode::Unknown,                                       // 899
    TypeCode::Unknown,                                       // 900
    TypeCode::Unknown,                                       // 901
    TypeCode::Unknown,                                       // 902
    TypeCode::Unknown,                                       // 903
    TypeCode::Unknown,                                       // 904
    TypeCode::Unknown,                                       // 905
    TypeCode::Unknown,                                       // 906
    TypeCode::Unknown,                                       // 907
    TypeCode::Unknown,                                       // 908
    TypeCode::Unknown,                                       // 909
    TypeCode::Unknown,                                       // 910
    TypeCode::Unknown,                                       // 911
    TypeCode::Unknown,                                       // 912
    TypeCode::Unknown,                                       // 913
    TypeCode::Unknown,                                       // 914
    TypeCode::Unknown,                                       // 915
    TypeCode::Unknown,                                       // 916
    TypeCode::Unknown,                                       // 917
    TypeCode::Unknown,                                       // 918
    TypeCode::Unknown,                                       // 919
    TypeCode::Unknown,                                       // 920
    TypeCode::Unknown,                                       // 921
    TypeCode::Unknown,                                       // 922
    TypeCode::Unknown,                                       // 923
    TypeCode::Unknown,                                       // 924
    TypeCode::Unknown,                                       // 925
    TypeCode::Unknown,                                       // 926
    TypeCode::Unknown,                                       // 927
    TypeCode::Unknown,                                       // 928
    TypeCode::Unknown,                                       // 929
    TypeCode::Unknown,                                       // 930
    TypeCode::Unknown,                                       // 931
    TypeCode::Unknown,                                       // 932
    TypeCode::Unknown,                                       // 933
    TypeCode::Unknown,                                       // 934
    TypeCode::Unknown,                                       // 935
    TypeCode::Unknown,                                       // 936
    TypeCode::Unknown,                                       // 937
    TypeCode::Unknown,                                       // 938
    TypeCode::Unknown,                                       // 939
    TypeCode::Unknown,                                       // 940
    TypeCode::Unknown,                                       // 941
    TypeCode::Unknown,                                       // 942
    TypeCode::Unknown,                                       // 943
    TypeCode::Unknown,                                       // 944
    TypeCode::Unknown,                                       // 945
    TypeCode::Unknown,                                       // 946
    TypeCode::Unknown,                                       // 947
    TypeCode::Unknown,                                       // 948
    TypeCode::Unknown,                                       // 949
    TypeCode::Unknown,                                       // 950
    TypeCode::Unknown,                                       // 951
    TypeCode::Unknown,                                       // 952
    TypeCode::Unknown,                                       // 953
    TypeCode::Unknown,                                       // 954
    TypeCode::Unknown,                                       // 955
    TypeCode::Unknown,                                       // 956
    TypeCode::Unknown,                                       // 957
    TypeCode::Unknown,                                       // 958
    TypeCode::Unknown,                                       // 959
    TypeCode::Unknown,                                       // 960
    TypeCode::Unknown,                                       // 961
    TypeCode::Unknown,                                       // 962
    TypeCode::Unknown,                                       // 963
    TypeCode::Unknown,                                       // 964
    TypeCode::Unknown,                                       // 965
    TypeCode::Unknown,                                       // 966
    TypeCode::Unknown,                                       // 967
    TypeCode::Unknown,                                       // 968
    TypeCode::Unknown,                                       // 969
    TypeCode::Unknown,                                       // 970
    TypeCode::Unknown,                                       // 971
    TypeCode::Unknown,                                       // 972
    TypeCode::Unknown,                                       // 973
    TypeCode::Unknown,                                       // 974
    TypeCode::Unknown,                                       // 975
    TypeCode::Unknown,                                       // 976
    TypeCode::Unknown,                                       // 977
    TypeCode::Unknown,                                       // 978
    TypeCode::Unknown,                                       // 979
    TypeCode::Unknown,                                       // 980
    TypeCode::Unknown,                                       // 981
    TypeCode::Unknown,                                       // 982
    TypeCode::Unknown,                                       // 983
    TypeCode::Unknown,                                       // 984
    TypeCode::Unknown,                                       // 985
    TypeCode::Unknown,                                       // 986
    TypeCode::Unknown,                                       // 987
    TypeCode::Unknown,                                       // 988
    TypeCode::Unknown,                                       // 989
    TypeCode::Unknown,                                       // 990
    TypeCode::Unknown,                                       // 991
    TypeCode::Unknown,                                       // 992
    TypeCode::Unknown,                                       // 993
    TypeCode::Unknown,                                       // 994
    TypeCode::Unknown,                                       // 995
    TypeCode::Unknown,                                       // 996
    TypeCode::Unknown,                                       // 997
    TypeCode::Unknown,                                       // 998
    TypeCode::Unknown,                                       // 999
];

impl From<&str> for TypeCode {
    /// Parses a three-digit BAI2 type code string into a [`TypeCode`].
    ///
    /// Returns [`TypeCode::Unknown`] for any code not defined in Appendix A,
    /// including any string that isn't exactly 3 ASCII digits.
    fn from(code: &str) -> Self {
        let bytes = code.as_bytes();
        if let [a @ b'0'..=b'9', b @ b'0'..=b'9', c @ b'0'..=b'9'] = *bytes {
            let n = (a - b'0') as usize * 100 + (b - b'0') as usize * 10 + (c - b'0') as usize;
            TYPE_CODE_TABLE[n]
        } else {
            Self::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_unknown_code_yields_unknown_variant() {
        assert_eq!(TypeCode::from("999999"), TypeCode::Unknown);
        assert_eq!(TypeCode::from(""), TypeCode::Unknown);
        assert_eq!(TypeCode::from("12"), TypeCode::Unknown);
        assert_eq!(TypeCode::from("abc"), TypeCode::Unknown);
        assert_eq!(TypeCode::from("999"), TypeCode::Unknown);
        assert_eq!(TypeCode::Unknown.transaction(), None);
        assert_eq!(TypeCode::Unknown.level(), None);
    }

    #[test]
    fn every_defined_code_round_trips_through_from_and_code() {
        assert_eq!(TypeCode::from("010"), TypeCode::OpeningLedger);
        assert_eq!(TypeCode::OpeningLedger.code(), "010");
        assert_eq!(TypeCode::from("011"), TypeCode::AverageOpeningLedgerMtd);
        assert_eq!(TypeCode::AverageOpeningLedgerMtd.code(), "011");
        assert_eq!(TypeCode::from("012"), TypeCode::AverageOpeningLedgerYtd);
        assert_eq!(TypeCode::AverageOpeningLedgerYtd.code(), "012");
        assert_eq!(TypeCode::from("015"), TypeCode::ClosingLedger);
        assert_eq!(TypeCode::ClosingLedger.code(), "015");
        assert_eq!(TypeCode::from("020"), TypeCode::AverageClosingLedgerMtd);
        assert_eq!(TypeCode::AverageClosingLedgerMtd.code(), "020");
        assert_eq!(
            TypeCode::from("021"),
            TypeCode::AverageClosingLedgerPreviousMonth
        );
        assert_eq!(TypeCode::AverageClosingLedgerPreviousMonth.code(), "021");
        assert_eq!(TypeCode::from("022"), TypeCode::AggregateBalanceAdjustments);
        assert_eq!(TypeCode::AggregateBalanceAdjustments.code(), "022");
        assert_eq!(
            TypeCode::from("024"),
            TypeCode::AverageClosingLedgerYtdPreviousMonth
        );
        assert_eq!(TypeCode::AverageClosingLedgerYtdPreviousMonth.code(), "024");
        assert_eq!(TypeCode::from("025"), TypeCode::AverageClosingLedgerYtd);
        assert_eq!(TypeCode::AverageClosingLedgerYtd.code(), "025");
        assert_eq!(TypeCode::from("030"), TypeCode::CurrentLedger);
        assert_eq!(TypeCode::CurrentLedger.code(), "030");
        assert_eq!(TypeCode::from("037"), TypeCode::AchNetPosition);
        assert_eq!(TypeCode::AchNetPosition.code(), "037");
        assert_eq!(
            TypeCode::from("039"),
            TypeCode::OpeningAvailableTotalSameDayAchDtcDeposit
        );
        assert_eq!(
            TypeCode::OpeningAvailableTotalSameDayAchDtcDeposit.code(),
            "039"
        );
        assert_eq!(TypeCode::from("040"), TypeCode::OpeningAvailable);
        assert_eq!(TypeCode::OpeningAvailable.code(), "040");
        assert_eq!(TypeCode::from("041"), TypeCode::AverageOpeningAvailableMtd);
        assert_eq!(TypeCode::AverageOpeningAvailableMtd.code(), "041");
        assert_eq!(TypeCode::from("042"), TypeCode::AverageOpeningAvailableYtd);
        assert_eq!(TypeCode::AverageOpeningAvailableYtd.code(), "042");
        assert_eq!(
            TypeCode::from("043"),
            TypeCode::AverageAvailablePreviousMonth
        );
        assert_eq!(TypeCode::AverageAvailablePreviousMonth.code(), "043");
        assert_eq!(
            TypeCode::from("044"),
            TypeCode::DisbursingOpeningAvailableBalance
        );
        assert_eq!(TypeCode::DisbursingOpeningAvailableBalance.code(), "044");
        assert_eq!(TypeCode::from("045"), TypeCode::ClosingAvailable);
        assert_eq!(TypeCode::ClosingAvailable.code(), "045");
        assert_eq!(TypeCode::from("050"), TypeCode::AverageClosingAvailableMtd);
        assert_eq!(TypeCode::AverageClosingAvailableMtd.code(), "050");
        assert_eq!(
            TypeCode::from("051"),
            TypeCode::AverageClosingAvailableLastMonth
        );
        assert_eq!(TypeCode::AverageClosingAvailableLastMonth.code(), "051");
        assert_eq!(
            TypeCode::from("054"),
            TypeCode::AverageClosingAvailableYtdLastMonth
        );
        assert_eq!(TypeCode::AverageClosingAvailableYtdLastMonth.code(), "054");
        assert_eq!(TypeCode::from("055"), TypeCode::AverageClosingAvailableYtd);
        assert_eq!(TypeCode::AverageClosingAvailableYtd.code(), "055");
        assert_eq!(TypeCode::from("056"), TypeCode::LoanBalance);
        assert_eq!(TypeCode::LoanBalance.code(), "056");
        assert_eq!(TypeCode::from("057"), TypeCode::TotalInvestmentPosition);
        assert_eq!(TypeCode::TotalInvestmentPosition.code(), "057");
        assert_eq!(
            TypeCode::from("059"),
            TypeCode::CurrentAvailableCrsSupressed
        );
        assert_eq!(TypeCode::CurrentAvailableCrsSupressed.code(), "059");
        assert_eq!(TypeCode::from("060"), TypeCode::CurrentAvailable);
        assert_eq!(TypeCode::CurrentAvailable.code(), "060");
        assert_eq!(TypeCode::from("061"), TypeCode::AverageCurrentAvailableMtd);
        assert_eq!(TypeCode::AverageCurrentAvailableMtd.code(), "061");
        assert_eq!(TypeCode::from("062"), TypeCode::AverageCurrentAvailableYtd);
        assert_eq!(TypeCode::AverageCurrentAvailableYtd.code(), "062");
        assert_eq!(TypeCode::from("063"), TypeCode::TotalFloat);
        assert_eq!(TypeCode::TotalFloat.code(), "063");
        assert_eq!(TypeCode::from("065"), TypeCode::TargetBalance);
        assert_eq!(TypeCode::TargetBalance.code(), "065");
        assert_eq!(TypeCode::from("066"), TypeCode::AdjustedBalance);
        assert_eq!(TypeCode::AdjustedBalance.code(), "066");
        assert_eq!(TypeCode::from("067"), TypeCode::AdjustedBalanceMtd);
        assert_eq!(TypeCode::AdjustedBalanceMtd.code(), "067");
        assert_eq!(TypeCode::from("068"), TypeCode::AdjustedBalanceYtd);
        assert_eq!(TypeCode::AdjustedBalanceYtd.code(), "068");
        assert_eq!(TypeCode::from("070"), TypeCode::N0DayFloat);
        assert_eq!(TypeCode::N0DayFloat.code(), "070");
        assert_eq!(TypeCode::from("072"), TypeCode::N1DayFloat);
        assert_eq!(TypeCode::N1DayFloat.code(), "072");
        assert_eq!(TypeCode::from("073"), TypeCode::FloatAdjustmentNa);
        assert_eq!(TypeCode::FloatAdjustmentNa.code(), "073");
        assert_eq!(TypeCode::from("074"), TypeCode::N2OrMoreDaysFloat);
        assert_eq!(TypeCode::N2OrMoreDaysFloat.code(), "074");
        assert_eq!(TypeCode::from("075"), TypeCode::N3OrMoreDaysFloat);
        assert_eq!(TypeCode::N3OrMoreDaysFloat.code(), "075");
        assert_eq!(TypeCode::from("076"), TypeCode::AdjustmentToBalances);
        assert_eq!(TypeCode::AdjustmentToBalances.code(), "076");
        assert_eq!(
            TypeCode::from("077"),
            TypeCode::AverageAdjustmentToBalancesMtd
        );
        assert_eq!(TypeCode::AverageAdjustmentToBalancesMtd.code(), "077");
        assert_eq!(
            TypeCode::from("078"),
            TypeCode::AverageAdjustmentToBalancesYtd
        );
        assert_eq!(TypeCode::AverageAdjustmentToBalancesYtd.code(), "078");
        assert_eq!(TypeCode::from("079"), TypeCode::N4DayFloat);
        assert_eq!(TypeCode::N4DayFloat.code(), "079");
        assert_eq!(TypeCode::from("080"), TypeCode::N5DayFloat);
        assert_eq!(TypeCode::N5DayFloat.code(), "080");
        assert_eq!(TypeCode::from("081"), TypeCode::N6DayFloat);
        assert_eq!(TypeCode::N6DayFloat.code(), "081");
        assert_eq!(TypeCode::from("082"), TypeCode::Average1DayFloatMtd);
        assert_eq!(TypeCode::Average1DayFloatMtd.code(), "082");
        assert_eq!(TypeCode::from("083"), TypeCode::Average1DayFloatYtd);
        assert_eq!(TypeCode::Average1DayFloatYtd.code(), "083");
        assert_eq!(TypeCode::from("084"), TypeCode::Average2DayFloatMtd);
        assert_eq!(TypeCode::Average2DayFloatMtd.code(), "084");
        assert_eq!(TypeCode::from("085"), TypeCode::Average2DayFloatYtd);
        assert_eq!(TypeCode::Average2DayFloatYtd.code(), "085");
        assert_eq!(TypeCode::from("086"), TypeCode::TransferCalculation);
        assert_eq!(TypeCode::TransferCalculation.code(), "086");
        assert_eq!(TypeCode::from("100"), TypeCode::TotalCredits);
        assert_eq!(TypeCode::TotalCredits.code(), "100");
        assert_eq!(TypeCode::from("101"), TypeCode::TotalCreditAmountMtd);
        assert_eq!(TypeCode::TotalCreditAmountMtd.code(), "101");
        assert_eq!(TypeCode::from("105"), TypeCode::CreditsNotDetailed);
        assert_eq!(TypeCode::CreditsNotDetailed.code(), "105");
        assert_eq!(TypeCode::from("106"), TypeCode::DepositsSubjectToFloat);
        assert_eq!(TypeCode::DepositsSubjectToFloat.code(), "106");
        assert_eq!(TypeCode::from("107"), TypeCode::TotalAdjustmentCreditsYtd);
        assert_eq!(TypeCode::TotalAdjustmentCreditsYtd.code(), "107");
        assert_eq!(TypeCode::from("108"), TypeCode::CreditAnyType);
        assert_eq!(TypeCode::CreditAnyType.code(), "108");
        assert_eq!(
            TypeCode::from("109"),
            TypeCode::CurrentDayTotalLockboxDeposits
        );
        assert_eq!(TypeCode::CurrentDayTotalLockboxDeposits.code(), "109");
        assert_eq!(TypeCode::from("110"), TypeCode::TotalLockboxDeposits);
        assert_eq!(TypeCode::TotalLockboxDeposits.code(), "110");
        assert_eq!(TypeCode::from("115"), TypeCode::LockboxDeposit);
        assert_eq!(TypeCode::LockboxDeposit.code(), "115");
        assert_eq!(TypeCode::from("116"), TypeCode::ItemInLockboxDeposit);
        assert_eq!(TypeCode::ItemInLockboxDeposit.code(), "116");
        assert_eq!(TypeCode::from("118"), TypeCode::LockboxAdjustmentCredit);
        assert_eq!(TypeCode::LockboxAdjustmentCredit.code(), "118");
        assert_eq!(
            TypeCode::from("120"),
            TypeCode::EdiTransactionCreditCrSummary
        );
        assert_eq!(TypeCode::EdiTransactionCreditCrSummary.code(), "120");
        assert_eq!(
            TypeCode::from("121"),
            TypeCode::EdiTransactionCreditCrDetail
        );
        assert_eq!(TypeCode::EdiTransactionCreditCrDetail.code(), "121");
        assert_eq!(TypeCode::from("122"), TypeCode::EdibanxCreditReceived);
        assert_eq!(TypeCode::EdibanxCreditReceived.code(), "122");
        assert_eq!(TypeCode::from("123"), TypeCode::EdibanxCreditReturn);
        assert_eq!(TypeCode::EdibanxCreditReturn.code(), "123");
        assert_eq!(TypeCode::from("130"), TypeCode::TotalConcentrationCredits);
        assert_eq!(TypeCode::TotalConcentrationCredits.code(), "130");
        assert_eq!(TypeCode::from("131"), TypeCode::TotalDtcCredits);
        assert_eq!(TypeCode::TotalDtcCredits.code(), "131");
        assert_eq!(TypeCode::from("135"), TypeCode::DtcConcentrationCredit);
        assert_eq!(TypeCode::DtcConcentrationCredit.code(), "135");
        assert_eq!(TypeCode::from("136"), TypeCode::ItemInDtcDeposit);
        assert_eq!(TypeCode::ItemInDtcDeposit.code(), "136");
        assert_eq!(TypeCode::from("140"), TypeCode::TotalAchCredits);
        assert_eq!(TypeCode::TotalAchCredits.code(), "140");
        assert_eq!(TypeCode::from("142"), TypeCode::AchCreditReceived);
        assert_eq!(TypeCode::AchCreditReceived.code(), "142");
        assert_eq!(TypeCode::from("143"), TypeCode::ItemInAchDeposit);
        assert_eq!(TypeCode::ItemInAchDeposit.code(), "143");
        assert_eq!(TypeCode::from("145"), TypeCode::AchConcentrationCredit);
        assert_eq!(TypeCode::AchConcentrationCredit.code(), "145");
        assert_eq!(TypeCode::from("146"), TypeCode::TotalBankCardDeposits);
        assert_eq!(TypeCode::TotalBankCardDeposits.code(), "146");
        assert_eq!(TypeCode::from("147"), TypeCode::IndividualBankCardDeposit);
        assert_eq!(TypeCode::IndividualBankCardDeposit.code(), "147");
        assert_eq!(
            TypeCode::from("150"),
            TypeCode::TotalPreauthorizedPaymentCredits
        );
        assert_eq!(TypeCode::TotalPreauthorizedPaymentCredits.code(), "150");
        assert_eq!(TypeCode::from("155"), TypeCode::PreauthorizedDraftCredit);
        assert_eq!(TypeCode::PreauthorizedDraftCredit.code(), "155");
        assert_eq!(TypeCode::from("156"), TypeCode::ItemInPacDeposit);
        assert_eq!(TypeCode::ItemInPacDeposit.code(), "156");
        assert_eq!(
            TypeCode::from("160"),
            TypeCode::TotalAchDisbursingFundingCredits
        );
        assert_eq!(TypeCode::TotalAchDisbursingFundingCredits.code(), "160");
        assert_eq!(
            TypeCode::from("162"),
            TypeCode::CorporateTradePaymentSettlementCr
        );
        assert_eq!(TypeCode::CorporateTradePaymentSettlementCr.code(), "162");
        assert_eq!(
            TypeCode::from("163"),
            TypeCode::CorporateTradePaymentCredits
        );
        assert_eq!(TypeCode::CorporateTradePaymentCredits.code(), "163");
        assert_eq!(TypeCode::from("164"), TypeCode::CorporateTradePaymentCredit);
        assert_eq!(TypeCode::CorporateTradePaymentCredit.code(), "164");
        assert_eq!(TypeCode::from("165"), TypeCode::PreauthorizedAchCredit);
        assert_eq!(TypeCode::PreauthorizedAchCredit.code(), "165");
        assert_eq!(TypeCode::from("166"), TypeCode::AchSettlementCr);
        assert_eq!(TypeCode::AchSettlementCr.code(), "166");
        assert_eq!(TypeCode::from("167"), TypeCode::AchSettlementCredits);
        assert_eq!(TypeCode::AchSettlementCredits.code(), "167");
        assert_eq!(
            TypeCode::from("168"),
            TypeCode::AchReturnItemOrAdjustmentSettlementCr
        );
        assert_eq!(
            TypeCode::AchReturnItemOrAdjustmentSettlementCr.code(),
            "168"
        );
        assert_eq!(TypeCode::from("169"), TypeCode::MiscellaneousAchCredit);
        assert_eq!(TypeCode::MiscellaneousAchCredit.code(), "169");
        assert_eq!(TypeCode::from("170"), TypeCode::TotalOtherCheckDeposits);
        assert_eq!(TypeCode::TotalOtherCheckDeposits.code(), "170");
        assert_eq!(TypeCode::from("171"), TypeCode::IndividualLoanDeposit);
        assert_eq!(TypeCode::IndividualLoanDeposit.code(), "171");
        assert_eq!(TypeCode::from("172"), TypeCode::DepositCorrection);
        assert_eq!(TypeCode::DepositCorrection.code(), "172");
        assert_eq!(TypeCode::from("173"), TypeCode::BankPreparedDeposit);
        assert_eq!(TypeCode::BankPreparedDeposit.code(), "173");
        assert_eq!(TypeCode::from("174"), TypeCode::OtherDeposit);
        assert_eq!(TypeCode::OtherDeposit.code(), "174");
        assert_eq!(TypeCode::from("175"), TypeCode::CheckDepositPackage);
        assert_eq!(TypeCode::CheckDepositPackage.code(), "175");
        assert_eq!(TypeCode::from("176"), TypeCode::RePresentedCheckDeposit);
        assert_eq!(TypeCode::RePresentedCheckDeposit.code(), "176");
        assert_eq!(TypeCode::from("178"), TypeCode::ListPostCredits);
        assert_eq!(TypeCode::ListPostCredits.code(), "178");
        assert_eq!(TypeCode::from("180"), TypeCode::TotalLoanProceeds);
        assert_eq!(TypeCode::TotalLoanProceeds.code(), "180");
        assert_eq!(TypeCode::from("182"), TypeCode::TotalBankPreparedDeposits);
        assert_eq!(TypeCode::TotalBankPreparedDeposits.code(), "182");
        assert_eq!(TypeCode::from("184"), TypeCode::DraftDeposit);
        assert_eq!(TypeCode::DraftDeposit.code(), "184");
        assert_eq!(TypeCode::from("185"), TypeCode::TotalMiscellaneousDeposits);
        assert_eq!(TypeCode::TotalMiscellaneousDeposits.code(), "185");
        assert_eq!(TypeCode::from("186"), TypeCode::TotalCashLetterCredits);
        assert_eq!(TypeCode::TotalCashLetterCredits.code(), "186");
        assert_eq!(TypeCode::from("187"), TypeCode::CashLetterCredit);
        assert_eq!(TypeCode::CashLetterCredit.code(), "187");
        assert_eq!(TypeCode::from("188"), TypeCode::TotalCashLetterAdjustments);
        assert_eq!(TypeCode::TotalCashLetterAdjustments.code(), "188");
        assert_eq!(TypeCode::from("189"), TypeCode::CashLetterAdjustmentCr);
        assert_eq!(TypeCode::CashLetterAdjustmentCr.code(), "189");
        assert_eq!(TypeCode::from("190"), TypeCode::TotalIncomingMoneyTransfers);
        assert_eq!(TypeCode::TotalIncomingMoneyTransfers.code(), "190");
        assert_eq!(
            TypeCode::from("191"),
            TypeCode::IndividualIncomingInternalMoneyTransfer
        );
        assert_eq!(
            TypeCode::IndividualIncomingInternalMoneyTransfer.code(),
            "191"
        );
        assert_eq!(TypeCode::from("195"), TypeCode::IncomingMoneyTransfer);
        assert_eq!(TypeCode::IncomingMoneyTransfer.code(), "195");
        assert_eq!(TypeCode::from("196"), TypeCode::MoneyTransferAdjustmentCr);
        assert_eq!(TypeCode::MoneyTransferAdjustmentCr.code(), "196");
        assert_eq!(TypeCode::from("198"), TypeCode::CompensationCr);
        assert_eq!(TypeCode::CompensationCr.code(), "198");
        assert_eq!(
            TypeCode::from("200"),
            TypeCode::TotalAutomaticTransferCredits
        );
        assert_eq!(TypeCode::TotalAutomaticTransferCredits.code(), "200");
        assert_eq!(
            TypeCode::from("201"),
            TypeCode::IndividualAutomaticTransferCredit
        );
        assert_eq!(TypeCode::IndividualAutomaticTransferCredit.code(), "201");
        assert_eq!(TypeCode::from("202"), TypeCode::BondOperationsCredit);
        assert_eq!(TypeCode::BondOperationsCredit.code(), "202");
        assert_eq!(TypeCode::from("205"), TypeCode::TotalBookTransferCredits);
        assert_eq!(TypeCode::TotalBookTransferCredits.code(), "205");
        assert_eq!(TypeCode::from("206"), TypeCode::BookTransferCredit);
        assert_eq!(TypeCode::BookTransferCredit.code(), "206");
        assert_eq!(
            TypeCode::from("207"),
            TypeCode::TotalInternationalMoneyTransferCredits
        );
        assert_eq!(
            TypeCode::TotalInternationalMoneyTransferCredits.code(),
            "207"
        );
        assert_eq!(
            TypeCode::from("208"),
            TypeCode::IndividualInternationalMoneyTransferCredit
        );
        assert_eq!(
            TypeCode::IndividualInternationalMoneyTransferCredit.code(),
            "208"
        );
        assert_eq!(TypeCode::from("210"), TypeCode::TotalInternationalCredits);
        assert_eq!(TypeCode::TotalInternationalCredits.code(), "210");
        assert_eq!(TypeCode::from("212"), TypeCode::ForeignLetterOfCredit);
        assert_eq!(TypeCode::ForeignLetterOfCredit.code(), "212");
        assert_eq!(TypeCode::from("213"), TypeCode::LetterOfCreditCr);
        assert_eq!(TypeCode::LetterOfCreditCr.code(), "213");
        assert_eq!(TypeCode::from("214"), TypeCode::ForeignExchangeOfCredit);
        assert_eq!(TypeCode::ForeignExchangeOfCredit.code(), "214");
        assert_eq!(TypeCode::from("215"), TypeCode::TotalLettersOfCreditCr);
        assert_eq!(TypeCode::TotalLettersOfCreditCr.code(), "215");
        assert_eq!(TypeCode::from("216"), TypeCode::ForeignRemittanceCredit);
        assert_eq!(TypeCode::ForeignRemittanceCredit.code(), "216");
        assert_eq!(TypeCode::from("218"), TypeCode::ForeignCollectionCredit);
        assert_eq!(TypeCode::ForeignCollectionCredit.code(), "218");
        assert_eq!(TypeCode::from("221"), TypeCode::ForeignCheckPurchase);
        assert_eq!(TypeCode::ForeignCheckPurchase.code(), "221");
        assert_eq!(TypeCode::from("222"), TypeCode::ForeignChecksDeposited);
        assert_eq!(TypeCode::ForeignChecksDeposited.code(), "222");
        assert_eq!(TypeCode::from("224"), TypeCode::CommissionCr);
        assert_eq!(TypeCode::CommissionCr.code(), "224");
        assert_eq!(
            TypeCode::from("226"),
            TypeCode::InternationalMoneyMarketTradingCr
        );
        assert_eq!(TypeCode::InternationalMoneyMarketTradingCr.code(), "226");
        assert_eq!(TypeCode::from("227"), TypeCode::StandingOrderCr);
        assert_eq!(TypeCode::StandingOrderCr.code(), "227");
        assert_eq!(
            TypeCode::from("229"),
            TypeCode::MiscellaneousInternationalCredit
        );
        assert_eq!(TypeCode::MiscellaneousInternationalCredit.code(), "229");
        assert_eq!(TypeCode::from("230"), TypeCode::TotalSecurityCredits);
        assert_eq!(TypeCode::TotalSecurityCredits.code(), "230");
        assert_eq!(TypeCode::from("231"), TypeCode::TotalCollectionCredits);
        assert_eq!(TypeCode::TotalCollectionCredits.code(), "231");
        assert_eq!(TypeCode::from("232"), TypeCode::SaleOfDebtSecurity);
        assert_eq!(TypeCode::SaleOfDebtSecurity.code(), "232");
        assert_eq!(TypeCode::from("233"), TypeCode::SecuritiesSold);
        assert_eq!(TypeCode::SecuritiesSold.code(), "233");
        assert_eq!(TypeCode::from("234"), TypeCode::SaleOfEquitySecurity);
        assert_eq!(TypeCode::SaleOfEquitySecurity.code(), "234");
        assert_eq!(
            TypeCode::from("235"),
            TypeCode::MaturedReverseRepurchaseOrder
        );
        assert_eq!(TypeCode::MaturedReverseRepurchaseOrder.code(), "235");
        assert_eq!(TypeCode::from("236"), TypeCode::MaturityOfDebtSecurity);
        assert_eq!(TypeCode::MaturityOfDebtSecurity.code(), "236");
        assert_eq!(TypeCode::from("237"), TypeCode::IndividualCollectionCredit);
        assert_eq!(TypeCode::IndividualCollectionCredit.code(), "237");
        assert_eq!(TypeCode::from("238"), TypeCode::CollectionOfDividends);
        assert_eq!(TypeCode::CollectionOfDividends.code(), "238");
        assert_eq!(
            TypeCode::from("239"),
            TypeCode::TotalBankersAcceptanceCredits
        );
        assert_eq!(TypeCode::TotalBankersAcceptanceCredits.code(), "239");
        assert_eq!(TypeCode::from("240"), TypeCode::CouponCollectionsBanks);
        assert_eq!(TypeCode::CouponCollectionsBanks.code(), "240");
        assert_eq!(TypeCode::from("241"), TypeCode::BankersAcceptancesCr);
        assert_eq!(TypeCode::BankersAcceptancesCr.code(), "241");
        assert_eq!(TypeCode::from("242"), TypeCode::CollectionOfInterestIncome);
        assert_eq!(TypeCode::CollectionOfInterestIncome.code(), "242");
        assert_eq!(TypeCode::from("243"), TypeCode::MaturedFedFundsPurchased);
        assert_eq!(TypeCode::MaturedFedFundsPurchased.code(), "243");
        assert_eq!(
            TypeCode::from("244"),
            TypeCode::InterestMaturedPrincipalPaymentCr
        );
        assert_eq!(TypeCode::InterestMaturedPrincipalPaymentCr.code(), "244");
        assert_eq!(TypeCode::from("245"), TypeCode::MonthlyDividends);
        assert_eq!(TypeCode::MonthlyDividends.code(), "245");
        assert_eq!(TypeCode::from("246"), TypeCode::CommercialPaperCr);
        assert_eq!(TypeCode::CommercialPaperCr.code(), "246");
        assert_eq!(TypeCode::from("247"), TypeCode::CapitalChangeCr);
        assert_eq!(TypeCode::CapitalChangeCr.code(), "247");
        assert_eq!(
            TypeCode::from("248"),
            TypeCode::SavingsBondsSalesAdjustmentCr
        );
        assert_eq!(TypeCode::SavingsBondsSalesAdjustmentCr.code(), "248");
        assert_eq!(TypeCode::from("249"), TypeCode::MiscellaneousSecurityCredit);
        assert_eq!(TypeCode::MiscellaneousSecurityCredit.code(), "249");
        assert_eq!(
            TypeCode::from("250"),
            TypeCode::TotalChecksPostedAndReturned
        );
        assert_eq!(TypeCode::TotalChecksPostedAndReturned.code(), "250");
        assert_eq!(TypeCode::from("251"), TypeCode::TotalDebitReversals);
        assert_eq!(TypeCode::TotalDebitReversals.code(), "251");
        assert_eq!(TypeCode::from("252"), TypeCode::DebitReversal);
        assert_eq!(TypeCode::DebitReversal.code(), "252");
        assert_eq!(
            TypeCode::from("254"),
            TypeCode::PostingErrorCorrectionCredit
        );
        assert_eq!(TypeCode::PostingErrorCorrectionCredit.code(), "254");
        assert_eq!(TypeCode::from("255"), TypeCode::CheckPostedAndReturned);
        assert_eq!(TypeCode::CheckPostedAndReturned.code(), "255");
        assert_eq!(TypeCode::from("256"), TypeCode::TotalAchReturnItemsCr);
        assert_eq!(TypeCode::TotalAchReturnItemsCr.code(), "256");
        assert_eq!(TypeCode::from("257"), TypeCode::IndividualAchReturnItemCr);
        assert_eq!(TypeCode::IndividualAchReturnItemCr.code(), "257");
        assert_eq!(TypeCode::from("258"), TypeCode::AchReversalCredit);
        assert_eq!(TypeCode::AchReversalCredit.code(), "258");
        assert_eq!(TypeCode::from("260"), TypeCode::TotalRejectedCredits);
        assert_eq!(TypeCode::TotalRejectedCredits.code(), "260");
        assert_eq!(TypeCode::from("261"), TypeCode::IndividualRejectedCredit);
        assert_eq!(TypeCode::IndividualRejectedCredit.code(), "261");
        assert_eq!(TypeCode::from("263"), TypeCode::OverdraftCr);
        assert_eq!(TypeCode::OverdraftCr.code(), "263");
        assert_eq!(TypeCode::from("266"), TypeCode::ReturnItemCr);
        assert_eq!(TypeCode::ReturnItemCr.code(), "266");
        assert_eq!(TypeCode::from("268"), TypeCode::ReturnItemAdjustmentCr);
        assert_eq!(TypeCode::ReturnItemAdjustmentCr.code(), "268");
        assert_eq!(TypeCode::from("270"), TypeCode::TotalZbaCredits);
        assert_eq!(TypeCode::TotalZbaCredits.code(), "270");
        assert_eq!(TypeCode::from("271"), TypeCode::NetZeroBalanceAmount);
        assert_eq!(TypeCode::NetZeroBalanceAmount.code(), "271");
        assert_eq!(
            TypeCode::from("274"),
            TypeCode::CumulativeZbaOrDisbursementCredits
        );
        assert_eq!(TypeCode::CumulativeZbaOrDisbursementCredits.code(), "274");
        assert_eq!(TypeCode::from("275"), TypeCode::ZbaCredit);
        assert_eq!(TypeCode::ZbaCredit.code(), "275");
        assert_eq!(TypeCode::from("276"), TypeCode::ZbaFloatAdjustment);
        assert_eq!(TypeCode::ZbaFloatAdjustment.code(), "276");
        assert_eq!(TypeCode::from("277"), TypeCode::ZbaCreditTransfer);
        assert_eq!(TypeCode::ZbaCreditTransfer.code(), "277");
        assert_eq!(TypeCode::from("278"), TypeCode::ZbaCreditAdjustment);
        assert_eq!(TypeCode::ZbaCreditAdjustment.code(), "278");
        assert_eq!(
            TypeCode::from("280"),
            TypeCode::TotalControlledDisbursingCredits
        );
        assert_eq!(TypeCode::TotalControlledDisbursingCredits.code(), "280");
        assert_eq!(
            TypeCode::from("281"),
            TypeCode::IndividualControlledDisbursingCredit
        );
        assert_eq!(TypeCode::IndividualControlledDisbursingCredit.code(), "281");
        assert_eq!(TypeCode::from("285"), TypeCode::TotalDtcDisbursingCredits);
        assert_eq!(TypeCode::TotalDtcDisbursingCredits.code(), "285");
        assert_eq!(
            TypeCode::from("286"),
            TypeCode::IndividualDtcDisbursingCredit
        );
        assert_eq!(TypeCode::IndividualDtcDisbursingCredit.code(), "286");
        assert_eq!(TypeCode::from("294"), TypeCode::TotalAtmCredits);
        assert_eq!(TypeCode::TotalAtmCredits.code(), "294");
        assert_eq!(TypeCode::from("295"), TypeCode::AtmCredit);
        assert_eq!(TypeCode::AtmCredit.code(), "295");
        assert_eq!(TypeCode::from("301"), TypeCode::CommercialDeposit);
        assert_eq!(TypeCode::CommercialDeposit.code(), "301");
        assert_eq!(TypeCode::from("302"), TypeCode::CorrespondentBankDeposit);
        assert_eq!(TypeCode::CorrespondentBankDeposit.code(), "302");
        assert_eq!(TypeCode::from("303"), TypeCode::TotalWireTransfersInFf);
        assert_eq!(TypeCode::TotalWireTransfersInFf.code(), "303");
        assert_eq!(TypeCode::from("304"), TypeCode::TotalWireTransfersInChf);
        assert_eq!(TypeCode::TotalWireTransfersInChf.code(), "304");
        assert_eq!(TypeCode::from("305"), TypeCode::TotalFedFundsSold);
        assert_eq!(TypeCode::TotalFedFundsSold.code(), "305");
        assert_eq!(TypeCode::from("306"), TypeCode::FedFundsSold);
        assert_eq!(TypeCode::FedFundsSold.code(), "306");
        assert_eq!(TypeCode::from("307"), TypeCode::TotalTrustCredits);
        assert_eq!(TypeCode::TotalTrustCredits.code(), "307");
        assert_eq!(TypeCode::from("308"), TypeCode::TrustCredit);
        assert_eq!(TypeCode::TrustCredit.code(), "308");
        assert_eq!(TypeCode::from("309"), TypeCode::TotalValueDatedFunds);
        assert_eq!(TypeCode::TotalValueDatedFunds.code(), "309");
        assert_eq!(TypeCode::from("310"), TypeCode::TotalCommercialDeposits);
        assert_eq!(TypeCode::TotalCommercialDeposits.code(), "310");
        assert_eq!(TypeCode::from("315"), TypeCode::TotalInternationalCreditsFf);
        assert_eq!(TypeCode::TotalInternationalCreditsFf.code(), "315");
        assert_eq!(
            TypeCode::from("316"),
            TypeCode::TotalInternationalCreditsChf
        );
        assert_eq!(TypeCode::TotalInternationalCreditsChf.code(), "316");
        assert_eq!(TypeCode::from("318"), TypeCode::TotalForeignCheckPurchased);
        assert_eq!(TypeCode::TotalForeignCheckPurchased.code(), "318");
        assert_eq!(TypeCode::from("319"), TypeCode::LateDeposit);
        assert_eq!(TypeCode::LateDeposit.code(), "319");
        assert_eq!(TypeCode::from("320"), TypeCode::TotalSecuritiesSoldFf);
        assert_eq!(TypeCode::TotalSecuritiesSoldFf.code(), "320");
        assert_eq!(TypeCode::from("321"), TypeCode::TotalSecuritiesSoldChf);
        assert_eq!(TypeCode::TotalSecuritiesSoldChf.code(), "321");
        assert_eq!(TypeCode::from("324"), TypeCode::TotalSecuritiesMaturedFf);
        assert_eq!(TypeCode::TotalSecuritiesMaturedFf.code(), "324");
        assert_eq!(TypeCode::from("325"), TypeCode::TotalSecuritiesMaturedChf);
        assert_eq!(TypeCode::TotalSecuritiesMaturedChf.code(), "325");
        assert_eq!(TypeCode::from("326"), TypeCode::TotalSecuritiesInterest);
        assert_eq!(TypeCode::TotalSecuritiesInterest.code(), "326");
        assert_eq!(TypeCode::from("327"), TypeCode::TotalSecuritiesMatured);
        assert_eq!(TypeCode::TotalSecuritiesMatured.code(), "327");
        assert_eq!(TypeCode::from("328"), TypeCode::TotalSecuritiesInterestFf);
        assert_eq!(TypeCode::TotalSecuritiesInterestFf.code(), "328");
        assert_eq!(TypeCode::from("329"), TypeCode::TotalSecuritiesInterestChf);
        assert_eq!(TypeCode::TotalSecuritiesInterestChf.code(), "329");
        assert_eq!(TypeCode::from("330"), TypeCode::TotalEscrowCredits);
        assert_eq!(TypeCode::TotalEscrowCredits.code(), "330");
        assert_eq!(TypeCode::from("331"), TypeCode::IndividualEscrowCredit);
        assert_eq!(TypeCode::IndividualEscrowCredit.code(), "331");
        assert_eq!(
            TypeCode::from("332"),
            TypeCode::TotalMiscellaneousSecuritiesCreditsFf
        );
        assert_eq!(
            TypeCode::TotalMiscellaneousSecuritiesCreditsFf.code(),
            "332"
        );
        assert_eq!(
            TypeCode::from("336"),
            TypeCode::TotalMiscellaneousSecuritiesCreditsChf
        );
        assert_eq!(
            TypeCode::TotalMiscellaneousSecuritiesCreditsChf.code(),
            "336"
        );
        assert_eq!(TypeCode::from("338"), TypeCode::TotalSecuritiesSold);
        assert_eq!(TypeCode::TotalSecuritiesSold.code(), "338");
        assert_eq!(TypeCode::from("340"), TypeCode::TotalBrokerDeposits);
        assert_eq!(TypeCode::TotalBrokerDeposits.code(), "340");
        assert_eq!(TypeCode::from("341"), TypeCode::TotalBrokerDepositsFf);
        assert_eq!(TypeCode::TotalBrokerDepositsFf.code(), "341");
        assert_eq!(TypeCode::from("342"), TypeCode::BrokerDeposit);
        assert_eq!(TypeCode::BrokerDeposit.code(), "342");
        assert_eq!(TypeCode::from("343"), TypeCode::TotalBrokerDepositsChf);
        assert_eq!(TypeCode::TotalBrokerDepositsChf.code(), "343");
        assert_eq!(TypeCode::from("344"), TypeCode::IndividualBackValueCredit);
        assert_eq!(TypeCode::IndividualBackValueCredit.code(), "344");
        assert_eq!(TypeCode::from("345"), TypeCode::ItemInBrokersDeposit);
        assert_eq!(TypeCode::ItemInBrokersDeposit.code(), "345");
        assert_eq!(TypeCode::from("346"), TypeCode::SweepInterestIncome);
        assert_eq!(TypeCode::SweepInterestIncome.code(), "346");
        assert_eq!(TypeCode::from("347"), TypeCode::SweepPrincipalSell);
        assert_eq!(TypeCode::SweepPrincipalSell.code(), "347");
        assert_eq!(TypeCode::from("348"), TypeCode::FuturesCredit);
        assert_eq!(TypeCode::FuturesCredit.code(), "348");
        assert_eq!(TypeCode::from("349"), TypeCode::PrincipalPaymentsCredit);
        assert_eq!(TypeCode::PrincipalPaymentsCredit.code(), "349");
        assert_eq!(TypeCode::from("350"), TypeCode::InvestmentSold);
        assert_eq!(TypeCode::InvestmentSold.code(), "350");
        assert_eq!(TypeCode::from("351"), TypeCode::IndividualInvestmentSold);
        assert_eq!(TypeCode::IndividualInvestmentSold.code(), "351");
        assert_eq!(TypeCode::from("352"), TypeCode::TotalCashCenterCredits);
        assert_eq!(TypeCode::TotalCashCenterCredits.code(), "352");
        assert_eq!(TypeCode::from("353"), TypeCode::CashCenterCredit);
        assert_eq!(TypeCode::CashCenterCredit.code(), "353");
        assert_eq!(TypeCode::from("354"), TypeCode::InterestCredit);
        assert_eq!(TypeCode::InterestCredit.code(), "354");
        assert_eq!(TypeCode::from("355"), TypeCode::InvestmentInterest);
        assert_eq!(TypeCode::InvestmentInterest.code(), "355");
        assert_eq!(TypeCode::from("356"), TypeCode::TotalCreditAdjustment);
        assert_eq!(TypeCode::TotalCreditAdjustment.code(), "356");
        assert_eq!(TypeCode::from("357"), TypeCode::CreditAdjustment);
        assert_eq!(TypeCode::CreditAdjustment.code(), "357");
        assert_eq!(TypeCode::from("358"), TypeCode::YtdAdjustmentCredit);
        assert_eq!(TypeCode::YtdAdjustmentCredit.code(), "358");
        assert_eq!(TypeCode::from("359"), TypeCode::InterestAdjustmentCredit);
        assert_eq!(TypeCode::InterestAdjustmentCredit.code(), "359");
        assert_eq!(
            TypeCode::from("360"),
            TypeCode::TotalCreditsLessWireTransferAndReturnedChecks
        );
        assert_eq!(
            TypeCode::TotalCreditsLessWireTransferAndReturnedChecks.code(),
            "360"
        );
        assert_eq!(
            TypeCode::from("361"),
            TypeCode::GrandTotalCreditsLessGrandTotalDebits
        );
        assert_eq!(
            TypeCode::GrandTotalCreditsLessGrandTotalDebits.code(),
            "361"
        );
        assert_eq!(TypeCode::from("362"), TypeCode::CorrespondentCollection);
        assert_eq!(TypeCode::CorrespondentCollection.code(), "362");
        assert_eq!(
            TypeCode::from("363"),
            TypeCode::CorrespondentCollectionAdjustmentCr
        );
        assert_eq!(TypeCode::CorrespondentCollectionAdjustmentCr.code(), "363");
        assert_eq!(TypeCode::from("364"), TypeCode::LoanParticipationCr);
        assert_eq!(TypeCode::LoanParticipationCr.code(), "364");
        assert_eq!(TypeCode::from("366"), TypeCode::CurrencyAndCoinDeposited);
        assert_eq!(TypeCode::CurrencyAndCoinDeposited.code(), "366");
        assert_eq!(TypeCode::from("367"), TypeCode::FoodStampLetterCr);
        assert_eq!(TypeCode::FoodStampLetterCr.code(), "367");
        assert_eq!(TypeCode::from("368"), TypeCode::FoodStampAdjustmentCr);
        assert_eq!(TypeCode::FoodStampAdjustmentCr.code(), "368");
        assert_eq!(TypeCode::from("369"), TypeCode::ClearingSettlementCredit);
        assert_eq!(TypeCode::ClearingSettlementCredit.code(), "369");
        assert_eq!(TypeCode::from("370"), TypeCode::TotalBackValueCredits);
        assert_eq!(TypeCode::TotalBackValueCredits.code(), "370");
        assert_eq!(TypeCode::from("372"), TypeCode::BackValueAdjustmentCr);
        assert_eq!(TypeCode::BackValueAdjustmentCr.code(), "372");
        assert_eq!(TypeCode::from("373"), TypeCode::CustomerPayrollCr);
        assert_eq!(TypeCode::CustomerPayrollCr.code(), "373");
        assert_eq!(TypeCode::from("374"), TypeCode::FrbStatementRecapCr);
        assert_eq!(TypeCode::FrbStatementRecapCr.code(), "374");
        assert_eq!(
            TypeCode::from("376"),
            TypeCode::SavingsBondLetterOrAdjustmentCr
        );
        assert_eq!(TypeCode::SavingsBondLetterOrAdjustmentCr.code(), "376");
        assert_eq!(TypeCode::from("377"), TypeCode::TreasuryTaxAndLoanCredit);
        assert_eq!(TypeCode::TreasuryTaxAndLoanCredit.code(), "377");
        assert_eq!(TypeCode::from("378"), TypeCode::TransferOfTreasuryCredit);
        assert_eq!(TypeCode::TransferOfTreasuryCredit.code(), "378");
        assert_eq!(
            TypeCode::from("379"),
            TypeCode::FrbGovernmentChecksCashLetterCredit
        );
        assert_eq!(TypeCode::FrbGovernmentChecksCashLetterCredit.code(), "379");
        assert_eq!(
            TypeCode::from("381"),
            TypeCode::FrbGovernmentCheckAdjustmentCr
        );
        assert_eq!(TypeCode::FrbGovernmentCheckAdjustmentCr.code(), "381");
        assert_eq!(TypeCode::from("382"), TypeCode::FrbPostalMoneyOrderCredit);
        assert_eq!(TypeCode::FrbPostalMoneyOrderCredit.code(), "382");
        assert_eq!(
            TypeCode::from("383"),
            TypeCode::FrbPostalMoneyOrderAdjustmentCr
        );
        assert_eq!(TypeCode::FrbPostalMoneyOrderAdjustmentCr.code(), "383");
        assert_eq!(
            TypeCode::from("384"),
            TypeCode::FrbCashLetterAutoChargeCredit
        );
        assert_eq!(TypeCode::FrbCashLetterAutoChargeCredit.code(), "384");
        assert_eq!(TypeCode::from("385"), TypeCode::TotalUniversalCredits);
        assert_eq!(TypeCode::TotalUniversalCredits.code(), "385");
        assert_eq!(
            TypeCode::from("386"),
            TypeCode::FrbCashLetterAutoChargeAdjustmentCr
        );
        assert_eq!(TypeCode::FrbCashLetterAutoChargeAdjustmentCr.code(), "386");
        assert_eq!(TypeCode::from("387"), TypeCode::FrbFineSortCashLetterCredit);
        assert_eq!(TypeCode::FrbFineSortCashLetterCredit.code(), "387");
        assert_eq!(TypeCode::from("388"), TypeCode::FrbFineSortAdjustmentCr);
        assert_eq!(TypeCode::FrbFineSortAdjustmentCr.code(), "388");
        assert_eq!(TypeCode::from("389"), TypeCode::TotalFreightPaymentCredits);
        assert_eq!(TypeCode::TotalFreightPaymentCredits.code(), "389");
        assert_eq!(TypeCode::from("390"), TypeCode::TotalMiscellaneousCredits);
        assert_eq!(TypeCode::TotalMiscellaneousCredits.code(), "390");
        assert_eq!(TypeCode::from("391"), TypeCode::UniversalCredit);
        assert_eq!(TypeCode::UniversalCredit.code(), "391");
        assert_eq!(TypeCode::from("392"), TypeCode::FreightPaymentCredit);
        assert_eq!(TypeCode::FreightPaymentCredit.code(), "392");
        assert_eq!(TypeCode::from("393"), TypeCode::ItemizedCreditOver10);
        assert_eq!(TypeCode::ItemizedCreditOver10.code(), "393");
        assert_eq!(TypeCode::from("394"), TypeCode::CumulativeCredits);
        assert_eq!(TypeCode::CumulativeCredits.code(), "394");
        assert_eq!(TypeCode::from("395"), TypeCode::CheckReversal);
        assert_eq!(TypeCode::CheckReversal.code(), "395");
        assert_eq!(TypeCode::from("397"), TypeCode::FloatAdjustmentCr);
        assert_eq!(TypeCode::FloatAdjustmentCr.code(), "397");
        assert_eq!(TypeCode::from("398"), TypeCode::MiscellaneousFeeRefund);
        assert_eq!(TypeCode::MiscellaneousFeeRefund.code(), "398");
        assert_eq!(TypeCode::from("399"), TypeCode::MiscellaneousCredit);
        assert_eq!(TypeCode::MiscellaneousCredit.code(), "399");
        assert_eq!(TypeCode::from("400"), TypeCode::TotalDebits);
        assert_eq!(TypeCode::TotalDebits.code(), "400");
        assert_eq!(TypeCode::from("401"), TypeCode::TotalDebitAmountMtd);
        assert_eq!(TypeCode::TotalDebitAmountMtd.code(), "401");
        assert_eq!(TypeCode::from("403"), TypeCode::TodaySTotalDebits);
        assert_eq!(TypeCode::TodaySTotalDebits.code(), "403");
        assert_eq!(
            TypeCode::from("405"),
            TypeCode::TotalDebitLessWireTransfersAndChargeBacks
        );
        assert_eq!(
            TypeCode::TotalDebitLessWireTransfersAndChargeBacks.code(),
            "405"
        );
        assert_eq!(TypeCode::from("406"), TypeCode::DebitsNotDetailed);
        assert_eq!(TypeCode::DebitsNotDetailed.code(), "406");
        assert_eq!(TypeCode::from("408"), TypeCode::FloatAdjustmentDb);
        assert_eq!(TypeCode::FloatAdjustmentDb.code(), "408");
        assert_eq!(TypeCode::from("409"), TypeCode::DebitAnyType);
        assert_eq!(TypeCode::DebitAnyType.code(), "409");
        assert_eq!(TypeCode::from("410"), TypeCode::TotalYtdAdjustment);
        assert_eq!(TypeCode::TotalYtdAdjustment.code(), "410");
        assert_eq!(
            TypeCode::from("412"),
            TypeCode::TotalDebitsExcludingReturnedItems
        );
        assert_eq!(TypeCode::TotalDebitsExcludingReturnedItems.code(), "412");
        assert_eq!(TypeCode::from("415"), TypeCode::LockboxDebit);
        assert_eq!(TypeCode::LockboxDebit.code(), "415");
        assert_eq!(TypeCode::from("416"), TypeCode::TotalLockboxDebits);
        assert_eq!(TypeCode::TotalLockboxDebits.code(), "416");
        assert_eq!(TypeCode::from("420"), TypeCode::EdiTransactionDebits);
        assert_eq!(TypeCode::EdiTransactionDebits.code(), "420");
        assert_eq!(TypeCode::from("421"), TypeCode::EdiTransactionDebit);
        assert_eq!(TypeCode::EdiTransactionDebit.code(), "421");
        assert_eq!(TypeCode::from("422"), TypeCode::EdibanxSettlementDebit);
        assert_eq!(TypeCode::EdibanxSettlementDebit.code(), "422");
        assert_eq!(TypeCode::from("423"), TypeCode::EdibanxReturnItemDebit);
        assert_eq!(TypeCode::EdibanxReturnItemDebit.code(), "423");
        assert_eq!(TypeCode::from("430"), TypeCode::TotalPayableThroughDrafts);
        assert_eq!(TypeCode::TotalPayableThroughDrafts.code(), "430");
        assert_eq!(TypeCode::from("435"), TypeCode::PayableThroughDraft);
        assert_eq!(TypeCode::PayableThroughDraft.code(), "435");
        assert_eq!(TypeCode::from("445"), TypeCode::AchConcentrationDebit);
        assert_eq!(TypeCode::AchConcentrationDebit.code(), "445");
        assert_eq!(
            TypeCode::from("446"),
            TypeCode::TotalAchDisbursementFundingDebits
        );
        assert_eq!(TypeCode::TotalAchDisbursementFundingDebits.code(), "446");
        assert_eq!(TypeCode::from("447"), TypeCode::AchDisbursementFundingDebit);
        assert_eq!(TypeCode::AchDisbursementFundingDebit.code(), "447");
        assert_eq!(TypeCode::from("450"), TypeCode::TotalAchDebits);
        assert_eq!(TypeCode::TotalAchDebits.code(), "450");
        assert_eq!(TypeCode::from("451"), TypeCode::AchDebitReceived);
        assert_eq!(TypeCode::AchDebitReceived.code(), "451");
        assert_eq!(
            TypeCode::from("452"),
            TypeCode::ItemInAchDisbursementOrDebit
        );
        assert_eq!(TypeCode::ItemInAchDisbursementOrDebit.code(), "452");
        assert_eq!(TypeCode::from("455"), TypeCode::PreauthorizedAchDebit);
        assert_eq!(TypeCode::PreauthorizedAchDebit.code(), "455");
        assert_eq!(
            TypeCode::from("462"),
            TypeCode::AccountHolderInitiatedAchDebit
        );
        assert_eq!(TypeCode::AccountHolderInitiatedAchDebit.code(), "462");
        assert_eq!(TypeCode::from("463"), TypeCode::CorporateTradePaymentDebits);
        assert_eq!(TypeCode::CorporateTradePaymentDebits.code(), "463");
        assert_eq!(TypeCode::from("464"), TypeCode::CorporateTradePaymentDebit);
        assert_eq!(TypeCode::CorporateTradePaymentDebit.code(), "464");
        assert_eq!(
            TypeCode::from("465"),
            TypeCode::CorporateTradePaymentSettlementDb
        );
        assert_eq!(TypeCode::CorporateTradePaymentSettlementDb.code(), "465");
        assert_eq!(TypeCode::from("466"), TypeCode::AchSettlementDb);
        assert_eq!(TypeCode::AchSettlementDb.code(), "466");
        assert_eq!(TypeCode::from("467"), TypeCode::AchSettlementDebits);
        assert_eq!(TypeCode::AchSettlementDebits.code(), "467");
        assert_eq!(
            TypeCode::from("468"),
            TypeCode::AchReturnItemOrAdjustmentSettlementDb
        );
        assert_eq!(
            TypeCode::AchReturnItemOrAdjustmentSettlementDb.code(),
            "468"
        );
        assert_eq!(TypeCode::from("469"), TypeCode::MiscellaneousAchDebit);
        assert_eq!(TypeCode::MiscellaneousAchDebit.code(), "469");
        assert_eq!(TypeCode::from("470"), TypeCode::TotalCheckPaid);
        assert_eq!(TypeCode::TotalCheckPaid.code(), "470");
        assert_eq!(TypeCode::from("471"), TypeCode::TotalCheckPaidCumulativeMtd);
        assert_eq!(TypeCode::TotalCheckPaidCumulativeMtd.code(), "471");
        assert_eq!(TypeCode::from("472"), TypeCode::CumulativeChecksPaid);
        assert_eq!(TypeCode::CumulativeChecksPaid.code(), "472");
        assert_eq!(TypeCode::from("474"), TypeCode::CertifiedCheckDebit);
        assert_eq!(TypeCode::CertifiedCheckDebit.code(), "474");
        assert_eq!(TypeCode::from("475"), TypeCode::CheckPaid);
        assert_eq!(TypeCode::CheckPaid.code(), "475");
        assert_eq!(
            TypeCode::from("476"),
            TypeCode::FederalReserveBankLetterDebit
        );
        assert_eq!(TypeCode::FederalReserveBankLetterDebit.code(), "476");
        assert_eq!(TypeCode::from("477"), TypeCode::BankOriginatedDebit);
        assert_eq!(TypeCode::BankOriginatedDebit.code(), "477");
        assert_eq!(TypeCode::from("478"), TypeCode::ListPostDebits);
        assert_eq!(TypeCode::ListPostDebits.code(), "478");
        assert_eq!(TypeCode::from("479"), TypeCode::ListPostDebit);
        assert_eq!(TypeCode::ListPostDebit.code(), "479");
        assert_eq!(TypeCode::from("480"), TypeCode::TotalLoanPayments);
        assert_eq!(TypeCode::TotalLoanPayments.code(), "480");
        assert_eq!(TypeCode::from("481"), TypeCode::IndividualLoanPayment);
        assert_eq!(TypeCode::IndividualLoanPayment.code(), "481");
        assert_eq!(TypeCode::from("482"), TypeCode::TotalBankOriginatedDebits);
        assert_eq!(TypeCode::TotalBankOriginatedDebits.code(), "482");
        assert_eq!(TypeCode::from("484"), TypeCode::Draft);
        assert_eq!(TypeCode::Draft.code(), "484");
        assert_eq!(TypeCode::from("485"), TypeCode::DtcDebit);
        assert_eq!(TypeCode::DtcDebit.code(), "485");
        assert_eq!(TypeCode::from("486"), TypeCode::TotalCashLetterDebits);
        assert_eq!(TypeCode::TotalCashLetterDebits.code(), "486");
        assert_eq!(TypeCode::from("487"), TypeCode::CashLetterDebit);
        assert_eq!(TypeCode::CashLetterDebit.code(), "487");
        assert_eq!(TypeCode::from("489"), TypeCode::CashLetterAdjustmentDb);
        assert_eq!(TypeCode::CashLetterAdjustmentDb.code(), "489");
        assert_eq!(TypeCode::from("490"), TypeCode::TotalOutgoingMoneyTransfers);
        assert_eq!(TypeCode::TotalOutgoingMoneyTransfers.code(), "490");
        assert_eq!(
            TypeCode::from("491"),
            TypeCode::IndividualOutgoingInternalMoneyTransfer
        );
        assert_eq!(
            TypeCode::IndividualOutgoingInternalMoneyTransfer.code(),
            "491"
        );
        assert_eq!(
            TypeCode::from("493"),
            TypeCode::CustomerTerminalInitiatedMoneyTransfer
        );
        assert_eq!(
            TypeCode::CustomerTerminalInitiatedMoneyTransfer.code(),
            "493"
        );
        assert_eq!(TypeCode::from("495"), TypeCode::OutgoingMoneyTransfer);
        assert_eq!(TypeCode::OutgoingMoneyTransfer.code(), "495");
        assert_eq!(TypeCode::from("496"), TypeCode::MoneyTransferAdjustmentDb);
        assert_eq!(TypeCode::MoneyTransferAdjustmentDb.code(), "496");
        assert_eq!(TypeCode::from("498"), TypeCode::CompensationDb);
        assert_eq!(TypeCode::CompensationDb.code(), "498");
        assert_eq!(
            TypeCode::from("500"),
            TypeCode::TotalAutomaticTransferDebits
        );
        assert_eq!(TypeCode::TotalAutomaticTransferDebits.code(), "500");
        assert_eq!(
            TypeCode::from("501"),
            TypeCode::IndividualAutomaticTransferDebit
        );
        assert_eq!(TypeCode::IndividualAutomaticTransferDebit.code(), "501");
        assert_eq!(TypeCode::from("502"), TypeCode::BondOperationsDebit);
        assert_eq!(TypeCode::BondOperationsDebit.code(), "502");
        assert_eq!(TypeCode::from("505"), TypeCode::TotalBookTransferDebits);
        assert_eq!(TypeCode::TotalBookTransferDebits.code(), "505");
        assert_eq!(TypeCode::from("506"), TypeCode::BookTransferDebit);
        assert_eq!(TypeCode::BookTransferDebit.code(), "506");
        assert_eq!(
            TypeCode::from("507"),
            TypeCode::TotalInternationalMoneyTransferDebits
        );
        assert_eq!(
            TypeCode::TotalInternationalMoneyTransferDebits.code(),
            "507"
        );
        assert_eq!(
            TypeCode::from("508"),
            TypeCode::IndividualInternationalMoneyTransferDebits
        );
        assert_eq!(
            TypeCode::IndividualInternationalMoneyTransferDebits.code(),
            "508"
        );
        assert_eq!(TypeCode::from("510"), TypeCode::TotalInternationalDebits);
        assert_eq!(TypeCode::TotalInternationalDebits.code(), "510");
        assert_eq!(TypeCode::from("512"), TypeCode::LetterOfCreditDebit);
        assert_eq!(TypeCode::LetterOfCreditDebit.code(), "512");
        assert_eq!(TypeCode::from("513"), TypeCode::LetterOfCreditDb);
        assert_eq!(TypeCode::LetterOfCreditDb.code(), "513");
        assert_eq!(TypeCode::from("514"), TypeCode::ForeignExchangeDebit);
        assert_eq!(TypeCode::ForeignExchangeDebit.code(), "514");
        assert_eq!(TypeCode::from("515"), TypeCode::TotalLettersOfCreditDb);
        assert_eq!(TypeCode::TotalLettersOfCreditDb.code(), "515");
        assert_eq!(TypeCode::from("516"), TypeCode::ForeignRemittanceDebit);
        assert_eq!(TypeCode::ForeignRemittanceDebit.code(), "516");
        assert_eq!(TypeCode::from("518"), TypeCode::ForeignCollectionDebit);
        assert_eq!(TypeCode::ForeignCollectionDebit.code(), "518");
        assert_eq!(TypeCode::from("522"), TypeCode::ForeignChecksPaid);
        assert_eq!(TypeCode::ForeignChecksPaid.code(), "522");
        assert_eq!(TypeCode::from("524"), TypeCode::CommissionDb);
        assert_eq!(TypeCode::CommissionDb.code(), "524");
        assert_eq!(
            TypeCode::from("526"),
            TypeCode::InternationalMoneyMarketTradingDb
        );
        assert_eq!(TypeCode::InternationalMoneyMarketTradingDb.code(), "526");
        assert_eq!(TypeCode::from("527"), TypeCode::StandingOrderDb);
        assert_eq!(TypeCode::StandingOrderDb.code(), "527");
        assert_eq!(
            TypeCode::from("529"),
            TypeCode::MiscellaneousInternationalDebit
        );
        assert_eq!(TypeCode::MiscellaneousInternationalDebit.code(), "529");
        assert_eq!(TypeCode::from("530"), TypeCode::TotalSecurityDebits);
        assert_eq!(TypeCode::TotalSecurityDebits.code(), "530");
        assert_eq!(TypeCode::from("531"), TypeCode::SecuritiesPurchased);
        assert_eq!(TypeCode::SecuritiesPurchased.code(), "531");
        assert_eq!(
            TypeCode::from("532"),
            TypeCode::TotalAmountOfSecuritiesPurchased
        );
        assert_eq!(TypeCode::TotalAmountOfSecuritiesPurchased.code(), "532");
        assert_eq!(TypeCode::from("533"), TypeCode::SecurityCollectionDebit);
        assert_eq!(TypeCode::SecurityCollectionDebit.code(), "533");
        assert_eq!(
            TypeCode::from("534"),
            TypeCode::TotalMiscellaneousSecuritiesDbFf
        );
        assert_eq!(TypeCode::TotalMiscellaneousSecuritiesDbFf.code(), "534");
        assert_eq!(TypeCode::from("535"), TypeCode::PurchaseOfEquitySecurities);
        assert_eq!(TypeCode::PurchaseOfEquitySecurities.code(), "535");
        assert_eq!(
            TypeCode::from("536"),
            TypeCode::TotalMiscellaneousSecuritiesDebitChf
        );
        assert_eq!(TypeCode::TotalMiscellaneousSecuritiesDebitChf.code(), "536");
        assert_eq!(TypeCode::from("537"), TypeCode::TotalCollectionDebit);
        assert_eq!(TypeCode::TotalCollectionDebit.code(), "537");
        assert_eq!(TypeCode::from("538"), TypeCode::MaturedRepurchaseOrder);
        assert_eq!(TypeCode::MaturedRepurchaseOrder.code(), "538");
        assert_eq!(
            TypeCode::from("539"),
            TypeCode::TotalBankersAcceptancesDebit
        );
        assert_eq!(TypeCode::TotalBankersAcceptancesDebit.code(), "539");
        assert_eq!(TypeCode::from("540"), TypeCode::CouponCollectionDebit);
        assert_eq!(TypeCode::CouponCollectionDebit.code(), "540");
        assert_eq!(TypeCode::from("541"), TypeCode::BankersAcceptancesDb);
        assert_eq!(TypeCode::BankersAcceptancesDb.code(), "541");
        assert_eq!(TypeCode::from("542"), TypeCode::PurchaseOfDebtSecurities);
        assert_eq!(TypeCode::PurchaseOfDebtSecurities.code(), "542");
        assert_eq!(TypeCode::from("543"), TypeCode::DomesticCollection);
        assert_eq!(TypeCode::DomesticCollection.code(), "543");
        assert_eq!(
            TypeCode::from("544"),
            TypeCode::InterestMaturedPrincipalPaymentDb
        );
        assert_eq!(TypeCode::InterestMaturedPrincipalPaymentDb.code(), "544");
        assert_eq!(TypeCode::from("546"), TypeCode::CommercialPaperDb);
        assert_eq!(TypeCode::CommercialPaperDb.code(), "546");
        assert_eq!(TypeCode::from("547"), TypeCode::CapitalChangeDb);
        assert_eq!(TypeCode::CapitalChangeDb.code(), "547");
        assert_eq!(
            TypeCode::from("548"),
            TypeCode::SavingsBondsSalesAdjustmentDb
        );
        assert_eq!(TypeCode::SavingsBondsSalesAdjustmentDb.code(), "548");
        assert_eq!(TypeCode::from("549"), TypeCode::MiscellaneousSecurityDebit);
        assert_eq!(TypeCode::MiscellaneousSecurityDebit.code(), "549");
        assert_eq!(TypeCode::from("550"), TypeCode::TotalDepositedItemsReturned);
        assert_eq!(TypeCode::TotalDepositedItemsReturned.code(), "550");
        assert_eq!(TypeCode::from("551"), TypeCode::TotalCreditReversals);
        assert_eq!(TypeCode::TotalCreditReversals.code(), "551");
        assert_eq!(TypeCode::from("552"), TypeCode::CreditReversal);
        assert_eq!(TypeCode::CreditReversal.code(), "552");
        assert_eq!(TypeCode::from("554"), TypeCode::PostingErrorCorrectionDebit);
        assert_eq!(TypeCode::PostingErrorCorrectionDebit.code(), "554");
        assert_eq!(TypeCode::from("555"), TypeCode::DepositedItemReturned);
        assert_eq!(TypeCode::DepositedItemReturned.code(), "555");
        assert_eq!(TypeCode::from("556"), TypeCode::TotalAchReturnItemsDb);
        assert_eq!(TypeCode::TotalAchReturnItemsDb.code(), "556");
        assert_eq!(TypeCode::from("557"), TypeCode::IndividualAchReturnItemDb);
        assert_eq!(TypeCode::IndividualAchReturnItemDb.code(), "557");
        assert_eq!(TypeCode::from("558"), TypeCode::AchReversalDebit);
        assert_eq!(TypeCode::AchReversalDebit.code(), "558");
        assert_eq!(TypeCode::from("560"), TypeCode::TotalRejectedDebits);
        assert_eq!(TypeCode::TotalRejectedDebits.code(), "560");
        assert_eq!(TypeCode::from("561"), TypeCode::IndividualRejectedDebit);
        assert_eq!(TypeCode::IndividualRejectedDebit.code(), "561");
        assert_eq!(TypeCode::from("563"), TypeCode::OverdraftDb);
        assert_eq!(TypeCode::OverdraftDb.code(), "563");
        assert_eq!(TypeCode::from("564"), TypeCode::OverdraftFee);
        assert_eq!(TypeCode::OverdraftFee.code(), "564");
        assert_eq!(TypeCode::from("566"), TypeCode::ReturnItemDb);
        assert_eq!(TypeCode::ReturnItemDb.code(), "566");
        assert_eq!(TypeCode::from("567"), TypeCode::ReturnItemFee);
        assert_eq!(TypeCode::ReturnItemFee.code(), "567");
        assert_eq!(TypeCode::from("568"), TypeCode::ReturnItemAdjustmentDb);
        assert_eq!(TypeCode::ReturnItemAdjustmentDb.code(), "568");
        assert_eq!(TypeCode::from("570"), TypeCode::TotalZbaDebits);
        assert_eq!(TypeCode::TotalZbaDebits.code(), "570");
        assert_eq!(TypeCode::from("574"), TypeCode::CumulativeZbaDebits);
        assert_eq!(TypeCode::CumulativeZbaDebits.code(), "574");
        assert_eq!(TypeCode::from("575"), TypeCode::ZbaDebit);
        assert_eq!(TypeCode::ZbaDebit.code(), "575");
        assert_eq!(TypeCode::from("577"), TypeCode::ZbaDebitTransfer);
        assert_eq!(TypeCode::ZbaDebitTransfer.code(), "577");
        assert_eq!(TypeCode::from("578"), TypeCode::ZbaDebitAdjustment);
        assert_eq!(TypeCode::ZbaDebitAdjustment.code(), "578");
        assert_eq!(
            TypeCode::from("580"),
            TypeCode::TotalControlledDisbursingDebits
        );
        assert_eq!(TypeCode::TotalControlledDisbursingDebits.code(), "580");
        assert_eq!(
            TypeCode::from("581"),
            TypeCode::IndividualControlledDisbursingDebit
        );
        assert_eq!(TypeCode::IndividualControlledDisbursingDebit.code(), "581");
        assert_eq!(
            TypeCode::from("583"),
            TypeCode::TotalDisbursingChecksPaidEarlyAmount
        );
        assert_eq!(TypeCode::TotalDisbursingChecksPaidEarlyAmount.code(), "583");
        assert_eq!(
            TypeCode::from("584"),
            TypeCode::TotalDisbursingChecksPaidLaterAmount
        );
        assert_eq!(TypeCode::TotalDisbursingChecksPaidLaterAmount.code(), "584");
        assert_eq!(
            TypeCode::from("585"),
            TypeCode::DisbursingFundingRequirement
        );
        assert_eq!(TypeCode::DisbursingFundingRequirement.code(), "585");
        assert_eq!(
            TypeCode::from("586"),
            TypeCode::FrbPresentmentEstimateFedEstimate
        );
        assert_eq!(TypeCode::FrbPresentmentEstimateFedEstimate.code(), "586");
        assert_eq!(TypeCode::from("587"), TypeCode::LateDebitsAfterNotification);
        assert_eq!(TypeCode::LateDebitsAfterNotification.code(), "587");
        assert_eq!(
            TypeCode::from("588"),
            TypeCode::TotalDisbursingChecksPaidLastAmount
        );
        assert_eq!(TypeCode::TotalDisbursingChecksPaidLastAmount.code(), "588");
        assert_eq!(TypeCode::from("590"), TypeCode::TotalDtcDebits);
        assert_eq!(TypeCode::TotalDtcDebits.code(), "590");
        assert_eq!(TypeCode::from("594"), TypeCode::TotalAtmDebits);
        assert_eq!(TypeCode::TotalAtmDebits.code(), "594");
        assert_eq!(TypeCode::from("595"), TypeCode::AtmDebit);
        assert_eq!(TypeCode::AtmDebit.code(), "595");
        assert_eq!(TypeCode::from("596"), TypeCode::TotalAprDebits);
        assert_eq!(TypeCode::TotalAprDebits.code(), "596");
        assert_eq!(TypeCode::from("597"), TypeCode::ArpDebit);
        assert_eq!(TypeCode::ArpDebit.code(), "597");
        assert_eq!(TypeCode::from("601"), TypeCode::EstimatedTotalDisbursement);
        assert_eq!(TypeCode::EstimatedTotalDisbursement.code(), "601");
        assert_eq!(TypeCode::from("602"), TypeCode::AdjustedTotalDisbursement);
        assert_eq!(TypeCode::AdjustedTotalDisbursement.code(), "602");
        assert_eq!(TypeCode::from("610"), TypeCode::TotalFundsRequired);
        assert_eq!(TypeCode::TotalFundsRequired.code(), "610");
        assert_eq!(TypeCode::from("611"), TypeCode::TotalWireTransfersOutChf);
        assert_eq!(TypeCode::TotalWireTransfersOutChf.code(), "611");
        assert_eq!(TypeCode::from("612"), TypeCode::TotalWireTransfersOutFf);
        assert_eq!(TypeCode::TotalWireTransfersOutFf.code(), "612");
        assert_eq!(TypeCode::from("613"), TypeCode::TotalInternationalDebitChf);
        assert_eq!(TypeCode::TotalInternationalDebitChf.code(), "613");
        assert_eq!(TypeCode::from("614"), TypeCode::TotalInternationalDebitFf);
        assert_eq!(TypeCode::TotalInternationalDebitFf.code(), "614");
        assert_eq!(
            TypeCode::from("615"),
            TypeCode::TotalFederalReserveBankCommercialBankDebit
        );
        assert_eq!(
            TypeCode::TotalFederalReserveBankCommercialBankDebit.code(),
            "615"
        );
        assert_eq!(
            TypeCode::from("616"),
            TypeCode::FederalReserveBankCommercialBankDebit
        );
        assert_eq!(
            TypeCode::FederalReserveBankCommercialBankDebit.code(),
            "616"
        );
        assert_eq!(TypeCode::from("617"), TypeCode::TotalSecuritiesPurchasedChf);
        assert_eq!(TypeCode::TotalSecuritiesPurchasedChf.code(), "617");
        assert_eq!(TypeCode::from("618"), TypeCode::TotalSecuritiesPurchasedFf);
        assert_eq!(TypeCode::TotalSecuritiesPurchasedFf.code(), "618");
        assert_eq!(TypeCode::from("621"), TypeCode::TotalBrokerDebitsChf);
        assert_eq!(TypeCode::TotalBrokerDebitsChf.code(), "621");
        assert_eq!(TypeCode::from("622"), TypeCode::BrokerDebit);
        assert_eq!(TypeCode::BrokerDebit.code(), "622");
        assert_eq!(TypeCode::from("623"), TypeCode::TotalBrokerDebitsFf);
        assert_eq!(TypeCode::TotalBrokerDebitsFf.code(), "623");
        assert_eq!(TypeCode::from("625"), TypeCode::TotalBrokerDebits);
        assert_eq!(TypeCode::TotalBrokerDebits.code(), "625");
        assert_eq!(TypeCode::from("626"), TypeCode::TotalFedFundsPurchased);
        assert_eq!(TypeCode::TotalFedFundsPurchased.code(), "626");
        assert_eq!(TypeCode::from("627"), TypeCode::FedFundsPurchased);
        assert_eq!(TypeCode::FedFundsPurchased.code(), "627");
        assert_eq!(TypeCode::from("628"), TypeCode::TotalCashCenterDebits);
        assert_eq!(TypeCode::TotalCashCenterDebits.code(), "628");
        assert_eq!(TypeCode::from("629"), TypeCode::CashCenterDebit);
        assert_eq!(TypeCode::CashCenterDebit.code(), "629");
        assert_eq!(TypeCode::from("630"), TypeCode::TotalDebitAdjustments);
        assert_eq!(TypeCode::TotalDebitAdjustments.code(), "630");
        assert_eq!(TypeCode::from("631"), TypeCode::DebitAdjustment);
        assert_eq!(TypeCode::DebitAdjustment.code(), "631");
        assert_eq!(TypeCode::from("632"), TypeCode::TotalTrustDebits);
        assert_eq!(TypeCode::TotalTrustDebits.code(), "632");
        assert_eq!(TypeCode::from("633"), TypeCode::TrustDebit);
        assert_eq!(TypeCode::TrustDebit.code(), "633");
        assert_eq!(TypeCode::from("634"), TypeCode::YtdAdjustmentDebit);
        assert_eq!(TypeCode::YtdAdjustmentDebit.code(), "634");
        assert_eq!(TypeCode::from("640"), TypeCode::TotalEscrowDebits);
        assert_eq!(TypeCode::TotalEscrowDebits.code(), "640");
        assert_eq!(TypeCode::from("641"), TypeCode::IndividualEscrowDebit);
        assert_eq!(TypeCode::IndividualEscrowDebit.code(), "641");
        assert_eq!(TypeCode::from("644"), TypeCode::IndividualBackValueDebit);
        assert_eq!(TypeCode::IndividualBackValueDebit.code(), "644");
        assert_eq!(TypeCode::from("646"), TypeCode::TransferCalculationDebit);
        assert_eq!(TypeCode::TransferCalculationDebit.code(), "646");
        assert_eq!(TypeCode::from("650"), TypeCode::InvestmentsPurchased);
        assert_eq!(TypeCode::InvestmentsPurchased.code(), "650");
        assert_eq!(
            TypeCode::from("651"),
            TypeCode::IndividualInvestmentPurchased
        );
        assert_eq!(TypeCode::IndividualInvestmentPurchased.code(), "651");
        assert_eq!(TypeCode::from("654"), TypeCode::InterestDebit);
        assert_eq!(TypeCode::InterestDebit.code(), "654");
        assert_eq!(
            TypeCode::from("655"),
            TypeCode::TotalInvestmentInterestDebits
        );
        assert_eq!(TypeCode::TotalInvestmentInterestDebits.code(), "655");
        assert_eq!(TypeCode::from("656"), TypeCode::SweepPrincipalBuy);
        assert_eq!(TypeCode::SweepPrincipalBuy.code(), "656");
        assert_eq!(TypeCode::from("657"), TypeCode::FuturesDebit);
        assert_eq!(TypeCode::FuturesDebit.code(), "657");
        assert_eq!(TypeCode::from("658"), TypeCode::PrincipalPaymentsDebit);
        assert_eq!(TypeCode::PrincipalPaymentsDebit.code(), "658");
        assert_eq!(TypeCode::from("659"), TypeCode::InterestAdjustmentDebit);
        assert_eq!(TypeCode::InterestAdjustmentDebit.code(), "659");
        assert_eq!(TypeCode::from("661"), TypeCode::AccountAnalysisFee);
        assert_eq!(TypeCode::AccountAnalysisFee.code(), "661");
        assert_eq!(
            TypeCode::from("662"),
            TypeCode::CorrespondentCollectionDebit
        );
        assert_eq!(TypeCode::CorrespondentCollectionDebit.code(), "662");
        assert_eq!(
            TypeCode::from("663"),
            TypeCode::CorrespondentCollectionAdjustmentDb
        );
        assert_eq!(TypeCode::CorrespondentCollectionAdjustmentDb.code(), "663");
        assert_eq!(TypeCode::from("664"), TypeCode::LoanParticipationDb);
        assert_eq!(TypeCode::LoanParticipationDb.code(), "664");
        assert_eq!(TypeCode::from("665"), TypeCode::InterceptDebits);
        assert_eq!(TypeCode::InterceptDebits.code(), "665");
        assert_eq!(TypeCode::from("666"), TypeCode::CurrencyAndCoinShipped);
        assert_eq!(TypeCode::CurrencyAndCoinShipped.code(), "666");
        assert_eq!(TypeCode::from("667"), TypeCode::FoodStampLetterDb);
        assert_eq!(TypeCode::FoodStampLetterDb.code(), "667");
        assert_eq!(TypeCode::from("668"), TypeCode::FoodStampAdjustmentDb);
        assert_eq!(TypeCode::FoodStampAdjustmentDb.code(), "668");
        assert_eq!(TypeCode::from("669"), TypeCode::ClearingSettlementDebit);
        assert_eq!(TypeCode::ClearingSettlementDebit.code(), "669");
        assert_eq!(TypeCode::from("670"), TypeCode::TotalBackValueDebits);
        assert_eq!(TypeCode::TotalBackValueDebits.code(), "670");
        assert_eq!(TypeCode::from("672"), TypeCode::BackValueAdjustmentDb);
        assert_eq!(TypeCode::BackValueAdjustmentDb.code(), "672");
        assert_eq!(TypeCode::from("673"), TypeCode::CustomerPayrollDb);
        assert_eq!(TypeCode::CustomerPayrollDb.code(), "673");
        assert_eq!(TypeCode::from("674"), TypeCode::FrbStatementRecapDb);
        assert_eq!(TypeCode::FrbStatementRecapDb.code(), "674");
        assert_eq!(
            TypeCode::from("676"),
            TypeCode::SavingsBondLetterOrAdjustmentDb
        );
        assert_eq!(TypeCode::SavingsBondLetterOrAdjustmentDb.code(), "676");
        assert_eq!(TypeCode::from("677"), TypeCode::TreasuryTaxAndLoanDebit);
        assert_eq!(TypeCode::TreasuryTaxAndLoanDebit.code(), "677");
        assert_eq!(TypeCode::from("678"), TypeCode::TransferOfTreasuryDebit);
        assert_eq!(TypeCode::TransferOfTreasuryDebit.code(), "678");
        assert_eq!(
            TypeCode::from("679"),
            TypeCode::FrbGovernmentChecksCashLetterDebit
        );
        assert_eq!(TypeCode::FrbGovernmentChecksCashLetterDebit.code(), "679");
        assert_eq!(
            TypeCode::from("681"),
            TypeCode::FrbGovernmentCheckAdjustmentDb
        );
        assert_eq!(TypeCode::FrbGovernmentCheckAdjustmentDb.code(), "681");
        assert_eq!(TypeCode::from("682"), TypeCode::FrbPostalMoneyOrderDebit);
        assert_eq!(TypeCode::FrbPostalMoneyOrderDebit.code(), "682");
        assert_eq!(
            TypeCode::from("683"),
            TypeCode::FrbPostalMoneyOrderAdjustmentDb
        );
        assert_eq!(TypeCode::FrbPostalMoneyOrderAdjustmentDb.code(), "683");
        assert_eq!(
            TypeCode::from("684"),
            TypeCode::FrbCashLetterAutoChargeDebit
        );
        assert_eq!(TypeCode::FrbCashLetterAutoChargeDebit.code(), "684");
        assert_eq!(TypeCode::from("685"), TypeCode::TotalUniversalDebits);
        assert_eq!(TypeCode::TotalUniversalDebits.code(), "685");
        assert_eq!(
            TypeCode::from("686"),
            TypeCode::FrbCashLetterAutoChargeAdjustmentDb
        );
        assert_eq!(TypeCode::FrbCashLetterAutoChargeAdjustmentDb.code(), "686");
        assert_eq!(TypeCode::from("687"), TypeCode::FrbFineSortCashLetterDebit);
        assert_eq!(TypeCode::FrbFineSortCashLetterDebit.code(), "687");
        assert_eq!(TypeCode::from("688"), TypeCode::FrbFineSortAdjustmentDb);
        assert_eq!(TypeCode::FrbFineSortAdjustmentDb.code(), "688");
        assert_eq!(TypeCode::from("689"), TypeCode::FrbFreightPaymentDebits);
        assert_eq!(TypeCode::FrbFreightPaymentDebits.code(), "689");
        assert_eq!(TypeCode::from("690"), TypeCode::TotalMiscellaneousDebits);
        assert_eq!(TypeCode::TotalMiscellaneousDebits.code(), "690");
        assert_eq!(TypeCode::from("691"), TypeCode::UniversalDebit);
        assert_eq!(TypeCode::UniversalDebit.code(), "691");
        assert_eq!(TypeCode::from("692"), TypeCode::FreightPaymentDebit);
        assert_eq!(TypeCode::FreightPaymentDebit.code(), "692");
        assert_eq!(TypeCode::from("693"), TypeCode::ItemizedDebitOver10);
        assert_eq!(TypeCode::ItemizedDebitOver10.code(), "693");
        assert_eq!(TypeCode::from("694"), TypeCode::DepositReversal);
        assert_eq!(TypeCode::DepositReversal.code(), "694");
        assert_eq!(TypeCode::from("695"), TypeCode::DepositCorrectionDebit);
        assert_eq!(TypeCode::DepositCorrectionDebit.code(), "695");
        assert_eq!(TypeCode::from("696"), TypeCode::RegularCollectionDebit);
        assert_eq!(TypeCode::RegularCollectionDebit.code(), "696");
        assert_eq!(TypeCode::from("697"), TypeCode::CumulativeDebits);
        assert_eq!(TypeCode::CumulativeDebits.code(), "697");
        assert_eq!(TypeCode::from("698"), TypeCode::MiscellaneousFees);
        assert_eq!(TypeCode::MiscellaneousFees.code(), "698");
        assert_eq!(TypeCode::from("699"), TypeCode::MiscellaneousDebit);
        assert_eq!(TypeCode::MiscellaneousDebit.code(), "699");
        assert_eq!(TypeCode::from("701"), TypeCode::PrincipalLoanBalance);
        assert_eq!(TypeCode::PrincipalLoanBalance.code(), "701");
        assert_eq!(TypeCode::from("703"), TypeCode::AvailableCommitmentAmount);
        assert_eq!(TypeCode::AvailableCommitmentAmount.code(), "703");
        assert_eq!(TypeCode::from("705"), TypeCode::PaymentAmountDue);
        assert_eq!(TypeCode::PaymentAmountDue.code(), "705");
        assert_eq!(TypeCode::from("707"), TypeCode::PrincipalAmountPastDue);
        assert_eq!(TypeCode::PrincipalAmountPastDue.code(), "707");
        assert_eq!(TypeCode::from("709"), TypeCode::InterestAmountPastDue);
        assert_eq!(TypeCode::InterestAmountPastDue.code(), "709");
        assert_eq!(TypeCode::from("720"), TypeCode::TotalLoanPayment);
        assert_eq!(TypeCode::TotalLoanPayment.code(), "720");
        assert_eq!(TypeCode::from("721"), TypeCode::AmountAppliedToInterest);
        assert_eq!(TypeCode::AmountAppliedToInterest.code(), "721");
        assert_eq!(TypeCode::from("722"), TypeCode::AmountAppliedToPrincipal);
        assert_eq!(TypeCode::AmountAppliedToPrincipal.code(), "722");
        assert_eq!(TypeCode::from("723"), TypeCode::AmountAppliedToEscrow);
        assert_eq!(TypeCode::AmountAppliedToEscrow.code(), "723");
        assert_eq!(TypeCode::from("724"), TypeCode::AmountAppliedToLateCharges);
        assert_eq!(TypeCode::AmountAppliedToLateCharges.code(), "724");
        assert_eq!(TypeCode::from("725"), TypeCode::AmountAppliedToBuydown);
        assert_eq!(TypeCode::AmountAppliedToBuydown.code(), "725");
        assert_eq!(TypeCode::from("726"), TypeCode::AmountAppliedToMiscFees);
        assert_eq!(TypeCode::AmountAppliedToMiscFees.code(), "726");
        assert_eq!(
            TypeCode::from("727"),
            TypeCode::AmountAppliedToDeferredInterestDetail
        );
        assert_eq!(
            TypeCode::AmountAppliedToDeferredInterestDetail.code(),
            "727"
        );
        assert_eq!(
            TypeCode::from("728"),
            TypeCode::AmountAppliedToServiceCharge
        );
        assert_eq!(TypeCode::AmountAppliedToServiceCharge.code(), "728");
        assert_eq!(TypeCode::from("760"), TypeCode::LoanDisbursement);
        assert_eq!(TypeCode::LoanDisbursement.code(), "760");
        assert_eq!(
            TypeCode::from("890"),
            TypeCode::ContainsNonMonetaryInformation
        );
        assert_eq!(TypeCode::ContainsNonMonetaryInformation.code(), "890");
    }
}
