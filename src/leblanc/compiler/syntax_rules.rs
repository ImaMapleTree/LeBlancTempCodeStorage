/*
    Rule 1 to be implemented:
    All parenthesis created by constructors and functions MUST be closed
    BEFORE the next semicolon.
 */

/*
    Rule 2 to be implemented:
    Any types of UNKNOWN(CLASSDEF(0)) are variables that do not exist in the scope and thus
    the code will not work
 */


use crate::leblanc::compiler::compiler_util::flatmap_node_tokens;
use crate::leblanc::compiler::lang::leblanc_lang::CompileVocab;
use crate::leblanc::compiler::lang::leblanc_operators::LBOperator::Assign;
use crate::leblanc::compiler::symbols::Symbol;
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::LeBlancType::*;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub;
use crate::leblanc::rustblanc::relationship::Node;
use crate::TypedToken;

pub struct RuleAnalyzer {
    open_parenthesis: Vec<ExtraTypeSymbol>,
    closed_parenthesis: Vec<ExtraTypeSymbol>
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct ExtraTypeSymbol{
    symbol: Symbol,
    vocab: CompileVocab
}

impl ExtraTypeSymbol {
    pub fn new(symbol: Symbol, vocab: CompileVocab) -> ExtraTypeSymbol {
        return ExtraTypeSymbol {
            symbol, vocab
        }
    }
}

impl RuleAnalyzer {
    pub fn new() -> RuleAnalyzer {
        return RuleAnalyzer {
            open_parenthesis: Vec::new(),
            closed_parenthesis: Vec::new()
        }
    }
    pub fn add_parenthesis(&mut self, symbol: Symbol, vocab: CompileVocab) {
        let ets = ExtraTypeSymbol::new(symbol, vocab.clone());
        match symbol.char() {
            '(' => {
                self.open_parenthesis.push(ets)
            },
            ')' => {
                if self.open_parenthesis.len() == 0 {
                    self.closed_parenthesis.push(ets)
                }
                self.open_parenthesis.pop(); },
            _ => {}
        }
    }

    pub fn evaluate_rule1(&self, errors: &mut Vec<ErrorStub>) {
        // RULE 0 Evaluation
        let t = LeBlancType::Class(0);
        self.open_parenthesis.iter().filter(|p| !(CompileVocab::FUNCTION.eq(&p.vocab) || CompileVocab::CONSTRUCTOR(t).eq(&p.vocab)))
            .for_each(|p| errors.insert(errors.len(), ErrorStub::ImbalancedDelimiter(p.symbol)));
        self.closed_parenthesis.iter().filter(|p| !(CompileVocab::FUNCTION.eq(&p.vocab) || CompileVocab::CONSTRUCTOR(t).eq(&p.vocab)))
            .for_each(|p| errors.insert(errors.len(), ErrorStub::ImbalancedDelimiter(p.symbol)));
    }

    pub fn evaluate_rule2(&self, errors: &mut Vec<ErrorStub>, tokens: &mut Vec<TypedToken>) {
        tokens.iter().filter(|t| t.lang_type().clone() == CompileVocab::UNKNOWN(Class(0)))
            .for_each(|t| { errors.insert(errors.len(), ErrorStub::UndeclaredVariable(t.clone())) });
    }

    /*
     * rule 3 is that all variables need to be assigned the correct type they're given
     */
    pub fn evaluate_rule3(&self, errors: &mut Vec<ErrorStub>, tokens: &mut Vec<TypedToken>) {
        for i in 0..tokens.len() {
            let token = &tokens[i];
            if token.lang_type() == CompileVocab::OPERATOR(Assign) {
                let prior_token = tokens.get(i-1).unwrap();
                let next_token = tokens.get(i+1).unwrap();
                if let CompileVocab::VARIABLE(match_type) = prior_token.lang_type() {
                    match next_token.lang_type() {
                        CompileVocab::CONSTANT(token_type) => { if match_type != token_type { errors.insert(errors.len(), ErrorStub::IncompatibleType(prior_token.clone())) } }
                        CompileVocab::VARIABLE(token_type) => { if match_type != token_type { errors.insert(errors.len(), ErrorStub::IncompatibleType(prior_token.clone())) } }
                        CompileVocab::CONSTRUCTOR(token_type) => { if match_type != token_type { errors.insert(errors.len(), ErrorStub::IncompatibleType(prior_token.clone())) } }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn evaluate(&self, errors: &mut Vec<ErrorStub>, tokens: &mut Vec<Node<TypedToken>>) {
        self.evaluate_rule1(errors);
        self.evaluate_rule2(errors, &mut flatmap_node_tokens(tokens));
        self.evaluate_rule3(errors, &mut flatmap_node_tokens(tokens));
    }
}