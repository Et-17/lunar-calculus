mod tree;

use crate::tokenization::{Token, TokenTypes};
pub use tree::*;

pub fn group_parenthesis(segment: &[Token]) -> Option<ParenthesisGrouping> {
    let mut tokens: Vec<ParenthesisGrouping> = Vec::new();
    for i in segment.to_owned() {
        tokens.push(ParenthesisGrouping::Token(i));
    }
    return Some(rec_par_group(&ParenthesisGrouping::Grouping(tokens))?);
}

pub fn rec_par_group(segment: &ParenthesisGrouping) -> Option<ParenthesisGrouping> {
    let mut new_grouping: Vec<ParenthesisGrouping> = Vec::new();
    let mut segment_iter = segment.group()?.iter();
    while let Some(i) = segment_iter.next().cloned() {
        match (*i.token()?).token_type {
            TokenTypes::OpeningParenthesis => {
                let mut next_group: Vec<ParenthesisGrouping> = Vec::new();
                let mut final_run = true;
                let mut skipped_levels = 0;
                while let Some(next) = segment_iter.next().cloned() {
                    match next.token()?.token_type {
                        TokenTypes::ClosingParenthesis => {
                            if skipped_levels == 0 {
                                break;
                            } else {
                                skipped_levels -= 1;
                            }
                        }
                        TokenTypes::OpeningParenthesis => {
                            final_run = false;
                            skipped_levels += 1;
                            next_group.push(next);
                        }
                        _ => next_group.push(next),
                    }
                }
                if !final_run {
                    next_group = next_group
                        .iter()
                        .map(rec_par_group)
                        .map(Option::unwrap)
                        .collect();
                }
                new_grouping.push(ParenthesisGrouping::Grouping(next_group));
            }
            TokenTypes::ClosingParenthesis => return None,
            TokenTypes::Let => return None,
            TokenTypes::DefinitionArrow => return None,
            _ => new_grouping.push(i),
        }
    }
    return Some(ParenthesisGrouping::Grouping(new_grouping));
}
