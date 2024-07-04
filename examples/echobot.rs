use ferrisgram::error::{GroupIteration, Result};
use ferrisgram::ext::filters::message;
use ferrisgram::ext::handlers::{CommandHandler, MessageHandler};
use ferrisgram::ext::{Context, Dispatcher, Updater};
use ferrisgram::types::LinkPreviewOptions;
use ferrisgram::Bot;
use std::env;
use std::sync::Arc;

#[allow(unused)]
#[tokio::main]
async fn main() {
    let bot_token = match env::var("FERRIS_BOT_TOKEN") {
        Ok(s) => s,
        Err(err) => panic!("failed to start bot: {}", err),
    };
    // This function creates a new bot instance and the error is handled accordingly
    let bot = match Bot::new(&bot_token, None).await {
        Ok(bot) => bot,
        Err(error) => panic!("failed to create bot: {}", error),
    };
    // dispatcher is a part of internal functionality of updater
    // you may use it for adding handlers.
    let mut dispatcher = &mut Dispatcher::new(bot.clone());

    // add_handler method maps the provided handler in group 0 automatically
    dispatcher.add_handler(CommandHandler::new("start", start));

    // add_handler_to_group is used to map the provided handler to a group manually.
    // note that handler groups are processed in ascending order.
    dispatcher.add_handler_to_group(
        MessageHandler::new(
            echo,
            // This will restrict our echo function to the messages which
            // contain either text or a caption.
            message::Text::filter().or(message::Caption::filter()),
        ),
        1,
    );

    let mut updater = Updater::new(bot, dispatcher);

    // This method will start long polling through the getUpdates method
    updater.start_polling(true).await;
    // If you want to use webhooks, then uncomment the following line and comment out the upper line.
    // Also, you need to enable "webhook" feature to use webhooks.
    // let mut wops = StartWebhookOpts::new().path("/bot");
    // updater.start_webhook(3848, wops).await;
}

// This is our callable function for the command handler that we declared earlier
// It will be triggered when someone send /start to the bot.
async fn start(bot: Arc<Bot>, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let mut link_preview_options = LinkPreviewOptions::new();
    link_preview_options.is_disabled = Some(true);
    // Ferrisgram offers some custom helpers which make your work easy
    // Here we have used one of those helpers known as msg.reply
    msg.reply(
        &bot,
        "Hey! I am an echo bot built using [Ferrisgram](https://github.com/ferrisgram/ferrisgram).
I will repeat your messages.",
    )
    // this method will ensure that our text will be sent with markdown formatting.
    .parse_mode("markdown".to_string())
    .link_preview_options(link_preview_options)
    // You must use this send() method in order to send the request to the API
    .send()
    .await?;

    // GroupIteration::EndGroups will end iteration of groups for an update.
    // This means that rest of the pending groups and their handlers won't be checked
    // for this particular update.
    Ok(GroupIteration::EndGroups)
}

// This is our callable function for our message handler which will be used to
// repeat the text.
async fn echo(bot: Arc<Bot>, ctx: Context) -> Result<GroupIteration> {
    // Command Handler recieves message updates which have chat as a compulsory field.
    // Hence we can unwrap effective chat without checking if it is none.
    let chat = ctx.effective_chat.unwrap();
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    // Every api method creates a builder which contains various parameters of that respective method.
    bot.copy_message(chat.id, chat.id, msg.message_id)
        // You must use this send() method in order to send the request to the API
        .send()
        .await?;

    // GroupIteration::EndGroups will end iteration of groups for an update.
    // This means that rest of the pending groups and their handlers won't be checked
    // for this particular update.
    Ok(GroupIteration::EndGroups)
}
