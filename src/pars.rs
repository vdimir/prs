// *
// *
// *

use std::marker::PhantomData;
use stream::TokenStream;

// ========================================= Parse Trait ==========================================

pub trait Parse {
    type Input;
    type Output;
    type Error;

    fn parse(&self, &mut Self::Input) -> Result<Self::Output, Self::Error>;
}

impl<'a, I, O, P, E> Parse for &'a P
    where P: Parse<Input=I, Output = O, Error=E>,
          // I: TokenStream,
{
    type Input = I;
    type Output = O;
    type Error = E;
    fn parse(&self, tokens: &mut I) -> Result<O, E> {
        (*self).parse(tokens)
    }
}


// ==================================== Parse Implementations =====================================

// -------------------------------------------- Token ---------------------------------------------
use result::ParseErr;

pub struct Token<S: TokenStream>(pub S::Token);

impl<S> Parse for Token<S>
    where S: TokenStream,
          S::Token: PartialEq + Copy
{
    type Input = S;
    type Output = S::Token;
    type Error = ParseErr<S::Token>;

    fn parse(&self, tokens: &mut S) -> Result<Self::Output, Self::Error> {
        let next_token = tokens.peek();
        let satified = next_token.map_or(false, |t| t == self.0);

        if satified {
            Ok(tokens.next().unwrap())
        } else {
            Err(ParseErr::Expected(self.0))
        }
    }
}

// ------------------------------------------ Predicate -------------------------------------------

// Use `name` or provide only `on_error()` for parsers and show none by default?
#[derive(Clone)]
pub struct Predicate<F, S>
where S: TokenStream,
     F: Fn(&S::Token) -> bool {
    name: String,
    predicate: F,
    _phantom: PhantomData<S>
}

impl<S, F> Parse for Predicate<F, S>
    where S: TokenStream,
        F: Fn(&S::Token) -> bool
{
    type Input = S;
    type Output = S::Token;
    type Error = ParseErr<String>;

    fn parse(&self, tokens: &mut S) -> Result<Self::Output, Self::Error> {
        let next_token = tokens.peek();
        let satified = next_token.map_or(false, |t| (self.predicate)(&t));

        if satified {
            Ok(tokens.next().unwrap())
        } else {
            Err(ParseErr::Expected(self.name.clone()))
        }
    }
}

pub fn predicate<F, T, S>(name: S, f: F) -> Predicate<F, T>
where T: TokenStream,
      F: Fn(&T::Token) -> bool,
      S: Into<String>
{
    Predicate {
        name: name.into(),
        predicate: f,
        _phantom: PhantomData
    }
}

// ------------------------------------------- FnParser -------------------------------------------
pub struct FnParser<F, I>(F, PhantomData<I>);

impl<S, R, E, F> Parse for FnParser<F, S>
    where F: Fn(&mut S) -> Result<R, E>
{
    type Input = S;
    type Output = R;
    type Error = E;

    fn parse(&self, tokens: &mut S) -> Result<R, E> {
        self.0(tokens)
    }
}

pub fn fn_parser<I, R, E, F>(f: F) -> FnParser<F, I>
    where F: Fn(&mut I) -> Result<R, E>
{
    FnParser(f, PhantomData)
}
