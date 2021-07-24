#![allow(non_snake_case)]
#![allow(clippy::new_without_default)]

pub mod config_path;
pub mod gui;

mod application_window;
mod channel;
mod choose_folder_button;
mod choose_output_folder_button;
mod choose_repository_folder_button;
mod commit_author_filter_entry;
mod commit_diff;
mod commit_diff_view;
mod commit_log;
mod commit_log_column;
mod commit_log_model;
mod commit_log_model_filter;
mod commit_log_view;
mod config_store;
mod date_time;
mod diff_colorizer;
mod diff_formatter;
mod dispatcher;
mod event;
mod event_handling;
mod generate_report_button;
mod gui_element_provider;
mod line_number;
mod month_filter_combo_box;
mod open_options_button;
mod options_dialog;
mod output_path_label;
mod output_path_store;
mod pane_with_commit_log_and_diff;
mod report_generator;
mod repository;
mod repository_path_label;
mod repository_store;
mod source;
mod text_view;
mod tree_view;
mod tree_view_column_config;
mod year_filter_spin_button;
