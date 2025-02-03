pub trait InputParser<A, T> {
    fn parse(&self, args: Vec<String>, context: &mut T) -> Result<A, impl std::fmt::Display>;
}

impl<A, T, F, E> InputParser<A, T> for F
where
    F: Fn(Vec<String>, &mut T) -> Result<A, E>,
    E: std::fmt::Display,
{
    fn parse(&self, args: Vec<String>, context: &mut T) -> Result<A, impl std::fmt::Display> {
        self(args, context)
    }
}

/// Implementation for clap ArgMatches
/// This allows you to use clap's ArgMatches as a parser for the Shell struct
impl<T> InputParser<clap::ArgMatches, T> for clap::Command {
    fn parse(
        &self,
        args: Vec<String>,
        _: &mut T,
    ) -> Result<clap::ArgMatches, impl std::fmt::Display> {
        self.clone()
            .try_get_matches_from_mut(args)
            .map_err(|e| e.to_string())
    }
}

pub struct ClapParser<T: clap::Parser>(std::marker::PhantomData<T>);

impl<T: clap::Parser> Default for ClapParser<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<P: clap::Parser, T> InputParser<P, T> for ClapParser<P> {
    fn parse(&self, args: Vec<String>, _: &mut T) -> Result<P, impl std::fmt::Display> {
        P::try_parse_from(args)
    }
}

pub struct ClapSubcommandParser<T: clap::Subcommand>(std::marker::PhantomData<T>);

impl<S: clap::Subcommand> Default for ClapSubcommandParser<S> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<S: clap::Subcommand, T> InputParser<S, T> for ClapSubcommandParser<S> {
    fn parse(&self, args: Vec<String>, _: &mut T) -> Result<S, impl std::fmt::Display> {
        S::augment_subcommands(clap::Command::default().multicall(true))
            .try_get_matches_from_mut(args)
            .map_err(|e| e.to_string())
            .and_then(|m| S::from_arg_matches(&m).map_err(|e| e.to_string()))
    }
}
