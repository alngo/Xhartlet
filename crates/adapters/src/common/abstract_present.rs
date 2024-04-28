#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock(type ViewModel=String;))]
pub trait Present<D> {
    /// View model
    type ViewModel;
    /// Present the given data `D`
    fn present(&self, data: D) -> Self::ViewModel;
}
