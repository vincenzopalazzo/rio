pub struct TriageConf {
    pub team: String,
    pub git: GitConf,
}

pub struct GitConf {
    pub owner: String,
    pub repo: String,
    pub labels: Vec<String>,
    pub since: String,
}
