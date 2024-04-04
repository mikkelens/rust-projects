use ical::generator::IcalCalendar;
use ical::property::Property;
use serde::{Deserialize, Serialize};

mod filtering;
use filtering::FilterBuild;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ComparableCalendar {
    source: IcalCalendar,
    pub(crate) filtered: String,
}
impl ComparableCalendar {
    fn invalidation_relevant_properties(&self) -> impl Iterator<Item = &Property> {
        // this seems to do what I want
        self.flattened_properties().filter(|&property| {
            !matches!(
                property.name.as_str(),
                "DTSTAMP" | "CREATED" | "LAST-MODIFIED"
            )
        })
    }
    fn flattened_properties(&self) -> impl Iterator<Item = &Property> {
        self.source
            .properties
            .iter()
            .chain(self.source.events.iter().flat_map(|event| {
                event.properties.iter().chain(
                    event
                        .alarms
                        .iter()
                        .flat_map(|alarm| alarm.properties.iter()),
                )
            }))
            .chain(self.source.todos.iter().flat_map(|todo| {
                todo.properties
                    .iter()
                    .chain(todo.alarms.iter().flat_map(|alarm| alarm.properties.iter()))
            }))
            .chain(
                self.source
                    .alarms
                    .iter()
                    .flat_map(|alarm| alarm.properties.iter()),
            )
            .chain(
                self.source
                    .journals
                    .iter()
                    .flat_map(|journal| journal.properties.iter()),
            )
    }
}

impl PartialEq for ComparableCalendar {
    fn eq(&self, other: &Self) -> bool {
        self.invalidation_relevant_properties()
            .eq(other.invalidation_relevant_properties())
    }
}
impl From<IcalCalendar> for ComparableCalendar {
    fn from(source: IcalCalendar) -> Self {
        Self {
            filtered: source.build(),
            source,
        }
    }
}
