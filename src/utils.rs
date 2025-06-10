/// Defines the order in which rule priorities are compared.
///
/// This enum can be used to determine whether rules with higher or lower
/// priority values should be evaluated or applied first.
#[derive(Debug, Clone, Copy, Default)]
pub enum PriorityOrder {
    /// Ascending order: lower priority values come first.
    ///
    /// This is the default order.
    #[default]
    Asc,

    /// Descending order: higher priority values come first.
    Desc,
}
