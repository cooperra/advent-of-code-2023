use core::ops::Range;
use regex_macro::regex;
use std::cmp::{max, min};

type Num = u64;
type Seeds = Vec<Num>;

fn seeds_from_line(line: &str) -> Seeds {
    let re = regex!(r"[0-9]+");
    re.find_iter(&line)
        .map(|m| m.as_str().parse::<Num>().unwrap())
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Almanac {
    pub seeds: Seeds,
    pub maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn new(seeds: Seeds, maps: Vec<AlmanacMap>) -> Self {
        Almanac { seeds, maps }
    }

    pub fn parse(mut lines: &mut impl Iterator<Item = impl AsRef<str>>) -> Self {
        let seeds: Seeds = seeds_from_line(lines.next().unwrap().as_ref());
        lines.next();
        let mut maps = Vec::<AlmanacMap>::new();
        while let Ok(Some(map)) = AlmanacMap::parse(&mut lines) {
            maps.push(map);
        }
        Self::new(seeds, maps)
    }

    pub fn seed_ranges(self: &Self) -> Vec<Range<Num>> {
        self.seeds
            .chunks_exact(2)
            .map(|slice: &[u64]| {
                let start = slice[0];
                let end = start + slice[1];
                start..end
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AlmanacMap {
    entries: Vec<AlmanacMapEntry>,
}

impl AlmanacMap {
    fn new(entries: Vec<AlmanacMapEntry>) -> Self {
        Self { entries }
    }

    fn parse<S>(lines: &mut impl Iterator<Item = S>) -> Result<Option<Self>, &'static str>
    where
        S: AsRef<str>,
    {
        // Check the title line
        let title_line_result = lines.next().ok_or("No title line");
        if title_line_result.is_err() {
            return Ok(None);
        }
        let title_line = title_line_result.unwrap();
        let title_re = regex!(r"^([A-z])+-to-([A-z]+) map:$");
        title_re
            .find(title_line.as_ref())
            .ok_or("Title line invalid")?;
        let mut entries: Vec<AlmanacMapEntry> = Vec::new();
        for line in lines {
            if line.as_ref() == "" {
                break;
            }
            entries.push(AlmanacMapEntry::from_line(line.as_ref()));
        }
        Ok(Some(Self::new(entries)))
    }

    pub fn map(self: &Self, num: Num) -> Num {
        self.entries
            .iter()
            .find_map(|entry| entry.map(num))
            .unwrap_or(num)
    }

    pub fn map_ranges(self: &Self, ranges: Vec<Range<Num>>) -> Vec<Range<Num>> {
        let partitioned_ranges: Vec<Range<Num>> = self.entries.iter().fold(
            ranges,
            |ranges: Vec<Range<Num>>, entry: &AlmanacMapEntry| -> Vec<Range<Num>> {
                entry.partition_ranges(ranges)
            },
        );
        for range in partitioned_ranges.iter() {
            assert!(range.start < range.end);
        }
        let mapped_ranges: Vec<_> = partitioned_ranges
            .into_iter()
            .map(|r: Range<Num>| -> Range<Num> { self.map(r.start)..(self.map(r.end - 1) + 1) })
            .collect();
        println!("{:#?}", &self);
        for range in mapped_ranges.iter() {
            assert!(range.start < range.end);
        }
        return mapped_ranges;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AlmanacMapEntry {
    dest_start: Num,
    source_start: Num,
    range_len: Num,
}

impl AlmanacMapEntry {
    pub fn new(dest_start: Num, source_start: Num, range_len: Num) -> Self {
        AlmanacMapEntry {
            dest_start,
            source_start,
            range_len,
        }
    }

    pub fn from_line(line: &str) -> Self {
        let re = regex!(r"([0-9]+) +([0-9]+) +([0-9]+)");
        let [dest_start, source_start, range_len] = re
            .captures(&line)
            .expect("Map parse failed.")
            .extract()
            .1
            .map(|s: &str| s.parse::<Num>().unwrap());
        Self::new(dest_start, source_start, range_len)
    }

    pub fn map(self: &Self, num: Num) -> Option<Num> {
        if self.source_range().contains(&num) {
            let delta = (self.dest_start as i64) - (self.source_start as i64);
            return Some((num as i64 + delta) as Num);
        } else {
            return None;
        }
    }

    fn source_range(self: &Self) -> Range<Num> {
        self.source_start..(self.source_start + self.range_len)
    }

    fn partition_range(self: &Self, range: Range<Num>) -> Vec<Range<Num>> {
        // Handle zero-length ranges
        if range.start == range.end {
            return Vec::new();
        }
        if self.range_len == 0 {
            return vec![range];
        }
        let source_range = self.source_range();
        if let Range { end: 0, .. } = source_range {
            println!("source_range {:#?}", source_range);
            panic!();
        }
        if range.start >= range.end {
            println!("{:#?}", range);
        }
        assert!(range.start < range.end);
        let mut range_below = None;
        let mut range_inside = None;
        let mut range_above = None;
        println!("range {:#?} source_range {:#?}", range, source_range);
        if range.start < source_range.start {
            range_below = Some(range.start..min(range.end, self.source_start))
        }
        if source_range.contains(&range.start)
            || source_range.contains(&(range.end - 1))
            || range.contains(&source_range.start) && range.contains(&(source_range.end - 1))
        {
            range_inside =
                Some(max(range.start, source_range.start)..min(range.end, source_range.end));
        }
        if source_range.end < range.end {
            range_above = Some(max(range.start, source_range.end)..range.end);
        }

        if let Some(Range { end: 0, .. }) = range_below {
            println!("below: {:#?}", range_below);
            panic!();
        }
        if let Some(Range { end: 0, .. }) = range_inside {
            println!("inside: {:#?}", range_inside);
            panic!();
        }
        if let Some(Range { end: 0, .. }) = range_above {
            println!("above: {:#?}", range_above);
            panic!();
        }

        let out: Vec<_> = [range_below, range_inside, range_above]
            .into_iter()
            .flatten()
            .collect();
        println!("{:#?}", &out);
        &out.iter().map(|r| assert!(r.start < r.end));
        out
    }

    fn partition_ranges(self: &Self, ranges: Vec<Range<Num>>) -> Vec<Range<Num>> {
        ranges
            .into_iter()
            .flat_map(|range: Range<Num>| {
                let start = range.start;
                let rs = self.partition_range(range);
                rs.iter().fold(start, |pend, r| {
                    assert_eq!(pend, r.start);
                    r.end
                });
                return rs;
            })
            .collect()
    }
}

pub fn get_example() -> Vec<&'static str> {
    let example = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];
    return example;
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_parsing_seeds() {
        let line: &str = "seeds: 79 14 55 13";
        let actual = seeds_from_line(&line);
        let expected: Seeds = vec![79, 14, 55, 13];
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn test_parsing_almanac_map() {
        let mut lines = get_example().into_iter().skip(2).take(4);
        let actual = AlmanacMap::parse(&mut lines);
        let expected = Ok(Some(AlmanacMap::new(vec![
            AlmanacMapEntry::new(50, 98, 2),
            AlmanacMapEntry::new(52, 50, 48),
        ])));
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn test_almanac_map() {
        let map = AlmanacMap::new(vec![
            AlmanacMapEntry::new(50, 98, 2),
            AlmanacMapEntry::new(52, 50, 48),
        ]);
        let inputs: [Num; 5] = [79, 14, 55, 13, 99];
        let expected: [Num; 5] = [81, 14, 57, 13, 51];
        let actual = inputs.map(|v| map.map(v));
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn test_almanac_entry_map() {
        let entry = AlmanacMapEntry::new(52, 50, 48);
        let inputs: [Num; 4] = [79, 14, 55, 13];
        let expected: [Option<Num>; 4] = [Some(81), None, Some(57), None];
        let actual = inputs.map(|v| entry.map(v));
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn test_almanac_seed_ranges() {
        let almanac = Almanac::new(vec![10, 5], Vec::new());
        let actual = almanac.seed_ranges();
        let expected = vec![10..15];
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn test_almanac_entry_partition_ranges() {
        let entry = AlmanacMapEntry::new(52, 50, 48);
        let input: Vec<Range<Num>> = vec![0..45, 45..55, 55..90, 90..100, 100..110];
        let expected: Vec<Range<Num>> =
            vec![0..45, 45..50, 50..55, 55..90, 90..98, 98..100, 100..110];
        let actual = entry.partition_ranges(input);
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn test_almanac_entry_partition_ranges_wide() {
        let entry = AlmanacMapEntry::new(52, 50, 48);
        let input: Vec<Range<Num>> = vec![0..110];
        let expected: Vec<Range<Num>> = vec![0..50, 50..98, 98..110];
        let actual = entry.partition_ranges(input);
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn test_almanac_entry_partition_ranges_edge() {
        let entry = AlmanacMapEntry::new(52, 50, 48);
        let input: Vec<Range<Num>> = vec![99..99];
        let expected: Vec<Range<Num>> = vec![];
        let actual = entry.partition_ranges(input);
        assert_eq!(&expected, &actual);
    }
}
