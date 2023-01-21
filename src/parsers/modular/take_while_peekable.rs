// We can't use take_while since it will consume the `.` token, for which we need to verify its
// existence first. Since our version should always have a number component first, it is fine for
// peekable to consume the first character, to store in the peekable iterator.

use std::iter::Peekable;

pub trait TakeWhilePeekable<'peekable, I>: Iterator
where
    I: Iterator,
{
    fn take_while_peekable<P>(&'peekable mut self, pred: P) -> TakeWhilePeekableImpl<I, P>
    where
        P: FnMut(&Self::Item) -> bool;
}

pub struct TakeWhilePeekableImpl<'peekable, I, P>
where
    I: Iterator,
{
    iter: &'peekable mut Peekable<I>,
    pred: P,
}

impl<'peekable, I> TakeWhilePeekable<'peekable, I> for Peekable<I>
where
    I: Iterator,
{
    fn take_while_peekable<P>(&'peekable mut self, pred: P) -> TakeWhilePeekableImpl<I, P>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        TakeWhilePeekableImpl { iter: self, pred }
    }
}

impl<'peekable, I, P> Iterator for TakeWhilePeekableImpl<'peekable, I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_if(&mut self.pred)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_while() {
        let iterable = &['h', 'e', 'l', 'l', 'o'];
        let iterator = iterable.iter();
        let mut peekable = iterator.peekable();
        let tw_peekable = peekable.take_while_peekable(|&&c| c != 'o');
        let collected = tw_peekable.cloned().collect::<Vec<_>>();

        // Taking the hell out of hello ;).
        assert_eq!(collected, vec!['h', 'e', 'l', 'l']);
    }

    #[test]
    fn empty() {
        let iterable = &[] as &[char; 0];
        let iterator = iterable.iter();
        let mut peekable = iterator.peekable();
        let mut tw_peekable = peekable.take_while_peekable(|&&c| c != 'o');

        // Taking the hell out of hello ;).
        assert!(tw_peekable.next().is_none());
    }

    mod by_mut_ref {
        use super::*;

        fn f(input: &mut Peekable<impl Iterator<Item = char>>) -> String {
            input
                .take_while_peekable(|&c| c != 'o')
                .fold(String::new(), |mut acc, next| {
                    acc.push(next);
                    acc
                })
        }

        #[test]
        fn restart_without_increment() {
            let iterable = &['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];
            let iterator = iterable.iter();
            let mut peekable = iterator.cloned().peekable();

            let test1 = f(&mut peekable);
            assert_eq!(test1, "hell".to_string());

            let test2 = f(&mut peekable);
            assert_eq!(test2, "".to_string());
        }

        #[test]
        fn restart_with_manual_increment() {
            let iterable = &['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];
            let iterator = iterable.iter();
            let mut peekable = iterator.cloned().peekable();

            let test1 = f(&mut peekable);
            assert_eq!(test1, "hell".to_string());

            let test2 = peekable.next_if(|&c| c == 'o');
            assert_eq!(test2.unwrap(), 'o');

            let test3 = f(&mut peekable);
            assert_eq!(test3, "w".to_string());
        }
    }
}
