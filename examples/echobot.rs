use ferrisgram::Bot;
use ferrisgram::error::{Result, GroupIteration};
use ferrisgram::ext::handlers::{CommandHandler, MessageHandler};
use ferrisgram::ext::filters::message;
use ferrisgram::ext::{Dispatcher, Updater, Context};

#[allow(unused)]
#[tokio::main]
async fn main() {  
    // This function creates a new bot instance and the error is handled accordingly 
    let bot = match Bot::new("Bot Token Here").await {
        Ok(bot) => bot,
        Err(error) => panic!("failed to create bot: {}", &error),
    };

    // dispatcher is a part of internal functionality of updater
    // you may use it for adding handlers.
    let mut dispatcher = &mut Dispatcher::new(&bot);

    // add_handler method maps the provided handler in group 0 automatically
    dispatcher.add_handler(CommandHandler::new("start", start));

    // add_handler_to_group is used to map the provided handler to a group manually.
    // note that handler groups are processed in ascending order.
    dispatcher.add_handler_to_group(
        MessageHandler::new(
            echo, 
            // This will restrict our echo function to the messages which 
            // contain either text or a caption.
            message::Text::filter().or(message::Caption::filter())
        ),
        1
    );

    let mut updater = Updater::new(&bot, dispatcher);

    // This method will start long polling through the getUpdates method
    updater.start_polling(true).await;
}

// This is our callable function for the command handler that we declared earlier
// It will be triggered when someone send /start to the bot.
async fn start(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Command Handler recieves message updates which have chat as a compulsory field.
    // Hence we can unwrap effective chat without checking if it is none.
    let chat = ctx.effective_chat.unwrap();
    // Every api method creates a builder which contains a various parameters of that respective method. 
    bot.send_message(
        chat.id, String::from(
"Hey! I am an echo bot built using [Ferrisgram](https://github.com/ferrisgram/ferrisgram).
I will repeat your messages."
        )
    )
    // this method will ensure that our text will be sent with markdown formatting.
    .parse_mode("markdown".to_string())
    // this method will disable the web page preview for out message
    .disable_web_page_preview(true)
    // You must use this send() method in order to send the request to the API
    .send().await?;

    // GroupIteration::EndGroups will end iteration of groups for an update.
    // This means that rest of the pending groups and their handlers won't be checked
    // for this particular update.
    Ok(GroupIteration::EndGroups)
}

// This is our callable function for our message handler which will be used to 
// repeat the text.
async fn echo(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Command Handler recieves message updates which have chat as a compulsory field.
    // Hence we can unwrap effective chat without checking if it is none.
    let chat = ctx.effective_chat.unwrap();
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    // Every api method creates a builder which contains a method named send()
    bot.copy_message(chat.id, chat.id, msg.message_id)
    // You must use this send() method in order to send the request to the API
    .send().await?;

    // GroupIteration::EndGroups will end iteration of groups for an update.
    // This means that rest of the pending groups and their handlers won't be checked
    // for this particular update.
    Ok(GroupIteration::EndGroups)
}