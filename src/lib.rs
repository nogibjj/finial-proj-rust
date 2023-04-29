/*A libaray for translating a given text

Formula for using Github copilot:
1. Write a comment that describes what you want to do with a function
2. Format code with cargo fmt
3. Lint your code with cargo clippy

*/
// use std::sync::Once;
use std::thread;

use exitfailure::ExitFailure;
use rust_bert::pipelines::translation::{
    Language, TranslationModel, TranslationModelBuilder,
};

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

// static mut TRANSLATION_MODEL: Option<TranslationModel> = None;

// static INIT_MODEL: Once = Once::new();

//build a function that translates a given text
pub fn init_translation_model() -> TranslationModel {
    let do_steps = move || -> Result<TranslationModel, ExitFailure> {
        let model = TranslationModelBuilder::new()
            .with_source_languages(vec![Language::English])
            .with_target_languages(vec![Language::Spanish])
            .create_model()?;

        // print_type_of(&model);/
        Ok(model)
    };

    let translation_model = thread::spawn(move || {
        do_steps().unwrap()
    }).join().expect("Thread panicked");
    return translation_model;
}



// pub fn translate_text(text: Vec<String>) -> anyhow::Result<(Vec<String>)> {
    
//     let model = TranslationModelBuilder::new()
//         .with_source_languages(vec![Language::English])
//         .with_target_languages(vec![Language::Spanish])
//         .create_model()?;

//     let mut answer = Vec::<String>::new();

//     for sentence in text{
//         //pass in the text to the model
//         let output = model.translate(&[sentence], None, Language::Spanish)?;
//         let mut counter = 0;
//         for result in output {
//             let string = String::from(result.clone());
//             answer.push(string);
//             // print_type_of(&(result.to_string()));
//             // println!("{}", result);
//         }
//     }

//     // println!("In answer: ");
//     // for i in answer{
//     //     println!("{}", i);
//     // }

//     Ok((answer))
// }

// pub fn dialogue(text: String) -> String {
//     let conversation_model = ConversationModel::new(Default::default()).unwrap();
//     let mut conversation_manager = ConversationManager::new();

//     let conversation_id = conversation_manager.create(&text);
//     let output = conversation_model.generate_responses(&mut conversation_manager);
//     let response = output.get(&conversation_id).unwrap();
//     response.to_string()
// }