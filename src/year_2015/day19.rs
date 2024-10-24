use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    let part1 = solve1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 1061);

    let part2 = solve2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 1006);
}

fn replacements<'a>(
    molecule: &'a str,
    from: &'a str,
    to: &'a str,
) -> impl Iterator<Item = String> + 'a {
    molecule.match_indices(from).map(move |(i, _)| {
        let mut s = String::new();

        s.push_str(&molecule[..i]);
        s.push_str(to);
        s.push_str(&molecule[(i + from.len())..]);

        s
    })
}

fn solve1(input: &Vec<String>) -> i64 {
    let mut replacements = vec![];
    let mut medicine_molecule = String::new();
    for line in input {
        if let Some((a, b)) = line.split_once(" => ") {
            replacements.push((a.to_string(), b.to_owned()));
        } else if !line.is_empty() {
            medicine_molecule = line.to_owned();
        }
    }
    let mut molecules = HashSet::new();
    for (from, to) in replacements {
        for s in self::replacements(&*medicine_molecule, &*from, &*to) {
            molecules.insert(s);
        }
    }
    molecules.len() as i64
}
fn solve2(input: &Vec<String>) -> i64 {
    let steps: i64 = 207;

    // from random import shuffle
    //
    // reps = [('Al','ThF'),('Al','ThRnFAr'),('B','BCa'),('B','TiB'),('B','TiRnFAr'),('Ca','CaCa'),('Ca','PB'),('Ca','PRnFAr'),('Ca','SiRnFYFAr'),('Ca','SiRnMgAr'),('Ca','SiTh'),('F','CaF'),('F','PMg'),('F','SiAl'),('H','CRnAlAr'),('H','CRnFYFYFAr'),('H','CRnFYMgAr'),('H','CRnMgYFAr'),('H','HCa'),('H','NRnFYFAr'),('H','NRnMgAr'),('H','NTh'),('H','OB'),('H','ORnFAr'),('Mg','BF'),('Mg','TiMg'),('N','CRnFAr'),('N','HSi'),('O','CRnFYFAr'),('O','CRnMgAr'),('O','HP'),('O','NRnFAr'),('O','OTi'),('P','CaP'),('P','PTi'),('P','SiRnFAr'),('Si','CaSi'),('Th','ThCa'),('Ti','BP'),('Ti','TiTi'),('e','HF'),('e','NAl'),('e','OMg')]
    // mol = "ORnPBPMgArCaCaCaSiThCaCaSiThCaCaPBSiRnFArRnFArCaCaSiThCaCaSiThCaCaCaCaCaCaSiRnFYFArSiRnMgArCaSiRnPTiTiBFYPBFArSiRnCaSiRnTiRnFArSiAlArPTiBPTiRnCaSiAlArCaPTiTiBPMgYFArPTiRnFArSiRnCaCaFArRnCaFArCaSiRnSiRnMgArFYCaSiRnMgArCaCaSiThPRnFArPBCaSiRnMgArCaCaSiThCaSiRnTiMgArFArSiThSiThCaCaSiRnMgArCaCaSiRnFArTiBPTiRnCaSiAlArCaPTiRnFArPBPBCaCaSiThCaPBSiThPRnFArSiThCaSiThCaSiThCaPTiBSiRnFYFArCaCaPRnFArPBCaCaPBSiRnTiRnFArCaPRnFArSiRnCaCaCaSiThCaRnCaFArYCaSiRnFArBCaCaCaSiThFArPBFArCaSiRnFArRnCaCaCaFArSiRnFArTiRnPMgArF"
    //
    // target = mol
    // part2 = 0
    //
    // while target != 'e':
    //     tmp = target
    //     for a, b in reps:
    //         if b not in target:
    //             continue
    //
    //         target = target.replace(b, a, 1)
    //         part2 += 1
    //
    //     if tmp == target:
    //         target = mol
    //         part2 = 0
    //         shuffle(reps)
    //
    // print part2

    steps
}
