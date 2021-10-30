//! Utilities for working with ranks, represented as `(Tier, Division)` tuples.

use std::iter::Peekable;

use strum::IntoEnumIterator;

use super::{ Tier, Division };

/// (Tier, Division) tuple representing a rank.
pub type Rank = ( Tier, Division );

/// Iterator for iterating `(Tier, Division)` rank tuples.
pub struct Iter {
    tier_iter: Peekable<<Tier as IntoEnumIterator>::Iterator>,
    div_iter: <Division as IntoEnumIterator>::Iterator,
}

impl Iterator for Iter {
    type Item = Rank;
    fn next(&mut self) -> Option<Self::Item> {
        // First find the tier (innermost loop).
        // If none found, we go to next tier (in unwrap_or_else case).
        let div = self.div_iter.next()
            .unwrap_or_else(|| {
                // If no divisions available, go to next tier, reset the divisions, and return I.
                self.tier_iter.next();
                self.div_iter = Division::iter();
                self.div_iter.next().unwrap()
            });

        // Then find the tier.
        let tier = *self.tier_iter.peek()?;
        // If its an apex tier go to next tier and reset the divisions.
        if tier.is_apex() {
            self.tier_iter.next();
            self.div_iter = Division::iter();
        }

        Some(( tier, div ))
    }
}

/// Returns an iterator over all `(Tier, Division)` pairs, ordered from highest rank to lowest rank.
///
/// Apex tiers are all division I, for example: `(Tier::CHALLENGER, Division::I)`.
/// This matches how they are represented by Riot. There is no "Challenger II", etc.
pub fn iter() -> Iter {
    Iter {
        tier_iter: Tier::iter().peekable(),
        div_iter: Division::iter(),
    }
}

/// Returns an iterator over all `(Tier, Division)` pairs, excluding apex (Master+) tiers,
/// ordered from highest (Diamond I) to lowest (Iron IV).
pub fn non_apex_iter() -> Iter {
    let mut tier_iter = Tier::iter().peekable();
    while tier_iter.peek().unwrap().is_apex() {
        tier_iter.next();
    }
    Iter {
        tier_iter,
        div_iter: Division::iter(),
    }
}

#[cfg(test)]
mod tests {
    use super::{ Tier, Division };

    #[test]
    fn iter() {
        let mut it = super::iter();
        assert_eq!(Some(( Tier::CHALLENGER,  Division::I )),  it.next());
        assert_eq!(Some(( Tier::GRANDMASTER, Division::I )),  it.next());
        assert_eq!(Some(( Tier::MASTER,      Division::I )),  it.next());
        assert_eq!(Some(( Tier::DIAMOND,     Division::I )),  it.next());
        assert_eq!(Some(( Tier::DIAMOND,     Division::II )), it.next());
        let mut last = None;
        for next in &mut it {
            last = Some(next);
        }
        assert_eq!(Some(( Tier::IRON, Division::IV )), last);
        assert_eq!(None, it.next());
    }

    
    #[test]
    fn non_apex_iter() {
        let mut it = super::non_apex_iter();
        assert_eq!(Some((Tier::DIAMOND,  Division::I)),   it.next());
        assert_eq!(Some((Tier::DIAMOND,  Division::II)),  it.next());
        assert_eq!(Some((Tier::DIAMOND,  Division::III)), it.next());
        assert_eq!(Some((Tier::DIAMOND,  Division::IV)),  it.next());
        assert_eq!(Some((Tier::PLATINUM, Division::I)),   it.next());
        let mut last = None;
        for next in &mut it {
            last = Some(next);
        }
        assert_eq!(Some((Tier::IRON, Division::IV)), last);
        assert_eq!(None, it.next());
    }
}