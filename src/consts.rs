pub const TEST_DEVICE_NAME: &str = "ESP_GATTS_DEMO";

pub const PROFILE_NUM: u32 = 2;
pub const ESP_GATT_IF_NONE: u32 = 0xff;

pub const PROFILE_A_APP_ID: u16 = 0;
pub const PROFILE_B_APP_ID: u16 = 1;

pub const ESP_UUID_LEN_16: u16 = 2;
pub const ESP_UUID_LEN_32: u16 = 4;
pub const ESP_UUID_LEN_128: u16 = 16;

pub const GATTS_SERVICE_UUID_TEST_A: u16 = 0x00FF;
pub const GATTS_CHAR_UUID_TEST_A: u16 = 0xFF01;
pub const GATTS_DESCR_UUID_TEST_A: u16 = 0x3333;
pub const GATTS_NUM_HANDLE_TEST_A: u16 = 4;

pub const GATTS_SERVICE_UUID_TEST_B: u16 = 0x00EE;
pub const GATTS_CHAR_UUID_TEST_B: u16 = 0xEE01;
pub const GATTS_DESCR_UUID_TEST_B: u16 = 0x2222;
pub const GATTS_NUM_HANDLE_TEST_B: u16 = 4;

pub const TEST_MANUFACTURER_DATA_LEN: u16 = 17;

pub const ADV_CONFIG_FLAG: u8 = 1 << 0;
pub const SCAN_RSP_CONFIG_FLAG: u8 = 1 << 1;

pub const ESP_GATT_CHAR_PROP_BIT_BROADCAST: u8 = 1 << 0; /* 0x01 */
/* relate to BTA_GATT_CHAR_PROP_BIT_BROADCAST in bta/bta_gatt_api.h */
pub const ESP_GATT_CHAR_PROP_BIT_READ: u8 = 1 << 1; /* 0x02 */
/* relate to BTA_GATT_CHAR_PROP_BIT_READ in bta/bta_gatt_api.h */
pub const ESP_GATT_CHAR_PROP_BIT_WRITE_NR: u8 = 1 << 2; /* 0x04 */
/* relate to BTA_GATT_CHAR_PROP_BIT_WRITE_NR in bta/bta_gatt_api.h */
pub const ESP_GATT_CHAR_PROP_BIT_WRITE: u8 = 1 << 3; /* 0x08 */
/* relate to BTA_GATT_CHAR_PROP_BIT_WRITE in bta/bta_gatt_api.h */
pub const ESP_GATT_CHAR_PROP_BIT_NOTIFY: u8 = 1 << 4; /* 0x10 */
/* relate to BTA_GATT_CHAR_PROP_BIT_NOTIFY in bta/bta_gatt_api.h */
pub const ESP_GATT_CHAR_PROP_BIT_INDICATE: u8 = 1 << 5; /* 0x20 */
/* relate to BTA_GATT_CHAR_PROP_BIT_INDICATE in bta/bta_gatt_api.h */
pub const ESP_GATT_CHAR_PROP_BIT_AUTH: u8 = 1 << 6; /* 0x40 */
/* relate to BTA_GATT_CHAR_PROP_BIT_AUTH in bta/bta_gatt_api.h */
pub const ESP_GATT_CHAR_PROP_BIT_EXT_PROP: u8 = 1 << 7; /* 0x80 */
/* relate to BTA_GATT_CHAR_PROP_BIT_EXT_PROP in bta/bta_gatt_api.h */

pub const CHAR1_STR: [u8; 3] = [0x11, 0x22, 0x33];

pub const ESP_GATT_PERM_READ: u16 = 1 << 0; /* bit 0 -  0x0001 */
/* relate to BTA_GATT_PERM_READ in bta/bta_gatt_api.h */
pub const ESP_GATT_PERM_READ_ENCRYPTED: u16 = 1 << 1; /* bit 1 -  0x0002 */
/* relate to BTA_GATT_PERM_READ_ENCRYPTED in bta/bta_gatt_api.h */
pub const ESP_GATT_PERM_READ_ENC_MITM: u16 = 1 << 2; /* bit 2 -  0x0004 */
/* relate to BTA_GATT_PERM_READ_ENC_MITM in bta/bta_gatt_api.h */
pub const ESP_GATT_PERM_WRITE: u16 = 1 << 4; /* bit 4 -  0x0010 */
/* relate to BTA_GATT_PERM_WRITE in bta/bta_gatt_api.h */
pub const ESP_GATT_PERM_WRITE_ENCRYPTED: u16 = 1 << 5; /* bit 5 -  0x0020 */
/* relate to BTA_GATT_PERM_WRITE_ENCRYPTED in bta/bta_gatt_api.h */
pub const ESP_GATT_PERM_WRITE_ENC_MITM: u16 = 1 << 6; /* bit 6 -  0x0040 */
/* relate to BTA_GATT_PERM_WRITE_ENC_MITM in bta/bta_gatt_api.h */
pub const ESP_GATT_PERM_WRITE_SIGNED: u16 = 1 << 7; /* bit 7 -  0x0080 */
/* relate to BTA_GATT_PERM_WRITE_SIGNED in bta/bta_gatt_api.h */
pub const ESP_GATT_PERM_WRITE_SIGNED_MITM: u16 = 1 << 8; /* bit 8 -  0x0100 */
/* relate to BTA_GATT_PERM_WRITE_SIGNED_MITM in bta/bta_gatt_api.h */
pub const ESP_GATT_PERM_READ_AUTHORIZATION: u16 = 1 << 9; /* bit 9 -  0x0200 */
pub const ESP_GATT_PERM_WRITE_AUTHORIZATION: u16 = 1 << 10; /* bit 10 - 0x0400 */

pub const GATTS_DEMO_CHAR_VAL_LEN_MAX: u16 = 0x40;

#[cfg(feature = "config_set_raw_adv_data")]
pub const CONFIG_SET_RAW_ADV_DATA: bool = true;
#[cfg(not(feature = "config_set_raw_adv_data"))]
pub const CONFIG_SET_RAW_ADV_DATA: bool = false;

pub const RAW_ADV_DATA: [u8; 10] = [0x02, 0x01, 0x06, 0x02, 0x0a, 0xeb, 0x03, 0x03, 0xab, 0xcd];

pub const RAW_SCAN_RSP_DATA: [u8; 16] = [
    0x0f, 0x09, 0x45, 0x53, 0x50, 0x5f, 0x47, 0x41, 0x54, 0x54, 0x53, 0x5f, 0x44, 0x45, 0x4d, 0x4f,
];

#[cfg(not(feature = "config_set_raw_adv_data"))]
pub const ADV_SERVICE_UUID128: [u8; 32] = [
    /* LSB <--------------------------------------------------------------------------------> MSB */
    //first uuid, 16bit, [12],[13] is the value
    0xfb, 0x34, 0x9b, 0x5f, 0x80, 0x00, 0x00, 0x80, 0x00, 0x10, 0x00, 0x00, 0xEE, 0x00, 0x00, 0x00,
    //second uuid, 32bit, [12], [13], [14], [15] is the value
    0xfb, 0x34, 0x9b, 0x5f, 0x80, 0x00, 0x00, 0x80, 0x00, 0x10, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00,
];

pub const ESP_BLE_ADV_FLAG_LIMIT_DISC: u8 = 0x01 << 0;
pub const ESP_BLE_ADV_FLAG_GEN_DISC: u8 = 0x01 << 1;
pub const ESP_BLE_ADV_FLAG_BREDR_NOT_SPT: u8 = 0x01 << 2;
pub const ESP_BLE_ADV_FLAG_DMT_CONTROLLER_SPT: u8 = 0x01 << 3;
pub const ESP_BLE_ADV_FLAG_DMT_HOST_SPT: u8 = 0x01 << 4;
pub const ESP_BLE_ADV_FLAG_NON_LIMIT_DISC: u8 = 0x00;

pub const ESP_GATT_UUID_CHAR_EXT_PROP: u16 = 0x2900; /*  Characteristic Extended Properties */
pub const ESP_GATT_UUID_CHAR_DESCRIPTION: u16 = 0x2901; /*  Characteristic User Description*/
pub const ESP_GATT_UUID_CHAR_CLIENT_CONFIG: u16 = 0x2902; /*  Client Characteristic Configuration */
pub const ESP_GATT_UUID_CHAR_SRVR_CONFIG: u16 = 0x2903; /*  Server Characteristic Configuration */
pub const ESP_GATT_UUID_CHAR_PRESENT_FORMAT: u16 = 0x2904; /*  Characteristic Presentation Format*/
pub const ESP_GATT_UUID_CHAR_AGG_FORMAT: u16 = 0x2905; /*  Characteristic Aggregate Format*/
pub const ESP_GATT_UUID_CHAR_VALID_RANGE: u16 = 0x2906; /*  Characteristic Valid Range */
pub const ESP_GATT_UUID_EXT_RPT_REF_DESCR: u16 = 0x2907; /*  External Report Reference */
pub const ESP_GATT_UUID_RPT_REF_DESCR: u16 = 0x2908; /*  Report Reference */
pub const ESP_GATT_UUID_NUM_DIGITALS_DESCR: u16 = 0x2909; /*  Number of Digitals */
pub const ESP_GATT_UUID_VALUE_TRIGGER_DESCR: u16 = 0x290A; /*  Value Trigger Setting */
pub const ESP_GATT_UUID_ENV_SENSING_CONFIG_DESCR: u16 = 0x290B; /*  Environmental Sensing Configuration */
pub const ESP_GATT_UUID_ENV_SENSING_MEASUREMENT_DESCR: u16 = 0x290C; /*  Environmental Sensing Measurement */
pub const ESP_GATT_UUID_ENV_SENSING_TRIGGER_DESCR: u16 = 0x290D; /*  Environmental Sensing Trigger Setting */
pub const ESP_GATT_UUID_TIME_TRIGGER_DESCR: u16 = 0x290E; /*  Time Trigger Setting */

pub const ESP_OK: i32 = 0; /* !< esp_err_t value indicating success (no error) */
pub const ESP_FAIL: i32 = -1; /* !< Generic esp_err_t code indicating failure */

pub const ESP_ERR_NO_MEM: i32 = 0x101; /* !< Out of memory */
pub const ESP_ERR_INVALID_ARG: i32 = 0x102; /* !< Invalid argument */
pub const ESP_ERR_INVALID_STATE: i32 = 0x103; /* !< Invalid state */
pub const ESP_ERR_INVALID_SIZE: i32 = 0x104; /* !< Invalid size */
pub const ESP_ERR_NOT_FOUND: i32 = 0x105; /* !< Requested resource not found */
pub const ESP_ERR_NOT_SUPPORTED: i32 = 0x106; /* !< Operation or feature not supported */
pub const ESP_ERR_TIMEOUT: i32 = 0x107; /* !< Operation timed out */
pub const ESP_ERR_INVALID_RESPONSE: i32 = 0x108; /* !< Received response was invalid */
pub const ESP_ERR_INVALID_CRC: i32 = 0x109; /* !< CRC or checksum was invalid */
pub const ESP_ERR_INVALID_VERSION: i32 = 0x10A; /* !< Version was invalid */
pub const ESP_ERR_INVALID_MAC: i32 = 0x10B; /* !< MAC address was invalid */
pub const ESP_ERR_NOT_FINISHED: i32 = 0x10C; /* !< There are items remained to retrieve */

pub const ESP_ERR_WIFI_BASE: i32 = 0x3000; /* !< Starting number of WiFi error codes */
pub const ESP_ERR_MESH_BASE: i32 = 0x4000; /* !< Starting number of MESH error codes */
pub const ESP_ERR_FLASH_BASE: i32 = 0x6000; /* !< Starting number of flash error codes */
pub const ESP_ERR_HW_CRYPTO_BASE: i32 = 0xc000; /* !< Starting number of HW cryptography module error codes */
pub const ESP_ERR_MEMPROT_BASE: i32 = 0xd000; /* !< Starting number of Memory Protection API error codes */

pub const PREPARE_BUF_MAX_SIZE: u16 = 1024;
