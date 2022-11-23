pub struct Rules {}

impl super::SeccompCtxRules for Rules {
    fn get_black_list(&self) -> Vec<&'static str> {
        let black_list = vec!["socket", "fork", "vfork", "kill"];

        black_list
    }
}
