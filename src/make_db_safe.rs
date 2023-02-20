
//a function to replace special characters with emoji so they do not cause breakages during parsing.
pub fn encode(text: String) -> String {

    text.replace(",","🌂")


}

pub fn decode(text: String) -> String {

    text.replace("🌂",",")

}
