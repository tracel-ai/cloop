pub trait InputParser<T> {
    type Output;
    fn parse(&self, input: &str, context: &mut T) -> Result<Self::Output, impl std::fmt::Display>;
}

impl<A, T, F, E> InputParser<T> for F
where
    F: Fn(&str, &mut T) -> Result<A, E>,
    E: std::fmt::Display,
{
    type Output = A;
    fn parse(&self, input: &str, context: &mut T) -> Result<A, impl std::fmt::Display> {
        self(input, context)
    }
}

/// Implementation for clap ArgMatches
/// This allows you to use clap's ArgMatches as a parser for the Shell struct
impl<T> InputParser<T> for clap::Command {
    type Output = clap::ArgMatches;
    fn parse(&self, input: &str, _: &mut T) -> Result<Self::Output, impl std::fmt::Display> {
        let raw_args = shlex::split(input).ok_or("Invalid quoting")?;
        self.clone()
            .try_get_matches_from_mut(raw_args)
            .map_err(|e| e.to_string())
    }
}

pub struct ClapParser<T: clap::Parser>(std::marker::PhantomData<T>);

impl<T: clap::Parser> Default for ClapParser<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<P: clap::Parser, T> InputParser<T> for ClapParser<P> {
    type Output = P;
    fn parse(&self, input: &str, _: &mut T) -> Result<Self::Output, impl std::fmt::Display> {
        let raw_args = shlex::split(input).ok_or("Invalid quoting")?;
        P::try_parse_from(raw_args).map_err(|e| e.to_string())
    }
}

pub struct ClapSubcommandParser<T: clap::Subcommand>(std::marker::PhantomData<T>);

impl<S: clap::Subcommand> Default for ClapSubcommandParser<S> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<S: clap::Subcommand, T> InputParser<T> for ClapSubcommandParser<S> {
    type Output = S;
    fn parse(&self, input: &str, _: &mut T) -> Result<Self::Output, impl std::fmt::Display> {
        let raw_args = shlex::split(input).ok_or("Invalid quoting")?;
        S::augment_subcommands(clap::Command::default().multicall(true))
            .try_get_matches_from_mut(raw_args)
            .map_err(|e| e.to_string())
            .and_then(|m| S::from_arg_matches(&m).map_err(|e| e.to_string()))
    }
}
