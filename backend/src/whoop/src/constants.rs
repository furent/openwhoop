#![allow(unused)]

use uuid::{uuid, Uuid};

pub const WHOOP_SERVICE: Uuid = uuid!("61080001-8d6d-82b8-614a-1c8cb0f8dcc6");
pub const CMD_TO_STRAP: Uuid = uuid!("61080002-8d6d-82b8-614a-1c8cb0f8dcc6");
pub const DATA_FROM_STRAP: Uuid = uuid!("61080005-8d6d-82b8-614a-1c8cb0f8dcc6");
pub const CMD_FROM_STRAP: Uuid = uuid!("61080003-8d6d-82b8-614a-1c8cb0f8dcc6");
pub const EVENTS_FROM_STRAP: Uuid = uuid!("61080004-8d6d-82b8-614a-1c8cb0f8dcc6");
pub const MEMFAULT: Uuid = uuid!("61080007-8d6d-82b8-614a-1c8cb0f8dcc6");

// PacketType enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    Command = 35,
    CommandResponse = 36,
    RealtimeData = 40,
    HistoricalData = 47,
    RealtimeRawData = 43,
    Event = 48,
    Metadata = 49,
    ConsoleLogs = 50,
    RealtimeImuDataStream = 51,
    HistoricalImuDataStream = 52,
}

// MetadataType enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MetadataType {
    HistoryStart = 1,
    HistoryEnd = 2,
    HistoryComplete = 3,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum EventNumber {
    Undefined = 0,
    Error = 1,
    ConsoleOutput = 2,
    BatteryLevel = 3,
    SystemControl = 4,
    External5vOn = 5,
    External5vOff = 6,
    ChargingOn = 7,
    ChargingOff = 8,
    WristOn = 9,
    WristOff = 10,
    BleConnectionUp = 11,
    BleConnectionDown = 12,
    RtcLost = 13,
    DoubleTap = 14,
    Boot = 15,
    SetRtc = 16,
    TemperatureLevel = 17,
    PairingMode = 18,
    SerialHeadConnected = 19,
    SerialHeadRemoved = 20,
    BatteryPackConnected = 21,
    BatteryPackRemoved = 22,
    BleBonded = 23,
    BleHrProfileEnabled = 24,
    BleHrProfileDisabled = 25,
    TrimAllData = 26,
    TrimAllDataEnded = 27,
    FlashInitComplete = 28,
    StrapConditionReport = 29,
    BootReport = 30,
    ExitVirginMode = 31,
    CaptouchAutothresholdAction = 32,
    BleRealtimeHrOn = 33,
    BleRealtimeHrOff = 34,
    AccelerometerReset = 35,
    AfeReset = 36,
    ShipModeEnabled = 37,
    ShipModeDisabled = 38,
    ShipModeBoot = 39,
    Ch1SaturationDetected = 40,
    Ch2SaturationDetected = 41,
    AccelerometerSaturationDetected = 42,
    BleSystemReset = 43,
    BleSystemOn = 44,
    BleSystemInitialized = 45,
    RawDataCollectionOn = 46,
    RawDataCollectionOff = 47,
    StrapDrivenAlarmSet = 56,
    StrapDrivenAlarmExecuted = 57,
    AppDrivenAlarmExecuted = 58,
    StrapDrivenAlarmDisabled = 59,
    HapticsFired = 60,
    ExtendedBatteryInformation = 63,
    HighFreqSyncPrompt = 96,
    HighFreqSyncEnabled = 97,
    HighFreqSyncDisabled = 98,
    HapticsTerminated = 100,
}

// CommandNumber enum - truncated for brevity, add more variants as needed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CommandNumber {
    LinkValid = 1,
    GetMaxProtocolVersion = 2,
    ToggleRealtimeHr = 3,
    ReportVersionInfo = 7,
    ToggleR7DataCollection = 16,
    SetClock = 10,
    GetClock = 11,
    ToggleGenericHrProfile = 14,
    RunHapticPatternMaverick = 19,
    AbortHistoricalTransmits = 20,
    SendHistoricalData = 22,
    HistoricalDataResult = 23,
    GetBatteryLevel = 26,
    RebootStrap = 29,
    ForceTrim = 25,
    PowerCycleStrap = 32,
    SetReadPointer = 33,
    GetDataRange = 34,
    GetHelloHarvard = 35,
    StartFirmwareLoad = 36,
    LoadFirmwareData = 37,
    ProcessFirmwareImage = 38,
    StartFirmwareLoadNew = 142,
    LoadFirmwareDataNew = 143,
    ProcessFirmwareImageNew = 144,
    VerifyFirmwareImage = 83,
    SetLedDrive = 39,
    GetLedDrive = 40,
    SetTiaGain = 41,
    GetTiaGain = 42,
    SetBiasOffset = 43,
    GetBiasOffset = 44,
    EnterBleDfu = 45,
    SetDpType = 52,
    ForceDpType = 53,
    SendR10R11Realtime = 63,
    SetAlarmTime = 66,
    GetAlarmTime = 67,
    RunAlarm = 68,
    DisableAlarm = 69,
    GetAdvertisingNameHarvard = 76,
    SetAdvertisingNameHarvard = 77,
    RunHapticsPattern = 79,
    GetAllHapticsPattern = 80,
    StartRawData = 81,
    StopRawData = 82,
    GetBodyLocationAndStatus = 84,
    EnterHighFreqSync = 96,
    ExitHighFreqSync = 97,
    GetExtendedBatteryInfo = 98,
    ResetFuelGauge = 99,
    CalibrateCapsense = 100,
    ToggleImuModeHistorical = 105,
    ToggleImuMode = 106,
    ToggleOpticalMode = 108,
    StartFfKeyExchange = 117,
    SendNextFf = 118,
    SetFfValue = 120,
    GetFfValue = 128,
    StopHaptics = 122,
    SelectWrist = 123,
    ToggleLabradorFiltered = 139,
    ToggleLabradorRawSave = 125,
    ToggleLabradorDataGeneration = 124,
    StartDeviceConfigKeyExchange = 115,
    SendNextDeviceConfig = 116,
    SetDeviceConfigValue = 119,
    GetDeviceConfigValue = 121,
    SetResearchPacket = 131,
    GetResearchPacket = 132,
    GetAdvertisingName = 141,
    SetAdvertisingName = 140,
    GetHello = 145,
    EnableOpticalData = 107,
}

impl PacketType {
    // Convert from u8 to PacketType
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            35 => Some(Self::Command),
            36 => Some(Self::CommandResponse),
            40 => Some(Self::RealtimeData),
            47 => Some(Self::HistoricalData),
            43 => Some(Self::RealtimeRawData),
            48 => Some(Self::Event),
            49 => Some(Self::Metadata),
            50 => Some(Self::ConsoleLogs),
            51 => Some(Self::RealtimeImuDataStream),
            52 => Some(Self::HistoricalImuDataStream),
            _ => None,
        }
    }

    // Convert PacketType to u8
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

impl CommandNumber {
    // Convert from u8 to CommandNumber
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::LinkValid),
            2 => Some(Self::GetMaxProtocolVersion),
            3 => Some(Self::ToggleRealtimeHr),
            7 => Some(Self::ReportVersionInfo),
            16 => Some(Self::ToggleR7DataCollection),
            10 => Some(Self::SetClock),
            11 => Some(Self::GetClock),
            14 => Some(Self::ToggleGenericHrProfile),
            19 => Some(Self::RunHapticPatternMaverick),
            20 => Some(Self::AbortHistoricalTransmits),
            22 => Some(Self::SendHistoricalData),
            23 => Some(Self::HistoricalDataResult),
            26 => Some(Self::GetBatteryLevel),
            29 => Some(Self::RebootStrap),
            25 => Some(Self::ForceTrim),
            32 => Some(Self::PowerCycleStrap),
            33 => Some(Self::SetReadPointer),
            34 => Some(Self::GetDataRange),
            35 => Some(Self::GetHelloHarvard),
            36 => Some(Self::StartFirmwareLoad),
            37 => Some(Self::LoadFirmwareData),
            38 => Some(Self::ProcessFirmwareImage),
            142 => Some(Self::StartFirmwareLoadNew),
            143 => Some(Self::LoadFirmwareDataNew),
            144 => Some(Self::ProcessFirmwareImageNew),
            83 => Some(Self::VerifyFirmwareImage),
            39 => Some(Self::SetLedDrive),
            40 => Some(Self::GetLedDrive),
            41 => Some(Self::SetTiaGain),
            42 => Some(Self::GetTiaGain),
            43 => Some(Self::SetBiasOffset),
            44 => Some(Self::GetBiasOffset),
            45 => Some(Self::EnterBleDfu),
            52 => Some(Self::SetDpType),
            53 => Some(Self::ForceDpType),
            63 => Some(Self::SendR10R11Realtime),
            66 => Some(Self::SetAlarmTime),
            67 => Some(Self::GetAlarmTime),
            68 => Some(Self::RunAlarm),
            69 => Some(Self::DisableAlarm),
            76 => Some(Self::GetAdvertisingNameHarvard),
            77 => Some(Self::SetAdvertisingNameHarvard),
            79 => Some(Self::RunHapticsPattern),
            80 => Some(Self::GetAllHapticsPattern),
            81 => Some(Self::StartRawData),
            82 => Some(Self::StopRawData),
            84 => Some(Self::GetBodyLocationAndStatus),
            96 => Some(Self::EnterHighFreqSync),
            97 => Some(Self::ExitHighFreqSync),
            98 => Some(Self::GetExtendedBatteryInfo),
            99 => Some(Self::ResetFuelGauge),
            100 => Some(Self::CalibrateCapsense),
            105 => Some(Self::ToggleImuModeHistorical),
            106 => Some(Self::ToggleImuMode),
            108 => Some(Self::ToggleOpticalMode),
            117 => Some(Self::StartFfKeyExchange),
            118 => Some(Self::SendNextFf),
            120 => Some(Self::SetFfValue),
            128 => Some(Self::GetFfValue),
            122 => Some(Self::StopHaptics),
            123 => Some(Self::SelectWrist),
            139 => Some(Self::ToggleLabradorFiltered),
            125 => Some(Self::ToggleLabradorRawSave),
            124 => Some(Self::ToggleLabradorDataGeneration),
            115 => Some(Self::StartDeviceConfigKeyExchange),
            116 => Some(Self::SendNextDeviceConfig),
            119 => Some(Self::SetDeviceConfigValue),
            121 => Some(Self::GetDeviceConfigValue),
            131 => Some(Self::SetResearchPacket),
            132 => Some(Self::GetResearchPacket),
            141 => Some(Self::GetAdvertisingName),
            140 => Some(Self::SetAdvertisingName),
            145 => Some(Self::GetHello),
            107 => Some(Self::EnableOpticalData),
            _ => None,
        }
    }

    // Convert CommandNumber to u8
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

impl MetadataType {
    // Convert from u8 to PacketType
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::HistoryStart),
            2 => Some(Self::HistoryEnd),
            3 => Some(Self::HistoryComplete),
            _ => None,
        }
    }

    // Convert PacketType to u8
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
