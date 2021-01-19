//! Retrieves device information similar to that returned by [roDeviceInfo](https://developer.roku.com/en-gb/docs/references/brightscript/components/rodeviceinfo.md#!). This command is accessed using an HTTP GET.
use crate::Device;
use crate::Error;
use serde::Deserialize;
use serde_xml_rs::from_str;

impl Device {
    /// Retrieves device information similar to that returned by [roDeviceInfo](https://developer.roku.com/en-gb/docs/references/brightscript/components/rodeviceinfo.md#!) for BrightScript.
    pub async fn device_info(&self) -> Result<DeviceInfo, Error> {
        let response = self
            .http
            .get(format!("{}/query/device-info", self.url))
            .recv_string()
            .await?;

        Ok(from_str(&response)?)
    }
}

/// Roku device information. Schema is somewhat based on [github.com/automated-channel-testing](https://github.com/rokudev/automated-channel-testing/blob/3274e9d9a8721a536b6dfae0d56646f5a2871325/src/ecpClient/response_mocks.go#L32)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeviceInfo {
    pub udn: String,
    pub serial_number: String,
    pub device_id: String,
    pub advertising_id: String,
    pub vendor_name: String,
    pub model_name: String,
    pub model_number: String,
    pub model_region: String,
    pub is_tv: bool,
    pub is_stick: bool,
    pub ui_resolution: String,
    pub supports_ethernet: bool,
    pub wifi_mac: String,
    pub wifi_driver: String,
    pub has_wifi_extender: bool,
    #[serde(rename = "has-wifi-5G-support")]
    pub has_wifi_5g_support: bool,
    pub can_use_wifi_extender: bool,
    pub ethernet_mac: Option<String>,
    pub bluetooth_mac: String,
    pub network_type: String,
    pub friendly_device_name: String,
    pub friendly_model_name: String,
    pub default_device_name: String,
    pub user_device_name: String,
    pub user_device_location: String,
    pub build_number: String,
    pub software_version: String,
    pub software_build: String,
    pub secure_device: bool,
    pub language: String,
    pub country: String,
    pub locale: String,
    pub time_zone_auto: bool,
    pub time_zone: String,
    pub time_zone_name: String,
    pub time_zone_tz: String,
    pub time_zone_offset: i32,
    pub clock_format: String,
    pub uptime: u32,
    pub power_mode: String,
    pub supports_suspend: bool,
    pub supports_find_remote: bool,
    pub find_remote_is_possible: bool,
    pub supports_audio_guide: bool,
    pub supports_rva: bool,
    pub developer_enabled: bool,
    pub keyed_developer_id: Option<String>,
    pub search_enabled: bool,
    pub search_channels_enabled: bool,
    pub voice_search_enabled: bool,
    pub notifications_enabled: bool,
    pub notifications_first_use: bool,
    pub supports_private_listening: bool,
    pub headphones_connected: bool,
    pub supports_ecs_textedit: bool,
    pub supports_ecs_microphone: bool,
    pub supports_wake_on_wlan: bool,
    pub has_play_on_roku: bool,
    pub has_mobile_screensaver: bool,
    pub support_url: String,
    pub grandcentral_version: String,
    pub trc_version: String,
    pub trc_channel_version: String,
    pub davinci_version: String,
}
