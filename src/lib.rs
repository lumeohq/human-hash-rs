//! Generate human readable digests for UUIDs
//!
//! Based on https://github.com/zacharyvoase/humanhash

extern crate uuid;

use uuid::Uuid;

const DEFAULT_WORDLIST: &'static [ &'static str ] = &[
    "ack", "alabama", "alanine", "alaska", "alpha", "angel", "apart", "april",
    "arizona", "arkansas", "artist", "asparagus", "aspen", "august", "autumn",
    "avocado", "bacon", "bakerloo", "batman", "beer", "berlin", "beryllium",
    "black", "blossom", "blue", "bluebird", "bravo", "bulldog", "burger",
    "butter", "california", "carbon", "cardinal", "carolina", "carpet", "cat",
    "ceiling", "charlie", "chicken", "coffee", "cola", "cold", "colorado",
    "comet", "connecticut", "crazy", "cup", "dakota", "december", "delaware",
    "delta", "diet", "don", "double", "early", "earth", "east", "echo",
    "edward", "eight", "eighteen", "eleven", "emma", "enemy", "equal",
    "failed", "fanta", "fifteen", "fillet", "finch", "fish", "five", "fix",
    "floor", "florida", "football", "four", "fourteen", "foxtrot", "freddie",
    "friend", "fruit", "gee", "georgia", "glucose", "golf", "green", "grey",
    "hamper", "happy", "harry", "hawaii", "helium", "high", "hot", "hotel",
    "hydrogen", "idaho", "illinois", "india", "indigo", "ink", "iowa",
    "island", "item", "jersey", "jig", "johnny", "juliet", "july", "jupiter",
    "kansas", "kentucky", "kilo", "king", "kitten", "lactose", "lake", "lamp",
    "lemon", "leopard", "lima", "lion", "lithium", "london", "louisiana",
    "low", "magazine", "magnesium", "maine", "mango", "march", "mars",
    "maryland", "massachusetts", "may", "mexico", "michigan", "mike",
    "minnesota", "mirror", "mississippi", "missouri", "mobile", "mockingbird",
    "monkey", "montana", "moon", "mountain", "muppet", "music", "nebraska",
    "neptune", "network", "nevada", "nine", "nineteen", "nitrogen", "north",
    "november", "nuts", "october", "ohio", "oklahoma", "one", "orange",
    "oranges", "oregon", "oscar", "oven", "oxygen", "papa", "paris", "pasta",
    "pennsylvania", "pip", "pizza", "pluto", "potato", "princess", "purple",
    "quebec", "queen", "quiet", "red", "river", "robert", "robin", "romeo",
    "rugby", "sad", "salami", "saturn", "september", "seven", "seventeen",
    "shade", "sierra", "single", "sink", "six", "sixteen", "skylark", "snake",
    "social", "sodium", "solar", "south", "spaghetti", "speaker", "spring",
    "stairway", "steak", "stream", "summer", "sweet", "table", "tango", "ten",
    "tennessee", "tennis", "texas", "thirteen", "three", "timing", "triple",
    "twelve", "twenty", "two", "uncle", "undress", "uniform", "uranus", "utah",
    "vegan", "venus", "vermont", "victor", "video", "violet", "virginia",
    "washington", "west", "whiskey", "white", "william", "winner", "winter",
    "wisconsin", "wolfram", "wyoming", "xray", "yankee", "yellow", "zebra",
    "zulu" ];

/// TODO: Not very dynamic
fn compress(bytes: &[u8], target: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();

    let seg_size = bytes.len() / target;

    for i in 0..target {
        let mut acc = 0u8;
        for j in (i*seg_size)..((i+1)*seg_size) {
            acc ^= bytes[j as usize];
        }
        ret.push(acc)
    }

    ret
}

pub fn digest_four(uuid: &Uuid) -> String {
    compress(uuid.as_bytes(), 4)
        .iter()
        .map(|&x| DEFAULT_WORDLIST[x as usize].to_string())
        .collect::<Vec<String>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::uuid::Uuid;
    use super::DEFAULT_WORDLIST;
    use super::digest_four;

    const TEST_UUID: &'static str = "bc0f47f93dd046578d7eee645999b95e";

    #[test]
    fn it_works() {
        let tuid = Uuid::parse_str(TEST_UUID).unwrap();

        let x = digest_four(&tuid);

        assert_eq!(x, "august-yankee-lima-coffee");
    }
}