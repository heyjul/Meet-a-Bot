use crate::{commands::Commands, models::activity::Activity};

pub fn parse_command(activity: &Activity) -> Option<Commands> {
    match activity.recipient.name {
        Some(ref name) => activity
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
    use crate::models::channel_account::ChannelAccount;
    use rstest::rstest;

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
            recipient: name.map_or(ChannelAccount::default(), |name| ChannelAccount {
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
