use crate::{domain::reading_therapy_config::ReadingTherapyConfig, ports::ConfigStorage};

pub async fn get_reading_therapy(
    storage: &dyn ConfigStorage<ReadingTherapyConfig>,
) -> ReadingTherapyConfig {
    storage.load().await.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Layout;
    use crate::use_cases::mocks::MockReadingConfigStorage;

    #[tokio::test]
    async fn test_get_reading_therapy_returns_default_on_error() {
        let storage = MockReadingConfigStorage {
            should_fail_load: true,
            ..Default::default()
        };
        let config = get_reading_therapy(&storage).await;
        assert_eq!(config, ReadingTherapyConfig::default());
    }

    #[tokio::test]
    async fn test_get_reading_therapy_returns_stored() {
        let mut expected = ReadingTherapyConfig::default();
        expected.change_layout(Layout::Vertical);

        let storage = MockReadingConfigStorage {
            config: Some(expected.clone()),
            ..Default::default()
        };
        let config = get_reading_therapy(&storage).await;
        assert_eq!(config, expected);
    }
}
