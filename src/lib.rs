mod consts;
mod gatts_profile_inst;
mod prepare_type_env;

use consts::*;
use gatts_profile_inst::*;
use prepare_type_env::*;
use log::{info, error};
use std::os::raw::c_char;

use esp_idf_hal::mutex::Mutex as EspMutex;
use esp_idf_sys::EspError;

type Singleton<T> = EspMutex<Option<Box<T>>>;

static GATTS_EVENT_HANDLER: Singleton<dyn FnMut(
    esp_idf_sys::esp_gatts_cb_event_t,
    esp_idf_sys::esp_gatt_if_t,
    *mut esp_idf_sys::esp_ble_gatts_cb_param_t,
) + Send> = EspMutex::new(None);

static STATUS: Singleton<Status> = EspMutex::new(None);

static GATTS_PROFILE_A_EVENT_HANDLER: Singleton<dyn FnMut(
    esp_idf_sys::esp_gatts_cb_event_t,
    esp_idf_sys::esp_gatt_if_t,
    *mut esp_idf_sys::esp_ble_gatts_cb_param_t,
) + Send> = EspMutex::new(None);

static GATTS_PROFILE_B_EVENT_HANDLER: Singleton<dyn FnMut(
    esp_idf_sys::esp_gatts_cb_event_t,
    esp_idf_sys::esp_gatt_if_t,
    *mut esp_idf_sys::esp_ble_gatts_cb_param_t,
) + Send> = EspMutex::new(None);

static GAP_EVENT_HANDLER: Singleton<dyn FnMut(
    esp_idf_sys::esp_gap_ble_cb_event_t,
    *mut esp_idf_sys::esp_ble_gap_cb_param_t,
) + Send> = EspMutex::new(None);

#[derive(Default)]
pub struct Status {
    gl_profile_tab: [GattsProfileInst; PROFILE_NUM as usize],
    adv_config_done: u8,
    adv_data: esp_idf_sys::esp_ble_adv_data_t,
    scan_rsp_data: esp_idf_sys::esp_ble_adv_data_t,
    a_property: esp_idf_sys::esp_gatt_char_prop_t,
    b_property: esp_idf_sys::esp_gatt_char_prop_t,
    a_prepare_write_env: PrepareTypeEnv,
    b_prepare_write_env: PrepareTypeEnv,
    gatts_demo_char1_val: esp_idf_sys::esp_attr_value_t,
    adv_params: esp_idf_sys::esp_ble_adv_params_t,
    //_marker: std::marker::PhantomData<&'a ()>,
}

impl Status {
    pub fn init() -> anyhow::Result<()> {
        *STATUS.lock() = Some(Box::new(
                Self {
                    gl_profile_tab: [
                        GattsProfileInst {
                            gatts_if: esp_idf_sys::ESP_GATT_IF_NONE as u16,
                            gatts_cb: Some(Self::gatts_profile_a_event_handler),
                            ..Default::default()
                        },
                        GattsProfileInst {
                            gatts_if: esp_idf_sys::ESP_GATT_IF_NONE as u16,
                            gatts_cb: Some(Self::gatts_profile_b_event_handler),
                            ..Default::default()
                        },
                    ],
                    adv_config_done: 0,
                    adv_data: esp_idf_sys::esp_ble_adv_data_t {
                        set_scan_rsp: false,
                        include_name: true,
                        include_txpower: false,
                        min_interval: 0x0006, //slave connection min interval, Time = min_interval * 1.25 msec
                        max_interval: 0x0010, //slave connection max interval, Time = max_interval * 1.25 msec
                        appearance: 0x00,
                        manufacturer_len: 0, //TEST_MANUFACTURER_DATA_LEN,
                        p_manufacturer_data: std::ptr::null::<()>() as *mut u8, //&test_manufacturer[0]
                        service_data_len: 0,
                        p_service_data: std::ptr::null::<()>() as *mut u8,
                        service_uuid_len: ADV_SERVICE_UUID128.len() as u16,
                        p_service_uuid: ADV_SERVICE_UUID128.as_mut_ptr(),
                        flag: (ESP_BLE_ADV_FLAG_GEN_DISC | ESP_BLE_ADV_FLAG_BREDR_NOT_SPT),
                    },
                    scan_rsp_data: esp_idf_sys::esp_ble_adv_data_t {
                        set_scan_rsp: true,
                        include_name: true,
                        include_txpower: true,
                        //.min_interval: 0x0006,
                        //.max_interval: 0x0010,
                        appearance: 0x00,
                        manufacturer_len: 0, //TEST_MANUFACTURER_DATA_LEN,
                        p_manufacturer_data: std::ptr::null::<()>() as *mut u8, //&test_manufacturer[0],
                        service_data_len: 0,
                        p_service_data: std::ptr::null::<()>() as *mut u8,
                        service_uuid_len: ADV_SERVICE_UUID128.len() as u16,
                        p_service_uuid: ADV_SERVICE_UUID128.as_mut_ptr(),
                        flag: (ESP_BLE_ADV_FLAG_GEN_DISC | ESP_BLE_ADV_FLAG_BREDR_NOT_SPT),
                        ..Default::default()
                    },
                    a_property: 0,
                    b_property: 0,
                    gatts_demo_char1_val: esp_idf_sys::esp_attr_value_t {
                        attr_max_len: GATTS_DEMO_CHAR_VAL_LEN_MAX,
                        attr_len: CHAR1_STR.len() as u16,
                        attr_value: CHAR1_STR.as_mut_ptr(),
                    },
                    adv_params: esp_idf_sys::esp_ble_adv_params_t {
                        adv_int_min: 0x20,
                        adv_int_max: 0x40,
                        adv_type: esp_idf_sys::esp_ble_adv_type_t_ADV_TYPE_IND,
                        own_addr_type: esp_idf_sys::esp_ble_addr_type_t_BLE_ADDR_TYPE_PUBLIC,
                        //.peer_addr            :
                        //.peer_addr_type       :
                        channel_map: esp_idf_sys::esp_ble_adv_channel_t_ADV_CHNL_ALL,
                        adv_filter_policy:
                            esp_idf_sys::esp_ble_adv_filter_t_ADV_FILTER_ALLOW_SCAN_ANY_CON_ANY,
                            ..Default::default()
                    },
                    ..Default::default()
                }
        ));
        unsafe {Self::register_gatts_event_handler()?;}
        unsafe {Self::register_gap_event_handler()?;}
        unsafe {Self::register_gatts_profile_a_event_handler()?;}
        unsafe {Self::register_gatts_profile_b_event_handler()?;}
        Ok(())
    }
}


impl Status {
    unsafe fn register_gatts_event_handler(
    ) -> anyhow::Result<()> {
        *GATTS_EVENT_HANDLER.lock() = Some(
            Box::new(|event, gatts_if, param_ptr| {
                let param = *param_ptr;
                if let Some(ref mut status) = *STATUS.lock() {
                    /* If event is register event, store the gatts_if for each profile */
                    if event == esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_REG_EVT {
                        if param.reg.status == esp_idf_sys::esp_gatt_status_t_ESP_GATT_OK {
                            status.gl_profile_tab[param.reg.app_id as usize].gatts_if = gatts_if as u16;
                        } else {
                            error!(
                                "Reg app failed, app_id {:04x}, status {}",
                                param.reg.app_id, param.reg.status
                            );
                            return;
                        }
                    }

                    /* If the gatts_if equal to profile A, call profile A cb handler,
                     * so here call each profile's callback */
                    for idx in 0..PROFILE_NUM {
                        if gatts_if as u32 == ESP_GATT_IF_NONE || /* ESP_GATT_IF_NONE, not specify a certain gatt_if, need to call every profile cb function */
                            gatts_if as u16 == status.gl_profile_tab[idx as usize].gatts_if
                            {
                                if let Some(gatts_cb) = status.gl_profile_tab[idx as usize].gatts_cb { gatts_cb(event, gatts_if, param_ptr) };
                            }
                    }
                } else {
                    panic!("status is not available")
                }
            })

        );
        Ok(())
    }

    unsafe fn register_gatts_profile_a_event_handler(
    ) -> anyhow::Result<()> {
        *GATTS_PROFILE_A_EVENT_HANDLER.lock() = Some(
            Box::new(|event, gatts_if, param_ptr| {
                let param = *param_ptr;
                if let Some(ref mut status) = *STATUS.lock() {
                    match event {
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_REG_EVT => {
                            info!(
                                "REGISTER_APP_EVT, status {}, app_id {}",
                                param.reg.status, param.reg.app_id
                            );
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize]
                                .service_id
                                .is_primary = true;
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize]
                                .service_id
                                .id
                                .inst_id = 0x00;
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize]
                                .service_id
                                .id
                                .uuid
                                .len = ESP_UUID_LEN_16;
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize]
                                .service_id
                                .id
                                .uuid
                                .uuid
                                .uuid16 = GATTS_SERVICE_UUID_TEST_A;

                            let test_device_name_ptr = TEST_DEVICE_NAME.as_ptr() as *const c_char;
                            if let Some(err) = EspError::from(unsafe {
                                esp_idf_sys::esp_ble_gap_set_device_name(test_device_name_ptr)
                            }) {
                                error!(
                                    "{}: set device name failed, error code =  {}",
                                    err,
                                    err.code()
                                );
                            };
                            if CONFIG_SET_RAW_ADV_DATA {
                                let raw_adv_data_ptr = RAW_ADV_DATA.as_mut_ptr();
                                let raw_adv_data_len = RAW_ADV_DATA.len() as u32;
                                if let Some(err) = EspError::from(unsafe {
                                    esp_idf_sys::esp_ble_gap_config_adv_data_raw(
                                        raw_adv_data_ptr,
                                        raw_adv_data_len,
                                    )
                                }) {
                                    error!(
                                        "{}: config raw adv data failed, error code = {}",
                                        err,
                                        err.code()
                                    );
                                };

                                status.adv_config_done |= ADV_CONFIG_FLAG;

                                let raw_scan_rsp_data_ptr = RAW_SCAN_RSP_DATA.as_mut_ptr();
                                let raw_scan_rsp_data_len = RAW_SCAN_RSP_DATA.len() as u32;
                                if let Some(err) = EspError::from(unsafe {
                                    esp_idf_sys::esp_ble_gap_config_scan_rsp_data_raw(
                                        raw_scan_rsp_data_ptr,
                                        raw_scan_rsp_data_len,
                                    )
                                }) {
                                    error!(
                                        "{}: config raw scan rsp data failed, error code = {}",
                                        err,
                                        err.code()
                                    );
                                };

                                status.adv_config_done |= SCAN_RSP_CONFIG_FLAG;
                            } else {
                                //config adv data
                                let adv_data_ptr = Box::into_raw(Box::new(status.adv_data));
                                if let Some(err) = EspError::from(unsafe {
                                    esp_idf_sys::esp_ble_gap_config_adv_data(adv_data_ptr)
                                }) {
                                    error!(
                                        "{}: config adv data failed, error code = {}",
                                        err,
                                        err.code()
                                    );
                                };

                                status.adv_config_done |= ADV_CONFIG_FLAG;

                                //config scan response data
                                let mut scan_rsp_data_ptr = Box::into_raw(Box::new(status.scan_rsp_data));
                                if let Some(err) = EspError::from(unsafe {
                                    esp_idf_sys::esp_ble_gap_config_adv_data(scan_rsp_data_ptr)
                                }) {
                                    error!(
                                        "{}: config scan response data failed, error code = {}",
                                        err,
                                        err.code()
                                    );
                                };

                                status.adv_config_done |= SCAN_RSP_CONFIG_FLAG;
                            }
                            let service_id =
                                Box::new(status.gl_profile_tab[PROFILE_A_APP_ID as usize].service_id);
                            let service_id_ptr = Box::into_raw(service_id);
                            if let Some(err) = EspError::from(unsafe {
                                esp_idf_sys::esp_ble_gatts_create_service(
                                    gatts_if,
                                    service_id_ptr,
                                    GATTS_NUM_HANDLE_TEST_A,
                                )
                            }) {
                                error!(
                                    "{}: create service failed, error code = {}",
                                    err,
                                    err.code()
                                );
                            };
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_READ_EVT => {
                            info!(
                                "GATT_READ_EVT, conn_id {}, trans_id {}, handle {}",
                                param.read.conn_id, param.read.trans_id, param.read.handle
                            );

                            let rsp_ptr = std::ptr::null_mut::<esp_idf_sys::esp_gatt_rsp_t>();
                            std::ptr::write_bytes::<esp_idf_sys::esp_gatt_rsp_t>(
                                rsp_ptr,
                                0,
                                std::mem::size_of::<esp_idf_sys::esp_gatt_rsp_t>(),
                            );
                            let mut rsp = *rsp_ptr;
                            rsp.attr_value.handle = param.read.handle;
                            rsp.attr_value.len = 4;
                            rsp.attr_value.value[0] = 0xde;
                            rsp.attr_value.value[1] = 0xed;
                            rsp.attr_value.value[2] = 0xbe;
                            rsp.attr_value.value[3] = 0xef;

                            if let Some(err) = EspError::from(unsafe {
                                esp_idf_sys::esp_ble_gatts_send_response(
                                    gatts_if,
                                    param.read.conn_id,
                                    param.read.trans_id,
                                    esp_idf_sys::esp_gatt_status_t_ESP_GATT_OK,
                                    rsp_ptr,
                                )
                            }) {
                                error!("{}: send response failed, error code = {}", err, err.code());
                            };
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_WRITE_EVT => {
                            info!(
                                "GATT_WRITE_EVT, conn_id {}, trans_id {}, handle {}",
                                param.write.conn_id, param.write.trans_id, param.write.handle
                            );

                            if !param.write.is_prep {
                                info!(
                                    "GATT_WRITE_EVT, value len {}, value :{}",
                                    param.write.len, *param.write.value
                                );

                                if status.gl_profile_tab[PROFILE_A_APP_ID as usize].descr_handle
                                    == param.write.handle
                                        && param.write.len == 2
                                {
                                    let descr_value =
                                        u16::from(*param.write.value.add(1)) << 8 | u16::from(*param.write.value.add(0));
                                    if descr_value == 0x0001 {
                                        if (status.a_property & ESP_GATT_CHAR_PROP_BIT_NOTIFY) != 0 {
                                            info!("notify enable");
                                            let mut notify_data = [0 as u8; 15];
                                            for i in 0..notify_data.len() {
                                                notify_data[i] = i as u8 % 0xff;
                                            }
                                            //the size of notify_data[] need less than MTU size
                                            if let Some(err) = EspError::from(unsafe {
                                                esp_idf_sys::esp_ble_gatts_send_indicate(
                                                    gatts_if,
                                                    param.write.conn_id,
                                                    status.gl_profile_tab[PROFILE_A_APP_ID as usize].char_handle,
                                                    notify_data.len() as u16,
                                                    notify_data.as_mut_ptr(),
                                                    false,
                                                )
                                            }) {
                                                error!(
                                                    "{}: send indicate failed, error code = {}",
                                                    err,
                                                    err.code()
                                                );
                                            };
                                        }
                                    } else if descr_value == 0x0002 {
                                        if (status.a_property & ESP_GATT_CHAR_PROP_BIT_INDICATE) != 0 {
                                            info!("indicate enable");
                                            let mut indicate_data = [0 as u8; 15];
                                            for i in 0..indicate_data.len() {
                                                indicate_data[i] = i as u8 % 0xff;
                                            }
                                            //the size of indicate_data[] need less than MTU size
                                            if let Some(err) = EspError::from(unsafe {
                                                esp_idf_sys::esp_ble_gatts_send_indicate(
                                                    gatts_if,
                                                    param.write.conn_id,
                                                    status.gl_profile_tab[PROFILE_A_APP_ID as usize].char_handle,
                                                    indicate_data.len() as u16,
                                                    indicate_data.as_mut_ptr(),
                                                    true,
                                                )
                                            }) {
                                                error!(
                                                    "{}: send indicate failed, error code = {}",
                                                    err,
                                                    err.code()
                                                );
                                            };
                                        }
                                    } else if descr_value == 0x0000 {
                                        error!("notify/indicate disable");
                                    } else {
                                        error!(
                                            "unknown descr value: {},  len: {}",
                                            *param.write.value, param.write.len
                                        );
                                    }
                                }
                            }
                            Self::example_write_event_env(
                                gatts_if,
                                status.a_prepare_write_env.as_mut_ptr(),
                                param_ptr,
                            );
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_EXEC_WRITE_EVT => {
                            info!("ESP_GATTS_EXEC_WRITE_EVT");
                            if let Some(err) = EspError::from(unsafe {
                                esp_idf_sys::esp_ble_gatts_send_response(
                                    gatts_if,
                                    param.write.conn_id,
                                    param.write.trans_id,
                                    esp_idf_sys::esp_gatt_status_t_ESP_GATT_OK,
                                    std::ptr::null::<esp_idf_sys::esp_gatt_rsp_t>()
                                    as *mut esp_idf_sys::esp_gatt_rsp_t,
                                )
                            }) {
                                error!("{}: send indicate failed, error code = {}", err, err.code());
                            };

                            Self::example_exec_write_event_env(
                                status.a_prepare_write_env.as_mut_ptr(),
                                param_ptr,
                            );
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_MTU_EVT => {
                            info!("ESP_GATTS_MTU_EVT, MTU {}", param.mtu.mtu)
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_UNREG_EVT => {}

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CREATE_EVT => {
                            info!(
                                "CREATE_SERVICE_EVT, status {},  service_handle {}",
                                param.create.status, param.create.service_handle
                            );
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize].service_handle =
                                param.create.service_handle;
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize]
                                .char_uuid
                                .len = ESP_UUID_LEN_16;
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize]
                                .char_uuid
                                .uuid
                                .uuid16 = GATTS_CHAR_UUID_TEST_A;

                            esp_idf_sys::esp_ble_gatts_start_service(
                                status.gl_profile_tab[PROFILE_A_APP_ID as usize].service_handle,
                            );
                            status.a_property = ESP_GATT_CHAR_PROP_BIT_READ
                                | ESP_GATT_CHAR_PROP_BIT_WRITE
                                | ESP_GATT_CHAR_PROP_BIT_NOTIFY;
                            if let Some(add_char_ret) = EspError::from(unsafe {
                                esp_idf_sys::esp_ble_gatts_add_char(
                                    status.gl_profile_tab[PROFILE_A_APP_ID as usize].service_handle,
                                    Box::into_raw(Box::new(
                                            status.gl_profile_tab[PROFILE_A_APP_ID as usize].char_uuid,
                                    )),
                                    ESP_GATT_PERM_READ | ESP_GATT_PERM_WRITE,
                                    status.a_property,
                                    Box::into_raw(Box::new(status.gatts_demo_char1_val)),
                                    std::ptr::null::<esp_idf_sys::esp_attr_control_t>()
                                    as *mut esp_idf_sys::esp_attr_control_t,
                                )
                            }) {
                                error!("add char failed, error code ={}", add_char_ret);
                            }
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_ADD_INCL_SRVC_EVT => {}

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_ADD_CHAR_EVT => {
                            let length = 0 as u16;
                            let prf_char = *std::ptr::null::<*mut *const u8>();

                            info!(
                                "ADD_CHAR_EVT, status {},  attr_handle {}, service_handle {}",
                                param.add_char.status,
                                param.add_char.attr_handle,
                                param.add_char.service_handle
                            );
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize].char_handle =
                                param.add_char.attr_handle;
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize]
                                .descr_uuid
                                .len = ESP_UUID_LEN_16;
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize]
                                .descr_uuid
                                .uuid
                                .uuid16 = ESP_GATT_UUID_CHAR_CLIENT_CONFIG;
                            if let Some(get_attr_ret) = EspError::from(unsafe {
                                esp_idf_sys::esp_ble_gatts_get_attr_value(
                                    param.add_char.attr_handle,
                                    Box::into_raw(Box::new(length)),
                                    prf_char,
                                ) as i32
                            }) {
                                if get_attr_ret.code() == ESP_FAIL {
                                    error!("ILLEGAL HANDLE");
                                }
                                error!("add char failed, error code = {}", get_attr_ret.code());
                            }

                            info!("the gatts demo char length = {}", length);
                            for i in 0..length {
                                info!("prf_char[{}] = {}", i, **prf_char.add(i as usize) as char);
                            }

                            if let Some(add_descr_ret) = EspError::from(unsafe {
                                esp_idf_sys::esp_ble_gatts_add_char_descr(
                                    status.gl_profile_tab[PROFILE_A_APP_ID as usize].service_handle,
                                    Box::into_raw(Box::new(
                                            status.gl_profile_tab[PROFILE_A_APP_ID as usize].descr_uuid,
                                    )),
                                    ESP_GATT_PERM_READ | ESP_GATT_PERM_WRITE,
                                    std::ptr::null_mut::<esp_idf_sys::esp_attr_value_t>(),
                                    std::ptr::null_mut::<esp_idf_sys::esp_attr_control_t>(),
                                )
                            }) {
                                error!(
                                    "add char descr failed, error code = {}",
                                    add_descr_ret.code()
                                );
                            }
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_ADD_CHAR_DESCR_EVT => {
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize].descr_handle =
                                param.add_char_descr.attr_handle;
                            info!(
                                "ADD_DESCR_EVT, status {}, attr_handle {}, service_handle {}",
                                param.add_char_descr.status,
                                param.add_char_descr.attr_handle,
                                param.add_char_descr.service_handle
                            );
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_DELETE_EVT => {}

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_START_EVT => {
                            info!(
                                "SERVICE_START_EVT, status {}, service_handle {}",
                                param.start.status, param.start.service_handle
                            );
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_STOP_EVT => {}

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CONNECT_EVT => {
                            let mut conn_params = esp_idf_sys::esp_ble_conn_update_params_t {
                                bda: [0; 6],
                                latency: 0,
                                max_int: 0,
                                min_int: 0,
                                timeout: 0,
                            };
                            std::ptr::copy_nonoverlapping(
                                Box::into_raw(Box::new(conn_params.bda)),
                                Box::into_raw(Box::new(param.connect.remote_bda)),
                                std::mem::size_of::<esp_idf_sys::esp_bd_addr_t>(),
                            );
                            /* For the IOS system, please reference the apple official documents about the ble connection parameters restrictions. */
                            conn_params.latency = 0;
                            conn_params.max_int = 0x20; // max_int = 0x20*1.25ms = 40ms
                            conn_params.min_int = 0x10; // min_int = 0x10*1.25ms = 20ms
                            conn_params.timeout = 400; // timeout = 400*10ms = 4000ms
                            info!("ESP_GATTS_CONNECT_EVT, conn_id {}, remote {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                                param.connect.conn_id,
                                param.connect.remote_bda[0], param.connect.remote_bda[1], param.connect.remote_bda[2],
                                param.connect.remote_bda[3], param.connect.remote_bda[4], param.connect.remote_bda[5]);
                            status.gl_profile_tab[PROFILE_A_APP_ID as usize].conn_id = param.connect.conn_id;
                            //start sent the update connection parameters to the peer device.
                            unsafe {
                                esp_idf_sys::esp_ble_gap_update_conn_params(Box::into_raw(Box::new(
                                            conn_params,
                                )))
                            };
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_DISCONNECT_EVT => {
                            info!(
                                "ESP_GATTS_DISCONNECT_EVT, disconnect reason 0x{:x}",
                                param.disconnect.reason
                            );
                            unsafe {
                                esp_idf_sys::esp_ble_gap_start_advertising(Box::into_raw(Box::new(
                                            status.adv_params,
                                )))
                            };
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CONF_EVT => {
                            info!(
                                "ESP_GATTS_CONF_EVT, status {} attr_handle {}",
                                param.conf.status, param.conf.handle
                            );
                            if param.conf.status != esp_idf_sys::esp_gatt_status_t_ESP_GATT_OK {
                                error!("value: {},  len: {}", *param.conf.value, param.conf.len);
                            }
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_OPEN_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CANCEL_OPEN_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CLOSE_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_LISTEN_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CONGEST_EVT => {}
                        _ => {}
                    }

                } else {
                    panic!("status is not available")
                }
            })

        );
        Ok(())
    }

    unsafe fn register_gatts_profile_b_event_handler(
    ) -> anyhow::Result<()> {
        *GATTS_PROFILE_B_EVENT_HANDLER.lock() = Some(
            Box::new(|event, gatts_if, param_ptr| {
                let param = *param_ptr;
                if let Some(ref mut status) = *STATUS.lock() {
                    match event {
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_REG_EVT => {
                            info!(
                                "REGISTER_APP_EVT, status {}, app_id {}",
                                param.reg.status, param.reg.app_id
                            );
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize]
                                .service_id
                                .is_primary = true;
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize]
                                .service_id
                                .id
                                .inst_id = 0x00;
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize]
                                .service_id
                                .id
                                .uuid
                                .len = ESP_UUID_LEN_16;
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize]
                                .service_id
                                .id
                                .uuid
                                .uuid
                                .uuid16 = GATTS_SERVICE_UUID_TEST_B;

                            esp_idf_sys::esp_ble_gatts_create_service(
                                gatts_if,
                                Box::into_raw(Box::new(
                                        status.gl_profile_tab[PROFILE_B_APP_ID as usize].service_id,
                                )),
                                GATTS_NUM_HANDLE_TEST_B,
                            );
                        }

                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_READ_EVT => {
                            info!(
                                "GATT_READ_EVT, conn_id {}, trans_id {}, handle {}",
                                param.read.conn_id, param.read.trans_id, param.read.handle
                            );
                            let rsp_ptr = std::ptr::null_mut::<esp_idf_sys::esp_gatt_rsp_t>();
                            std::ptr::write_bytes::<esp_idf_sys::esp_gatt_rsp_t>(
                                rsp_ptr,
                                0,
                                std::mem::size_of::<esp_idf_sys::esp_gatt_rsp_t>(),
                            );
                            let mut rsp = *rsp_ptr;
                            rsp.attr_value.handle = param.read.handle;
                            rsp.attr_value.len = 4;
                            rsp.attr_value.value[0] = 0xde;
                            rsp.attr_value.value[1] = 0xed;
                            rsp.attr_value.value[2] = 0xbe;
                            rsp.attr_value.value[3] = 0xef;
                            esp_idf_sys::esp_ble_gatts_send_response(
                                gatts_if,
                                param.read.conn_id,
                                param.read.trans_id,
                                esp_idf_sys::esp_gatt_status_t_ESP_GATT_OK,
                                rsp_ptr,
                            );
                        }
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_WRITE_EVT => {
                            info!(
                                "GATT_WRITE_EVT, conn_id {}, trans_id {}, handle {}",
                                param.write.conn_id, param.write.trans_id, param.write.handle
                            );
                            if !param.write.is_prep {
                                info!("GATT_WRITE_EVT, value len {}, value  =>", param.write.len);
                                error!("value: {},  len: {}", *param.write.value, param.write.len);
                                if status.gl_profile_tab[PROFILE_B_APP_ID as usize].descr_handle
                                    == param.write.handle
                                        && param.write.len == 2
                                {
                                    let descr_value = u16::from(*param.write.value.add(1)) << 8 | u16::from(*param.write.value);
                                    if descr_value == 0x0001 {
                                        if status.b_property != 0 & ESP_GATT_CHAR_PROP_BIT_NOTIFY {
                                            info!("notify enable");
                                            let mut notify_data = [0 as u8; 15];
                                            for i in 0..notify_data.len() {
                                                notify_data[i] = i as u8 % 0xff;
                                            }
                                            //the size of notify_data[] need less than MTU size
                                            esp_idf_sys::esp_ble_gatts_send_indicate(
                                                gatts_if,
                                                param.write.conn_id,
                                                status.gl_profile_tab[PROFILE_B_APP_ID as usize].char_handle,
                                                notify_data.len() as u16,
                                                notify_data.as_mut_ptr(),
                                                false,
                                            );
                                        }
                                    } else if descr_value == 0x0002 {
                                        if status.b_property != 0 & ESP_GATT_CHAR_PROP_BIT_INDICATE {
                                            info!("indicate enable");
                                            let mut indicate_data = [0 as u8; 15];
                                            for i in 0..indicate_data.len() {
                                                indicate_data[i] = i as u8 % 0xff;
                                            }
                                            //the size of indicate_data[] need less than MTU size
                                            esp_idf_sys::esp_ble_gatts_send_indicate(
                                                gatts_if,
                                                param.write.conn_id,
                                                status.gl_profile_tab[PROFILE_B_APP_ID as usize].char_handle,
                                                indicate_data.len() as u16,
                                                indicate_data.as_mut_ptr(),
                                                true,
                                            );
                                        }
                                    } else if descr_value == 0x0000 {
                                        info!("notify/indicate disable");
                                    } else {
                                        info!("unknown value");
                                    }
                                }
                            }
                            Self::example_write_event_env(
                                gatts_if,
                                status.b_prepare_write_env.as_mut_ptr(),
                                param_ptr,
                            );
                        }
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_EXEC_WRITE_EVT => {
                            info!("ESP_GATTS_EXEC_WRITE_EVT");
                            esp_idf_sys::esp_ble_gatts_send_response(
                                gatts_if,
                                param.write.conn_id,
                                param.write.trans_id,
                                esp_idf_sys::esp_gatt_status_t_ESP_GATT_OK,
                                std::ptr::null_mut::<esp_idf_sys::esp_gatt_rsp_t>(),
                            );
                            Self::example_exec_write_event_env(
                                status.b_prepare_write_env.as_mut_ptr(),
                                param_ptr,
                            );
                        }
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_MTU_EVT => {
                            info!("ESP_GATTS_MTU_EVT, MTU {}", param.mtu.mtu);
                        }
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_UNREG_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CREATE_EVT => {
                            info!(
                                "CREATE_SERVICE_EVT, status {},  service_handle {}",
                                param.create.status, param.create.service_handle
                            );
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize].service_handle =
                                param.create.service_handle;
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize]
                                .char_uuid
                                .len = ESP_UUID_LEN_16;
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize]
                                .char_uuid
                                .uuid
                                .uuid16 = GATTS_CHAR_UUID_TEST_B;

                            esp_idf_sys::esp_ble_gatts_start_service(
                                status.gl_profile_tab[PROFILE_B_APP_ID as usize].service_handle,
                            );
                            status.b_property = ESP_GATT_CHAR_PROP_BIT_READ
                                | ESP_GATT_CHAR_PROP_BIT_WRITE
                                | ESP_GATT_CHAR_PROP_BIT_NOTIFY;
                            if let Some(add_char_ret) = EspError::from(esp_idf_sys::esp_ble_gatts_add_char(
                                    status.gl_profile_tab[PROFILE_B_APP_ID as usize].service_handle,
                                    Box::into_raw(Box::new(
                                            status.gl_profile_tab[PROFILE_B_APP_ID as usize].char_uuid,
                                    )),
                                    ESP_GATT_PERM_READ | ESP_GATT_PERM_WRITE,
                                    status.b_property,
                                    std::ptr::null_mut::<esp_idf_sys::esp_attr_value_t>(),
                                    std::ptr::null_mut::<esp_idf_sys::esp_attr_control_t>(),
                            )) {
                                error!("add char failed, error code ={}", add_char_ret.code());
                            }
                        }
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_ADD_INCL_SRVC_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_ADD_CHAR_EVT => {
                            info!(
                                "ADD_CHAR_EVT, status {},  attr_handle {}, service_handle {}",
                                param.add_char.status,
                                param.add_char.attr_handle,
                                param.add_char.service_handle
                            );

                            status.gl_profile_tab[PROFILE_B_APP_ID as usize].char_handle =
                                param.add_char.attr_handle;
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize]
                                .descr_uuid
                                .len = ESP_UUID_LEN_16;
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize]
                                .descr_uuid
                                .uuid
                                .uuid16 = ESP_GATT_UUID_CHAR_CLIENT_CONFIG;
                            esp_idf_sys::esp_ble_gatts_add_char_descr(
                                status.gl_profile_tab[PROFILE_B_APP_ID as usize].service_handle,
                                Box::into_raw(Box::new(
                                        status.gl_profile_tab[PROFILE_B_APP_ID as usize].descr_uuid,
                                )),
                                ESP_GATT_PERM_READ | ESP_GATT_PERM_WRITE,
                                std::ptr::null_mut::<esp_idf_sys::esp_attr_value_t>(),
                                std::ptr::null_mut::<esp_idf_sys::esp_attr_control_t>(),
                            );
                        }
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_ADD_CHAR_DESCR_EVT => {
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize].descr_handle =
                                param.add_char_descr.attr_handle;
                            info!(
                                "ADD_DESCR_EVT, status {}, attr_handle {}, service_handle {}",
                                param.add_char_descr.status,
                                param.add_char_descr.attr_handle,
                                param.add_char_descr.service_handle
                            );
                        }
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_DELETE_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_START_EVT => {
                            info!(
                                "SERVICE_START_EVT, status {}, service_handle {}",
                                param.start.status, param.start.service_handle
                            );
                        }
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_STOP_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CONNECT_EVT => {
                            info!(
                                "CONNECT_EVT, conn_id {}, remote {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:",
                                param.connect.conn_id,
                                param.connect.remote_bda[0],
                                param.connect.remote_bda[1],
                                param.connect.remote_bda[2],
                                param.connect.remote_bda[3],
                                param.connect.remote_bda[4],
                                param.connect.remote_bda[5]
                            );
                            status.gl_profile_tab[PROFILE_B_APP_ID as usize].conn_id = param.connect.conn_id;
                        }
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CONF_EVT => {
                            info!(
                                "ESP_GATTS_CONF_EVT status {} attr_handle {}",
                                param.conf.status, param.conf.handle
                            );
                            if param.conf.status != esp_idf_sys::esp_gatt_status_t_ESP_GATT_OK {
                                info!("value: {}, len: {}", *param.conf.value, param.conf.len);
                            }
                        }
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_DISCONNECT_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_OPEN_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CANCEL_OPEN_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CLOSE_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_LISTEN_EVT => {}
                        esp_idf_sys::esp_gatts_cb_event_t_ESP_GATTS_CONGEST_EVT => {}
                        _ => {}
                    }
                } else {
                    panic!("status is not available")
                }
            })
        );
        Ok(())
    }

    unsafe fn example_write_event_env (
        gatts_if: esp_idf_sys::esp_gatt_if_t,
        prepare_write_env_ptr: *mut PrepareTypeEnv,
        param_ptr: *mut esp_idf_sys::esp_ble_gatts_cb_param_t,
    ) {
        let param = *param_ptr;
        let mut prepare_write_env = *prepare_write_env_ptr;
        let mut status = esp_idf_sys::esp_gatt_status_t_ESP_GATT_OK;
        let layout = std::alloc::Layout::new::<u8>();
        if param.write.need_rsp {
            if param.write.is_prep {
                if prepare_write_env.prepare_buf == std::ptr::null::<u8>() as *mut u8 {
                    prepare_write_env.prepare_buf = std::alloc::alloc(layout);
                    *prepare_write_env.prepare_buf =
                        PREPARE_BUF_MAX_SIZE as u8 * std::mem::size_of::<u8>() as u8;
                    std::alloc::dealloc(prepare_write_env.prepare_buf, layout);

                    prepare_write_env.prepare_len = 0;
                    if prepare_write_env.prepare_buf == std::ptr::null::<u8>() as *mut u8 {
                        error!("Gatt_server prep no mem");
                        status = esp_idf_sys::esp_gatt_status_t_ESP_GATT_NO_RESOURCES;
                    }
                } else {
                    if param.write.offset > PREPARE_BUF_MAX_SIZE as u16 {
                        status = esp_idf_sys::esp_gatt_status_t_ESP_GATT_INVALID_OFFSET;
                    } else if (param.write.offset + param.write.len) > PREPARE_BUF_MAX_SIZE as u16 {
                        status = esp_idf_sys::esp_gatt_status_t_ESP_GATT_INVALID_ATTR_LEN;
                    }
                }

                let gatt_rsp = std::alloc::alloc(layout);
                *gatt_rsp = std::mem::size_of::<esp_idf_sys::esp_gatt_rsp_t>() as u8;
                std::alloc::dealloc(gatt_rsp, layout);
                let mut gatt_rsp = *gatt_rsp.cast::<esp_idf_sys::esp_gatt_rsp_t>();
                gatt_rsp.attr_value.len = param.write.len;
                gatt_rsp.attr_value.handle = param.write.handle;
                gatt_rsp.attr_value.offset = param.write.offset;
                gatt_rsp.attr_value.auth_req =
                    esp_idf_sys::esp_gatt_auth_req_t_ESP_GATT_AUTH_REQ_NONE as u8;
                std::ptr::copy_nonoverlapping(
                    Box::into_raw(Box::new(gatt_rsp.attr_value.value)),
                    param.write.value.cast::<[u8; 600]>(),
                    param.write.len as usize,
                );
                if let Some(err) = EspError::from(esp_idf_sys::esp_ble_gatts_send_response(
                        gatts_if,
                        param.write.conn_id,
                        param.write.trans_id,
                        status,
                        Box::into_raw(Box::new(gatt_rsp)),
                )) {
                    error!("Send response. error code = {}", err.code());
                        }
                drop(gatt_rsp);
                if status != esp_idf_sys::esp_gatt_status_t_ESP_GATT_OK {
                    return;
                }
                std::ptr::copy_nonoverlapping(
                    prepare_write_env
                    .prepare_buf
                    .add(param.write.offset as usize),
                    param.write.value.cast::<u8>(),
                    param.write.len as usize,
                );
                prepare_write_env.prepare_len += param.write.len;
            } else {
                esp_idf_sys::esp_ble_gatts_send_response(
                    gatts_if,
                    param.write.conn_id,
                    param.write.trans_id,
                    status,
                    std::ptr::null::<esp_idf_sys::esp_gatt_rsp_t>() as *mut esp_idf_sys::esp_gatt_rsp_t,
                );
            }
        }
    }

    unsafe fn example_exec_write_event_env(
        prepare_write_env_ptr: *mut PrepareTypeEnv,
        param_ptr: *mut esp_idf_sys::esp_ble_gatts_cb_param_t,
    ) {
        let param = *param_ptr;
        let mut prepare_write_env = *prepare_write_env_ptr;
        if param.exec_write.exec_write_flag
            == esp_idf_sys::esp_gatt_prep_write_type_ESP_GATT_PREP_WRITE_EXEC as u8
        {
            info!(
                "prepare_buf: {}, prepare_len: {}",
                *prepare_write_env.prepare_buf, prepare_write_env.prepare_len
            );
        } else {
            error!("ESP_GATT_PREP_WRITE_CANCEL");
        }
        if let Some(_ptr) = std::ptr::NonNull::<u8>::new(prepare_write_env.prepare_buf) {
            drop(prepare_write_env.prepare_buf);
            prepare_write_env.prepare_buf = std::ptr::null_mut::<u8>();
        }
        prepare_write_env.prepare_len = 0;
    }

    pub extern "C" fn gatts_event_handler(
        event: esp_idf_sys::esp_gatts_cb_event_t,
        gatts_if: esp_idf_sys::esp_gatt_if_t,
        param_ptr: *mut esp_idf_sys::esp_ble_gatts_cb_param_t,
    ) {
        if let Some(ref mut callback) = *GATTS_EVENT_HANDLER.lock() {
            callback(event, gatts_if, param_ptr)
        } else {
            panic!("callback: gatts_event_handler is not available");
        }
    }

    extern "C" fn gatts_profile_a_event_handler(
        event: esp_idf_sys::esp_gatts_cb_event_t,
        gatts_if: esp_idf_sys::esp_gatt_if_t,
        param_ptr: *mut esp_idf_sys::esp_ble_gatts_cb_param_t,
    ) {
        if let Some(ref mut callback) = *GATTS_PROFILE_A_EVENT_HANDLER.lock() {
            callback(event, gatts_if, param_ptr)
        } else {
            panic!("callback: gatts_profile_a_event_handler is not available");
        }
    }

    extern "C" fn gatts_profile_b_event_handler(
        event: esp_idf_sys::esp_gatts_cb_event_t,
        gatts_if: esp_idf_sys::esp_gatt_if_t,
        param_ptr: *mut esp_idf_sys::esp_ble_gatts_cb_param_t,
    ) {
        if let Some(ref mut callback) = *GATTS_PROFILE_B_EVENT_HANDLER.lock() {
            callback(event, gatts_if, param_ptr)
        } else {
            panic!("callback: gatts_profile_b_event_handler is not available");
        }
    }

    pub extern "C" fn gap_event_handler(
        event: esp_idf_sys::esp_gap_ble_cb_event_t,
        param_ptr: *mut esp_idf_sys::esp_ble_gap_cb_param_t,
    ) {
        if let Some(ref mut callback) = *GAP_EVENT_HANDLER.lock() {
            callback(event, param_ptr)
        } else {
            panic!("callback: gap_event_handler is not available");
        }

    }

    unsafe fn register_gap_event_handler() -> anyhow::Result<()> {
        *GAP_EVENT_HANDLER.lock() = Some(
            Box::new(|event, param_ptr| {
                let param = *param_ptr;
                if let Some(ref mut status) = *STATUS.lock() {
                    match event {
                        esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_RAW_SET_COMPLETE_EVT => {
                            if CONFIG_SET_RAW_ADV_DATA {
                                status.adv_config_done &= !ADV_CONFIG_FLAG;
                                if status.adv_config_done==0 {
                                    esp_idf_sys::esp_ble_gap_start_advertising(
                                        Box::into_raw(Box::new(status.adv_params))
                                    );
                                }
                            }
                        }
                        esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RSP_DATA_RAW_SET_COMPLETE_EVT => {
                            if CONFIG_SET_RAW_ADV_DATA {
                                status.adv_config_done &= !SCAN_RSP_CONFIG_FLAG;
                                if status.adv_config_done==0 {
                                    esp_idf_sys:: esp_ble_gap_start_advertising(
                                        Box::into_raw(Box::new(status.adv_params))
                                    );
                                }
                            }
                        }
                        esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_SET_COMPLETE_EVT => {
                            if !CONFIG_SET_RAW_ADV_DATA {
                                status.adv_config_done &= !ADV_CONFIG_FLAG;
                                if status.adv_config_done == 0 {
                                    esp_idf_sys:: esp_ble_gap_start_advertising(
                                        Box::into_raw(Box::new(status.adv_params))
                                    );
                                }
                            }
                        }
                        esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RSP_DATA_SET_COMPLETE_EVT => {
                            if !CONFIG_SET_RAW_ADV_DATA {
                                status.adv_config_done &= !SCAN_RSP_CONFIG_FLAG;
                                if status.adv_config_done == 0 {
                                    esp_idf_sys:: esp_ble_gap_start_advertising(
                                        Box::into_raw(Box::new(status.adv_params))
                                    );
                                }
                            }
                        }
                        esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_START_COMPLETE_EVT => {
                            //advertising start complete event to indicate advertising start successfully or failed
                            if param.adv_start_cmpl.status != esp_idf_sys::esp_bt_status_t_ESP_BT_STATUS_SUCCESS {
                                error!("advertising start failed");
                            }
                        }
                        esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_STOP_COMPLETE_EVT => {
                            if param.adv_stop_cmpl.status != esp_idf_sys::esp_bt_status_t_ESP_BT_STATUS_SUCCESS {
                                error!("advertising stop failed");
                            } else {
                                info!("stop adv successfully");
                            }
                        }
                        esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_UPDATE_CONN_PARAMS_EVT => {
                            info!("update connection params status = {}, min_int = {}, max_int = {}, conn_int = {},latency = {}, timeout = {}",
                                param.update_conn_params.status,
                                param.update_conn_params.min_int,
                                param.update_conn_params.max_int,
                                param.update_conn_params.conn_int,
                                param.update_conn_params.latency,
                                param.update_conn_params.timeout);
                        }
                        _ => {}
                    }
                }
            })
        );
        Ok(())
    }
}

unsafe impl Send for Status {}

pub fn get_bt_cfg() -> esp_idf_sys::esp_bt_controller_config_t {
    esp_idf_sys::esp_bt_controller_config_t {
        controller_task_stack_size: esp_idf_sys::ESP_TASK_BT_CONTROLLER_STACK as u16,
        controller_task_prio: esp_idf_sys::ESP_TASK_BT_CONTROLLER_PRIO as u8,
        hci_uart_no: esp_idf_sys::BT_HCI_UART_NO_DEFAULT as u8,
        hci_uart_baudrate: esp_idf_sys::BT_HCI_UART_BAUDRATE_DEFAULT,
        scan_duplicate_mode: esp_idf_sys::SCAN_DUPLICATE_MODE as u8,
        scan_duplicate_type: esp_idf_sys::SCAN_DUPLICATE_TYPE_VALUE as u8,
        normal_adv_size: esp_idf_sys::NORMAL_SCAN_DUPLICATE_CACHE_SIZE as u16,
        mesh_adv_size: esp_idf_sys::MESH_DUPLICATE_SCAN_CACHE_SIZE as u16,
        send_adv_reserved_size: esp_idf_sys::SCAN_SEND_ADV_RESERVED_SIZE as u16,
        controller_debug_flag: esp_idf_sys::CONTROLLER_ADV_LOST_DEBUG_BIT,
        mode: esp_idf_sys::esp_bt_mode_t_ESP_BT_MODE_BLE as u8,
        ble_max_conn: esp_idf_sys::CONFIG_BTDM_CTRL_BLE_MAX_CONN_EFF as u8,
        bt_max_acl_conn: esp_idf_sys::CONFIG_BTDM_CTRL_BR_EDR_MAX_ACL_CONN_EFF as u8,
        bt_sco_datapath: esp_idf_sys::CONFIG_BTDM_CTRL_BR_EDR_SCO_DATA_PATH_EFF as u8,
        auto_latency: esp_idf_sys::BTDM_CTRL_AUTO_LATENCY_EFF != 0,
        bt_legacy_auth_vs_evt: esp_idf_sys::BTDM_CTRL_LEGACY_AUTH_VENDOR_EVT_EFF != 0,
        bt_max_sync_conn: esp_idf_sys::CONFIG_BTDM_CTRL_BR_EDR_MAX_SYNC_CONN_EFF as u8,
        ble_sca: esp_idf_sys::CONFIG_BTDM_BLE_SLEEP_CLOCK_ACCURACY_INDEX_EFF as u8,
        pcm_role: esp_idf_sys::CONFIG_BTDM_CTRL_PCM_ROLE_EFF as u8,
        pcm_polar: esp_idf_sys::CONFIG_BTDM_CTRL_PCM_POLAR_EFF as u8,
        hli: esp_idf_sys::BTDM_CTRL_HLI != 0,
        magic: esp_idf_sys::ESP_BT_CONTROLLER_CONFIG_MAGIC_VAL,
    }
}
