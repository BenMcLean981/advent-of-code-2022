pub(crate) struct Cd {
    pub arg: String,
}

impl From<Vec<String>> for Cd {
    fn from(command: Vec<String>) -> Self {
        let arg = command[0].split(" ").collect::<Vec<_>>()[2];
        return Cd {
            arg: arg.to_string(),
        };
    }
}
