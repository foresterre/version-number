use std::str::FromStr;

/// A parser for `N` components
pub struct Parser<const N: usize> {
    components: [ComponentParser; N],
}

impl<const N: usize> Parser<N> {
    pub fn parse<I: AsRef<str>>(&self, _input: I) -> crate::Version {
        todo!()
    }
}

impl Parser<2> {
    pub fn parse_core_version<I: AsRef<str>>(&self, _input: I) -> crate::CoreVersion {
        todo!()
    }
}

impl Parser<3> {
    pub fn parse_full_version<I: AsRef<str>>(&self, _input: I) -> crate::CoreVersion {
        todo!()
    }
}

pub struct Component(u64);

#[derive(Default)]
pub struct ComponentParser;

impl ComponentParser {
    pub fn parse(&self, input: Span) -> Result<(Span, Component), ()> {

        input.span
            .iter()
            .take_while(|&tok| matches!(tok, 0u8..=9u8))
            .map(|tok| u64::from(tok - b'0'))
            .scan(0u64, |state, next| state.checked_mul(10))

        todo!()
    }
}

//
struct Span<'input> {
    span: &'input [u8],
}

/// A consuming builder to build a parser which can parse two **xor** three component version numbers.
#[derive(Default)]
pub struct ParserBuilder<OptComponentParser: HasPatch> {
    major: ComponentParser,
    minor: ComponentParser,
    patch: OptComponentParser,
}

impl ParserBuilder<Full> {
    pub fn with_patch(self) -> Self {
        let mut inner = self;
        inner.patch = Full {
            patch: ComponentParser,
        };
        inner
    }

    pub fn build(self) -> Parser<3> {
        todo!()
    }
}

impl ParserBuilder<Core> {
    pub fn build(self) -> Parser<2> {
        todo!()
    }
}

pub trait HasPatch {}
pub struct Core;
impl HasPatch for Core {}
pub struct Full {
    patch: ComponentParser,
}
impl HasPatch for Full {}
