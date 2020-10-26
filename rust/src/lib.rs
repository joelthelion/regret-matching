extern crate alga;
extern crate nalgebra;
extern crate num_traits;
use nalgebra as na;
use num_traits::identities::Zero;

pub trait Problem {
    type State;
    type RewardVector : alga::linear::FiniteDimInnerSpace;
}
pub struct RegretMatchingAgent<T: Problem> {
    v : T::RewardVector
}

impl<T: Problem> RegretMatchingAgent<T> {
    fn new() -> Self {
        RegretMatchingAgent { v : T::RewardVector::zero() }
    }
    fn update_regret(&self, s: &T::State, v: &T::RewardVector) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct CoinFlip {}
    impl Problem for CoinFlip {
        type State = ();
        type RewardVector = na::Vector1<f32>;
    }
    impl CoinFlip {
    }
    #[test]
    fn it_works() {
        let a = RegretMatchingAgent::<CoinFlip>::new();
        a.update_regret(&(), &na::Vector1::<f32>::zeros());
        assert_eq!(2 + 2, 4);
    }
}
