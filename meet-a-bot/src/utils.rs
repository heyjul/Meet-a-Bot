use teams_api::models::Activity;

use crate::commands::Commands;

pub fn parse_command(activity: &Activity) -> Option<Commands> {
    match activity.recipient.as_ref().and_then(|x| x.name.as_deref()) {
        Some(name) => activity
            .text
            .as_ref()
            .and_then(|text| {
                text.strip_prefix(&format!("<at>{name}</at>"))
                    .or(Some(text))
            })
            .and_then(|text| text.split_ascii_whitespace().next())
            .and_then(|command| Commands::try_from(command).ok()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use teams_api::models::{Activity, ChannelAccount};

    #[rstest]
    #[case(None, None, None)]
    #[case(None, Some("help"), None)]
    #[case(Some("Foo"), None, None)]
    #[case(Some("Foo"), Some("<at>Foo</at>"), None)]
    #[case(Some("Foo"), Some("feedback"), Some(Commands::Feedback))]
    #[case(Some("Foo"), Some("<at>Foo</at>feedback"), Some(Commands::Feedback))]
    #[case(Some("Foo"), Some("<at>Foo</at> feedback"), Some(Commands::Feedback))]
    #[case(
        Some("Foo"),
        Some("<at>Foo</at> feedback baz"),
        Some(Commands::Feedback)
    )]
    fn test_parse_command(
        #[case] name: Option<&str>,
        #[case] text: Option<&str>,
        #[case] expected: Option<Commands>,
    ) {
        // Arrange
        let activity = Activity {
            recipient: name.map(|name| ChannelAccount {
                name: Some(name.to_owned()),
                ..Default::default()
            }),
            text: text.map(|x| x.to_owned()),
            ..Default::default()
        };

        // Act
        let result = parse_command(&activity);

        // Assert
        assert_eq!(expected, result);
    }
}
