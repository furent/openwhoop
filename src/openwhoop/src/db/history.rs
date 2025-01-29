use chrono::NaiveDateTime;
use db_entities::heart_rate;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QuerySelect};
use whoop::{Activity, ParsedHistoryReading};

use super::DatabaseHandler;

pub struct SearchHistory {
    pub from: Option<NaiveDateTime>,
}

impl SearchHistory {
    pub(crate) fn conditions(self) -> Condition {
        Condition::all().add_option(self.from.map(|from| heart_rate::Column::Time.gt(from)))
    }
}

impl DatabaseHandler {
    pub async fn search_history(
        &self,
        options: SearchHistory,
    ) -> anyhow::Result<Vec<ParsedHistoryReading>> {
        let history = heart_rate::Entity::find()
            .filter(options.conditions())
            .filter(heart_rate::Column::Activity.is_not_null())
            .limit(100000)
            .all(&self.db)
            .await?
            .into_iter()
            .map(Self::parse_reading)
            .collect();

        Ok(history)
    }

    fn parse_reading(model: heart_rate::Model) -> ParsedHistoryReading {
        ParsedHistoryReading {
            time: model.time,
            bpm: model.bpm.try_into().unwrap_or(u8::MAX),
            rr: model
                .rr_intervals
                .split(',')
                .map(|rr| rr.parse().unwrap_or_default())
                .collect(),
            activity: model.activity.map(Activity::from).unwrap(),
        }
    }
}
