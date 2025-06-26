use super::LogHistory;

pub fn get_action<'a>(insertion_list: &'a [LogHistory], creation_list: &'a [LogHistory]) -> &'a LogHistory {
    let max_insertion_action = insertion_list
        .iter()
        .max_by_key(|log_history| log_history.deviation)
        .expect("Should have found maximum deviation in list of insertion deviations");

    let best_action = creation_list
        .iter()
        .chain(insertion_list.iter())
        .max_by_key(|log_history| log_history.deviation)
        .expect("Should have found maximum deviation in list of deviations");


    if max_insertion_action.deviation >= 0 {
        max_insertion_action
    } else {
        best_action
    }
}
