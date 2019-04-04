use crate::sreq::Sreq;
use crate::znp_codec::Subsys;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ZbDeviceInfoProp {
    DevState = 0,
    IeeeAddr = 1,
    ShortAddr = 2,
    ParentShortAddr = 3,
    ParentIeeeAddr = 4,
    Channel = 5,
    PanId = 6,
    ExtPanId = 7,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ZbGetDeviceInfoReq {
    pub param: ZbDeviceInfoProp,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ZbGetDeviceInfoRsp {
    pub param: ZbDeviceInfoProp,
    pub value: [u8; 8],
}
impl Sreq for ZbGetDeviceInfoReq {
    type Srsp = ZbGetDeviceInfoRsp;
    const SUBSYS: Subsys = Subsys::SAPI;
    const CMD_ID: u8 = 0x06;
    const MAX_SIZE: usize = 9;
}

#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum ConfigId {
    ///EXTADDR
    Extaddr = 0x1,
    ///BOOTCOUNTER
    Bootcounter = 0x2,
    ///STARTUP_OPTION
    StartupOption = 0x3,
    ///START_DELAY
    StartDelay = 0x4,
    ///NIB
    Nib = 0x21,
    ///DEVICE_LIST
    DeviceList = 0x22,
    ///ADDRMGR
    Addrmgr = 0x23,
    ///POLL_RATE
    PollRate = 0x24,
    ///QUEUED_POLL_RATE
    QueuedPollRate = 0x25,
    ///RESPONSE_POLL_RATE
    ResponsePollRate = 0x26,
    ///REJOIN_POLL_RATE
    RejoinPollRate = 0x27,
    ///DATA_RETRIES
    DataRetries = 0x28,
    ///POLL_FAILURE_RETRIES
    PollFailureRetries = 0x29,
    ///STACK_PROFILE
    StackProfile = 0x2a,
    ///INDIRECT_MSG_TIMEOUT
    IndirectMsgTimeout = 0x2b,
    ///ROUTE_EXPIRY_TIME
    RouteExpiryTime = 0x2c,
    ///EXTENDED_PAN_ID
    ExtendedPanId = 0x2d,
    ///BCAST_RETRIES
    BcastRetries = 0x2e,
    ///PASSIVE_ACK_TIMEOUT
    PassiveAckTimeout = 0x2f,
    ///BCAST_DELIVERY_TIME
    BcastDeliveryTime = 0x30,
    ///NWK_MODE
    NwkMode = 0x31,
    ///CONCENTRATOR_ENABLE
    ConcentratorEnable = 0x32,
    ///CONCENTRATOR_DISCOVERY
    ConcentratorDiscovery = 0x33,
    ///CONCENTRATOR_RADIUS
    ConcentratorRadius = 0x34,
    ///CONCENTRATOR_RC
    ConcentratorRc = 0x36,
    ///NWK_MGR_MODE
    NwkMgrMode = 0x37,
    ///SRC_RTG_EXPIRY_TIME
    SrcRtgExpiryTime = 0x38,
    ///ROUTE_DISCOVERY_TIME
    RouteDiscoveryTime = 0x39,
    ///NWK_ACTIVE_KEY_INFO
    NwkActiveKeyInfo = 0x3a,
    ///NWK_ALTERN_KEY_INFO
    NwkAlternKeyInfo = 0x3b,
    ///ROUTER_OFF_ASSOC_CLEANUP
    RouterOffAssocCleanup = 0x3c,
    ///NWK_LEAVE_REQ_ALLOWED
    NwkLeaveReqAllowed = 0x3d,
    ///NWK_CHILD_AGE_ENABLE
    NwkChildAgeEnable = 0x3e,
    ///DEVICE_LIST_KA_TIMEOUT
    DeviceListKaTimeout = 0x3f,
    ///BINDING_TABLE
    BindingTable = 0x41,
    ///GROUP_TABLE
    GroupTable = 0x42,
    ///APS_FRAME_RETRIES
    ApsFrameRetries = 0x43,
    ///APS_ACK_WAIT_DURATION
    ApsAckWaitDuration = 0x44,
    ///APS_ACK_WAIT_MULTIPLIER
    ApsAckWaitMultiplier = 0x45,
    ///BINDING_TIME
    BindingTime = 0x46,
    ///APS_USE_EXT_PANID
    ApsUseExtPanid = 0x47,
    ///APS_USE_INSECURE_JOIN
    ApsUseInsecureJoin = 0x48,
    ///COMMISSIONED_NWK_ADDR
    CommissionedNwkAddr = 0x49,
    ///APS_NONMEMBER_RADIUS
    ApsNonmemberRadius = 0x4b,
    ///APS_LINK_KEY_TABLE
    ApsLinkKeyTable = 0x4c,
    ///APS_DUPREJ_TIMEOUT_INC
    ApsDuprejTimeoutInc = 0x4d,
    ///APS_DUPREJ_TIMEOUT_COUNT
    ApsDuprejTimeoutCount = 0x4e,
    ///APS_DUPREJ_TABLE_SIZE
    ApsDuprejTableSize = 0x4f,
    ///DIAGNOSTIC_STATS
    DiagnosticStats = 0x50,
    ///SECURITY_LEVEL
    SecurityLevel = 0x61,
    ///PRECFGKEY
    Precfgkey = 0x62,
    ///PRECFGKEYS_ENABLE
    PrecfgkeysEnable = 0x63,
    ///SECURITY_MODE
    SecurityMode = 0x64,
    ///SECURE_PERMIT_JOIN
    SecurePermitJoin = 0x65,
    ///APS_LINK_KEY_TYPE
    ApsLinkKeyType = 0x66,
    ///APS_ALLOW_R19_SECURITY
    ApsAllowR19Security = 0x67,
    ///IMPLICIT_CERTIFICATE
    ImplicitCertificate = 0x69,
    ///DEVICE_PRIVATE_KEY
    DevicePrivateKey = 0x6a,
    ///CA_PUBLIC_KEY
    CaPublicKey = 0x6b,
    ///KE_MAX_DEVICES
    KeMaxDevices = 0x6c,
    ///USE_DEFAULT_TCLK
    UseDefaultTclk = 0x6d,
    ///RNG_COUNTER
    RngCounter = 0x6f,
    ///RANDOM_SEED
    RandomSeed = 0x70,
    ///TRUSTCENTER_ADDR
    TrustcenterAddr = 0x71,
    ///USERDESC
    Userdesc = 0x81,
    ///NWKKEY
    Nwkkey = 0x82,
    ///PANID
    Panid = 0x83,
    ///CHANLIST
    Chanlist = 0x84,
    ///LEAVE_CTRL
    LeaveCtrl = 0x85,
    ///SCAN_DURATION
    ScanDuration = 0x86,
    ///LOGICAL_TYPE
    LogicalType = 0x87,
    ///NWKMGR_MIN_TX
    NwkmgrMinTx = 0x88,
    ///NWKMGR_ADDR
    NwkmgrAddr = 0x89,
    ///ZDO_DIRECT_CB
    ZdoDirectCb = 0x8f,
    ///SCENE_TABLE
    SceneTable = 0x91,
    ///MIN_FREE_NWK_ADDR
    MinFreeNwkAddr = 0x92,
    ///MAX_FREE_NWK_ADDR
    MaxFreeNwkAddr = 0x93,
    ///MIN_FREE_GRP_ID
    MinFreeGrpId = 0x94,
    ///MAX_FREE_GRP_ID
    MaxFreeGrpId = 0x95,
    ///MIN_GRP_IDS
    MinGrpIds = 0x96,
    ///MAX_GRP_IDS
    MaxGrpIds = 0x97,
    ///OTA_BLOCK_REQ_DELAY
    OtaBlockReqDelay = 0x98,
    ///SAPI_ENDPOINT
    SapiEndpoint = 0xa1,
    ///SAS_SHORT_ADDR
    SasShortAddr = 0xb1,
    ///SAS_EXT_PANID
    SasExtPanid = 0xb2,
    ///SAS_PANID
    SasPanid = 0xb3,
    ///SAS_CHANNEL_MASK
    SasChannelMask = 0xb4,
    ///SAS_PROTOCOL_VER
    SasProtocolVer = 0xb5,
    ///SAS_STACK_PROFILE
    SasStackProfile = 0xb6,
    ///SAS_STARTUP_CTRL
    SasStartupCtrl = 0xb7,
    ///SAS_TC_ADDR
    SasTcAddr = 0xc1,
    ///SAS_TC_MASTER_KEY
    SasTcMasterKey = 0xc2,
    ///SAS_NWK_KEY
    SasNwkKey = 0xc3,
    ///SAS_USE_INSEC_JOIN
    SasUseInsecJoin = 0xc4,
    ///SAS_PRECFG_LINK_KEY
    SasPrecfgLinkKey = 0xc5,
    ///SAS_NWK_KEY_SEQ_NUM
    SasNwkKeySeqNum = 0xc6,
    ///SAS_NWK_KEY_TYPE
    SasNwkKeyType = 0xc7,
    ///SAS_NWK_MGR_ADDR
    SasNwkMgrAddr = 0xc8,
    ///SAS_CURR_TC_MASTER_KEY
    SasCurrTcMasterKey = 0xd1,
    ///SAS_CURR_NWK_KEY
    SasCurrNwkKey = 0xd2,
    ///SAS_CURR_PRECFG_LINK_KEY
    SasCurrPrecfgLinkKey = 0xd3,
}

/// ZB_READ_CONFIGURATION
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadConfig {
    pub id: ConfigId,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadConfigRsp {
    /// Success 0 or Failure 1
    pub status: u8,
    pub id: ConfigId,
    pub value: Vec<u8>,
}
impl Sreq for ReadConfig {
    type Srsp = ReadConfigRsp;
    const SUBSYS: Subsys = Subsys::SAPI;
    const CMD_ID: u8 = 0x04;
    const MAX_SIZE: usize = 0x83;
}
