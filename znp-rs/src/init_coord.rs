use super::cmd;
use super::znp::Sender;
use cmd::sys::{ResetReq, ResetType};
use cmd::types::ShortAddr;
use cmd::zb::{ConfigId, ReadConfig};
pub async fn init(znp: &mut Sender) {
    use cmd::zb::{ZbDeviceInfoProp, ZbGetDeviceInfoReq};
    for param in vec![
        ZbDeviceInfoProp::DevState,
        ZbDeviceInfoProp::IeeeAddr,
        ZbDeviceInfoProp::ShortAddr,
    ] {
        let cmd = ZbGetDeviceInfoReq { param };
        let res = await!(znp.sreq(cmd));
        println!("{:x?}", res);
    }

    use cmd::sys::NvRead;
    let cmd = NvRead {
        /// ZNP_HAS_CONFIGURED
        id: 0x0F00,
        offset: 0x00,
    };
    let res = await!(znp.sreq(cmd));
    // Expecting [0x55]
    println!("{:x?}", res);
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

    use cmd::zdo::StartupFromApp;
    let cmd = StartupFromApp {
        delay: 100, /* this was 100, why? When would you want this? */
    };
    let res = await!(znp.sreq(cmd));
    println!("StartupFromApp {:x?}", res);

    use cmd::zdo::NodeDescReq;
    let cmd = NodeDescReq {
        dest_addr: ShortAddr(0),
        query_addr: ShortAddr(0),
    };
    let res = await!(znp.sreq(cmd));
    println!("NodeDescReq {:x?}", res.unwrap().status);

    use cmd::zdo::ActiveEpReq;
    let cmd = ActiveEpReq {
        dest_addr: ShortAddr(0),
        query_addr: ShortAddr(0),
    };
    let res = await!(znp.sreq(cmd));
    println!("Active EPs {:x?}", res.unwrap());

    use cmd::af::Register;
    let endpoint_profile_ids = [0x0104, 0x0101, 0x0105, 0x0107, 0x0108, 0x0109];
    for (ep, &app_prof) in (1..).zip(&endpoint_profile_ids) {
        let cmd = Register {
            ep,
            app_prof,
            dev_type: 0x0005,
            // in_clusters: vec![0, 1026, 1029, 6],
            ..Default::default()
        };
        let res = await!(znp.sreq(cmd));
        println!("Register ep {:x} {:x?}", ep, res.unwrap());
    }
    let cmd = Register {
        ep: 11,
        app_prof: 0x0104,
        dev_type: 0x0400,
        in_clusters: vec![0x0000, 0x0501, 0x0003],
        out_clusters: vec![0x0500, 0x0502, 0x0003],
        ..Default::default()
    };
    let res = await!(znp.sreq(cmd));
    println!("Register ep 11 {:x?}", res.unwrap());

    let cmd = ActiveEpReq {
        dest_addr: ShortAddr(0),
        query_addr: ShortAddr(0),
    };
    let res = await!(znp.sreq(cmd));
    println!("Active EPs {:x?}", res.unwrap());

    use cmd::zdo::MgmtPermitJoinReq;
    let cmd = MgmtPermitJoinReq {
        addr_mode: 0x02,
        dest_addr: ShortAddr(0x0000),
        duration: 0x0,
        tc_significance: 0,
    };
    let res = await!(znp.sreq(cmd));
    println!("MgmtPermitJoinReq {:x?}", res.unwrap().status);
    let cmd = MgmtPermitJoinReq {
        addr_mode: 0x0f,
        dest_addr: ShortAddr(0xfffc),
        duration: 0xff,
        tc_significance: 0,
    };
    let res = await!(znp.sreq(cmd));
    println!("MgmtPermitJoinReq {:x?}", res.unwrap().status);
    // let cmd = MgmtPermitJoinReq {
    //     addr_mode: 0x02,
    //     dest_addr: ShortAddr(0x0000),
    //     duration: 0xff,
    //     tc_significance: 0,
    // };
    // let res = await!(znp.sreq(cmd));
    // println!("MgmtPermitJoinReq {:x?}", res.unwrap().status);
}

pub async fn soft_reset(znp: &mut Sender) {
    await!(znp.areq(ResetReq {
        typ: ResetType::Soft
    }))
}
