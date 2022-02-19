// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};


/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
/// - PassportElementErrorDataField
/// - PassportElementErrorFrontSide
/// - PassportElementErrorReverseSide
/// - PassportElementErrorSelfie
/// - PassportElementErrorFile
/// - PassportElementErrorFiles
/// - PassportElementErrorTranslationFile
/// - PassportElementErrorTranslationFiles
/// - PassportElementErrorUnspecified
/// <https://core.telegram.org/bots/api#passportelementerror>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassportElementError {
}