use super::cmd::zb::ReadConfig;
use super::znp::Znp;
pub async fn init(znp: &mut Znp) {
    struct NvParam {
        configid: u8,
        len: u8,
        value: Vec<u8>,
    }
    let startupOption = NvParam {
        /// STARTUP_OPTION
        configid: 0x03,
        len: 0x01,
        value: vec![0x00],
    };
    let panId = NvParam {
        /// PANID
        configid: 0x83,
        len: 0x02,
        value: vec![0xFF, 0xFF],
    };
    let extPanId = NvParam {
        /// EXTENDED_PAN_ID
        configid: 0x2D,
        len: 0x08,
        value: vec![0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD],
    };
    let channelList = NvParam {
        /// CHANLIST
        configid: 0x84,
        len: 0x04,
        value: vec![0x00, 0x08, 0x00, 0x00], // Little endian. Default is 0x00000800 for CH11;  Ex: value: [ 0x00, 0x00, 0x00, 0x04 ] for CH26, [ 0x00, 0x00, 0x20, 0x00 ] for CH15.
    };
    let logicalType = NvParam {
        /// LOGICAL_TYPE
        configid: 0x87,
        len: 0x01,
        value: vec![0x00],
    };
    let precfgkey = NvParam {
        /// PRECFGKEY
        configid: 0x62,
        len: 0x10,
        value: vec![
            0x01, 0x03, 0x05, 0x07, 0x09, 0x0B, 0x0D, 0x0F, 0x00, 0x02, 0x04, 0x06, 0x08, 0x0A,
            0x0C, 0x0D,
        ],
    };
    let precfgkeysEnable = NvParam {
        /// PRECFGKEYS_ENABLE
        configid: 0x63,
        len: 0x01,
        value: vec![0x00], // value: 0 (FALSE) only coord defualtKey need to be set, and OTA to set other devices in the network.
                           // value: 1 (TRUE) Not only coord, but also all devices need to set their defualtKey (the same key). Or they can't not join the network.
    };
    let zdoDirectCb = NvParam {
        /// ZDO_DIRECT_CB
        configid: 0x8F,
        len: 0x01,
        value: vec![0x01],
    };
    let allParams = [
        startupOption,
        panId,
        extPanId,
        channelList,
        logicalType,
        precfgkey,
        precfgkeysEnable,
        zdoDirectCb,
    ];
    for param in &allParams {
        let cmd = ReadConfig { id: param.configid };
        let res = await!(znp.sreq(cmd));
        println!("expected {:x?}", &param.value);
        println!("got      {:x?}", res.unwrap().value);
    }
}
