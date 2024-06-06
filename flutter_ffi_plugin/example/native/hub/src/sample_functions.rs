use crate::messages;
use rinf::debug_print;
pub async fn calculate_precious_data() {
    use messages::counter_number::*;

    MyPreciousData {
        input_string: 4450.to_string(),
    }
    .send_signal_to_dart();

    let mut receiver = MyPreciousData1::get_dart_signal_receiver();
    while let Some(dart_signal) = receiver.recv().await {
        let my_precious_data = dart_signal.message;

        let new_numbers: Vec<i32> = my_precious_data
            .input_numbers
            .into_iter()
            .map(|x| x + 1)
            .collect();

        let new_string = my_precious_data.input_string.to_uppercase();

        debug_print!("{new_numbers:?}");
        debug_print!("{new_string}");
    }
}
