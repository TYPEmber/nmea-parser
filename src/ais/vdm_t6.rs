use super::*;
use chrono::offset::LocalResult;

// -------------------------------------------------------------------------------------------------

/// Type 6: Binary Addressed Message
#[cfg(feature = "Chinese_binary_information")]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BinaryAddressedMessage {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// User ID (2 bits)
    pub sequence_number: u8,

    /// User ID (30 bits)
    pub destination_mmsi: u32,

    /// Retransmit flag
    pub retransmit_flag: bool,

    /// Designated area code, DAC (10 bits)
    pub dac: u16,

    /// Functional ID, FID (6 bits)
    pub fid: u8,
    // TODO: data (depending on DAC and FID )
    pub data: ApplicationIdentifier,
}

#[cfg(not(feature = "Chinese_binary_information"))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct BinaryAddressedMessage {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// User ID (2 bits)
    pub sequence_number: u8,

    /// User ID (30 bits)
    pub destination_mmsi: u32,

    /// Retransmit flag
    pub retransmit_flag: bool,

    /// Designated area code, DAC (10 bits)
    pub dac: u16,

    /// Functional ID, FID (6 bits)
    pub fid: u8,
    // TODO: data (depending on DAC and FID
}

impl LatLon for BinaryAddressedMessage {
    fn latitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }

    fn longitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }
}

#[cfg(feature = "Chinese_binary_information")]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Climate {
    // 5 bits
    pub forcast_time: u8,
    // 5 from_bit
    pub weather_1: WeatherType,
    //13bits
    pub longitude_1: Option<f64>,
    //13 bits
    pub latitude_1: Option<f64>,
    // 7bits
    pub wind_speed_1: Option<u8>,
    // 9 bits
    pub wind_direction_1: Option<u16>,
    // 11bits
    pub air_temperature_1: Option<f64>,
    // 9 bits
    pub presure_1: Option<u16>,
    // 8bits
    pub visibility_1: Option<f64>,

    // 5 from_bit
    pub weather_2: WeatherType,
    //13bits
    pub longitude_2: Option<f64>,
    //13 bits
    pub latitude_2: Option<f64>,
    // 7bits
    pub wind_speed_2: Option<u8>,
    // 9 bits
    pub wind_direction_2: Option<u16>,
    // 11bits
    pub air_temperature_2: Option<f64>,
    // 9 bits
    pub presure_2: Option<u16>,
    // 8bits
    pub visibility_2: Option<f64>,

    // 5 from_bit
    pub weather_3: WeatherType,
    //13bits
    pub longitude_3: Option<f64>,
    //13 bits
    pub latitude_3: Option<f64>,
    // 7bits
    pub wind_speed_3: Option<u8>,
    // 9 bits
    pub wind_direction_3: Option<u16>,
    // 11bits
    pub air_temperature_3: Option<f64>,
    // 9 bits
    pub presure_3: Option<u16>,
    // 8bits
    pub visibility_3: Option<f64>,

    // 5 from_bit
    pub weather_4: WeatherType,
    //13bits
    pub longitude_4: Option<f64>,
    //13 bits
    pub latitude_4: Option<f64>,
    // 7bits
    pub wind_speed_4: Option<u8>,
    // 9 bits
    pub wind_direction_4: Option<u16>,
    // 11bits
    pub air_temperature_4: Option<f64>,
    // 9 bits
    pub presure_4: Option<u16>,
    // 8bits
    pub visibility_4: Option<f64>,
    // 3bits
    pub source: u8,
}


#[cfg(feature = "Chinese_binary_information")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationIdentifier {
    // all the other AI
    Unknown,
    // dac=412 fi=1
    Climate(Climate),
    // dac=412,fi=2
    MarineEnviroment(MarineEnviroment),
    // dac=412, fi=4
    Hydrology(Hydrology),
    // dac=412, fi=13
    ClimateCollection(ClimateCollection),
}

#[cfg(feature = "Chinese_binary_information")]
impl Default for ApplicationIdentifier {
    fn default() -> ApplicationIdentifier {
        ApplicationIdentifier::Unknown
    }
}

#[cfg(feature = "Chinese_binary_information")]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum WeatherType {
    Sunny = 1,
    Cloudy = 2,
    Overcast = 3,
    Shower = 4,
    ThouderyShower = 5,
    ThouderyShowerWithHail = 6,
    Sleet = 7,
    LightRain = 8,
    ModerateRain = 9,
    HeavyRain = 10,
    Rainstorm = 11,
    LargeRainstorm = 12,
    ExtraLargeRainstrom = 13,
    SnowShower = 14,
    LightSnow = 15,
    ModerateSnow = 16,
    HeavySnow = 17,
    Blizzard = 18,
    Foggy = 19,
    FreezingRain = 20,
    SandStorm = 21,
    LightRainToModerateRain = 22,
    ModerateRainToHeavyRain = 23,
    HeavyRainToRainstrom = 24,
    RainstromToLargeRainstrom = 25,
    LargeRainstromToExtraLargeRainstrom = 26,
    LightSnowToModerateSnow = 27,
    ModerateSnowToHeavySnow = 28,
    HeavySnowToBlizzard = 29,
    Dust = 30,
    Sand = 31,
    Unknown = 32,
}

#[cfg(feature = "Chinese_binary_information")]
impl WeatherType {
    pub fn new(data: u8) -> WeatherType {
        match data {
            1 => WeatherType::Sunny,
            2 => WeatherType::Cloudy,
            3 => WeatherType::Overcast,
            4 => WeatherType::Shower,
            5 => WeatherType::ThouderyShower,
            6 => WeatherType::ThouderyShowerWithHail,
            7 => WeatherType::Sleet,
            8 => WeatherType::LightRain,
            9 => WeatherType::ModerateRain,
            10 => WeatherType::HeavyRain,
            11 => WeatherType::Rainstorm,
            12 => WeatherType::LargeRainstorm,
            13 => WeatherType::ExtraLargeRainstrom,
            14 => WeatherType::SnowShower,
            15 => WeatherType::LightSnow,
            16 => WeatherType::ModerateSnow,
            17 => WeatherType::HeavySnow,
            18 => WeatherType::Blizzard,
            19 => WeatherType::Foggy,
            20 => WeatherType::FreezingRain,
            21 => WeatherType::SandStorm,
            22 => WeatherType::LightRainToModerateRain,
            23 => WeatherType::ModerateRainToHeavyRain,
            24 => WeatherType::HeavyRainToRainstrom,
            25 => WeatherType::RainstromToLargeRainstrom,
            26 => WeatherType::LargeRainstromToExtraLargeRainstrom,
            27 => WeatherType::LightSnowToModerateSnow,
            28 => WeatherType::ModerateSnowToHeavySnow,
            29 => WeatherType::HeavySnowToBlizzard,
            30 => WeatherType::Dust,
            31 => WeatherType::Sand,
            _ => WeatherType::Unknown,
        }
    }

    pub fn to_value(&self) -> u8 {
        *self as u8
    }
}

#[cfg(feature = "Chinese_binary_information")]
impl std::fmt::Display for WeatherType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeatherType::Sunny => write!(f, "sunny"),
            WeatherType::Cloudy => write!(f, "cloudy"),
            WeatherType::Overcast => write!(f, "overcast"),
            WeatherType::Shower => write!(f, "shower"),
            WeatherType::ThouderyShower => write!(f, "thoudery shower"),
            WeatherType::ThouderyShowerWithHail => write!(f, "thoudery shower with hail"),
            WeatherType::Sleet => write!(f, "sleet"),
            WeatherType::LightRain => write!(f, "lightRain"),
            WeatherType::ModerateRain => write!(f, "moderateRain"),
            WeatherType::HeavyRain => write!(f, "heavyRain"),
            WeatherType::Rainstorm => write!(f, "rainstorm"),
            WeatherType::LargeRainstorm => write!(f, "largeRainstorm"),
            WeatherType::ExtraLargeRainstrom => write!(f, "extraLargeRainstrom"),
            WeatherType::SnowShower => write!(f, "snowShower"),
            WeatherType::LightSnow => write!(f, "lightSnow"),
            WeatherType::ModerateSnow => write!(f, "moderateSnow"),
            WeatherType::HeavySnow => write!(f, "heavySnow"),
            WeatherType::Blizzard => write!(f, "blizzard"),
            WeatherType::Foggy => write!(f, "foggy"),
            WeatherType::FreezingRain => write!(f, "freezingRain"),
            WeatherType::SandStorm => write!(f, "sandStorm"),
            WeatherType::LightRainToModerateRain => write!(f, "lightRainToModerateRain"),
            WeatherType::ModerateRainToHeavyRain => write!(f, "moderateRainToHeavyRain"),
            WeatherType::HeavyRainToRainstrom => write!(f, "heavy rain to rainstrom"),
            WeatherType::RainstromToLargeRainstrom => write!(f, "rainstromToLargeRainstrom"),
            WeatherType::LargeRainstromToExtraLargeRainstrom => {
                write!(f, "largeRainstromToExtraLargeRainstrom")
            }
            WeatherType::LightSnowToModerateSnow => write!(f, "lightSnowToModerateSnow"),
            WeatherType::ModerateSnowToHeavySnow => write!(f, "moderateSnowToHeavySnow"),
            WeatherType::HeavySnowToBlizzard => write!(f, "heavySnowToBlizzard"),
            WeatherType::Dust => write!(f, "dust"),
            WeatherType::Sand => write!(f, "sand"),
            WeatherType::Unknown => write!(f, "unknown"),
        }
    }
}

#[cfg(feature = "Chinese_binary_information")]
impl Default for WeatherType {
    fn default() -> WeatherType {
        WeatherType::Unknown
    }
}

#[cfg(feature = "Chinese_binary_information")]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarineEnviroment {
    // 5 bits
    pub forcast_time: u8,
    // 13bits
    pub longitude_1: Option<f64>,
    //13 bits
    pub latitude_1: Option<f64>,
    // 8bits
    pub flowing_speed_1: Option<f64>,
    // 9 bits
    pub flowing_direction_1: Option<u16>,
    // 8bits
    pub wave_height_1: Option<f64>,
    // 9bits
    pub wave_direction_1: Option<u16>,
    // 11bits
    pub sea_temperature_1: Option<f64>,

    // 13bits
    pub longitude_2: Option<f64>,
    //13 bits
    pub latitude_2: Option<f64>,
    // 8bits
    pub flowing_speed_2: Option<f64>,
    // 9 bits
    pub flowing_direction_2: Option<u16>,
    // 8bits
    pub wave_height_2: Option<f64>,
    // 9bits
    pub wave_direction_2: Option<u16>,
    // 11bits
    pub sea_temperature_2: Option<f64>,

    // 13bits
    pub longitude_3: Option<f64>,
    //13 bits
    pub latitude_3: Option<f64>,
    // 8bits
    pub flowing_speed_3: Option<f64>,
    // 9 bits
    pub flowing_direction_3: Option<u16>,
    // 8bits
    pub wave_height_3: Option<f64>,
    // 9bits
    pub wave_direction_3: Option<u16>,
    // 11bits
    pub sea_temperature_3: Option<f64>,

    // 13bits
    pub longitude_4: Option<f64>,
    //13 bits
    pub latitude_4: Option<f64>,
    // 8bits
    pub flowing_speed_4: Option<f64>,
    // 9 bits
    pub flowing_direction_4: Option<u16>,
    // 8bits
    pub wave_height_4: Option<f64>,
    // 9bits
    pub wave_direction_4: Option<u16>,
    // 11bits
    pub sea_temperature_4: Option<f64>,

    // 3bits
    pub source: u8,
}

#[cfg(feature = "Chinese_binary_information")]
impl LatLon for MarineEnviroment {
    fn latitude(&self) -> Option<f64> {
        self.flowing_speed_1 // TODO: depends on data
    }

    fn longitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }
}

#[cfg(feature = "Chinese_binary_information")]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hydrology {
    // 5bits
    pub forcast_time: u8,
    // 13bits
    pub longitude_1: Option<f64>,
    // 13 bits
    pub latitude_1: Option<f64>,
    // 16 bits
    pub high_tide_1: Option<DateTime<Utc>>,
    // 16 bits
    pub low_tide_1: Option<DateTime<Utc>>,
    // 9 bits
    pub water_level_1: Option<f64>,

    // 13bits
    pub longitude_2: Option<f64>,
    // 13 bits
    pub latitude_2: Option<f64>,
    // 16 bits
    pub high_tide_2: Option<DateTime<Utc>>,
    // 16 bits
    pub low_tide_2: Option<DateTime<Utc>>,
    // 9 bits
    pub water_level_2: Option<f64>,

    // 13bits
    pub longitude_3: Option<f64>,
    // 13 bits
    pub latitude_3: Option<f64>,
    // 16 bits
    pub high_tide_3: Option<DateTime<Utc>>,
    // 16 bits
    pub low_tide_3: Option<DateTime<Utc>>,
    // 9 bits
    pub water_level_3: Option<f64>,

    // 3 bits
    pub source: u8,
}

#[cfg(feature = "Chinese_binary_information")]
impl LatLon for Hydrology {
    fn latitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }

    fn longitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }
}

#[cfg(feature = "Chinese_binary_information")]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClimateCollection {
    // 20 bits
    pub collection_time: Option<DateTime<Utc>>,

    // 28 bits
    pub longitude: Option<f64>,

    // 27 bits
    pub latitude: Option<f64>,

    // 7bits
    pub wind_speed: Option<u8>,

    // 9 bits
    pub wind_direction: Option<u16>,

    // 11 bits
    pub air_temperature: Option<f64>,

    // 11 bits
    pub sea_temperature: Option<f64>,

    // 8 bits
    pub flowing_speed: Option<f64>,

    // 9 bits
    pub flowing_direction: Option<u16>,

    // 9 bits
    pub pressure: Option<u16>,

    // 8 bits
    pub visibility: Option<f64>,

    // 8 bits
    pub wave_height: Option<f64>,

    // 9 bits
    pub wave_direction: Option<u16>,

    // 7 bits
    pub humidity: Option<u8>,

    // 9 bits
    pub water_level: Option<f64>,
}

#[cfg(feature = "Chinese_binary_information")]
impl LatLon for ClimateCollection {
    fn latitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }

    fn longitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }
}

#[cfg(feature = "Chinese_binary_information")]
fn handle_climate(bv: &BitVec) -> ApplicationIdentifier {
    let mut lon_raw1 = 0.0;
    let mut lat_raw1 = 0.0;
    let mut lon_raw2 = 0.0;
    let mut lat_raw2 = 0.0;
    let mut lon_raw3 = 0.0;
    let mut lat_raw3 = 0.0;
    let mut lon_raw4;
    let lat_raw4;

    ApplicationIdentifier::Climate(Climate {
        forcast_time: { pick_u64(&bv, 88, 5) as u8 },
        weather_1: { WeatherType::new(pick_u64(&bv, 93, 5) as u8) },
        longitude_1: {
            let lon_raw = pick_u64(&bv, 98, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                lon_raw1 = ((lon_raw as f64) / 60.0) + 60.0;
                Some(lon_raw1)
            }
        },

        latitude_1: {
            let lat_raw = pick_u64(&bv, 111, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                lat_raw1 = ((lat_raw as f64) / 60.0) - 50.0;
                Some(lat_raw1)
            }
        },

        wind_speed_1: {
            let speed_raw = pick_u64(&bv, 124, 7) as u8;
            if speed_raw > 120 {
                None
            } else {
                Some(speed_raw)
            }
        },

        wind_direction_1: {
            let direc_raw = pick_u64(&bv, 131, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        air_temperature_1: {
            let temp_raw = pick_i64(&bv, 140, 11) as i32;
            if temp_raw < -600 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        presure_1: {
            let presure_raw = pick_u64(&bv, 151, 9) as u16;
            if presure_raw > 400 {
                None
            } else {
                Some(presure_raw + 800)
            }
        },

        visibility_1: {
            let visibility_raw = pick_u64(&bv, 160, 8) as u16;
            if visibility_raw > 250 {
                None
            } else {
                Some((visibility_raw as f64) / 10.0)
            }
        },

        weather_2: { WeatherType::new(pick_u64(&bv, 168, 5) as u8) },
        longitude_2: {
            let lon_raw = pick_u64(&bv, 173, 7) as u32;
            if lon_raw > 59 || lon_raw < 1 {
                None
            } else {
                lon_raw2 = lon_raw1 + ((lon_raw as f64) / 60.0);
                if lon_raw2 > 180.0 {
                    lon_raw2 = (lon_raw2 - 180.0) - 180.0;
                }
                Some(lon_raw2)
            }
        },
        latitude_2: {
            let lat_raw = pick_u64(&bv, 180, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw2 = lat_raw1 + ((lat_raw as f64) / 60.0);
                Some(lat_raw2)
            }
        },

        wind_speed_2: {
            let speed_raw = pick_u64(&bv, 187, 7) as u8;
            if speed_raw > 120 {
                None
            } else {
                Some(speed_raw)
            }
        },

        wind_direction_2: {
            let direc_raw = pick_u64(&bv, 194, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        air_temperature_2: {
            let temp_raw = pick_i64(&bv, 203, 11) as i32;
            if temp_raw < -600 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        presure_2: {
            let presure_raw = pick_u64(&bv, 214, 9) as u16;
            if presure_raw > 400 {
                None
            } else {
                Some(presure_raw + 800)
            }
        },

        visibility_2: {
            let visibility_raw = pick_u64(&bv, 223, 8) as u16;
            if visibility_raw > 250 {
                None
            } else {
                Some((visibility_raw as f64) / 10.0)
            }
        },

        weather_3: { WeatherType::new(pick_u64(&bv, 231, 5) as u8) },
        longitude_3: {
            let lon_raw = pick_u64(&bv, 236, 7) as u32;
            if lon_raw > 59 || lon_raw < 1 {
                None
            } else {
                lon_raw3 = lon_raw2 + ((lon_raw as f64) / 60.0);
                if lon_raw3 > 180.0 {
                    lon_raw3 = (lon_raw3 - 180.0) - 180.0;
                }
                Some(lon_raw3)
            }
        },
        latitude_3: {
            let lat_raw = pick_u64(&bv, 243, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw3 = lat_raw2 + ((lat_raw as f64) / 60.0);
                Some(lat_raw3)
            }
        },

        wind_speed_3: {
            let speed_raw = pick_u64(&bv, 250, 7) as u8;
            if speed_raw > 120 {
                None
            } else {
                Some(speed_raw)
            }
        },

        wind_direction_3: {
            let direc_raw = pick_u64(&bv, 257, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        air_temperature_3: {
            let temp_raw = pick_i64(&bv, 266, 11) as i32;
            if temp_raw < -600 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        presure_3: {
            let presure_raw = pick_u64(&bv, 277, 9) as u16;
            if presure_raw > 400 {
                None
            } else {
                Some(presure_raw + 800)
            }
        },

        visibility_3: {
            let visibility_raw = pick_u64(&bv, 286, 8) as u16;
            if visibility_raw > 250 {
                None
            } else {
                Some((visibility_raw as f64) / 10.0)
            }
        },

        weather_4: { WeatherType::new(pick_u64(&bv, 294, 5) as u8) },
        longitude_4: {
            let lon_raw = pick_u64(&bv, 299, 7) as u32;
            if lon_raw > 59 || lon_raw < 1 {
                None
            } else {
                lon_raw4 = lon_raw3 + ((lon_raw as f64) / 60.0);
                if lon_raw4 > 180.0 {
                    lon_raw4 = (lon_raw4 - 180.0) - 180.0;
                }
                Some(lon_raw4)
            }
        },
        latitude_4: {
            let lat_raw = pick_u64(&bv, 306, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw4 = lat_raw3 + ((lat_raw as f64) / 60.0);
                Some(lat_raw4)
            }
        },

        wind_speed_4: {
            let speed_raw = pick_u64(&bv, 313, 7) as u8;
            if speed_raw > 120 {
                None
            } else {
                Some(speed_raw)
            }
        },

        wind_direction_4: {
            let direc_raw = pick_u64(&bv, 320, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        air_temperature_4: {
            let temp_raw = pick_i64(&bv, 329, 11) as i32;
            if temp_raw < -600 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        presure_4: {
            let presure_raw = pick_u64(&bv, 340, 9) as u16;
            if presure_raw > 400 {
                None
            } else {
                Some(presure_raw + 800)
            }
        },

        visibility_4: {
            let visibility_raw = pick_u64(&bv, 349, 8) as u16;
            if visibility_raw > 250 {
                None
            } else {
                Some((visibility_raw as f64) / 10.0)
            }
        },
        source: { pick_u64(&bv, 357, 3) as u8 },
    })
}

#[cfg(feature = "Chinese_binary_information")]
fn handle_marine_enviroment(bv: &BitVec) -> ApplicationIdentifier {
    let mut lon_raw1 = 0.0;
    let mut lat_raw1 = 0.0;
    let mut lon_raw2 = 0.0;
    let mut lat_raw2 = 0.0;
    let mut lon_raw3 = 0.0;
    let mut lat_raw3 = 0.0;
    let mut lon_raw4;
    let lat_raw4;
    ApplicationIdentifier::MarineEnviroment(MarineEnviroment {
        forcast_time: { pick_u64(&bv, 88, 5) as u8 },
        longitude_1: {
            let lon_raw = pick_u64(&bv, 93, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                lon_raw1 = ((lon_raw as f64) / 60.0) + 60.0;
                Some(lon_raw1)
            }
        },

        latitude_1: {
            let lat_raw = pick_u64(&bv, 106, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                lat_raw1 = ((lat_raw as f64) / 60.0) - 50.0;
                Some(lat_raw1)
            }
        },

        flowing_speed_1: {
            let speed_raw = pick_u64(&bv, 119, 8) as u32;
            if speed_raw > 250 {
                None
            } else {
                Some((speed_raw as f64) / 10.0)
            }
        },

        flowing_direction_1: {
            let direc_raw = pick_u64(&bv, 127, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        wave_height_1: {
            let height_raw = pick_u64(&bv, 136, 8) as u32;
            if height_raw > 250 {
                None
            } else {
                Some((height_raw as f64) / 10.0)
            }
        },

        wave_direction_1: {
            let direc_raw = pick_u64(&bv, 144, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        sea_temperature_1: {
            let temp_raw = pick_i64(&bv, 153, 11) as i32;
            if temp_raw < -100 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        longitude_2: {
            let lon_raw = pick_u64(&bv, 164, 7) as u32;
            if lon_raw > 59 || lon_raw < 1 {
                None
            } else {
                lon_raw2 = lon_raw1 + ((lon_raw as f64) / 60.0);
                if lon_raw2 > 180.0 {
                    lon_raw2 = (lon_raw2 - 180.0) - 180.0;
                }
                Some(lon_raw2)
            }
        },
        latitude_2: {
            let lat_raw = pick_u64(&bv, 171, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw2 = lat_raw1 + ((lat_raw as f64) / 60.0);
                Some(lat_raw2)
            }
        },

        flowing_speed_2: {
            let speed_raw = pick_u64(&bv, 178, 8) as u32;
            if speed_raw > 250 {
                None
            } else {
                Some((speed_raw as f64) / 10.0)
            }
        },

        flowing_direction_2: {
            let direc_raw = pick_u64(&bv, 186, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        wave_height_2: {
            let height_raw = pick_u64(&bv, 195, 8) as u32;
            if height_raw > 250 {
                None
            } else {
                Some((height_raw as f64) / 10.0)
            }
        },

        wave_direction_2: {
            let direc_raw = pick_u64(&bv, 203, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        sea_temperature_2: {
            let temp_raw = pick_i64(&bv, 212, 11) as i32;
            if temp_raw < -100 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        longitude_3: {
            let lon_raw = pick_u64(&bv, 223, 7) as u32;
            if lon_raw > 59 || lon_raw < 1 {
                None
            } else {
                lon_raw3 = lon_raw2 + ((lon_raw as f64) / 60.0);
                if lon_raw3 > 180.0 {
                    lon_raw3 = (lon_raw3 - 180.0) - 180.0;
                }
                Some(lon_raw3)
            }
        },
        latitude_3: {
            let lat_raw = pick_u64(&bv, 230, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw3 = lat_raw2 + ((lat_raw as f64) / 60.0);
                Some(lat_raw3)
            }
        },

        flowing_speed_3: {
            let speed_raw = pick_u64(&bv, 237, 8) as u32;
            if speed_raw > 250 {
                None
            } else {
                Some((speed_raw as f64) / 10.0)
            }
        },

        flowing_direction_3: {
            let direc_raw = pick_u64(&bv, 245, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        wave_height_3: {
            let height_raw = pick_u64(&bv, 254, 8) as u32;
            if height_raw > 250 {
                None
            } else {
                Some((height_raw as f64) / 10.0)
            }
        },

        wave_direction_3: {
            let direc_raw = pick_u64(&bv, 262, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        sea_temperature_3: {
            let temp_raw = pick_i64(&bv, 271, 11) as i32;
            if temp_raw < -100 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        longitude_4: {
            let lon_raw = pick_u64(&bv, 282, 7) as u32;
            if lon_raw > 59 || lon_raw < 1 {
                None
            } else {
                lon_raw4 = lon_raw3 + ((lon_raw as f64) / 60.0);
                if lon_raw4 > 180.0 {
                    lon_raw4 = (lon_raw4 - 180.0) - 180.0;
                }
                Some(lon_raw4)
            }
        },
        latitude_4: {
            let lat_raw = pick_u64(&bv, 289, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw4 = lat_raw3 + ((lat_raw as f64) / 60.0);
                Some(lat_raw4)
            }
        },

        flowing_speed_4: {
            let speed_raw = pick_u64(&bv, 296, 8) as u32;
            if speed_raw > 250 {
                None
            } else {
                Some((speed_raw as f64) / 10.0)
            }
        },

        flowing_direction_4: {
            let direc_raw = pick_u64(&bv, 304, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        wave_height_4: {
            let height_raw = pick_u64(&bv, 313, 8) as u32;
            if height_raw > 250 {
                None
            } else {
                Some((height_raw as f64) / 10.0)
            }
        },

        wave_direction_4: {
            let direc_raw = pick_u64(&bv, 321, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        sea_temperature_4: {
            let temp_raw = pick_i64(&bv, 330, 11) as i32;
            if temp_raw < -100 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },
        source: { pick_u64(&bv, 341, 3) as u8 },
    })
}

#[cfg(feature = "Chinese_binary_information")]
fn handle_hydrology(bv: &BitVec) -> ApplicationIdentifier {
    ApplicationIdentifier::Hydrology(Hydrology {
        forcast_time: { pick_u64(&bv, 88, 5) as u8 },
        longitude_1: {
            let lon_raw = pick_u64(&bv, 93, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                Some(((lon_raw as f64) / 60.0) + 60.0)
            }
        },

        latitude_1: {
            let lat_raw = pick_u64(&bv, 106, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                Some(((lat_raw as f64) / 60.0) - 50.0)
            }
        },

        high_tide_1: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 130, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 125, 5) as u32,
                    pick_u64(&bv, 119, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        low_tide_1: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 146, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 141, 5) as u32,
                    pick_u64(&bv, 135, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        water_level_1: {
            let level_raw = pick_i64(&bv, 151, 9) as i32;
            if level_raw < -250 || level_raw > 250 {
                None
            } else {
                Some((level_raw as f64) / 10.0)
            }
        },

        longitude_2: {
            let lon_raw = pick_u64(&bv, 160, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                Some(((lon_raw as f64) / 60.0) + 60.0)
            }
        },

        latitude_2: {
            let lat_raw = pick_u64(&bv, 173, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                Some(((lat_raw as f64) / 60.0) - 50.0)
            }
        },

        high_tide_2: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 197, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 192, 5) as u32,
                    pick_u64(&bv, 186, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        low_tide_2: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 213, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 208, 5) as u32,
                    pick_u64(&bv, 202, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        water_level_2: {
            let level_raw = pick_i64(&bv, 218, 9) as i32;
            if level_raw < -250 || level_raw > 250 {
                None
            } else {
                Some((level_raw as f64) / 10.0)
            }
        },

        longitude_3: {
            let lon_raw = pick_u64(&bv, 227, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                Some(((lon_raw as f64) / 60.0) + 60.0)
            }
        },

        latitude_3: {
            let lat_raw = pick_u64(&bv, 240, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                Some(((lat_raw as f64) / 60.0) - 50.0)
            }
        },

        high_tide_3: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 264, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 259, 5) as u32,
                    pick_u64(&bv, 253, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        low_tide_3: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 280, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 275, 5) as u32,
                    pick_u64(&bv, 269, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        water_level_3: {
            let level_raw = pick_i64(&bv, 285, 9) as i32;
            if level_raw < -250 || level_raw > 250 {
                None
            } else {
                Some((level_raw as f64) / 10.0)
            }
        },

        source: { pick_u64(&bv, 294, 3) as u8 },
    })
}

#[cfg(feature = "Chinese_binary_information")]
fn handle_climate_collections(bv: &BitVec) -> ApplicationIdentifier {
    ApplicationIdentifier::ClimateCollection(ClimateCollection {
        collection_time: {
            let now = Utc::now();
            match Utc
                .ymd_opt(
                    now.year(),
                    pick_u64(&bv, 104, 4) as u32,
                    pick_u64(&bv, 99, 5) as u32,
                )
                .and_hms_opt(pick_u64(&bv, 94, 5) as u32, pick_u64(&bv, 88, 6) as u32, 0)
            {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        longitude: {
            let lon_raw = pick_i64(&bv, 108, 28) as i32;
            let lon = (lon_raw as f64) / 600000.0;
            if lon > 180.0 || lon < -180.0 {
                None
            } else {
                Some(lon)
            }
        },

        latitude: {
            let lat_raw = pick_i64(&bv, 136, 27) as i32;
            let lat = (lat_raw as f64) / 600000.0;
            if lat > 90.0 || lat < -90.0 {
                None
            } else {
                Some(lat)
            }
        },

        wind_speed: {
            let speed_raw = pick_u64(&bv, 163, 7) as u8;
            if speed_raw > 120 {
                None
            } else {
                Some(speed_raw)
            }
        },

        wind_direction: {
            let direc_raw = pick_u64(&bv, 170, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        air_temperature: {
            let temp_raw = pick_i64(&bv, 179, 11) as i32;
            if temp_raw < -600 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        sea_temperature: {
            let temp_raw = pick_i64(&bv, 190, 11) as i32;
            if temp_raw < -100 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        flowing_speed: {
            let speed_raw = pick_u64(&bv, 201, 8) as u32;
            if speed_raw > 250 {
                None
            } else {
                Some((speed_raw as f64) / 10.0)
            }
        },

        flowing_direction: {
            let direc_raw = pick_u64(&bv, 209, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        pressure: {
            let presure_raw = pick_u64(&bv, 218, 9) as u16;
            if presure_raw > 400 {
                None
            } else {
                Some(presure_raw + 800)
            }
        },

        visibility: {
            let visibility_raw = pick_u64(&bv, 227, 8) as u32;
            if visibility_raw > 250 {
                None
            } else {
                Some((visibility_raw as f64) / 10.0)
            }
        },

        wave_height: {
            let height_raw = pick_u64(&bv, 235, 8) as u32;
            if height_raw > 250 {
                None
            } else {
                Some((height_raw as f64) / 10.0)
            }
        },

        wave_direction: {
            let direc_raw = pick_u64(&bv, 243, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        humidity: {
            let humidity_raw = pick_u64(&bv, 252, 7) as u8;
            if humidity_raw > 99 {
                None
            } else {
                Some(humidity_raw)
            }
        },

        water_level: {
            let level_raw = pick_i64(&bv, 259, 9) as i32;
            if level_raw < -250 || level_raw > 250 {
                None
            } else {
                Some((level_raw as f64) / 10.0)
            }
        },
    })
}
// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 6: Binary Addressed Message. Implementation of the 920-bit data field is
/// unimplemented currently.
#[cfg(feature = "Chinese_binary_information")]
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    let dac_t: u16 = pick_u64(&bv, 72, 10) as u16;
    let fid_t: u8 = pick_u64(&bv, 82, 6) as u8;
    Ok(ParsedMessage::BinaryAddressedMessage(
        BinaryAddressedMessage {
            own_vessel: { own_vessel },
            station: { station },
            mmsi: { pick_u64(&bv, 8, 30) as u32 },
            sequence_number: { pick_u64(&bv, 38, 2) as u8 },
            destination_mmsi: { pick_u64(&bv, 40, 30) as u32 },
            retransmit_flag: { pick_u64(&bv, 70, 1) != 0 },
            dac: { dac_t },
            fid: { fid_t }, // TODO: data (depending on DAC and FID
            data: {
                match dac_t {
                    412 => match fid_t {
                        1 => handle_climate(&bv),
                        2 => handle_marine_enviroment(&bv),
                        4 => handle_hydrology(&bv),
                        13 => handle_climate_collections(&bv),
                        _ => ApplicationIdentifier::Unknown,
                    },
                    _ => ApplicationIdentifier::Unknown,
                }
            },
        },
    ))
}

#[cfg(not(feature = "Chinese_binary_information"))]
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    Ok(ParsedMessage::BinaryAddressedMessage(
        BinaryAddressedMessage {
            own_vessel: { own_vessel },
            station: { station },
            mmsi: { pick_u64(&bv, 8, 30) as u32 },
            sequence_number: { pick_u64(&bv, 38, 2) as u8 },
            destination_mmsi: { pick_u64(&bv, 40, 30) as u32 },
            retransmit_flag: { pick_u64(&bv, 70, 1) != 0 },
            dac: { pick_u64(&bv, 72, 10) as u16 },
            fid: { pick_u64(&bv, 82, 6) as u8 }, // TODO: data (depending on DAC and FID
        },
    ))
}
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type6() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,B,6B?n;be:cbapalgc;i6?Ow4,2*4A") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::BinaryAddressedMessage(bam) => {
                        assert_eq!(bam.mmsi, 150834090);
                        assert_eq!(bam.sequence_number, 3);
                        assert_eq!(bam.destination_mmsi, 313240222);
                        assert_eq!(bam.retransmit_flag, false);
                        assert_eq!(bam.dac, 669);
                        assert_eq!(bam.fid, 11);
                        // TODO: check data
                    }
                    ParsedMessage::Incomplete => {
                        assert!(false);
                    }
                    _ => {
                        assert!(false);
                    }
                }
            }
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
    }
    
}
