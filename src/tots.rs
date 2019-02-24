#![allow(unused)]

use super::patch::{eq, nl};
use failure::{Error, Fail};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use std::cell::RefCell;

use proc_macro2::{Ident, Span};

pub fn ident_from_str(s: &str) -> Ident {
    syn::Ident::new(s, Span::call_site())
}

#[derive(Fail, Debug)]
#[fail(display = "{}", _0)]
pub struct TypescriptParseError(pest::error::Error<Rule>);

impl TypescriptParseError {
    /// Return the column of where the error ocurred.
    #[allow(unused)]
    pub fn column(&self) -> usize {
        match self.0.line_col {
            pest::error::LineColLocation::Pos((_, col)) => col,
            pest::error::LineColLocation::Span((_, col), _) => col,
        }
    }
    #[allow(unused)]
    pub fn row(&self) -> usize {
        match self.0.line_col {
            pest::error::LineColLocation::Pos((row, _)) => row,
            pest::error::LineColLocation::Span((row, _), _) => row,
        }
    }
}   
pub struct TSType {
    ident: syn::Ident,
    args: Vec<syn::Type>,
    path: Vec<syn::Ident>,          // full path
    return_type: Option<syn::Type>, // only if function
}

#[derive(Parser)]
#[grammar = "entries.pest"]
struct TypescriptParser;

pub struct Typescript {
}

struct Ret {
    result: TokenStream,
}

impl Entry {



    pub fn parse(&self, ts: &TSType) -> Result<TokenStream, Error> {
        let pair = TypescriptParser::parse(Rule::typescript, &self.expr)
            .map_err(TypescriptParseError)?
            .next() // skip SOI
            .unwrap();
        let mut content = vec![];
        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::EOI => break,
                other => assert_eq!(other, Rule::expr),
            }
            content.push({
                self.parse_expr(ts, item)?.result
            });
        }
        assert!(content.len() == 1);

        Ok(quote!(
            {
                #(#content)*
            }
        ))
    }
    fn parse_expr(&self, ts: &TSType, expr: Pair<'_, Rule>) -> Result<Ret, Error> {
        // expr = { union | "(" ~ expr ~ ")" }
        let mut content = vec![];

        for u in expr.into_inner() {
            content.push(match u.as_rule() {
                Rule::union => self.parse_union(ts, u)?.result,
                Rule::expr => self.parse_expr(ts, u)?.result,

                _ => unreachable!(),
            })
        }
        assert!(content.len() == 1);

        Ok(Ret {
            result: quote!( #(#content)* )
        })
    }
    fn parse_item(&self, ts: &TSType, item: Pair<'_, Rule>) -> Result<Ret, Error> {
        use std::str::FromStr;
        let mut i = item.into_inner();
        // item = { singleton ~ array  }
        let (singleton, array) = (i.next().unwrap(), i.next().unwrap());

        let mut content = vec![];
        let array = array.as_str();
        // singleton = { str | map | tuple | typ | "(" ~ union ~ ")" }
        for singleton_pair in singleton.into_inner() {
            content.push(match singleton_pair.as_rule() {
                Rule::map => self.parse_map(ts, singleton_pair)?.result,
                Rule::str => self.parse_struct(ts, singleton_pair)?.result,
                Rule::tuple => self.parse_tuple(ts, singleton_pair)?.result,
                Rule::typ => self.parse_typ(ts, singleton_pair)?.result,
                Rule::union => self.parse_union(ts, singleton_pair)?.result,
                _ => unreachable!(),
            });
        }
        assert!(content.len() == 1);
        
        let arr = proc_macro2::TokenStream::from_str(array).unwrap();

        Ok(Ret {
            result: quote!( #(#content)* #arr )
        })

    }
    fn to_ts(&self, ty: & syn::Type) -> proc_macro2::TokenStream {
        quote!()
    }
    fn parse_typ(&self, ts: &TSType, typ: Pair<'_, Rule>) -> Result<Ret, Error> {
        // typ = { "number" | "object" | "string" | "boolean" | "null" | #ident }
        let ident = typ.as_str();
        let k = if ident.starts_with("#") {
            let idx = ident[1..].parse::<usize>()?;
            if idx >= ts.args.len() {
                use pest::error::{ErrorVariant, Error};
                let span = typ.as_span();
                let err = Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message : format!("type is out of bounds {}", idx)} , 
                        span);
                return Err(err.into())
            }
            self.to_ts(&ts.args[idx])
        } else { 
            let i = ident_from_str(ident);
            quote!(#i)
        };

        Ok(Ret {
            result: quote!(
               #k
            )
        })
    }
    fn parse_map(&self, ts: &TSType, map: Pair<'_, Rule>) -> Result<Ret, Error> {
        // map = {  "{" ~ "[" ~ "key" ~ ":" ~ key ~ "]" ~ ":" ~ expr ~ "}" }
        let mut i = map.into_inner();
        let (typ, expr) = (i.next().unwrap(), i.next().unwrap());

        let key = self.parse_typ(ts, typ)?.result;

        let result = self.parse_expr(ts, expr)?.result;


        Ok(Ret {
            result: quote!(
                {[key: #key]: #result }
            )
        })
    }
    fn parse_union(
        &self,
        ts: &TSType,
        union: Pair<'_, Rule>,
    ) -> Result<Ret, Error> {
        // union = {   item ~ ("|" ~ item)*  }
        let mut results = vec![];
        for item in union.into_inner() {
            match item.as_rule() {
                Rule::item => results.push(self.parse_item(ts, item)?.result),
                _ => unreachable!(),
            }
        }
        let newl = nl();
        let nl = (0..results.len()).map(|_| quote!(#newl));

        return Ok(
            Ret {
                result: quote!( #(#nl | #results)* )
            }
        )
    }
    fn parse_tuple(&self, ts: &TSType, tuple: Pair<'_, Rule>) -> Result<Ret, Error> {
        // tuple = { "[" ~ expr ~ ("," ~ expr )+ ~ "]" }
        let mut content = vec![];

        for expr in tuple.into_inner() {

            match expr.as_rule() {
                Rule::expr => {
                    let verify = self.parse_expr(ts, expr)?.result;
                    content.push(quote! {
                        #verify;
                    });
                }
                _ => unreachable!(),
            }
        }
        Ok(Ret {
            result: quote!(
               [ #(#content),* ]
            )
        })
    }
    fn parse_struct(&self, ts: &TSType, pair: Pair<'_, Rule>) -> Result<Ret, Error> {
        // str = {  "{" ~ (ident ~ ":" ~ expr)? ~ ("," ~ ident ~ ":" ~ expr )* ~ "}" }
        let mut keys = vec![];
        let mut values = vec![];
        for expr in pair.into_inner() {
            match expr.as_rule() {
                Rule::ident => keys.push(ident_from_str(&expr.as_str())),
                Rule::expr => values.push(self.parse_expr(ts, expr)?.result),
                _ => unreachable!(),
            }
        }
        Ok(Ret {
            result: quote!({ #(#keys : #values),* } )
        })
    }

}

#[derive(Hash, PartialEq, Debug)]
pub struct Entry {
    pub path: Vec<Vec<String>>,
    pub generics: Vec<String>,
    pub expr: String,
}



#[derive(Debug)]
pub struct EntryList {
    pub entries: Vec<Entry>,
}

impl EntryList {

    pub fn parse(typescript: &str) -> Result<EntryList, Error> {
        let pair = TypescriptParser::parse(Rule::markup, typescript)
            .map_err(TypescriptParseError)?
            .next() // skip SOI
            .unwrap();
        let mut entrylist = EntryList { entries: vec![] };

        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::EOI => break,
                other => { assert_eq!(other, Rule::entrylist);}
            };
            for entry in item.into_inner() {
                let e = entrylist.parse_entry(entry)?;
                entrylist.entries.push(e);

            }
        }
        Ok(entrylist)

    }
    fn parse_entry(&mut self, entry: Pair<'_, Rule> ) -> Result<Entry, Error> {
        let mut generics = vec![];
        let mut path = vec![];
        let mut expr = String::default();
        
        for e in entry.into_inner() {
            match e.as_rule() {
                Rule::generics => {generics = self.parse_generics(e)?},
                Rule::lhslist => {path = self.parse_lhslist(e)?},
                Rule::expr => { 
                    expr = e.as_str().trim().into();
                }
                _ =>  unreachable!()
            }
        }

        Ok(Entry {
            path,
            generics,
            expr,
        })

    }
    fn parse_generics(&mut self, generics: Pair<'_, Rule> ) -> Result<Vec<String>, Error> {
        let mut ret = vec![];
        for e in generics.into_inner() {
            match e.as_rule() {
                Rule::ident => ret.push(e.as_str().into()),
                _ =>  unreachable!()
            }
        }

        Ok(ret)
    }
    fn parse_lhslist(&mut self, lhslist: Pair<'_, Rule> ) -> Result<Vec<Vec<String>>, Error> {
        let mut ret = vec![];
        for e in lhslist.into_inner() {
            match e.as_rule() {
                Rule::lhs => {
                    let mut path = vec![];
                    for lhs in e.into_inner() {
                        match lhs.as_rule() {
                            Rule::ident => { path.push(lhs.as_str().into())},
                            _ => unreachable!()
                        }
                    }
                    ret.push(path)
                }
                _ =>  unreachable!()
            }
        }

        Ok(ret)
    }
}