use std::collections::HashSet;

/// https://adventofcode.com/2020/day/1

/// find the two entries that sum to 2020 and then multiply those two numbers together.
fn part1(expense: &[i32], sum: &i32) -> Option<i32> { 
    let set: HashSet<&i32> = expense.into_iter().collect();
    for e in expense {
        let companion = sum - e;
        if set.contains(&companion) {
            return Some(e * companion);
        }
    }
    None
}

/// find the three entries that sum to 2020 and then multiply those two numbers together.
fn part2(expense: &[i32], sum: &i32) -> Option<i32> {
    for e in expense {
        let remainder = sum - e;
        if let Some(s) = part1(expense, &remainder) {
            return Some(s * *e);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    const TEST: &'static [i32] = &[1721, 979, 366, 299, 675, 1456];
    const TEST1: &'static [i32] = &[1150,1579,1361,1319,1201,1253,1806,1783,1164,1772,1920,1428,1918,
    245,1504,1952,1057,1977,704,1119,1971,1200,1650,1795,1877,1932,1811,1981,1803,1366,1580,1986,1976,
    1063,1895,1143,1991,1061,1855,1947,1134,1800,1898,1778,1964,1949,1103,1770,1321,2005,1758,1181,
    1140,1873,1946,1540,1909,1710,1705,1313,1196,1084,1870,1610,1708,1810,1133,1375,1264,1921,1624,
    41,1899,1226,1757,1978,1485,1385,1526,1653,1130,1223,1577,1912,1894,276,954,1269,1769,1924,93,
    1165,1812,1092,1402,1284,1903,1884,1581,1887,1963,1983,1233,1445,1974,1956,1691,1954,2000,1469,
    1875,955,1334,1116,1700,1818,1790,1704,1901,1072,1848,1990,1724,1719,1638,1311,1474,1837,1801,
    1929,1791,1317,1643,1632,1813,1488,1129,1998,1771,1793,1074,1826,1935,1462,1230,1797,1878,1751,
    1993,1437,1967,1844,1438,1969,1175,1823,1124,1922,154,936,1117,1145,1308,1320,1767,1850,1809,
    1350,1820,1082,1597,1913,1766,1701,1294,1556,2006,1480,1953,1104,1861,1966,1248,1671,1955,1863,
    1202,1356,1842,2010,1288,1067,1576,1295,1760,1888,1639,1282,1633,1619];

    #[test]
    fn t() {
        assert_eq!(super::part1(&TEST, &2020).unwrap(), 514579);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&TEST1, &2020).unwrap(), 1014624);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&TEST1, &2020).unwrap(), 80072256);
    }
}
