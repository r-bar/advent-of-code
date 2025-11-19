use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SEED_REGEX: Regex = Regex::new(r"^seeds: +(.*)").unwrap();
    static ref MAP_REGEX: Regex = Regex::new(r"^((?<source>\w+)-to-(?<dest>\w+)) map:$").unwrap();
    static ref ENTRY_REGEX: Regex =
        Regex::new(r"^(?<dest>\d+) +(?<source>\d+) +(?<length>\d+)$").unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RangeMapEntry {
    source: usize,
    dest: usize,
    length: usize,
}

impl PartialOrd for RangeMapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.source.cmp(&other.source))
    }
}

impl Ord for RangeMapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source.cmp(&other.source)
    }
}

#[derive(Debug, Clone, Default)]
pub struct RangeMap {
    list: Vec<RangeMapEntry>,
}

impl RangeMap {
    pub fn new() -> Self {
        RangeMap { list: Vec::new() }
    }

    pub fn push(&mut self, source: usize, dest: usize, length: usize) {
        let entry = RangeMapEntry {
            source,
            dest,
            length,
        };
        let index = self.list.partition_point(|x| x.source < source);
        self.list.insert(index, entry);
    }

    #[cfg(test)]
    fn get_entry(&self, i: usize) -> Option<&RangeMapEntry> {
        Some(&self.list[i as usize])
    }

    pub fn get(&self, i: usize) -> usize {
        let index = self
            .list
            .binary_search_by(|entry| {
                if entry.source <= i && i < entry.source + entry.length {
                    std::cmp::Ordering::Equal
                } else if entry.source > i {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
        match index {
            Err(_) => i,
            Ok(index) => {
                let entry = self.list[index];
                let delta = i - entry.source;
                entry.dest + delta
            }
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct Almanac {
    pub seeds: Vec<usize>,
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum AlmanacMap {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl FromStr for AlmanacMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed-to-soil" => Ok(AlmanacMap::SeedToSoil),
            "soil-to-fertilizer" => Ok(AlmanacMap::SoilToFertilizer),
            "fertilizer-to-water" => Ok(AlmanacMap::FertilizerToWater),
            "water-to-light" => Ok(AlmanacMap::WaterToLight),
            "light-to-temperature" => Ok(AlmanacMap::LightToTemperature),
            "temperature-to-humidity" => Ok(AlmanacMap::TemperatureToHumidity),
            "humidity-to-location" => Ok(AlmanacMap::HumidityToLocation),
            _ => Err(anyhow::anyhow!("Invalid map name")),
        }
    }
}

impl Almanac {
    fn parse_seeds(line: &str) -> anyhow::Result<Vec<usize>> {
        let captures = SEED_REGEX
            .captures(line)
            .ok_or_else(|| anyhow::anyhow!("Invalid seeds line"))?;
        let seeds_str = captures.get(1).unwrap().as_str();
        let seeds = seeds_str
            .split_whitespace()
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        Ok(seeds)
    }

    fn parse_map(line: &str) -> anyhow::Result<AlmanacMap> {
        let captures = MAP_REGEX
            .captures(line)
            .ok_or_else(|| anyhow::anyhow!("Invalid map line"))?;
        let name = captures.get(1).unwrap().as_str();
        name.parse()
    }

    fn parse_entry(line: &str) -> anyhow::Result<(usize, usize, usize)> {
        let captures = ENTRY_REGEX
            .captures(line)
            .ok_or_else(|| anyhow::anyhow!("Invalid entry line"))?;
        let dest = captures.name("dest").unwrap().as_str().parse::<usize>()?;
        let source = captures.name("source").unwrap().as_str().parse::<usize>()?;
        let length = captures.name("length").unwrap().as_str().parse::<usize>()?;
        Ok((dest, source, length))
    }

    pub fn seed_to_location(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.get(seed);
        let fertilizer = self.soil_to_fertilizer.get(soil);
        let water = self.fertilizer_to_water.get(fertilizer);
        let light = self.water_to_light.get(water);
        let temperature = self.light_to_temperature.get(light);
        let humidity = self.temperature_to_humidity.get(temperature);
        self.humidity_to_location.get(humidity)
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut almanac = Almanac {
            seeds: Vec::new(),
            seed_to_soil: RangeMap::new(),
            soil_to_fertilizer: RangeMap::new(),
            fertilizer_to_water: RangeMap::new(),
            water_to_light: RangeMap::new(),
            light_to_temperature: RangeMap::new(),
            temperature_to_humidity: RangeMap::new(),
            humidity_to_location: RangeMap::new(),
        };
        let mut current_map = None;
        for line in s.lines() {
            if let Ok(seeds) = Almanac::parse_seeds(line) {
                almanac.seeds = seeds;
            }
            if let Ok(new_map_section) = Almanac::parse_map(line) {
                current_map = Some(new_map_section);
            }
            if let Ok((dest, source, length)) = Almanac::parse_entry(line) {
                match current_map {
                    Some(AlmanacMap::SeedToSoil) => almanac.seed_to_soil.push(source, dest, length),
                    Some(AlmanacMap::SoilToFertilizer) => {
                        almanac.soil_to_fertilizer.push(source, dest, length)
                    }
                    Some(AlmanacMap::FertilizerToWater) => {
                        almanac.fertilizer_to_water.push(source, dest, length)
                    }
                    Some(AlmanacMap::WaterToLight) => {
                        almanac.water_to_light.push(source, dest, length)
                    }
                    Some(AlmanacMap::LightToTemperature) => {
                        almanac.light_to_temperature.push(source, dest, length)
                    }
                    Some(AlmanacMap::TemperatureToHumidity) => {
                        almanac.temperature_to_humidity.push(source, dest, length)
                    }
                    Some(AlmanacMap::HumidityToLocation) => {
                        almanac.humidity_to_location.push(source, dest, length)
                    }
                    None => unreachable!(),
                }
            }
        }
        Ok(almanac)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_parse_seeds() {
        let line = "seeds: 79 14 55 13";
        let seeds = Almanac::parse_seeds(line).unwrap();
        assert_eq!(seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_parse_map() {
        let line = "soil-to-fertilizer map:";
        let map = Almanac::parse_map(line).unwrap();
        assert_eq!(map, AlmanacMap::SoilToFertilizer);
    }

    #[test]
    fn test_parse_entry() {
        let line = "50 98 2";
        let (dest, source, length) = Almanac::parse_entry(line).unwrap();
        assert_eq!(dest, 50);
        assert_eq!(source, 98);
        assert_eq!(length, 2);
    }

    #[test]
    fn test_parse_example() {
        let almanac = Almanac::from_str(EXAMPLE).unwrap();
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.seed_to_soil.len(), 2);
        assert_eq!(almanac.seed_to_soil.get_entry(0).unwrap().source, 50);
        assert_eq!(almanac.seed_to_soil.get_entry(0).unwrap().dest, 52);
        assert_eq!(almanac.seed_to_soil.get_entry(0).unwrap().length, 48);
        assert_eq!(almanac.soil_to_fertilizer.len(), 3);
        assert_eq!(almanac.fertilizer_to_water.len(), 4);
        assert_eq!(almanac.water_to_light.len(), 2);
        assert_eq!(almanac.light_to_temperature.len(), 3);
        assert_eq!(almanac.temperature_to_humidity.len(), 2);
        assert_eq!(almanac.humidity_to_location.len(), 2);
    }

    #[test]
    fn test_example_seed_to_location() {
        let almanac = Almanac::from_str(EXAMPLE).unwrap();
        assert_eq!(almanac.seed_to_location(79), 82);
        assert_eq!(almanac.seed_to_location(14), 43);
        assert_eq!(almanac.seed_to_location(55), 86);
        assert_eq!(almanac.seed_to_location(13), 35);
    }
}
