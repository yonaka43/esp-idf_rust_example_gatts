#[derive(Default, Clone, Copy)]
pub struct GattsProfileInst {
    pub gatts_cb: esp_idf_sys::esp_gatts_cb_t,
    pub gatts_if: u16,
    pub app_id: u16,
    pub conn_id: u16,
    pub service_handle: u16,
    pub service_id: esp_idf_sys::esp_gatt_srvc_id_t,
    pub char_handle: u16,
    pub char_uuid: esp_idf_sys::esp_bt_uuid_t,
    pub perm: esp_idf_sys::esp_gatt_perm_t,
    pub property: esp_idf_sys::esp_gatt_char_prop_t,
    pub descr_handle: u16,
    pub descr_uuid: esp_idf_sys::esp_bt_uuid_t,
}
