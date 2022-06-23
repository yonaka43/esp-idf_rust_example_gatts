use log::{error, info};
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_sys::EspError;

const PROFILE_A_APP_ID: u16 = 0;
const PROFILE_B_APP_ID: u16 = 1;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::log::EspLogger::initialize_default();

    // Initialize NVS.
    let _nvs = EspDefaultNvs::new()?;

    if let Some(err) = EspError::from(unsafe {
        esp_idf_sys::esp_bt_controller_mem_release(
            esp_idf_sys::esp_bt_mode_t_ESP_BT_MODE_CLASSIC_BT,
        )
    }) {
        error!("{}", err);
    };

    let mut bt_cfg = app::get_bt_cfg();

    if let Some(err) = EspError::from(unsafe { esp_idf_sys::esp_bt_controller_init(&mut bt_cfg) }) {
        error!("{}: initialize controller failed", err);
    };

    if let Some(err) = EspError::from(unsafe {
        esp_idf_sys::esp_bt_controller_enable(esp_idf_sys::esp_bt_mode_t_ESP_BT_MODE_BLE)
    }) {
        error!("{}: enable controller failed", err);
    };

    if let Some(err) = EspError::from(unsafe { esp_idf_sys::esp_bluedroid_init() }) {
        error!("{}: init bluetooth failed", err);
    };

    if let Some(err) = EspError::from(unsafe { esp_idf_sys::esp_bluedroid_enable() }) {
        error!("{}: enable bluetooth failed", err);
    };

    app::Status::init()?;

    if let Some(err) = EspError::from(unsafe {
        esp_idf_sys::esp_ble_gatts_register_callback(Some(app::Status::gatts_event_handler))
    }) {
        error!("{}: gatts register error, error code = {}", err, err.code());
    };
    
    if let Some(err) = EspError::from(unsafe {
        esp_idf_sys::esp_ble_gap_register_callback(Some(app::Status::gap_event_handler))
    }) {
        error!("{}: gap register error, error code = {}", err, err.code());
    }

    if let Some(err) =
        EspError::from(unsafe { esp_idf_sys::esp_ble_gatts_app_register(PROFILE_A_APP_ID) })
    {
        error!(
            "{}: gatts app register error, error code = {}",
            err,
            err.code()
        );
    };
    
    if let Some(err) =
        EspError::from(unsafe { esp_idf_sys::esp_ble_gatts_app_register(PROFILE_B_APP_ID) })
    {
        error!(
            "{}: gatts app register error, error code = {}",
            err,
            err.code()
        );
    };

    if let Some(err) = EspError::from(unsafe { esp_idf_sys::esp_ble_gatt_set_local_mtu(512) }) {
        error!(
            "{}: set local  MTU failed, error code = {}",
            err,
            err.code()
        );
    };

    Ok(())
}
