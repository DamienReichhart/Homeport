//! The lines a search found, in order, in blocks: what a task found in a
//! slice is taken as it came, whole, so a search that matches every line
//! of a million never copies what it already holds to make room for
//! more. A line is found by its number or by its place among them, and
//! the oldest are let go of as the buffer drops them.

use std::collections::VecDeque;

use super::Hit;

/// The most lines of a block that grows a few at a time.
const BLOCK: usize = 65_536;

#[derive(Debug, Default)]
struct Block {
    hits: Vec<Hit>,
    /// How many matches its lines hold, those let go of left out.
    matches: u64,
}

/// The lines that hold a match, among those read so far.
#[derive(Debug, Default)]
pub struct Hits {
    blocks: VecDeque<Block>,
    /// How many lines of the first block were let go of.
    skipped: usize,
    len: usize,
    /// How many matches they hold together.
    matches: u64,
}

fn count(hits: &[Hit]) -> u64 {
    hits.iter().map(|hit| u64::from(hit.count)).sum()
}

impl Hits {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// How many matches the lines hold together.
    pub fn matches(&self) -> u64 {
        self.matches
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Adds lines that follow those held. A few are added to the last
    /// block; many are a block of their own, taken as they are.
    pub fn push(&mut self, hits: Vec<Hit>) {
        if hits.is_empty() {
            return;
        }
        debug_assert!(self.back().is_none_or(|last| last.seq < hits[0].seq));
        let matches = count(&hits);
        self.len += hits.len();
        self.matches += matches;
        match self.blocks.back_mut() {
            Some(last) if hits.len() < BLOCK / 4 && last.hits.len() + hits.len() <= BLOCK => {
                last.hits.extend(hits);
                last.matches += matches;
            }
            _ => self.blocks.push_back(Block { hits, matches }),
        }
    }

    /// Lets go of the lines before `first`. Returns how many matches
    /// they held.
    pub fn evict(&mut self, first: u64) -> u64 {
        let mut gone = 0;
        while let Some(block) = self.blocks.front_mut() {
            let kept = &block.hits[self.skipped..];
            if kept.last().is_some_and(|last| last.seq < first) {
                gone += block.matches;
                self.len -= kept.len();
                self.skipped = 0;
                self.blocks.pop_front();
                continue;
            }
            let passed = kept.partition_point(|hit| hit.seq < first);
            let held = count(&kept[..passed]);
            block.matches -= held;
            gone += held;
            self.len -= passed;
            self.skipped += passed;
            break;
        }
        self.matches -= gone;
        gone
    }

    /// The block the line at `index` is in, and its place there.
    fn place(&self, index: usize) -> Option<(usize, usize)> {
        if index >= self.len {
            return None;
        }
        let mut at = index + self.skipped;
        for (block, held) in self.blocks.iter().enumerate() {
            if at < held.hits.len() {
                return Some((block, at));
            }
            at -= held.hits.len();
        }
        None
    }

    /// The line at `index` among them.
    pub fn get(&self, index: usize) -> Option<Hit> {
        let (block, at) = self.place(index)?;
        Some(self.blocks[block].hits[at])
    }

    pub fn front(&self) -> Option<Hit> {
        self.get(0)
    }

    pub fn back(&self) -> Option<Hit> {
        self.blocks
            .back()
            .and_then(|block| block.hits.last().copied())
    }

    /// Where the line `seq` is among them, or where it would be.
    pub fn position(&self, seq: u64) -> Result<usize, usize> {
        let mut before = 0;
        for (index, block) in self.blocks.iter().enumerate() {
            let skipped = if index == 0 { self.skipped } else { 0 };
            let kept = &block.hits[skipped..];
            if kept.last().is_some_and(|last| last.seq < seq) {
                before += kept.len();
                continue;
            }
            return match kept.binary_search_by_key(&seq, |hit| hit.seq) {
                Ok(at) => Ok(before + at),
                Err(at) => Err(before + at),
            };
        }
        Err(before)
    }

    /// How many matches the lines before the one at `index` hold.
    pub fn matches_before(&self, index: usize) -> u64 {
        let Some((block, at)) = self.place(index) else {
            return self.matches;
        };
        let whole: u64 = self.blocks.iter().take(block).map(|b| b.matches).sum();
        let skipped = if block == 0 { self.skipped } else { 0 };
        whole + count(&self.blocks[block].hits[skipped..at])
    }

    /// The lines from the one at `index` on.
    pub fn from(&self, index: usize) -> impl Iterator<Item = Hit> + '_ {
        let (first, at) = self.place(index).unwrap_or((self.blocks.len(), 0));
        self.blocks
            .iter()
            .enumerate()
            .skip(first)
            .flat_map(move |(block, held)| {
                let start = if block == first { at } else { 0 };
                held.hits[start..].iter().copied()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hits(seqs: impl IntoIterator<Item = u64>) -> Vec<Hit> {
        seqs.into_iter().map(|seq| Hit { seq, count: 2 }).collect()
    }

    #[test]
    fn lines_are_found_by_number_and_by_place_across_blocks() {
        let mut held = Hits::default();
        // A few at a time, a slice whole, then a few again: three blocks.
        held.push(hits((0..10).map(|n| n * 2)));
        held.push(hits((0..40_000).map(|n| 100 + n * 2)));
        held.push(hits([90_000, 90_002]));
        held.push(hits([90_004]));
        assert_eq!(held.len(), 40_013);
        assert_eq!(held.matches(), 80_026);
        assert_eq!(held.blocks.len(), 2, "the few joined the last block");
        assert_eq!(held.get(0).map(|hit| hit.seq), Some(0));
        assert_eq!(held.get(10).map(|hit| hit.seq), Some(100));
        assert_eq!(held.get(40_012).map(|hit| hit.seq), Some(90_004));
        assert_eq!(held.get(40_013), None);
        assert_eq!(held.back().map(|hit| hit.seq), Some(90_004));
        assert_eq!(held.position(18), Ok(9));
        assert_eq!(held.position(19), Err(10));
        assert_eq!(held.position(104), Ok(12));
        assert_eq!(held.position(95_000), Err(40_013));
        assert_eq!(held.matches_before(12), 24);
        assert_eq!(held.matches_before(40_013), 80_026);
        let tail: Vec<u64> = held.from(40_009).map(|hit| hit.seq).collect();
        assert_eq!(tail, [80_098, 90_000, 90_002, 90_004]);
        assert_eq!(held.from(99_999).count(), 0);
    }

    #[test]
    fn the_oldest_are_let_go_of_within_a_block_and_whole_blocks_at_a_time() {
        let mut held = Hits::default();
        held.push(hits(0..20_000));
        held.push(hits(20_000..40_000));
        assert_eq!(held.evict(5), 10);
        assert_eq!((held.len(), held.matches()), (39_995, 79_990));
        assert_eq!(held.front().map(|hit| hit.seq), Some(5));
        assert_eq!(held.position(5), Ok(0));
        assert_eq!(held.position(3), Err(0));
        assert_eq!(held.matches_before(10), 20);
        // Past the first block and into the second.
        assert_eq!(held.evict(20_010), 2 * 20_005);
        assert_eq!(held.len(), 19_990);
        assert_eq!(held.get(0).map(|hit| hit.seq), Some(20_010));
        assert_eq!(held.position(20_011), Ok(1));
        assert_eq!(held.evict(20_010), 0);
        assert_eq!(held.evict(u64::MAX), 2 * 19_990);
        assert!(held.is_empty() && held.matches() == 0 && held.front().is_none());
        held.push(hits([50_000]));
        assert_eq!(held.len(), 1);
    }
}
