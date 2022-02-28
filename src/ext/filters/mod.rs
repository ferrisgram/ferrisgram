pub mod message;
pub mod callback_query;
pub mod inline_query;

pub use message::MessageFilter;
pub use callback_query::CallbackQueryFilter;
pub use inline_query::InlineQueryFilter;

#[macro_export]
macro_rules! filter_extension {
    ($($t:ty, $uobj: ty, $filter: ty)*) => {
        $(
            impl $t {
                #[inline]
                /// This method is used to add an 'and' condition filter with the parent filter. 
                pub fn and(mut self, filter: Box<$filter>) -> Box<Self> {
                    self.and_filter = Some(filter);
                    Box::from(self)
                }
                /// This method is used to add an 'or' condition filter with the parent filter.
                pub fn or(mut self, filter: Box<$filter>) -> Box<Self> {
                    self.or_filter = Some(filter);
                    Box::from(self)
                }
                pub fn invert(mut self) -> Box<Self> {
                    self.inverted = true;
                    Box::from(self)
                }
                pub fn check_integral_filter(&self, m: &$uobj, mut parent_result: bool) -> bool {
                    if self.inverted {
                        parent_result = !parent_result
                    }
                    if self.and_filter.is_none() && self.or_filter.is_none() {
                        return parent_result
                    }
                    (
                        parent_result 
                        && 
                        self.and_filter.is_some() 
                        && 
                        self.and_filter.as_ref().unwrap().check_filter(m)
                    ) || (
                        self.or_filter.is_some() 
                        && 
                        self.or_filter.as_ref().unwrap().check_filter(m)
                    )
            
                }
            }
        )*
    }
}