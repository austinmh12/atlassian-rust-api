/// Enables the ability to manage nodes in a cluster. These are not very well documented
/// on the Data Center docs [https://docs.atlassian.com/software/jira/docs/api/REST/9.17.0/]
mod get_all_nodes;
mod get_current_index;
mod delete_node;
mod set_node_offline;
mod zdu;

// TODO: When checking Jira Cloud vs Jira DC add feature for each and make this only available in Jira DC