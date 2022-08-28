#![feature(generic_associated_types)]

use lib::*;

pub trait Functor<T> where T : Copy {
    type Instance<K>;
    fn fmap<I>(&self, f : fn(T) -> I) -> Self::Instance<I>;
}

pub trait Applicative<T> where T : Copy {
    type Instance<K>;

    fn pure(v : T) -> Self::Instance<T>;
    fn liftA2<A,B,C>(f : fn(A,B) -> C, h : Self::Instance<A>, g : Self::Instance<B>) -> Self::Instance<C>;
}

enum Maybe<A> {
    Just(A),
    Nothing
}

impl<A : std::fmt::Display> Maybe<A> {
    fn disp(&self) {
        match self {
            Maybe::Just(a) => println!("Just {}", a),
            Maybe::Nothing => println!("Nothing"),
        }
    }
}

pub trait Monad<T> where T : Copy {
    type Instance<K>;
    fn ret(v : T) -> Self::Instance<T>;
    fn bind<G>(&self, f : &dyn Fn (&T) -> Self::Instance<G>) -> Self::Instance<G>;
}



impl<I : Copy> Applicative<I> for Maybe<I> {
    type Instance<K> = Maybe<K>;
    
    fn pure(v : I) -> Self::Instance<I> {
        return Maybe::Just(v);
    }

    fn liftA2<A,B,C>(f : fn(A,B) -> C, h : Self::Instance<A>, g : Self::Instance<B>) -> Self::Instance<C> {
        match h {
            Maybe::Just(a) => match g {
                Maybe::Just(b) => Maybe::Just(f(a,b)),
                Maybe::Nothing => Maybe::Nothing,
            }
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

impl<I : Copy> Functor<I> for Maybe<I> {

    type Instance<K> = Maybe<K>;
    
    fn fmap<K>(&self, f : fn(I) -> K) -> Self::Instance<K>
    {
        match(self) {
            Maybe::Just(a) => Maybe::Just(f(*a)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}


impl<I : Copy> Monad<I> for Maybe<I> {
    type Instance<K> = Maybe<K>;
    fn ret(v : I) -> Self::Instance<I> {
        Maybe::Just(v)
    }

    fn bind<G>(&self, f : &dyn Fn (&I) -> Self::Instance<G>) -> Self::Instance<G> {
        match self {
            Maybe::Just(a) => f(a),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

fn getSensor(x : usize) -> Maybe<usize> {
    if(x == 0) {
        return Maybe::Just(30);
    }
    if(x == 1) {
        return Maybe::Just(20);
    }
    return Maybe::Nothing;
}

fn cond<M>(c : bool, action : M) -> M where M : Monad<(), Instance<()> = M> {
    if c {
        return action;
    } else {
        return M::ret(());
    }
}

fn validate_sensors() -> Maybe<usize> {
    do_prog!(
        let val = getSensor(1)!;
        let val2 = getSensor(0)!;
        let b = cond(3 != 2, Maybe::pure(()))!;
        let x = val + val2*10;
        if x > 200 {
            return Maybe::pure(x);
        } else {
            return Maybe::Nothing;
        }
    )
}

fn main() {

    let gt_than_2 = |x : usize| {if x > 2 {Maybe::<usize>::ret(x)} else {Maybe::Nothing}};

    validate_sensors().disp();
}
