use super::super::common;
use hydrus_api::wrapper::builders::or_chain_builder::OrChainBuilder;
use hydrus_api::wrapper::builders::tag_builder::TagBuilder;
use hydrus_api::wrapper::or_chain::OrChain;

#[test]
fn it_parses_from_string() {
    common::setup();
    let chain_string =
        "'character:megumin' or 'character:aqua' OR '-character:hatsune miku'or 'terminator'";
    let chain = OrChain::from(chain_string);
    assert_eq!(
        chain,
        OrChainBuilder::new()
            .add_tag("character:megumin".into())
            .add_tag("character:aqua".into())
            .add_tag(
                TagBuilder::new("hatsune miku")
                    .namespace("character")
                    .negate()
                    .build()
            )
            .add_tag("terminator".into())
            .build()
    );
}
