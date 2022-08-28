#![feature(generic_associated_types)]

use proc_macro::TokenStream;
use syn::{braced, parse_macro_input, token, Field, Ident, Result, Token, Expr, ExprCall};
use syn::token::{Paren,Let,Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use std::boxed::Box;
use quote::{quote, ToTokens};

enum DoEnum {
    Let(LetStruct),
    Bind(BindStruct),
    Ret(RetStruct),
    IfElse(IfElseStruct),
    Nop,
}

struct LetStruct {
    id : Ident,
    e : Expr,
    tail : Box<DoEnum>,
}

struct BindStruct {
    id : Ident,
    action : ExprCall,
    tail : Box<DoEnum>,
}

struct IfElseStruct {
    cond : Expr,
    b1 : Box<DoEnum>,
    b2 : Box<DoEnum>,
}

struct RetStruct {
    s : Expr,
}

impl ToTokens for DoEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match(self) {
            DoEnum::Nop => (),
            DoEnum::Let(l) => {
                let inner = &(*l.tail);
                let id = &l.id;
                let expr = &l.e;
                tokens.extend(quote!{
                    (&|#id| { return #inner })(#expr)
                });
            },
            DoEnum::Bind(b) => {
                let inner = &(*b.tail);
                let action = &b.action;
                let id = &b.id;
                tokens.extend(quote!{
                    #action . bind(&|&#id| {#inner})
                });
            },
            DoEnum::Ret(r) => {
                let exp = &r.s;
                tokens.extend(quote!{
                    #exp
                });
            }
            DoEnum::IfElse(ie) => {
                let exp1 = &(*ie.b1);
                let exp2 = &(*ie.b2);
                let cond = &ie.cond;
                tokens.extend(quote!{
                    if(#cond) {
                        #exp1
                    } else {
                        #exp2
                    }
                })
            }
        }
    }
}

impl Parse for DoEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![let]) {
            input.parse::<Let>()?;
            let id : Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            let lookahead = input.lookahead1();
            if input.peek(Ident) && input.peek2(Paren) {
                let action : ExprCall = input.parse()?;
                input.parse::<Token![!]>()?;
                input.parse::<Token![;]>()?;
                let node : DoEnum = input.parse()?;
                return Ok(DoEnum::Bind(BindStruct{id : id, action : action, tail : Box::new(node)}));
            }
            else
            {
                let expr : Expr = input.parse()?;
                input.parse::<Token![;]>()?;
                let node : DoEnum = input.parse()?;
                return Ok(DoEnum::Let(LetStruct{id : id, e : expr, tail : Box::new(node)}));
            }
        }
        if lookahead.peek(Token![return]) {
            input.parse::<Token![return]>()?;
            let e : Expr = input.parse()?;
            input.parse::<Token![;]>()?;
            return Ok(DoEnum::Ret(RetStruct{s : e}));
        }
        if lookahead.peek(Token![if]) {
            input.parse::<Token![if]>()?;
            let cond : Expr = input.parse()?;
            let s1;
            let s2;
            braced!(s1 in input);
            input.parse::<Token![else]>()?;
            braced!(s2 in input);
            return Ok(DoEnum::IfElse(IfElseStruct{cond:cond, b1:Box::new(s1.parse()?), b2:Box::new(s2.parse()?)}))
        }
        return Ok(DoEnum::Nop);
    }
}


#[proc_macro]
pub fn do_prog(tokens : TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DoEnum);

    let expanded = quote!{#input};
    TokenStream::from(expanded)
}