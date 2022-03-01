# **FerrisGram**

## **ext::filters**
This directory contains the filters for the handlers of the ferrisgram library which you can use to customise the incoming updates.

### **Custom filters**
It is also possible to write our own filters. In essence, a filter is simply an object whose `check_filter` function receives a `Update` instance and returns either `True` or `False`. This object has to implement filter traits like `MessageFilter`, which allows it to be combined with other filters of that same trait. If the combination of all filters evaluates to `True`, the message will be handled.

Say we wanted to allow only those messages that contain the text "ferrisgram is awesome", we could write a custom filter as so:

```rust
use ferrisgram::types::Message;
use ferrisgram::ext::filters::{MessageFilter, message};

#[derive(Clone)]
pub struct CustomFilter {}
impl MessageFilter for CustomFilter {
    fn check_filter(&self, m: &Message) -> bool {
        m.text.is_some() 
        && 
        m.text.as_ref().unwrap() == "ferrisgram is awesome"
    }
}
impl CustomFilter {
    fn filter() -> Box<Self> {
        Box::new(Self {})
    }
}
```

Now, you can use this custom filter with a message handler by calling the `filter()` method as so:

```rust
use ferrisgram::ext::handlers::MessageHandler;

updater.dispatcher.add_handler(MessageHandler::new(callback_function, CustomFilter::filter()));
```

### **Advanced Custom Filters**
Ferrisgram provides `filter_extension` macro which can be used to add support of methods like `.and`, `.or`, `.invert` which increases the capabilities of your custom filter.

Let us learn about it through an example, suppose we want to use this extension in our `CustomFilter` then we will use it as so:

```rust
use ferrisgram::filter_extension;
use ferrisgram::ext::filters::message::MessageFilter;

#[derive(Clone)]
pub struct CustomFilter {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool
}
impl MessageFilter for CustomFilter {
    fn check_filter(&self, m: &Message) -> bool {
        m.text.is_some() 
        && 
        m.text.as_ref().unwrap() == "ferrisgram is awesome"
    }
}
impl CustomFilter {
    fn filter() -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
        })
    }
}
filter_extension!(CustomFilter, Message, dyn MessageFilter);
```
The struct can of course be named however you want, the only important things are:
- You must add `and_filter`, `or_filter` and `inverted` fields with their respective data types in order to use filter extension.
- `filter_extension` takes three arguments which are as follows:
    - __First Argument__: Struct of your custom filter.
    - __Second Argument__: Type of the update it will recieve, for example `Message`, `CallbackQuery` etc.
    - __Third Argument__: Base Filter which a handler can accept. Available Base Filters are as follows:
        - `MessageFilter`: it used with `Message` as 2nd argument.
        - `CallbackQueryFilter`: it used with `CallbackQuery` as 2nd argument.
        - `InlineQueryFilter`: it used with `InlineQuery` as 2nd argument. 
    
    **Note**: You must add `dyn` keyword with Base Filters in argument to the `filter_extension`.
 

## **Contributing**
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update examples as appropriate.


## **License**
[![MIT](https://upload.wikimedia.org/wikipedia/commons/thumb/0/0c/MIT_logo.svg/200px-MIT_logo.svg.png)](https://opensource.org/licenses/MIT)
<br>Licensed Under <a href="https://opensource.org/licenses/MIT">The MIT License</a>