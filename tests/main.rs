use parse_wiki_text::Configuration;
#[test]
fn test() {
  let result = Configuration::default().parse(TRICKY_WIKITEXT);
}

// this resulted in a seemingly infinite loop, from the Kid Chocolate wiki page
const TRICKY_WIKITEXT: &str = r###"
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
{{x
"###;
