use skyscraper::{
    html::HtmlDocument,
    xpath::{self, XpathItemTree},
};

pub struct DetailsPage {
    pub doc: HtmlDocument,
}

impl DetailsPage {
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let xpath_item_tree = XpathItemTree::from(&self.doc);
        let name_xpath = xpath::parse(r#"//*[@id="logoutForm"]/ul/li[1]/a/span[1]"#)?;
        let item_set = name_xpath.apply(&xpath_item_tree)?;

        if item_set.len() == 0 {
            // return error if can't find name element
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to get name element",
            )));
        }

        let name = &item_set[0];
        let name = name.extract_as_node().extract_as_tree_node();
        let name = name.text(&xpath_item_tree);

        Ok(String::from(name))
    }
}
