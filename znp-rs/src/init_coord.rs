use super::cmd::zb::{ConfigId, ReadConfig};
use super::znp::Znp;
pub async fn init(znp: &mut Znp) {
    struct NvParam {
        configid: ConfigId,
        len: u8,
        value: Vec<u8>,
    }
    let startup_option = NvParam {
        configid: ConfigId::StartupOption,
        len: 0x01,
        value: vec![0x00],
    };
    let pan_id = NvParam {
        configid: ConfigId::Panid,
        len: 0x02,
        // Koenk default is 0x1a62, shepherd value is 0xffff
        value: vec![0x62, 0x1A],
    };
    let ext_pan_id = NvParam {
        configid: ConfigId::ExtendedPanId,
        len: 0x08,
        value: vec![0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD],
    };
    let channel_list = NvParam {
        configid: ConfigId::Chanlist,
        len: 0x04,
        // Little endian. Default is 0x00000800 for CH11;  Ex: value: [ 0x00, 0x00, 0x00, 0x04 ] for CH26, [ 0x00, 0x00, 0x20, 0x00 ] for CH15.
        value: vec![0x00, 0x08, 0x00, 0x00],
    };
    let logical_type = NvParam {
        configid: ConfigId::LogicalType,
        len: 0x01,
        value: vec![0x00],
    };
    let precfgkey = NvParam {
        configid: ConfigId::Precfgkey,
        len: 0x10,
        value: vec![
            0x01, 0x03, 0x05, 0x07, 0x09, 0x0B, 0x0D, 0x0F, 0x00, 0x02, 0x04, 0x06, 0x08, 0x0A,
            0x0C, 0x0D,
        ],
    };
    let precfgkeys_enable = NvParam {
        configid: ConfigId::PrecfgkeysEnable,
        len: 0x01,
        value: vec![0x00], // value: 0 (FALSE) only coord defualtKey need to be set, and OTA to set other devices in the network.
                           // value: 1 (TRUE) Not only coord, but also all devices need to set their defualtKey (the same key). Or they can't not join the network.
    };
    let zdo_direct_cb = NvParam {
        configid: ConfigId::ZdoDirectCb,
        len: 0x01,
        value: vec![0x01],
    };
    let all_params = [
        startup_option,
        pan_id,
        ext_pan_id,
        channel_list,
        logical_type,
        precfgkey,
        precfgkeys_enable,
        zdo_direct_cb,
    ];
    for param in &all_params {
        let cmd = ReadConfig { id: param.configid };
        let res = await!(znp.sreq(cmd));
        println!("expected {:x?}", &param.value);
        println!("got      {:x?}", res.unwrap().value);
    }
}
