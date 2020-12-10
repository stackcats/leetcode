impl Solution {
    pub fn interpret(mut command: String) -> String {
        command.replace("(al)", "al").replace("()", "o")
    }
}
