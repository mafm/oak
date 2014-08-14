// Copyright 2014 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use front::ast::{Expression_, Expression, CharacterInterval, CharacterClassExpr};
pub use front::ast::{
  StrLiteral, AnySingleChar, NonTerminalSymbol, Sequence,
  Choice, ZeroOrMore, OneOrMore, Optional, NotPredicate,
  AndPredicate, CharacterClass};

pub use rust::{ExtCtxt, Span, Spanned, SpannedIdent};
pub use middle::attribute::attribute::*;
pub use identifier::*;
pub use std::collections::hashmap::HashMap;

pub use FGrammar = front::ast::Grammar;
use attribute::model_checker;
use attribute::model::AttributeArray;

pub struct Grammar{
  pub name: Ident,
  pub rules: HashMap<Ident, Rule>,
  pub attributes: GrammarAttributes
}

impl Grammar
{
  pub fn new(cx: &ExtCtxt, fgrammar: FGrammar) -> Option<Grammar>
  {
    let grammar_model = GrammarAttributes::model();
    let grammar_model = model_checker::check_all(cx, grammar_model, fgrammar.attributes);
    
    let rules_len = fgrammar.rules.len();
    let mut rules_models = Vec::with_capacity(rules_len);
    let mut rules: HashMap<Ident, Rule> = HashMap::with_capacity(rules_len);
    for rule in fgrammar.rules.move_iter() {
      let rule_model = RuleAttributes::model();
      let rule_model = model_checker::check_all(cx, rule_model, rule.attributes);

      let rule_name = rule.name.node.clone();
      if rules.contains_key(&rule_name) {
        Grammar::duplicate_rules(cx, rules.get(&rule_name).name.span, rule.name.span);
      } else {
        let rule = Rule::new(cx, rule.name, rule.def, &rule_model);
        if rule.is_some() {
          rules.insert(rule_name, rule.unwrap());
          rules_models.push((rule_name, rule_model));
        }
      }
    }

    if rules.len() == rules_len {
      let attributes = GrammarAttributes::new(cx, rules_models, grammar_model);
      let grammar = Grammar{
        name: fgrammar.name,
        rules: rules,
        attributes: attributes
      };
      Some(grammar)
    } else {
      None
    }
  }

  fn duplicate_rules(cx: &ExtCtxt, pre: Span, current: Span)
  {
    cx.span_err(current, "Duplicate rule definition.");
    cx.span_note(pre, "Previous declaration here.");
  }
}

pub struct Rule{
  pub name: SpannedIdent,
  pub attributes: RuleAttributes,
  pub def: Box<Expression>,
}

impl Rule
{
  fn new(cx: &ExtCtxt, name: SpannedIdent, def: Box<Expression>, attrs: &AttributeArray) -> Option<Rule>
  {
    let attributes = RuleAttributes::new(cx, attrs);
    Some(Rule{
      name: name,
      attributes: attributes,
      def: def
    })
  }
}