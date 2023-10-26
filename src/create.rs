use crate::{actor, link, network, scenario, session, task, token};

pub fn get_create_statements() -> Vec<String> {
    vec![
        session::get_create_statement(),
        actor::get_create_statement(),
        network::get_create_statement(),
        scenario::get_create_statement(),
        token::get_create_statement(),
        task::get_create_statement(),
        link::get_create_statement(),
    ]
}
