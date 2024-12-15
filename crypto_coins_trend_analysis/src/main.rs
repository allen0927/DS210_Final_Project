mod computation_analysis {
    pub mod computation_algorithms;
}

mod data_cleaning_load {
    pub mod data_loader;
}

mod utility {
    pub mod helper_algorithm;
}

mod connected_component {
    pub mod find_connected_component;
}

mod tests {
    pub mod utility_tests;
}

use computation_analysis::computation_algorithms;
use data_cleaning_load::data_loader;
use utility::helper_algorithm;
use connected_component::find_connected_component;
use tests::utility_tests;

/**************************************************************
*
*   Execution level of the project, used all the wrapper functions
*   from other modules to generate analysis result
*
***************************************************************/


fn main() {
    let path: &str = "../selected_rows.csv";
    let graphs = data_loader::load_csv_convert_graph(path);
    println!("finished loading graph......");
    match graphs {
        Ok((graph_prior_crash, graph_during_crash, graph_after_crash)) => {
            data_loader::display_graph(&graph_prior_crash, "Transaction Graph Before Crash");
            data_loader::display_graph(&graph_during_crash, "Transaction Graph During Crash");
            data_loader::display_graph(&graph_after_crash, "Transaction Graph After Crash");

            computation_algorithms::analyze_graphs(&graph_prior_crash, &graph_during_crash, &graph_after_crash);
            computation_algorithms::analyze_centrality_across_periods(&graph_prior_crash, &graph_during_crash, &graph_after_crash);
            find_connected_component::analyze_largest_components(&graph_prior_crash, &graph_during_crash, &graph_after_crash);
        },
        Err(e) => eprintln!("Error loading graph: {}", e),
    }
}
