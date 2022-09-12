mod tree;

use crate::tokenization::{Token, TokenTypes};
pub use tree::*;

pub fn group_lambda(segment: &ParenthesisGrouping) -> Option<LambdaGrouping> {
    let mut grouping = LambdaGrouping::new(Vec::new(), Vec::new());
    let tokens = segment.group()?;
    if !matches!(tokens[0].token()?.token_type, TokenTypes::LambdaSlash) {
        return None;
    }
    let args = tokens[1].group()?;
    for i in 0..(args.len() / 2) {
        if !matches!(
            args[(i * 2) + 1].token()?.token_type,
            TokenTypes::LambdaArgumentSeparator
        ) {
            return None;
        }
        let next_tok = args[i * 2].token()?;
        if !matches!(next_tok.token_type, TokenTypes::IdentifierName) {
            return None;
        }
        grouping.arguments.push(next_tok.to_owned().value);
    }
    if args.len() % 2 == 1 {
        grouping
            .arguments
            .push(args.last()?.token()?.to_owned().value);
    }
    if !matches!(tokens[2].token()?.token_type, TokenTypes::LambdaArrow) {
        return None;
    }
    grouping.body = tokens[3..].to_vec();
    return Some(grouping);
}

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
