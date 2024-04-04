// Evil extenions

use std::future::Future;

pub trait AsyncMap<In, Out, E, Fun, Fut>
where
    Fun: Fn(In) -> Fut,
    Fut: Future<Output = Result<Out, E>>,
{
    async fn transpose_flatten(self, fun: Fun) -> Result<Out, E>;
}

impl<In, Out, E, Fun, Fut> AsyncMap<In, Out, E, Fun, Fut> for Result<In, E>
where
    Fun: Fn(In) -> Fut,
    Fut: Future<Output = Result<Out, E>>,
{
    async fn transpose_flatten(self, fun: Fun) -> Result<Out, E> {
        fun(self?).await
    }
}

// would use [str::pattern::Pattern] instead. But it's unstable :(
pub trait SplitUtil {
    fn split_twice(&self, delim: char) -> Option<(&str, &str, &str)>;
    fn split_maybe_once(&self, delim: char) -> (&str, Option<&str>);
}

impl SplitUtil for &str {
    fn split_twice(&self, delim: char) -> Option<(&str, &str, &str)> {
        self.split_once(delim)
            .and_then(|(a, b)| b.split_once(delim).map(|(b, c)| (a, b, c)))
    }

    fn split_maybe_once(&self, delim: char) -> (&str, Option<&str>) {
        match self.split_once(delim) {
            Some((a, b)) => (a, Some(b)),
            None => (self, None),
        }
    }
}

