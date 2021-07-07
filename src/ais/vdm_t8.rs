use super::*;
use chrono::offset::LocalResult;
use vdm_t6::{Climate, MarineEnviroment, WeatherType};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BinaryBroadcastMessage {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// Designated area code, DAC (10 bits)
    pub dac: u16,

    /// Functional ID, FID (6 bits)
    pub fid: u8,

    // TODO: data (depending on DAC and FID )
    pub data: ApplicationIdentifier,
}

impl LatLon for BinaryBroadcastMessage {
    fn latitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }

    fn longitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }
}

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
}

impl Default for ApplicationIdentifier {
    fn default() -> ApplicationIdentifier {
        ApplicationIdentifier::Unknown
    }
}
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

    // 13bits
    pub longitude_4: Option<f64>,
    // 13 bits
    pub latitude_4: Option<f64>,
    // 16 bits
    pub high_tide_4: Option<DateTime<Utc>>,
    // 16 bits
    pub low_tide_4: Option<DateTime<Utc>>,
    // 9 bits
    pub water_level_4: Option<f64>,

    // 3 bits
    pub source: u8,
}

pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    let dac_t: u16 = pick_u64(&bv, 40, 10) as u16;
    let fid_t: u8 = pick_u64(&bv, 50, 6) as u8;
    Ok(ParsedMessage::BinaryBroadcastMessage(
        BinaryBroadcastMessage {
            own_vessel: { own_vessel },
            station: { station },
            mmsi: { pick_u64(&bv, 8, 30) as u32 },
            dac: { dac_t },
            fid: { fid_t }, // TODO: data (depending on DAC and FID
            data: {
                match dac_t {
                    412 => match fid_t {
                        1 => handle_climate(&bv),
                        2 => handle_marine_enviroment(&bv),
                        4 => handle_hydrology(&bv),
                        _ => ApplicationIdentifier::Unknown,
                    },
                    _ => ApplicationIdentifier::Unknown,
                }
            },
        },
    ))
}

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
        forcast_time: { pick_u64(&bv, 56, 5) as u8 },
        weather_1: { WeatherType::new(pick_u64(&bv, 61, 5) as u8) },
        longitude_1: {
            let lon_raw = pick_u64(&bv, 66, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                lon_raw1 = ((lon_raw as f64) / 60.0) + 60.0;
                Some(lon_raw1)
            }
        },

        latitude_1: {
            let lat_raw = pick_u64(&bv, 79, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                lat_raw1 = ((lat_raw as f64) / 60.0) - 50.0;
                Some(lat_raw1)
            }
        },

        wind_speed_1: {
            let speed_raw = pick_u64(&bv, 92, 7) as u8;
            if speed_raw > 120 {
                None
            } else {
                Some(speed_raw)
            }
        },

        wind_direction_1: {
            let direc_raw = pick_u64(&bv, 99, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        air_temperature_1: {
            let temp_raw = pick_i64(&bv, 108, 11) as i32;
            if temp_raw < -600 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        presure_1: {
            let presure_raw = pick_u64(&bv, 119, 9) as u16;
            if presure_raw > 400 {
                None
            } else {
                Some(presure_raw + 800)
            }
        },

        visibility_1: {
            let visibility_raw = pick_u64(&bv, 128, 8) as u16;
            if visibility_raw > 250 {
                None
            } else {
                Some((visibility_raw as f64) / 10.0)
            }
        },

        weather_2: { WeatherType::new(pick_u64(&bv, 136, 5) as u8) },
        longitude_2: {
            let lon_raw = pick_u64(&bv, 141, 7) as u32;
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
            let lat_raw = pick_u64(&bv, 148, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw2 = lat_raw1 + ((lat_raw as f64) / 60.0);
                Some(lat_raw2)
            }
        },

        wind_speed_2: {
            let speed_raw = pick_u64(&bv, 155, 7) as u8;
            if speed_raw > 120 {
                None
            } else {
                Some(speed_raw)
            }
        },

        wind_direction_2: {
            let direc_raw = pick_u64(&bv, 162, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        air_temperature_2: {
            let temp_raw = pick_i64(&bv, 171, 11) as i32;
            if temp_raw < -600 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        presure_2: {
            let presure_raw = pick_u64(&bv, 182, 9) as u16;
            if presure_raw > 400 {
                None
            } else {
                Some(presure_raw + 800)
            }
        },

        visibility_2: {
            let visibility_raw = pick_u64(&bv, 191, 8) as u16;
            if visibility_raw > 250 {
                None
            } else {
                Some((visibility_raw as f64) / 10.0)
            }
        },

        weather_3: { WeatherType::new(pick_u64(&bv, 199, 5) as u8) },
        longitude_3: {
            let lon_raw = pick_u64(&bv, 204, 7) as u32;
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
            let lat_raw = pick_u64(&bv, 211, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw3 = lat_raw2 + ((lat_raw as f64) / 60.0);
                Some(lat_raw3)
            }
        },

        wind_speed_3: {
            let speed_raw = pick_u64(&bv, 218, 7) as u8;
            if speed_raw > 120 {
                None
            } else {
                Some(speed_raw)
            }
        },

        wind_direction_3: {
            let direc_raw = pick_u64(&bv, 225, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        air_temperature_3: {
            let temp_raw = pick_i64(&bv, 234, 11) as i32;
            if temp_raw < -600 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        presure_3: {
            let presure_raw = pick_u64(&bv, 245, 9) as u16;
            if presure_raw > 400 {
                None
            } else {
                Some(presure_raw + 800)
            }
        },

        visibility_3: {
            let visibility_raw = pick_u64(&bv, 254, 8) as u16;
            if visibility_raw > 250 {
                None
            } else {
                Some((visibility_raw as f64) / 10.0)
            }
        },

        weather_4: { WeatherType::new(pick_u64(&bv, 262, 5) as u8) },
        longitude_4: {
            let lon_raw = pick_u64(&bv, 267, 7) as u32;
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
            let lat_raw = pick_u64(&bv, 274, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw4 = lat_raw3 + ((lat_raw as f64) / 60.0);
                Some(lat_raw4)
            }
        },

        wind_speed_4: {
            let speed_raw = pick_u64(&bv, 281, 7) as u8;
            if speed_raw > 120 {
                None
            } else {
                Some(speed_raw)
            }
        },

        wind_direction_4: {
            let direc_raw = pick_u64(&bv, 288, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        air_temperature_4: {
            let temp_raw = pick_i64(&bv, 297, 11) as i32;
            if temp_raw < -600 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        presure_4: {
            let presure_raw = pick_u64(&bv, 308, 9) as u16;
            if presure_raw > 400 {
                None
            } else {
                Some(presure_raw + 800)
            }
        },

        visibility_4: {
            let visibility_raw = pick_u64(&bv, 317, 8) as u16;
            if visibility_raw > 250 {
                None
            } else {
                Some((visibility_raw as f64) / 10.0)
            }
        },
        source: { pick_u64(&bv, 325, 3) as u8 },
    })
}

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
        forcast_time: { pick_u64(&bv, 56, 5) as u8 },
        longitude_1: {
            let lon_raw = pick_u64(&bv, 61, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                lon_raw1 = ((lon_raw as f64) / 60.0) + 60.0;
                Some(lon_raw1)
            }
        },

        latitude_1: {
            let lat_raw = pick_u64(&bv, 74, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                lat_raw1 = ((lat_raw as f64) / 60.0) - 50.0;
                Some(lat_raw1)
            }
        },

        flowing_speed_1: {
            let speed_raw = pick_u64(&bv, 87, 8) as u32;
            if speed_raw > 250 {
                None
            } else {
                Some((speed_raw as f64) / 10.0)
            }
        },

        flowing_direction_1: {
            let direc_raw = pick_u64(&bv, 95, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        wave_height_1: {
            let height_raw = pick_u64(&bv, 104, 8) as u32;
            if height_raw > 250 {
                None
            } else {
                Some((height_raw as f64) / 10.0)
            }
        },

        wave_direction_1: {
            let direc_raw = pick_u64(&bv, 112, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        sea_temperature_1: {
            let temp_raw = pick_i64(&bv, 121, 11) as i32;
            if temp_raw < -100 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        longitude_2: {
            let lon_raw = pick_u64(&bv, 132, 7) as u32;
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
            let lat_raw = pick_u64(&bv, 139, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw2 = lat_raw1 + ((lat_raw as f64) / 60.0);
                Some(lat_raw2)
            }
        },

        flowing_speed_2: {
            let speed_raw = pick_u64(&bv, 146, 8) as u32;
            if speed_raw > 250 {
                None
            } else {
                Some((speed_raw as f64) / 10.0)
            }
        },

        flowing_direction_2: {
            let direc_raw = pick_u64(&bv, 154, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        wave_height_2: {
            let height_raw = pick_u64(&bv, 163, 8) as u32;
            if height_raw > 250 {
                None
            } else {
                Some((height_raw as f64) / 10.0)
            }
        },

        wave_direction_2: {
            let direc_raw = pick_u64(&bv, 171, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        sea_temperature_2: {
            let temp_raw = pick_i64(&bv, 180, 11) as i32;
            if temp_raw < -100 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        longitude_3: {
            let lon_raw = pick_u64(&bv, 191, 7) as u32;
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
            let lat_raw = pick_u64(&bv, 198, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw3 = lat_raw2 + ((lat_raw as f64) / 60.0);
                Some(lat_raw3)
            }
        },

        flowing_speed_3: {
            let speed_raw = pick_u64(&bv, 205, 8) as u32;
            if speed_raw > 250 {
                None
            } else {
                Some((speed_raw as f64) / 10.0)
            }
        },

        flowing_direction_3: {
            let direc_raw = pick_u64(&bv, 213, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        wave_height_3: {
            let height_raw = pick_u64(&bv, 222, 8) as u32;
            if height_raw > 250 {
                None
            } else {
                Some((height_raw as f64) / 10.0)
            }
        },

        wave_direction_3: {
            let direc_raw = pick_u64(&bv, 230, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        sea_temperature_3: {
            let temp_raw = pick_i64(&bv, 239, 11) as i32;
            if temp_raw < -100 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },

        longitude_4: {
            let lon_raw = pick_u64(&bv, 250, 7) as u32;
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
            let lat_raw = pick_u64(&bv, 257, 7) as u32;
            if lat_raw > 59 || lat_raw < 1 {
                None
            } else {
                lat_raw4 = lat_raw3 + ((lat_raw as f64) / 60.0);
                Some(lat_raw4)
            }
        },

        flowing_speed_4: {
            let speed_raw = pick_u64(&bv, 264, 8) as u32;
            if speed_raw > 250 {
                None
            } else {
                Some((speed_raw as f64) / 10.0)
            }
        },

        flowing_direction_4: {
            let direc_raw = pick_u64(&bv, 272, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        wave_height_4: {
            let height_raw = pick_u64(&bv, 281, 8) as u32;
            if height_raw > 250 {
                None
            } else {
                Some((height_raw as f64) / 10.0)
            }
        },

        wave_direction_4: {
            let direc_raw = pick_u64(&bv, 289, 9) as u16;
            if direc_raw > 359 {
                None
            } else {
                Some(direc_raw)
            }
        },

        sea_temperature_4: {
            let temp_raw = pick_i64(&bv, 298, 11) as i32;
            if temp_raw < -100 || temp_raw > 600 {
                None
            } else {
                Some((temp_raw as f64) / 10.0)
            }
        },
        source: { pick_u64(&bv, 309, 3) as u8 },
    })
}

fn handle_hydrology(bv: &BitVec) -> ApplicationIdentifier {
    ApplicationIdentifier::Hydrology(Hydrology {
        forcast_time: { pick_u64(&bv, 56, 5) as u8 },
        longitude_1: {
            let lon_raw = pick_u64(&bv, 61, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                Some(((lon_raw as f64) / 60.0) + 60.0)
            }
        },

        latitude_1: {
            let lat_raw = pick_u64(&bv, 74, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                Some(((lat_raw as f64) / 60.0) - 50.0)
            }
        },

        high_tide_1: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 98, 5) as u32)
                .and_hms_opt(pick_u64(&bv, 93, 5) as u32, pick_u64(&bv, 87, 6) as u32, 0)
            {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        low_tide_1: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 114, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 109, 5) as u32,
                    pick_u64(&bv, 103, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        water_level_1: {
            let level_raw = pick_i64(&bv, 119, 9) as i32;
            if level_raw < -250 || level_raw > 250 {
                None
            } else {
                Some((level_raw as f64) / 10.0)
            }
        },

        longitude_2: {
            let lon_raw = pick_u64(&bv, 128, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                Some(((lon_raw as f64) / 60.0) + 60.0)
            }
        },

        latitude_2: {
            let lat_raw = pick_u64(&bv, 141, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                Some(((lat_raw as f64) / 60.0) - 50.0)
            }
        },

        high_tide_2: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 165, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 160, 5) as u32,
                    pick_u64(&bv, 154, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        low_tide_2: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 181, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 176, 5) as u32,
                    pick_u64(&bv, 170, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        water_level_2: {
            let level_raw = pick_i64(&bv, 186, 9) as i32;
            if level_raw < -250 || level_raw > 250 {
                None
            } else {
                Some((level_raw as f64) / 10.0)
            }
        },

        longitude_3: {
            let lon_raw = pick_u64(&bv, 195, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                Some(((lon_raw as f64) / 60.0) + 60.0)
            }
        },

        latitude_3: {
            let lat_raw = pick_u64(&bv, 208, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                Some(((lat_raw as f64) / 60.0) - 50.0)
            }
        },

        high_tide_3: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 232, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 227, 5) as u32,
                    pick_u64(&bv, 221, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        low_tide_3: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 248, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 243, 5) as u32,
                    pick_u64(&bv, 237, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        water_level_3: {
            let level_raw = pick_i64(&bv, 253, 9) as i32;
            if level_raw < -250 || level_raw > 250 {
                None
            } else {
                Some((level_raw as f64) / 10.0)
            }
        },

        longitude_4: {
            let lon_raw = pick_u64(&bv, 262, 13) as u32;
            if lon_raw > 7200 {
                None
            } else {
                Some(((lon_raw as f64) / 60.0) + 60.0)
            }
        },

        latitude_4: {
            let lat_raw = pick_u64(&bv, 275, 13) as u32;
            if lat_raw > 7200 {
                None
            } else {
                Some(((lat_raw as f64) / 60.0) - 50.0)
            }
        },

        high_tide_4: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 299, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 294, 5) as u32,
                    pick_u64(&bv, 288, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        low_tide_4: {
            let now = Utc::now();
            match Utc
                .ymd_opt(now.year(), now.month(), pick_u64(&bv, 315, 5) as u32)
                .and_hms_opt(
                    pick_u64(&bv, 310, 5) as u32,
                    pick_u64(&bv, 304, 6) as u32,
                    0,
                ) {
                LocalResult::Single(res) => Some(res),
                _ => None,
            }
        },

        water_level_4: {
            let level_raw = pick_i64(&bv, 320, 9) as i32;
            if level_raw < -250 || level_raw > 250 {
                None
            } else {
                Some((level_raw as f64) / 10.0)
            }
        },

        source: { pick_u64(&bv, 329, 3) as u8 },
    })
}
